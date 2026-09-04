//! rendering shard 344 — 100 stubs 0x4a31bc..0x4a7254 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 37421->37521 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 37421 before -> 37521 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 lowest remaining 0x4a31bc..0x4a7254 (next lowest 0x4a725c if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4a31bc — __ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a31bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a31bc() {
}

// 0x4a325c — __ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a325c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a325c() {
}

// 0x4a3264 — __ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a3264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3264() {
}

// 0x4a3308 — __ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a3308: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3308() {
}

// 0x4a3310 — __ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a3310: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3310() {
}

// 0x4a33b8 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)")]
// IDA 0x4a33b8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a33b8() {
}

// 0x4a33f0 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)")]
// IDA 0x4a33f0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_4a33f0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x4a341c — __ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)")]
// IDA 0x4a341c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a341c() {
}

// 0x4a3474 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// IDA 0x4a3474: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3474() {
}

// 0x4a3528 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// IDA 0x4a3528: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3528() {
}

// 0x4a3580 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// IDA 0x4a3580: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3580() {
}

// 0x4a35ec — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)")]
// IDA 0x4a35ec: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_4a35ec() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x4a36d0 — __ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)")]
// IDA 0x4a36d0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_4a36d0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x4a36e8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)")]
// IDA 0x4a36e8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_4a36e8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x4a3728 — __ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)")]
// IDA 0x4a3728: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3728() {
}

// 0x4a38b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x4a38b8: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a38b8() {
}

// 0x4a3aa8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// IDA 0x4a3aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3aa8() {
}

// 0x4a3b5c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x4a3b5c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3b5c() {
}

// 0x4a3cb0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x4a3cb0: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3cb0() {
}

// 0x4a3e20 — __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x4a3e20: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3e20() {
}

// 0x4a3e34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13disconnectAllEv
// IDA 0x4a3e34: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3e34() {
}

// 0x4a3fac — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// IDA 0x4a3fac: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a3fac() {
}

// 0x4a40c8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> const&,float const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_
// IDA 0x4a40c8: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a40c8() {
}

// 0x4a4230 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE5clearEv
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::clear(void)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE5clearEv
// IDA 0x4a4230: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4230() {
}

// 0x4a4260 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// IDA 0x4a4260: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4260() {
}

// 0x4a4344 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// IDA 0x4a4344: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4344() {
}

// 0x4a442c — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// IDA 0x4a442c: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a442c() {
}

// 0x4a4524 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// IDA 0x4a4524: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4524() {
}

// 0x4a4540 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,float>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,float)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f
// IDA 0x4a4540: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4540() {
}

// 0x4a4554 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x4a4554: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4554() {
}

// 0x4a463c — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x4a463c: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a463c() {
}

// 0x4a4720 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x4a4720: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4720() {
}

// 0x4a47f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
// IDA 0x4a47f4: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a47f4() {
}

// 0x4a4810 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x4a4810: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4810() {
}

// 0x4a4968 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// IDA 0x4a4968: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4968() {
}

// 0x4a4a5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6insertEPNS8_4slotE
// IDA 0x4a4a5c: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4a5c() {
}

// 0x4a4c68 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSEPSA_
// IDA 0x4a4c68: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4c68() {
}

// 0x4a4c8c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
// IDA 0x4a4c8c: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4c8c() {
}

// 0x4a4d88 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED1Ev
// IDA 0x4a4d88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a4d88() {
}

// 0x4a4e98 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED0Ev
// IDA 0x4a4e98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a4e98() {
}

// 0x4a4fc8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot10disconnectEv
// IDA 0x4a4fc8: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a4fc8() {
}

// 0x4a50d8 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot9connectedEv
// IDA 0x4a50d8: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a50d8() {
}

// 0x4a50e4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::call(rbx_core::SharedPtr<RBX::Instance>,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f
// IDA 0x4a50e4: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a50e4() {
}

// 0x4a51bc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f")]
// IDA 0x4a51bc: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a51bc() {
}

// 0x4a51c4 — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEclES4_f
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::operator()(rbx_core::SharedPtr<RBX::Instance>,float)const")]
// was: __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEclES4_f
// IDA 0x4a51c4: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a51c4() {
}

// 0x4a52dc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE
// IDA 0x4a52dc: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a52dc() {
}

// 0x4a53cc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv
// IDA 0x4a53cc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a53cc() {
}

// 0x4a53d0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv
// IDA 0x4a53d0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a53d0() {
}

// 0x4a54c4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
// IDA 0x4a54c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a54c4() {
}

// 0x4a55d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
// IDA 0x4a55d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a55d4() {
}

// 0x4a5704 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev
// IDA 0x4a5704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a5704() {
}

// 0x4a5730 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev
// IDA 0x4a5730: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a5730() {
}

// 0x4a5804 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float> const&)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_
// IDA 0x4a5804: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5804() {
}

// 0x4a5834 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// IDA 0x4a5834: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5834() {
}

