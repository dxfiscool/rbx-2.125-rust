//! core shard mi — 120 core stubs EA-sorted asc global gap filler not yet in any crate.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 not yet in any crate (0x47e5f8..0x4bb734, 39343 uncovered before batch, scan 120 gaps).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getSize(void) [0x47e5f8]")]
// 0x47e5f8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE7getSizeEv — RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getSize(void)
// type: int(void)
pub fn stub_0x47e5f8() -> ! { todo!("0x47e5f8 __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE7getSizeEv") }

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void) [0x483eb0]")]
// 0x483eb0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv — RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)
// type: 
pub fn stub_0x483eb0() -> ! { todo!("0x483eb0 __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv") }

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void) [0x483eb4]")]
// 0x483eb4 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv — RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// type: 
pub fn stub_0x483eb4() -> ! { todo!("0x483eb4 __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv") }

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::StaticData::~StaticData() [0x483fc4]")]
// 0x483fc4 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10StaticDataD1Ev — RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::StaticData::~StaticData()
// type: 
pub fn stub_0x483fc4() -> ! { todo!("0x483fc4 __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10StaticDataD1Ev") }

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>> *) [0x484074]")]
// 0x484074 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E — std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>> *)
// type: int(void)
pub fn stub_0x484074() -> ! { todo!("0x484074 __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E") }

#[doc(alias = "G3D::Vector3 const& rbx::any_cast<G3D::Vector3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4a62d8]")]
// 0x4a62d8 — __ZN3rbx8any_castIRKN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE — G3D::Vector3 const& rbx::any_cast<G3D::Vector3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int __fastcall(_DWORD)
pub fn stub_0x4a62d8() -> ! { todo!("0x4a62d8 __ZN3rbx8any_castIRKN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_initialize_map(unsigned long) [0x4ace68]")]
// 0x4ace68 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_initialize_mapEm — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_initialize_map(unsigned long)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_0x4ace68() -> ! { todo!("0x4ace68 __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_initialize_mapEm") }

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_allocate_map(unsigned long) [0x4acfe4]")]
// 0x4acfe4 — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_allocate_mapEm — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_allocate_map(unsigned long)
// type: 
pub fn stub_0x4acfe4() -> ! { todo!("0x4acfe4 __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_allocate_mapEm") }

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_create_nodes(RBX::BindableFunction::Invocation**,RBX::BindableFunction::Invocation**) [0x4acffc]")]
// 0x4acffc — __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_create_nodesEPPS2_S6_ — std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_create_nodes(RBX::BindableFunction::Invocation**,RBX::BindableFunction::Invocation**)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x4acffc() -> ! { todo!("0x4acffc __ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_create_nodesEPPS2_S6_") }

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::deque(std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>> const&) [0x4ad0f0]")]
// 0x4ad0f0 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EEC2ERKS4_ — std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::deque(std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>> const&)
// type: 
pub fn stub_0x4ad0f0() -> ! { todo!("0x4ad0f0 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EEC2ERKS4_") }

#[doc(alias = "std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>>(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::__false_type) [0x4ad224]")]
// 0x4ad224 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX16BindableFunction10InvocationERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type — std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>>(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::__false_type)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
pub fn stub_0x4ad224() -> ! { todo!("0x4ad224 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX16BindableFunction10InvocationERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type") }

#[doc(alias = "RBX::CustomEvent::CustomEvent(void) [0x4b0954]")]
// 0x4b0954 — __ZN3RBX11CustomEventC2Ev — RBX::CustomEvent::CustomEvent(void)
// type: _DWORD __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0954() -> ! { todo!("0x4b0954 __ZN3RBX11CustomEventC2Ev") }

#[doc(alias = "RBX::CustomEvent::~CustomEvent() [0x4b0b98]")]
// 0x4b0b98 — __ZN3RBX11CustomEventD1Ev — RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0b98() -> ! { todo!("0x4b0b98 __ZN3RBX11CustomEventD1Ev") }

#[doc(alias = "RBX::CustomEvent::~CustomEvent() [0x4b0b9c]")]
// 0x4b0b9c — __ZN3RBX11CustomEventD0Ev — RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0b9c() -> ! { todo!("0x4b0b9c __ZN3RBX11CustomEventD0Ev") }

