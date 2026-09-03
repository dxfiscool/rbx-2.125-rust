//! rendering shard 280 — 150 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 30470->30620 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30470 before -> 30620 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3cc9ac — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x3cc9ac: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3cc9ac() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3cca90 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm
// IDA 0x3cca90: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3cca90() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3ccaa8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_
// IDA 0x3ccaa8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3ccaa8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3ccae4 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x3ccae4: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ccae4() {
}


// 0x3ccc74 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::resize(unsigned long,RBX::Camera::CameraType)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_
// IDA 0x3ccc74: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ccc74() {
}


// 0x3ccca8 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::push_back(RBX::Camera::CameraType const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_
// IDA 0x3ccca8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3ccca8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x3cccd0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x3cccd0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cccd0() {
}


// 0x3ccd28 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x3ccd28: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ccd28() {
}


// 0x3ccddc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x3ccddc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ccddc() {
}


// 0x3cce34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x3cce34: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cce34() {
}


// 0x3cce9c — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,RBX::Camera::CameraType const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x3cce9c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3cce9c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3ccf80 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm
// IDA 0x3ccf80: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3ccf80() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3ccf98 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_
// IDA 0x3ccf98: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3ccf98() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3ccfd4 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,unsigned long,RBX::Camera::CameraType const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x3ccfd4: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ccfd4() {
}


// 0x3cd164 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Camera::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cd164: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd164() {
}


// 0x3cd2e8 — __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x3cd2e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cd2e8() {
}


// 0x3cd39c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x3cd39c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd39c() {
}


// 0x3cd4f0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x3cd4f0: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd4f0() {
}


// 0x3cd57c — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x3cd57c: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd57c() {
}


// 0x3cd590 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IbEEvRKT_
// type: int __fastcall(int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<bool>(bool const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute1IbEEvRKT_
// IDA 0x3cd590: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd590() {
}


// 0x3cd6d4 — __ZN5boost8functionIFvbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// IDA 0x3cd6d4: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd6d4() {
}


// 0x3cd7b8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvbE6invokeERNS1_15function_bufferEb
// type: int __fastcall(_DWORD *, char)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,bool>::invoke(boost::detail::function::function_buffer &,bool)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvbE6invokeERNS1_15function_bufferEb
// IDA 0x3cd7b8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd7b8() {
}


// 0x3cd7d0 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x3cd7d0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd7d0() {
}


// 0x3cd8b8 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x3cd8b8: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd8b8() {
}


// 0x3cd99c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIbEEvRT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<bool>(bool &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIbEEvRT_
// IDA 0x3cd99c: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd99c() {
}


// 0x3cd9b4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x3cd9b4: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cd9b4() {
}


// 0x3cdb0c — __ZN3rbx7signals6signalIFvbEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::function<void ()(bool)>>(boost::function<void ()(bool)> const&)")]
// was: __ZN3rbx7signals6signalIFvbEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// IDA 0x3cdb0c: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cdb0c() {
}


// 0x3cdc00 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::function<void ()(bool)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED1Ev
// IDA 0x3cdc00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cdc00() {
}


// 0x3cdd10 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::function<void ()(bool)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED0Ev
// IDA 0x3cdd10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cdd10() {
}


// 0x3cde40 — __ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv
// IDA 0x3cde40: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cde40() {
}


// 0x3cdf30 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// IDA 0x3cdf30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cdf30() {
}


// 0x3ce040 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
// IDA 0x3ce040: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ce040() {
}


// 0x3ce170 — __ZN3rbx7signals6signalIFvbEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvbEE4slotD1Ev
// IDA 0x3ce170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ce170() {
}


// 0x3ce19c — __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x3ce19c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ce19c() {
}


// 0x3ce250 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x3ce250: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce250() {
}


// 0x3ce454 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x3ce454: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce454() {
}


// 0x3ce4c8 — __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Camera,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Camera::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_6CameraEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x3ce4c8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce4c8() {
}


// 0x3cede0 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x3cede0: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cede0() {
}


// 0x3cef84 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EEC2EMS2_FbiEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::BoundFuncDesc(bool (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EEC2EMS2_FbiEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cef84: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cef84() {
}


// 0x3cf0fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3cf0fc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf0fc() {
}


// 0x3cf12c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EED0Ev
// IDA 0x3cf12c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cf12c() {
}


// 0x3cf200 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3cf200: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf200() {
}


// 0x3cf240 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbiEibE4callEPS2_S4_RNS0_7VariantERKi
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(int),int,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(int),RBX::Reflection::Variant &,int const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbiEibE4callEPS2_S4_RNS0_7VariantERKi
// IDA 0x3cf240: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf240() {
}


// 0x3cf278 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::BoundFuncDesc(void (RBX::Camera::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cf278: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf278() {
}