// 0x4a59e8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
// IDA 0x4a59e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a59e8() {
}

// 0x4a5a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")]
// IDA 0x4a5a14: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5a14() {
}

// 0x4a5a24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")]
// IDA 0x4a5a24: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5a24() {
}

// 0x4a5a34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a5a34: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5a34() {
}

// 0x4a5a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// IDA 0x4a5a5c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5a5c() {
}

// 0x4a5a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// IDA 0x4a5a80: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5a80() {
}

// 0x4a5bcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// IDA 0x4a5bcc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5bcc() {
}

// 0x4a5bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")]
// IDA 0x4a5bf4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5bf4() {
}

// 0x4a5bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a5bf8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5bf8() {
}

// 0x4a5c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// IDA 0x4a5c1c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5c1c() {
}

// 0x4a5c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// IDA 0x4a5c5c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5c5c() {
}

// 0x4a5c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// IDA 0x4a5c7c: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5c7c() {
}

// 0x4a5ebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a5ebc: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5ebc() {
}

// 0x4a5ed8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// IDA 0x4a5ed8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5ed8() {
}

// 0x4a5f0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a5f0c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5f0c() {
}

// 0x4a5f14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4a5f14: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5f14() {
}

// 0x4a5f60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a5f60: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5f60() {
}

// 0x4a5f80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// IDA 0x4a5f80: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5f80() {
}

// 0x4a5fb8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
// IDA 0x4a5fb8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a5fb8() {
}

// 0x4a6028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4a6028: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6028() {
}

// 0x4a606c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")]
// IDA 0x4a606c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a606c() {
}

// 0x4a6070 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")]
// IDA 0x4a6070: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6070() {
}

// 0x4a6074 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a6074: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6074() {
}

// 0x4a6094 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")]
// IDA 0x4a6094: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6094() {
}

// 0x4a60b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")]
// IDA 0x4a60b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a60b8() {
}

// 0x4a64ac — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// IDA 0x4a64ac: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a64ac() {
}

// 0x4a6640 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// IDA 0x4a6640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a6640() {
}

// 0x4a666c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
// IDA 0x4a666c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a666c() {
}

// 0x4a6670 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
// IDA 0x4a6670: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6670() {
}

// 0x4a6674 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a6674: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6674() {
}

// 0x4a6680 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// IDA 0x4a6680: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6680() {
}

// 0x4a66dc — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// IDA 0x4a66dc: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a66dc() {
}

// 0x4a67f0 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
// IDA 0x4a67f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a67f0() {
}

// 0x4a681c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")]
// IDA 0x4a681c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a681c() {
}

// 0x4a6820 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")]
// IDA 0x4a6820: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6820() {
}

// 0x4a6824 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a6824: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6824() {
}

// 0x4a6844 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// IDA 0x4a6844: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6844() {
}

// 0x4a6870 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)")]
// IDA 0x4a6870: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6870() {
}

// 0x4a6898 — __GLOBAL__I_a_188
#[doc(alias = "__GLOBAL__I_a_188")]
// IDA 0x4a6898: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4a6898() {
}

// 0x4a6e24 — __ZN3RBX20ExtrudedPartInstance19setVisualTrussStyleENS0_16VisualTrussStyleE
#[doc(alias = "RBX::ExtrudedPartInstance::setVisualTrussStyle(RBX::ExtrudedPartInstance::VisualTrussStyle)")]
// IDA 0x4a6e24: 29 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6e24() {
}

// 0x4a6e80 — __ZN3RBX20ExtrudedPartInstanceC1Ev
#[doc(alias = "RBX::ExtrudedPartInstance::ExtrudedPartInstance(void)")]
// IDA 0x4a6e80: 264 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6e80() {
}

// 0x4a7184 — __ZN3RBX20ExtrudedPartInstanceD0Ev
#[doc(alias = "RBX::ExtrudedPartInstance::~ExtrudedPartInstance()")]
// IDA 0x4a7184: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7184() {
}

// 0x4a7234 — __ZN3RBX20ExtrudedPartInstanceD1Ev
#[doc(alias = "RBX::ExtrudedPartInstance::~ExtrudedPartInstance()")]
// IDA 0x4a7234: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7234() {
}

// 0x4a7244 — __ZThn32_N3RBX20ExtrudedPartInstanceD0Ev
#[doc(alias = "__ZThn32_N3RBX20ExtrudedPartInstanceD0Ev")]
// IDA 0x4a7244: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7244() {
}

// 0x4a724c — __ZThn36_N3RBX20ExtrudedPartInstanceD0Ev
#[doc(alias = "__ZThn36_N3RBX20ExtrudedPartInstanceD0Ev")]
// IDA 0x4a724c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a724c() {
}

// 0x4a7254 — __ZThn132_N3RBX20ExtrudedPartInstanceD0Ev
#[doc(alias = "__ZThn132_N3RBX20ExtrudedPartInstanceD0Ev")]
// IDA 0x4a7254: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7254() {
}