#[doc(alias = "RBX::CustomEvent::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *) [0x4b0c3c]")]
// 0x4b0c3c — __ZN3RBX11CustomEvent17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::CustomEvent::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::CustomEvent *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x4b0c3c() -> ! { todo!("0x4b0c3c __ZN3RBX11CustomEvent17onServiceProviderEPNS_15ServiceProviderES2_") }

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent() [0x4b0ea4]")]
// 0x4b0ea4 — __ZThn32_N3RBX11CustomEventD1Ev — `non-virtual thunk to'RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0ea4() -> ! { todo!("0x4b0ea4 __ZThn32_N3RBX11CustomEventD1Ev") }

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent() [0x4b0eac]")]
// 0x4b0eac — __ZThn32_N3RBX11CustomEventD0Ev — `non-virtual thunk to'RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0eac() -> ! { todo!("0x4b0eac __ZThn32_N3RBX11CustomEventD0Ev") }

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent() [0x4b0f60]")]
// 0x4b0f60 — __ZThn36_N3RBX11CustomEventD1Ev — `non-virtual thunk to'RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0f60() -> ! { todo!("0x4b0f60 __ZThn36_N3RBX11CustomEventD1Ev") }

#[doc(alias = "non-virtual thunk toRBX::CustomEvent::~CustomEvent() [0x4b0f68]")]
// 0x4b0f68 — __ZThn36_N3RBX11CustomEventD0Ev — `non-virtual thunk to'RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b0f68() -> ! { todo!("0x4b0f68 __ZThn36_N3RBX11CustomEventD0Ev") }

#[doc(alias = "RBX::CustomEvent::~CustomEvent() [0x4b100c]")]
// 0x4b100c — __ZN3RBX11CustomEventD2Ev — RBX::CustomEvent::~CustomEvent()
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0x4b100c() -> ! { todo!("0x4b100c __ZN3RBX11CustomEventD2Ev") }

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*) [0x4b1244]")]
// 0x4b1244 — __ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_ — boost::shared_ptr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*)
// type: 
pub fn stub_0x4b1244() -> ! { todo!("0x4b1244 __ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_") }

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag) [0x4b13b4]")]
// 0x4b13b4 — __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(boost::weak_ptr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag)
// type: 
pub fn stub_0x4b13b4() -> ! { todo!("0x4b13b4 __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE") }

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::list(std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>> const&) [0x4b1430]")]
// 0x4b1430 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EEC2ERKS6_ — std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::list(std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>> const&)
// type: 
pub fn stub_0x4b1430() -> ! { todo!("0x4b1430 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EEC2ERKS6_") }

#[doc(alias = "void std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::__false_type) [0x4b14f8]")]
// 0x4b14f8 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type — void std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::__false_type)
// type: int __fastcall(int)
pub fn stub_0x4b14f8() -> ! { todo!("0x4b14f8 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type") }

#[doc(alias = "std::_List_base<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_clear(void) [0x4b151c]")]
// 0x4b151c — __ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv — std::_List_base<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_clear(void)
// type: 
pub fn stub_0x4b151c() -> ! { todo!("0x4b151c __ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv") }

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_create_node(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&) [0x4b1544]")]
// 0x4b1544 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_ — std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_create_node(boost::weak_ptr<RBX::CustomEventReceiver> const&)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0x4b1544() -> ! { todo!("0x4b1544 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_") }

#[doc(alias = "RBX::CustomEventReceiver::CustomEventReceiver(void) [0x4b22d0]")]
// 0x4b22d0 — __ZN3RBX19CustomEventReceiverC2Ev — RBX::CustomEventReceiver::CustomEventReceiver(void)
// type: _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this)
pub fn stub_0x4b22d0() -> ! { todo!("0x4b22d0 __ZN3RBX19CustomEventReceiverC2Ev") }

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&) [0x4b2680]")]
// 0x4b2680 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)
// type: 
pub fn stub_0x4b2680() -> ! { todo!("0x4b2680 __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_") }