// 0x3cf3f0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3cf3f0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf3f0() {
}


// 0x3cf420 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EED0Ev
// IDA 0x3cf420: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cf420() {
}


// 0x3cf4f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3cf4f4: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf4f4() {
}


// 0x3cf528 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EEC2EMS2_FbfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::BoundFuncDesc(bool (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EEC2EMS2_FbfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cf528: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf528() {
}


// 0x3cf6a0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3cf6a0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf6a0() {
}


// 0x3cf6d0 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EED0Ev
// IDA 0x3cf6d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cf6d0() {
}


// 0x3cf7a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,bool ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFbfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3cf7a4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf7a4() {
}


// 0x3cf7e4 — __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbfEfbE4callEPS2_S4_RNS0_7VariantERKf
// type: int __fastcall(int, char *, int, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Camera,bool (RBX::Camera::*)(float),float,bool>::call(RBX::Camera*,bool (RBX::Camera::*)(float),RBX::Reflection::Variant &,float const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_6CameraEMS2_FbfEfbE4callEPS2_S4_RNS0_7VariantERKf
// IDA 0x3cf7e4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf7e4() {
}


// 0x3cf81c — __ZN3RBX10Reflection9ArgHelper6getArgIfLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x3cf81c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf81c() {
}


// 0x3cf9bc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EEC2EMS2_FvS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::BoundFuncDesc(void (RBX::Camera::*)(RBX::Camera::CameraPanMode),char const*,char const*,RBX::Camera::CameraPanMode,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EEC2EMS2_FvS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cf9bc: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cf9bc() {
}


// 0x3cfb68 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3cfb68: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cfb68() {
}


// 0x3cfb98 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EED0Ev
// IDA 0x3cfb98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3cfb98() {
}


// 0x3cfc6c — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(RBX::Camera::CameraPanMode),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvNS2_13CameraPanModeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3cfc6c: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cfc6c() {
}


// 0x3cfca0 — __ZN3RBX10Reflection9ArgHelper6getArgINS_6Camera13CameraPanModeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::Camera::CameraPanMode RBX::Reflection::ArgHelper::getArg<RBX::Camera::CameraPanMode,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Camera::CameraPanMode> const&,boost::disable_if<boost::is_same<RBX::Camera::CameraPanMode,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_6Camera13CameraPanModeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x3cfca0: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cfca0() {
}


// 0x3cfe30 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_6Camera13CameraPanModeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Camera::CameraPanMode>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Camera::CameraPanMode &,boost::enable_if<boost::is_enum<RBX::Camera::CameraPanMode>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_6Camera13CameraPanModeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x3cfe30: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cfe30() {
}


// 0x3cfe84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE13initSingletonEv
// IDA 0x3cfe84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3cfe84() {
}


// 0x3cfe88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera13CameraPanModeEEEE14doGetSingletonEv
// IDA 0x3cfe88: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cfe88() {
}


// 0x3cff78 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::BoundFuncDesc(float (RBX::Camera::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3cff78: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cff78() {
}


// 0x3d007c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EED0Ev
// IDA 0x3d007c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d007c() {
}


// 0x3d0130 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3d0130: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0130() {
}


// 0x3d0154 — __ZN3RBX10Reflection11Call0HelperINS_6CameraEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, __int64 (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Camera,float (RBX::Camera::*)(void),float>::call(RBX::Camera*,float (RBX::Camera::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_6CameraEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
// IDA 0x3d0154: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0154() {
}


// 0x3d0184 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::BoundFuncDesc(void (RBX::Camera::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3d0184: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0184() {
}


// 0x3d02fc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3d02fc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d02fc() {
}


// 0x3d032c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EED0Ev
// IDA 0x3d032c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d032c() {
}


// 0x3d0400 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3d0400: 21 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0400() {
}


// 0x3d043c — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3d043c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d043c() {
}


// 0x3d04e0 — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED0Ev
// IDA 0x3d04e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d04e0() {
}


// 0x3d0510 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10isReadOnlyEv
// IDA 0x3d0510: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0510() {
}


// 0x3d0520 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11isWriteOnlyEv
// IDA 0x3d0520: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0520() {
}


// 0x3d0530 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x3d0530: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0530() {
}


// 0x3d0558 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x3d0558: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0558() {
}


// 0x3d0670 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x3d0670: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0670() {
}


// 0x3d0738 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x3d0738: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0738() {
}


// 0x3d075c — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x3d075c: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d075c() {
}


// 0x3d0830 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3d0830: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0830() {
}


// 0x3d0854 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// IDA 0x3d0854: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0854() {
}


// 0x3d0868 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// IDA 0x3d0868: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0868() {
}


// 0x3d08e4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// IDA 0x3d08e4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d08e4() {
}


