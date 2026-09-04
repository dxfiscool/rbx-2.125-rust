//! audio generated_118 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0xf2d1b4..0xf2d7e4 EA-sorted asc filler after 0xf2d1a4, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0xf2d1b4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12PartDragToolEPNS_12PartInstanceEN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEEEENSC_IT_EET0_T1_T2_T3_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_f2d1b4() -> ! {
    todo!("0xf2d1b4 rbx_core::SharedPtr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf2d1c4 — j___ZN5boost10shared_ptrIN3RBX12PartDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool>::shared_ptr<RBX::PartDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d1c4() -> ! {
    todo!("0xf2d1c4 rbx_core::SharedPtr<RBX::PartDragTool>::shared_ptr<RBX::PartDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf2d1d4 — j___ZN5boost6detail12shared_countC2IPN3RBX12PartDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d1d4() {
    // IDA 0xf2d1d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf2d1e4 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12PartDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::PartDragTool,RBX::PartDragTool>(rbx_core::SharedPtr<RBX::PartDragTool> const*,RBX::PartDragTool *)const")]
pub fn stub_f2d1e4() {
    // IDA 0xf2d1e4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf2d1f4 — j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE4initEiRKNS_23ReferenceCountedPointerINS_13MemoryManagerEEE
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")]
pub fn stub_f2d1f4() -> ! {
    todo!("0xf2d1f4 G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")
}

// 0xf2d204 — j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6appendERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")]
pub fn stub_f2d204() -> ! {
    todo!("0xf2d204 G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")
}

// 0xf2d214 — j___ZN3RBX7Extents18negativeMaxExtentsEv
// type: _DWORD __fastcall(RBX::Extents *__hidden this)
#[doc(alias = "RBX::Extents::negativeMaxExtents(void)")]
pub fn stub_f2d214() -> ! {
    todo!("0xf2d214 RBX::Extents::negativeMaxExtents(void)")
}

// 0xf2d224 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")]
pub fn stub_f2d224() -> ! {
    todo!("0xf2d224 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")
}

// 0xf2d234 — j___ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
// type: _DWORD __fastcall(RBX::POLY::Edge *__hidden this, const RBX::POLY::Face *, unsigned int)
#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")]
pub fn stub_f2d234() -> ! {
    todo!("0xf2d234 RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")
}

// 0xf2d244 — j___ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")]
pub fn stub_f2d244() -> ! {
    todo!("0xf2d244 std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")
}

// 0xf2d254 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")]
pub fn stub_f2d254() -> ! {
    todo!("0xf2d254 RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")
}

// 0xf2d264 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")]
pub fn stub_f2d264() -> ! {
    todo!("0xf2d264 std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")
}

// 0xf2d274 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")]
pub fn stub_f2d274() -> ! {
    todo!("0xf2d274 std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")
}

// 0xf2d284 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")]
pub fn stub_f2d284() -> ! {
    todo!("0xf2d284 std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")
}

// 0xf2d294 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_f2d294() -> ! {
    todo!("0xf2d294 rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf2d2a4 — j___ZN5boost10shared_ptrIN3RBX11LuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d2a4() -> ! {
    todo!("0xf2d2a4 rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf2d2b4 — j___ZN5boost6detail12shared_countC2IPN3RBX11LuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d2b4() {
    // IDA 0xf2d2b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf2d2c4 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11LuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(rbx_core::SharedPtr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const")]
pub fn stub_f2d2c4() {
    // IDA 0xf2d2c4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf2d2d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9ExplosionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")]
pub fn stub_f2d2d4() -> ! {
    todo!("0xf2d2d4 rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")
}

// 0xf2d2e4 — j___ZN5boost10shared_ptrIN3RBX9ExplosionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f2d2e4() -> ! {
    todo!("0xf2d2e4 rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf2d2f4 — j___ZN5boost6detail12shared_countC2IPN3RBX9ExplosionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f2d2f4() {
    // IDA 0xf2d2f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf2d304 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(rbx_core::SharedPtr<RBX::Explosion> const*,RBX::Explosion *)const")]
pub fn stub_f2d304() {
    // IDA 0xf2d304: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf2d314 — j___ZN3RBX10Reflection11Call3HelperINS_10LuaDraggerEMS2_FvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEES6_S8_SD_vE4callEPS2_SF_RNS0_7VariantERKS6_RKS8_RKSD_
// type: int __fastcall(int, int, int, int, char, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
pub fn stub_f2d314() -> ! {
    todo!("0xf2d314 RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")
}

// 0xf2d324 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f2d324() -> ! {
    todo!("0xf2d324 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf2d334 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f2d334() -> ! {
    todo!("0xf2d334 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf2d344 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_f2d344() -> ! {
    todo!("0xf2d344 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf2d354 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f2d354() -> ! {
    todo!("0xf2d354 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf2d364 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED2Ev
// type: int __fastcall(int, int, int, int, void *, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
pub fn stub_f2d364() {
    // IDA 0xf2d364: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf2d374 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f2d374() -> ! {
    todo!("0xf2d374 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf2d384 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(RBX::RbxRay),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f2d384() -> ! {
    todo!("0xf2d384 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(RBX::RbxRay),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf2d394 — j___ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::BoundFuncDesc(void (RBX::LuaDragger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f2d394() -> ! {
    todo!("0xf2d394 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::BoundFuncDesc(void (RBX::LuaDragger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf2d3a4 — j___ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f2d3a4() -> ! {
    todo!("0xf2d3a4 G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf2d3b4 — j___ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f2d3b4() -> ! {
    todo!("0xf2d3b4 G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf2d3c4 — j___ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f2d3c4() -> ! {
    todo!("0xf2d3c4 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf2d3d4 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")]
pub fn stub_f2d3d4() -> ! {
    todo!("0xf2d3d4 bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")
}

// 0xf2d3e4 — j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_f2d3e4() -> ! {
    todo!("0xf2d3e4 j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv")
}

// 0xf2d3f4 — j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_f2d3f4() -> ! {
    todo!("0xf2d3f4 j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")
}

// 0xf2d404 — j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_f2d404() {
    // IDA 0xf2d404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf2d414 — j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")]
pub fn stub_f2d414() -> ! {
    todo!("0xf2d414 j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")
}

// 0xf2d424 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10LuaDraggerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")]
pub fn stub_f2d424() -> ! {
    todo!("0xf2d424 rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")
}

// 0xf2d434 — j___ZN5boost10shared_ptrIN3RBX10LuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f2d434() -> ! {
    todo!("0xf2d434 rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf2d444 — j___ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f2d444() -> ! {
    todo!("0xf2d444 rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf2d454 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorINS_8weak_ptrIN3RBX12PartInstanceEEESaIS9_EEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEESC_ENS0_5list1IRKSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_f2d454() -> ! {
    todo!("0xf2d454 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf2d464 — j___ZN5boost6detail12shared_countC2IPN3RBX10LuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f2d464() {
    // IDA 0xf2d464: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf2d474 — j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_f2d474() -> ! {
    todo!("0xf2d474 j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv")
}

// 0xf2d484 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10LuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(rbx_core::SharedPtr<RBX::LuaDragger> const*,RBX::LuaDragger *)const")]
pub fn stub_f2d484() {
    // IDA 0xf2d484: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf2d494 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost8weak_ptrIN3RBX12PartInstanceEEEPS7_EET0_T_SC_SB_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*>(rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*)")]
pub fn stub_f2d494() -> ! {
    todo!("0xf2d494 rbx_core::WeakPtr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*>(rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*)")
}

// 0xf2d4a4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
pub fn stub_f2d4a4() -> ! {
    todo!("0xf2d4a4 rbx_core::WeakPtr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")
}

// 0xf2d4b4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector()")]
pub fn stub_f2d4b4() {
    // IDA 0xf2d4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf2d4c4 — j___ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>)")]
pub fn stub_f2d4c4() -> ! {
    todo!("0xf2d4c4 rbx_core::WeakPtr<RBX::PartInstance>* std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>)")
}

// 0xf2d4d4 — j___ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEaSERKS6_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::operator=(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_f2d4d4() -> ! {
    todo!("0xf2d4d4 std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::operator=(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")
}

// 0xf2d4e4 — j___ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_
#[doc(alias = "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")]
pub fn stub_f2d4e4() -> ! {
    todo!("0xf2d4e4 std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")
}

// 0xf2d4f4 — j___ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev
#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")]
pub fn stub_f2d4f4() {
    // IDA 0xf2d4f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf2d504 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SD_SD_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")]
pub fn stub_f2d504() -> ! {
    todo!("0xf2d504 __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")
}

// 0xf2d514 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_PS9_INS2_8weak_ptrINS4_12PartInstanceEEESaISH_EEENSD_5list2INS2_3argILi1EEENSD_5valueISK_EEEEEEET0_T_SV_SU_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")]
pub fn stub_f2d514() -> ! {
    todo!("0xf2d514 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")
}

// 0xf2d524 — j___ZN3RBX11shared_fromINS_11LuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::shared_from<RBX::LuaDragTool>(RBX::LuaDragTool*)")]
pub fn stub_f2d524() -> ! {
    todo!("0xf2d524 rbx_core::SharedPtr<RBX::LuaDragTool> RBX::shared_from<RBX::LuaDragTool>(RBX::LuaDragTool*)")
}

// 0xf2d534 — j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v")]
pub fn stub_f2d534() -> ! {
    todo!("0xf2d534 j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v")
}

// 0xf2d544 — j___ZN5boost10shared_ptrIN3RBX10LuaDraggerEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::operator=(rbx_core::SharedPtr<RBX::LuaDragger> const&)")]
pub fn stub_f2d544() -> ! {
    todo!("0xf2d544 rbx_core::SharedPtr<RBX::LuaDragger>::operator=(rbx_core::SharedPtr<RBX::LuaDragger> const&)")
}

// 0xf2d554 — j___ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")]
pub fn stub_f2d554() -> ! {
    todo!("0xf2d554 rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")
}

// 0xf2d564 — j___ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")]
pub fn stub_f2d564() -> ! {
    todo!("0xf2d564 rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")
}

// 0xf2d574 — j___ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")]
pub fn stub_f2d574() -> ! {
    todo!("0xf2d574 rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")
}

// 0xf2d584 — j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
pub fn stub_f2d584() -> ! {
    todo!("0xf2d584 j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")
}

// 0xf2d594 — j___ZN3RBX11MegaDragger12getMousePartEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::getMousePart(void)")]
pub fn stub_f2d594() -> ! {
    todo!("0xf2d594 RBX::MegaDragger::getMousePart(void)")
}

// 0xf2d5a4 — j___ZN3RBX11shared_fromINS_12PartDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)")]
pub fn stub_f2d5a4() -> ! {
    todo!("0xf2d5a4 rbx_core::SharedPtr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)")
}

// 0xf2d5b4 — j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
pub fn stub_f2d5b4() -> ! {
    todo!("0xf2d5b4 j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")
}

// 0xf2d5c4 — j___ZN3G3D5ArrayImLi10ELm32EE6appendERKm
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")]
pub fn stub_f2d5c4() -> ! {
    todo!("0xf2d5c4 G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")
}

// 0xf2d5d4 — j___ZN3G3D5ArrayImLi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::resize(int,bool)")]
pub fn stub_f2d5d4() -> ! {
    todo!("0xf2d5d4 G3D::Array<unsigned long,10,32ul>::resize(int,bool)")
}

// 0xf2d5e4 — j___ZN3G3D5ArrayImLi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::realloc(int)")]
pub fn stub_f2d5e4() -> ! {
    todo!("0xf2d5e4 G3D::Array<unsigned long,10,32ul>::realloc(int)")
}

// 0xf2d5f4 — j___ZN3G3D5ArrayImLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::Array(void)")]
pub fn stub_f2d5f4() -> ! {
    todo!("0xf2d5f4 G3D::Array<unsigned long,10,32ul>::Array(void)")
}

// 0xf2d604 — j___ZN3G3D5ArrayImLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::~Array()")]
pub fn stub_f2d604() {
    // IDA 0xf2d604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf2d614 — j___ZN3RBX11shared_fromINS_17FilteredSelectionINS_8InstanceEEEEEN5boost10shared_ptrIT_EEPS6_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")]
pub fn stub_f2d614() -> ! {
    todo!("0xf2d614 rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")
}

// 0xf2d624 — j___ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_PKS3_
#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")]
pub fn stub_f2d624() -> ! {
    todo!("0xf2d624 RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")
}

// 0xf2d634 — j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
pub fn stub_f2d634() -> ! {
    todo!("0xf2d634 j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")
}

// 0xf2d644 — j___ZN3RBX8Instance15queryTypedChildINS_10SelectableEEEPT_i
#[doc(alias = "RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")]
pub fn stub_f2d644() -> ! {
    todo!("0xf2d644 RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")
}

// 0xf2d654 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_f2d654() -> ! {
    todo!("0xf2d654 rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")
}

// 0xf2d664 — j___ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d664() -> ! {
    todo!("0xf2d664 rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf2d674 — j___ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEaSERKS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
pub fn stub_f2d674() -> ! {
    todo!("0xf2d674 rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")
}

// 0xf2d684 — j___ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_f2d684() {
    // IDA 0xf2d684: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf2d694 — j___ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_8InstanceEEEE13createServiceEv
#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")]
pub fn stub_f2d694() -> ! {
    todo!("0xf2d694 RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")
}

// 0xf2d6a4 — j___ZNK3RBX9Selection10isSelectedEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Selection *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Selection::isSelected(RBX::Instance const*)const")]
pub fn stub_f2d6a4() -> ! {
    todo!("0xf2d6a4 RBX::Selection::isSelected(RBX::Instance const*)const")
}

// 0xf2d6b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
pub fn stub_f2d6b4() {
    // IDA 0xf2d6b4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf2d6c4 — j___ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection11AddIteratorEEET0_T_SD_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
pub fn stub_f2d6c4() -> ! {
    todo!("0xf2d6c4 RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")
}

// 0xf2d6d4 — j___ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14RemoveIteratorEEET0_T_SD_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
pub fn stub_f2d6d4() -> ! {
    todo!("0xf2d6d4 RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")
}

// 0xf2d6e4 — j___ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14ToggleIteratorEEET0_T_SD_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
pub fn stub_f2d6e4() -> ! {
    todo!("0xf2d6e4 RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")
}

// 0xf2d6f4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f2d6f4() {
    // IDA 0xf2d6f4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0xf2d704 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
pub fn stub_f2d704() {
    // IDA 0xf2d704: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf2d714 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f2d714() -> ! {
    todo!("0xf2d714 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf2d724 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSC_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")]
pub fn stub_f2d724() -> ! {
    todo!("0xf2d724 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")
}

// 0xf2d734 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
pub fn stub_f2d734() {
    // IDA 0xf2d734: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf2d744 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_f2d744() -> ! {
    todo!("0xf2d744 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf2d754 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EEaSERKSA_
// type: int(void)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_f2d754() -> ! {
    todo!("0xf2d754 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0xf2d764 — j___ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection11AddIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
pub fn stub_f2d764() -> ! {
    todo!("0xf2d764 RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")
}

// 0xf2d774 — j___ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14RemoveIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
pub fn stub_f2d774() -> ! {
    todo!("0xf2d774 RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")
}

// 0xf2d784 — j___ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14ToggleIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
pub fn stub_f2d784() -> ! {
    todo!("0xf2d784 RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")
}

// 0xf2d794 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKS5_EEET_SF_SF_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")]
pub fn stub_f2d794() -> ! {
    todo!("0xf2d794 __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")
}

// 0xf2d7a4 — j___ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")]
pub fn stub_f2d7a4() -> ! {
    todo!("0xf2d7a4 RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")
}

// 0xf2d7b4 — j___ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
pub fn stub_f2d7b4() -> ! {
    todo!("0xf2d7b4 std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")
}

// 0xf2d7c4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
pub fn stub_f2d7c4() -> ! {
    todo!("0xf2d7c4 RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")
}

// 0xf2d7d4 — j___ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_f2d7d4() -> ! {
    todo!("0xf2d7d4 std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")
}

// 0xf2d7e4 — j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
pub fn stub_f2d7e4() -> ! {
    todo!("0xf2d7e4 std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")
}