#[doc(alias = "RBX::CustomEventReceiver::setCurrentValue(float) [0x4b26f4]")]
// 0x4b26f4 — __ZN3RBX19CustomEventReceiver15setCurrentValueEf — RBX::CustomEventReceiver::setCurrentValue(float)
// type: _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this, float)
pub fn stub_0x4b26f4() -> ! { todo!("0x4b26f4 __ZN3RBX19CustomEventReceiver15setCurrentValueEf") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::disconnectAll(void) [0x4b283c]")]
// 0x4b283c — __ZN3rbx7signals6signalIFvfEE13disconnectAllEv — rbx::signals::signal<void ()(float)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x4b283c() -> ! { todo!("0x4b283c __ZN3rbx7signals6signalIFvfEE13disconnectAllEv") }

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot> const&) [0x4b29b4]")]
// 0x4b29b4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_ — boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> const&)
// type: int *__fastcall(int *, int *)
pub fn stub_0x4b29b4() -> ! { todo!("0x4b29b4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::safe_static_init_mutex(void) [0x4b29d8]")]
// 0x4b29d8 — __ZN3rbx7signals6signalIFvfEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(float)>::safe_static_init_mutex(void)
// type: 
pub fn stub_0x4b29d8() -> ! { todo!("0x4b29d8 __ZN3rbx7signals6signalIFvfEE22safe_static_init_mutexEv") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::safe_static_do_get_mutex(void) [0x4b29dc]")]
// 0x4b29dc — __ZN3rbx7signals6signalIFvfEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(float)>::safe_static_do_get_mutex(void)
// type: 
pub fn stub_0x4b29dc() -> ! { todo!("0x4b29dc __ZN3rbx7signals6signalIFvfEE24safe_static_do_get_mutexEv") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::insert(rbx::signals::signal<void ()(float)>::slot *) [0x4b2ad4]")]
// 0x4b2ad4 — __ZN3rbx7signals6signalIFvfEE6insertEPNS3_4slotE — rbx::signals::signal<void ()(float)>::insert(rbx::signals::signal<void ()(float)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x4b2ad4() -> ! { todo!("0x4b2ad4 __ZN3rbx7signals6signalIFvfEE6insertEPNS3_4slotE") }

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*) [0x4b2ce0]")]
// 0x4b2ce0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_ — boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)
// type: 
pub fn stub_0x4b2ce0() -> ! { todo!("0x4b2ce0 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot() [0x4b2d04]")]
// 0x4b2d04 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()
// type: 
pub fn stub_0x4b2d04() -> ! { todo!("0x4b2d04 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot() [0x4b2d30]")]
// 0x4b2d30 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()
// type: 
pub fn stub_0x4b2d30() -> ! { todo!("0x4b2d30 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::disconnect(void) [0x4b2e04]")]
// 0x4b2e04 — __ZN3rbx7signals6signalIFvfEE4slot10disconnectEv — rbx::signals::signal<void ()(float)>::slot::disconnect(void)
// type: 
pub fn stub_0x4b2e04() -> ! { todo!("0x4b2e04 __ZN3rbx7signals6signalIFvfEE4slot10disconnectEv") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::connected(void)const [0x4b2f14]")]
// 0x4b2f14 — __ZNK3rbx7signals6signalIFvfEE4slot9connectedEv — rbx::signals::signal<void ()(float)>::slot::connected(void)const
// type: 
pub fn stub_0x4b2f14() -> ! { todo!("0x4b2f14 __ZNK3rbx7signals6signalIFvfEE4slot9connectedEv") }

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float) [0x4b2f20]")]
// 0x4b2f20 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)
// type: 
pub fn stub_0x4b2f20() -> ! { todo!("0x4b2f20 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf") }

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float) [0x4b2f34]")]
// 0x4b2f34 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf — `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)
// type: 
pub fn stub_0x4b2f34() -> ! { todo!("0x4b2f34 __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf") }

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &) [0x4b2f48]")]
// 0x4b2f48 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_ — void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)
// type: 
pub fn stub_0x4b2f48() -> ! { todo!("0x4b2f48 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::remove(rbx::signals::signal<void ()(float)>::slot *) [0x4b2f68]")]
// 0x4b2f68 — __ZN3rbx7signals6signalIFvfEE6removeEPNS3_4slotE — rbx::signals::signal<void ()(float)>::remove(rbx::signals::signal<void ()(float)>::slot *)
// type: int __fastcall(int, char *)
pub fn stub_0x4b2f68() -> ! { todo!("0x4b2f68 __ZN3rbx7signals6signalIFvfEE6removeEPNS3_4slotE") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::safe_static_init_mutex(void) [0x4b3058]")]
// 0x4b3058 — __ZN3rbx7signals6signalIFvfEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(float)>::slot::safe_static_init_mutex(void)
// type: 
pub fn stub_0x4b3058() -> ! { todo!("0x4b3058 __ZN3rbx7signals6signalIFvfEE4slot22safe_static_init_mutexEv") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::safe_static_do_get_mutex(void) [0x4b305c]")]
// 0x4b305c — __ZN3rbx7signals6signalIFvfEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(float)>::slot::safe_static_do_get_mutex(void)
// type: 
pub fn stub_0x4b305c() -> ! { todo!("0x4b305c __ZN3rbx7signals6signalIFvfEE4slot24safe_static_do_get_mutexEv") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::~slot() [0x4b314c]")]
// 0x4b314c — __ZN3rbx7signals6signalIFvfEE4slotD1Ev — rbx::signals::signal<void ()(float)>::slot::~slot()
// type: int __fastcall(int)
pub fn stub_0x4b314c() -> ! { todo!("0x4b314c __ZN3rbx7signals6signalIFvfEE4slotD1Ev") }

#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::~slot() [0x4b3178]")]
// 0x4b3178 — __ZN3rbx7signals6signalIFvfEE4slotD0Ev — rbx::signals::signal<void ()(float)>::slot::~slot()
// type: 
pub fn stub_0x4b3178() -> ! { todo!("0x4b3178 __ZN3rbx7signals6signalIFvfEE4slotD0Ev") }

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable() [0x4b324c]")]
// 0x4b324c — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()
// type: 
pub fn stub_0x4b324c() -> ! { todo!("0x4b324c __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev") }

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable() [0x4b3278]")]
// 0x4b3278 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()
// type: 
pub fn stub_0x4b3278() -> ! { todo!("0x4b3278 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *) [0x4b40c4]")]
// 0x4b40c4 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b40c4() -> ! { todo!("0x4b40c4 __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&) [0x4b4898]")]
// 0x4b4898 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)
// type: 
pub fn stub_0x4b4898() -> ! { todo!("0x4b4898 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void) [0x4b48e8]")]
// 0x4b48e8 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void)
// type: 
pub fn stub_0x4b48e8() -> ! { todo!("0x4b48e8 __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *) [0x4b4954]")]
// 0x4b4954 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b4954() -> ! { todo!("0x4b4954 __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *) [0x4b4960]")]
// 0x4b4960 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *)
// type: 
pub fn stub_0x4b4960() -> ! { todo!("0x4b4960 __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc") }

#[doc(alias = "RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b4a30]")]
// 0x4b4a30 — __ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b4a30() -> ! { todo!("0x4b4a30 __ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *) [0x4b4b9c]")]
// 0x4b4b9c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)
// type: 
pub fn stub_0x4b4b9c() -> ! { todo!("0x4b4b9c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&) [0x4b52c0]")]
// 0x4b52c0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)
// type: 
pub fn stub_0x4b52c0() -> ! { todo!("0x4b52c0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void) [0x4b5310]")]
// 0x4b5310 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)
// type: 
pub fn stub_0x4b5310() -> ! { todo!("0x4b5310 __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *) [0x4b537c]")]
// 0x4b537c — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b537c() -> ! { todo!("0x4b537c __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *) [0x4b5388]")]
// 0x4b5388 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *)
// type: void()
pub fn stub_0x4b5388() -> ! { todo!("0x4b5388 __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc") }

#[doc(alias = "RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b5458]")]
// 0x4b5458 — __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b5458() -> ! { todo!("0x4b5458 __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *) [0x4b55c4]")]
// 0x4b55c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)
// type: 
pub fn stub_0x4b55c4() -> ! { todo!("0x4b55c4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&) [0x4b5ce8]")]
// 0x4b5ce8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4b5ce8() -> ! { todo!("0x4b5ce8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void) [0x4b5d38]")]
// 0x4b5d38 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)
// type: 
pub fn stub_0x4b5d38() -> ! { todo!("0x4b5d38 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *) [0x4b5da4]")]
// 0x4b5da4 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b5da4() -> ! { todo!("0x4b5da4 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *) [0x4b5db0]")]
// 0x4b5db0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)
// type: 
pub fn stub_0x4b5db0() -> ! { todo!("0x4b5db0 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc") }

#[doc(alias = "RBX::InputObject::UserInputState const& rbx::any_cast<RBX::InputObject::UserInputState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b5e80]")]
// 0x4b5e80 — __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::InputObject::UserInputState const& rbx::any_cast<RBX::InputObject::UserInputState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
pub fn stub_0x4b5e80() -> ! { todo!("0x4b5e80 __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *) [0x4b5fec]")]
// 0x4b5fec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *)
// type: 
pub fn stub_0x4b5fec() -> ! { todo!("0x4b5fec __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&) [0x4b6710]")]
// 0x4b6710 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&)
// type: 
pub fn stub_0x4b6710() -> ! { todo!("0x4b6710 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void) [0x4b6760]")]
// 0x4b6760 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void)
// type: 
pub fn stub_0x4b6760() -> ! { todo!("0x4b6760 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::construct_func(char const*,char *) [0x4b67cc]")]
// 0x4b67cc — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b67cc() -> ! { todo!("0x4b67cc __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::destruct_func(char *) [0x4b67d8]")]
// 0x4b67d8 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::destruct_func(char *)
// type: 
pub fn stub_0x4b67d8() -> ! { todo!("0x4b67d8 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc") }

#[doc(alias = "RBX::InputObject::UserInputType const& rbx::any_cast<RBX::InputObject::UserInputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b68a8]")]
// 0x4b68a8 — __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::InputObject::UserInputType const& rbx::any_cast<RBX::InputObject::UserInputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b68a8() -> ! { todo!("0x4b68a8 __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *) [0x4b6a14]")]
// 0x4b6a14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *)
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4b6a14() -> ! { todo!("0x4b6a14 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&) [0x4b6e18]")]
// 0x4b6e18 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)
// type: 
pub fn stub_0x4b6e18() -> ! { todo!("0x4b6e18 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void) [0x4b6e68]")]
// 0x4b6e68 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void)
// type: 
pub fn stub_0x4b6e68() -> ! { todo!("0x4b6e68 __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::destruct_func(char *) [0x4b6ed4]")]
// 0x4b6ed4 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::destruct_func(char *)
// type: 
pub fn stub_0x4b6ed4() -> ! { todo!("0x4b6ed4 __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc") }

#[doc(alias = "RBX::Explosion::ExplosionType const& rbx::any_cast<RBX::Explosion::ExplosionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b6ed8]")]
// 0x4b6ed8 — __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Explosion::ExplosionType const& rbx::any_cast<RBX::Explosion::ExplosionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b6ed8() -> ! { todo!("0x4b6ed8 __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&) [0x4b7740]")]
// 0x4b7740 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)
// type: 
pub fn stub_0x4b7740() -> ! { todo!("0x4b7740 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void) [0x4b7790]")]
// 0x4b7790 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void)
// type: 
pub fn stub_0x4b7790() -> ! { todo!("0x4b7790 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::construct_func(char const*,char *) [0x4b77fc]")]
// 0x4b77fc — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b77fc() -> ! { todo!("0x4b77fc __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::destruct_func(char *) [0x4b7808]")]
// 0x4b7808 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::destruct_func(char *)
// type: 
pub fn stub_0x4b7808() -> ! { todo!("0x4b7808 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc") }

#[doc(alias = "RBX::Voxel::WaterCellDirection const& rbx::any_cast<RBX::Voxel::WaterCellDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b78d8]")]
// 0x4b78d8 — __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::WaterCellDirection const& rbx::any_cast<RBX::Voxel::WaterCellDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b78d8() -> ! { todo!("0x4b78d8 __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *) [0x4b7a44]")]
// 0x4b7a44 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *)
// type: 
pub fn stub_0x4b7a44() -> ! { todo!("0x4b7a44 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&) [0x4b8168]")]
// 0x4b8168 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4b8168() -> ! { todo!("0x4b8168 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void) [0x4b81b8]")]
// 0x4b81b8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void)
// type: 
pub fn stub_0x4b81b8() -> ! { todo!("0x4b81b8 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::construct_func(char const*,char *) [0x4b8224]")]
// 0x4b8224 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b8224() -> ! { todo!("0x4b8224 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::destruct_func(char *) [0x4b8230]")]
// 0x4b8230 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::destruct_func(char *)
// type: void()
pub fn stub_0x4b8230() -> ! { todo!("0x4b8230 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc") }

#[doc(alias = "RBX::Voxel::WaterCellForce const& rbx::any_cast<RBX::Voxel::WaterCellForce const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b8300]")]
// 0x4b8300 — __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::WaterCellForce const& rbx::any_cast<RBX::Voxel::WaterCellForce const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b8300() -> ! { todo!("0x4b8300 __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *) [0x4b846c]")]
// 0x4b846c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *)
// type: 
pub fn stub_0x4b846c() -> ! { todo!("0x4b846c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&) [0x4b8b90]")]
// 0x4b8b90 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)
// type: 
pub fn stub_0x4b8b90() -> ! { todo!("0x4b8b90 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void) [0x4b8be0]")]
// 0x4b8be0 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)
// type: 
pub fn stub_0x4b8be0() -> ! { todo!("0x4b8be0 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *) [0x4b8c4c]")]
// 0x4b8c4c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4b8c4c() -> ! { todo!("0x4b8c4c __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *) [0x4b8c58]")]
// 0x4b8c58 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)
// type: 
pub fn stub_0x4b8c58() -> ! { todo!("0x4b8c58 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE13destruct_funcEPc") }

#[doc(alias = "RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b8d28]")]
// 0x4b8d28 — __ZN3rbx8any_castIRKN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b8d28() -> ! { todo!("0x4b8d28 __ZN3rbx8any_castIRKN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *) [0x4b8e94]")]
// 0x4b8e94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4b8e94() -> ! { todo!("0x4b8e94 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&) [0x4b95b8]")]
// 0x4b95b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)
// type: 
pub fn stub_0x4b95b8() -> ! { todo!("0x4b95b8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void) [0x4b9608]")]
// 0x4b9608 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)
// type: 
pub fn stub_0x4b9608() -> ! { todo!("0x4b9608 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *) [0x4b9674]")]
// 0x4b9674 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4b9674() -> ! { todo!("0x4b9674 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *) [0x4b9680]")]
// 0x4b9680 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)
// type: 
pub fn stub_0x4b9680() -> ! { todo!("0x4b9680 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc") }

#[doc(alias = "RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4b9750]")]
// 0x4b9750 — __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4b9750() -> ! { todo!("0x4b9750 __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *) [0x4b98bc]")]
// 0x4b98bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)
// type: 
pub fn stub_0x4b98bc() -> ! { todo!("0x4b98bc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&) [0x4b9fe0]")]
// 0x4b9fe0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)
// type: 
pub fn stub_0x4b9fe0() -> ! { todo!("0x4b9fe0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void) [0x4ba030]")]
// 0x4ba030 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)
// type: _DWORD *()
pub fn stub_0x4ba030() -> ! { todo!("0x4ba030 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *) [0x4ba09c]")]
// 0x4ba09c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4ba09c() -> ! { todo!("0x4ba09c __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *) [0x4ba0a8]")]
// 0x4ba0a8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)
// type: 
pub fn stub_0x4ba0a8() -> ! { todo!("0x4ba0a8 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc") }

#[doc(alias = "RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4ba178]")]
// 0x4ba178 — __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4ba178() -> ! { todo!("0x4ba178 __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *) [0x4ba2e4]")]
// 0x4ba2e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)
// type: 
pub fn stub_0x4ba2e4() -> ! { todo!("0x4ba2e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&) [0x4baa08]")]
// 0x4baa08 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)
// type: 
pub fn stub_0x4baa08() -> ! { todo!("0x4baa08 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void) [0x4baa58]")]
// 0x4baa58 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)
// type: 
pub fn stub_0x4baa58() -> ! { todo!("0x4baa58 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *) [0x4baac4]")]
// 0x4baac4 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4baac4() -> ! { todo!("0x4baac4 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *) [0x4baad0]")]
// 0x4baad0 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)
// type: 
pub fn stub_0x4baad0() -> ! { todo!("0x4baad0 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc") }

#[doc(alias = "RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4baba0]")]
// 0x4baba0 — __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4baba0() -> ! { todo!("0x4baba0 __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *) [0x4bad0c]")]
// 0x4bad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)
// type: 
pub fn stub_0x4bad0c() -> ! { todo!("0x4bad0c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&) [0x4bb430]")]
// 0x4bb430 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)
// type: 
pub fn stub_0x4bb430() -> ! { todo!("0x4bb430 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void) [0x4bb480]")]
// 0x4bb480 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)
// type: 
pub fn stub_0x4bb480() -> ! { todo!("0x4bb480 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *) [0x4bb4ec]")]
// 0x4bb4ec — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)
// type: 
pub fn stub_0x4bb4ec() -> ! { todo!("0x4bb4ec __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc") }

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *) [0x4bb4f8]")]
// 0x4bb4f8 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)
// type: 
pub fn stub_0x4bb4f8() -> ! { todo!("0x4bb4f8 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc") }

#[doc(alias = "RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0x4bb5c8]")]
// 0x4bb5c8 — __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: 
pub fn stub_0x4bb5c8() -> ! { todo!("0x4bb5c8 __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE") }

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *) [0x4bb734]")]
// 0x4bb734 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)
// type: 
pub fn stub_0x4bb734() -> ! { todo!("0x4bb734 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E") }