// 0x3d0904 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x3d0904: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0904() {
}


// 0x3d09e4 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x3d09e4: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d09e4() {
}


// 0x3d09ec — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x3d09ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d09ec() {
}


// 0x3d09f0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x3d09f0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d09f0() {
}


// 0x3d09f4 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3d09f4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d09f4() {
}


// 0x3d0a14 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x3d0a14: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0a14() {
}


// 0x3d0a38 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::PropDescriptor<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>(char const*,char const*,float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3d0a38: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0a38() {
}


// 0x3d0b4c — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED0Ev
// IDA 0x3d0b4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d0b4c() {
}


// 0x3d0b78 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x3d0b78: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0b78() {
}


// 0x3d0b7c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x3d0b7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0b7c() {
}


// 0x3d0b80 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3d0b80: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0b80() {
}


// 0x3d0ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x3d0ba0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0ba0() {
}


// 0x3d0d6c — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::EnumPropDescriptor<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>(char const*,char const*,RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3d0d6c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0d6c() {
}


// 0x3d0f20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED0Ev
// IDA 0x3d0f20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d0f20() {
}


// 0x3d0f4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10isReadOnlyEv
// IDA 0x3d0f4c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0f4c() {
}


// 0x3d0f5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11isWriteOnlyEv
// IDA 0x3d0f5c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0f5c() {
}


// 0x3d0f6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x3d0f6c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0f6c() {
}


// 0x3d0f94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x3d0f94: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0f94() {
}


// 0x3d0fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x3d0fb8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0fb8() {
}


// 0x3d1104 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x3d1104: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1104() {
}


// 0x3d1128 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14hasStringValueEv
// IDA 0x3d1128: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1128() {
}


// 0x3d112c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x3d112c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d112c() {
}


// 0x3d1150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x3d1150: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1150() {
}


// 0x3d1190 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x3d1190: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1190() {
}


// 0x3d11b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3d11b0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d11b0() {
}


// 0x3d13f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x3d13f0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d13f0() {
}


// 0x3d140c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x3d140c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d140c() {
}


// 0x3d1440 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x3d1440: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1440() {
}


// 0x3d1448 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x3d1448: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1448() {
}


// 0x3d1494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x3d1494: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1494() {
}


// 0x3d14b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x3d14b4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d14b4() {
}


// 0x3d14e8 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToIndex(RBX::Camera::CameraType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToIndexES3_
// IDA 0x3d14e8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d14e8() {
}


// 0x3d1558 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x3d1558: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1558() {
}


// 0x3d1598 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3d1598: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1598() {
}


// 0x3d159c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3d159c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d159c() {
}


// 0x3d15a0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3d15a0: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d15a0() {
}


// 0x3d15c0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x3d15c0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d15c0() {
}


// 0x3d15e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE13initSingletonEv
// IDA 0x3d15e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3d15e4() {
}


// 0x3d15e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE14doGetSingletonEv
// IDA 0x3d15e8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d15e8() {
}


// 0x3d16d8 — __ZN3RBX6CameraD2Ev
// type: void __fastcall(RBX::Camera *this, int, int, int)
#[doc(alias = "RBX::Camera::~Camera()")]
// was: __ZN3RBX6CameraD2Ev
// IDA 0x3d16d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d16d8() {
}


// 0x3d194c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x3d194c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d194c() {
}


// 0x3d1974 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x3d1974: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1974() {
}


// 0x3d199c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x3d199c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d199c() {
}


// 0x3d19c4 — __GLOBAL__I_a_166
#[doc(alias = "__GLOBAL__I_a_166")]
// was: __GLOBAL__I_a_166
// IDA 0x3d19c4: 713 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d19c4() {
}


// 0x3d22c0 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC1Ev
// IDA 0x3d22c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3d22c0() {
}


// 0x3d22c4 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC2Ev
// IDA 0x3d22c4: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d22c4() {
}


// 0x3d249c — __ZN3RBX20ChangeHistoryService10setEnabledEb
// type: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, int)
#[doc(alias = "RBX::ChangeHistoryService::setEnabled(bool)")]
// was: __ZN3RBX20ChangeHistoryService10setEnabledEb
// IDA 0x3d249c: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d249c() {
}


// 0x3d24b8 — __ZN3RBX20ChangeHistoryService17resetBaseWaypointEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::resetBaseWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService17resetBaseWaypointEv
// IDA 0x3d24b8: 32 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d24b8() {
}


// 0x3d250c — __ZN3RBX20ChangeHistoryService4playEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::play(void)")]
// was: __ZN3RBX20ChangeHistoryService4playEv
// IDA 0x3d250c: 333 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d250c() {
}


// 0x3d28cc — __ZN3RBX20ChangeHistoryService6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::unplay(void)")]
// was: __ZN3RBX20ChangeHistoryService6unplayEv
// IDA 0x3d28cc: 297 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d28cc() {
}


// 0x3d2c28 — __ZN3RBX20ChangeHistoryService10canUnplay2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, RBX::ChangeHistoryService *)
#[doc(alias = "RBX::ChangeHistoryService::canUnplay2(void)")]
// was: __ZN3RBX20ChangeHistoryService10canUnplay2Ev
// IDA 0x3d2c28: 235 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d2c28() {
}


// 0x3d2ea0 — __ZN3RBX20ChangeHistoryService8canPlay2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int)
#[doc(alias = "RBX::ChangeHistoryService::canPlay2(void)")]
// was: __ZN3RBX20ChangeHistoryService8canPlay2Ev
// IDA 0x3d2ea0: 237 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d2ea0() {
}


// 0x3d3120 — __ZN3RBX20ChangeHistoryService4Item12unplayDeleteEv
// type: void __fastcall(RBX::Instance **this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayDelete(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12unplayDeleteEv
// IDA 0x3d3120: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d3120() {
}


// 0x3d3328 — __ZN3RBX20ChangeHistoryService4Item14unplayPropertyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(RBX::Instance **, void **)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayProperty(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item14unplayPropertyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// IDA 0x3d3328: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d3328() {
}


// 0x3d3518 — __ZN3RBX20ChangeHistoryService4Item17unplayClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: _DWORD *__fastcall(RBX::Instance **, unsigned int *)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item17unplayClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// IDA 0x3d3518: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d3518() {
}


// 0x3d367c — __ZN3RBX20ChangeHistoryServiceC1Ev
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void)")]
// was: __ZN3RBX20ChangeHistoryServiceC1Ev
// IDA 0x3d367c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3d367c() {
}


// 0x3d3680 — __ZN3RBX20ChangeHistoryServiceC2Ev
// type: __guard *__fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void)")]
// was: __ZN3RBX20ChangeHistoryServiceC2Ev
// IDA 0x3d3680: 313 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d3680() {
}


// 0x3d39cc — __ZN3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// was: __ZN3RBX20ChangeHistoryServiceD0Ev
// IDA 0x3d39cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d39cc() {
}


// 0x3d3a6c — __ZN3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// was: __ZN3RBX20ChangeHistoryServiceD1Ev
// IDA 0x3d3a6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3d3a6c() {
}


// 0x3d3a70 — __ZThn32_N3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX20ChangeHistoryServiceD0Ev")]
// was: __ZThn32_N3RBX20ChangeHistoryServiceD0Ev
// IDA 0x3d3a70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d3a70() {
}


// 0x3d3a78 — __ZThn36_N3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX20ChangeHistoryServiceD0Ev")]
// was: __ZThn36_N3RBX20ChangeHistoryServiceD0Ev
// IDA 0x3d3a78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d3a78() {
}


// 0x3d3a80 — __ZN3RBX20ChangeHistoryServiceD2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
// was: __ZN3RBX20ChangeHistoryServiceD2Ev
// IDA 0x3d3a80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d3a80() {
}


// 0x3d3f08 — __ZThn32_N3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "__ZThn32_N3RBX20ChangeHistoryServiceD1Ev")]
// was: __ZThn32_N3RBX20ChangeHistoryServiceD1Ev
// IDA 0x3d3f08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d3f08() {
}


// 0x3d3f10 — __ZThn36_N3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "__ZThn36_N3RBX20ChangeHistoryServiceD1Ev")]
// was: __ZThn36_N3RBX20ChangeHistoryServiceD1Ev
// IDA 0x3d3f10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d3f10() {
}


// 0x3d3f18 — __ZN3RBX20ChangeHistoryService6attachEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::attach(void)")]
// was: __ZN3RBX20ChangeHistoryService6attachEv
// IDA 0x3d3f18: 337 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d3f18() {
}


// 0x3d429c — __ZN3RBX20ChangeHistoryService7dettachEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::dettach(void)")]
// was: __ZN3RBX20ChangeHistoryService7dettachEv
// IDA 0x3d429c: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d429c() {
}


// 0x3d434c — __ZNK3RBX20ChangeHistoryService17getUnplayWaypointERSsi
// type: int __fastcall(RBX::ChangeHistoryService *this, std::string *, int)
#[doc(alias = "RBX::ChangeHistoryService::getUnplayWaypoint(std::string &,int)const")]
// was: __ZNK3RBX20ChangeHistoryService17getUnplayWaypointERSsi
// IDA 0x3d434c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d434c() {
}


// 0x3d43a4 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE
// type: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, const char *, const RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*,RBX::Instance const*)")]
// was: __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE
// IDA 0x3d43a4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d43a4() {
}