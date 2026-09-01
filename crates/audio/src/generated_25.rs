//! audio generated_25 — next 120 stubs EA-sorted, from ida/export.json
//! Filter: legacy GA gap-fill (workspace EA-sorted asc, skip existing) — audio crate filler
//! Batch: 120 stubs | skeleton batch | range 0x2dfc24..0x2e6070 EA-sorted asc after 0x002df2b8, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x2dfc24 — __ZN3RBX7Dragger11moveExtentsERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D7Vector3E
#[doc(alias = "RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")]
pub fn stub_2dfc24() -> ! {
    todo!("0x2dfc24 RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")
}

// 0x2dfc98 — __ZN3RBX7Extents18negativeMaxExtentsEv
#[doc(alias = "RBX::Extents::negativeMaxExtents(void)")]
pub fn stub_2dfc98() -> ! {
    todo!("0x2dfc98 RBX::Extents::negativeMaxExtents(void)")
}

// 0x2dfda8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")]
pub fn stub_2dfda8() -> ! {
    todo!("0x2dfda8 G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")
}

// 0x2dfe04 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")]
pub fn stub_2dfe04() -> ! {
    todo!("0x2dfe04 std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")
}

// 0x2dfea0 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")]
pub fn stub_2dfea0() -> ! {
    todo!("0x2dfea0 std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")
}

// 0x2dfed8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE4initEiRKNS_23ReferenceCountedPointerINS_13MemoryManagerEEE
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")]
pub fn stub_2dfed8() -> ! {
    todo!("0x2dfed8 G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")
}

// 0x2dff0c — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")]
pub fn stub_2dff0c() -> ! {
    todo!("0x2dff0c std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")
}

// 0x2e00a0 — __ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")]
pub fn stub_2e00a0() -> ! {
    todo!("0x2e00a0 std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")
}

// 0x2e00c4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")]
pub fn stub_2e00c4() -> ! {
    todo!("0x2e00c4 RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")
}

// 0x2e0140 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")]
pub fn stub_2e0140() -> ! {
    todo!("0x2e0140 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")
}

// 0x2e01b0 — __ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")]
pub fn stub_2e01b0() -> ! {
    todo!("0x2e01b0 RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")
}

// 0x2e02d0 — __ZN3G3D6SphereD0Ev
#[doc(alias = "G3D::Sphere::~Sphere()")]
pub fn stub_2e02d0() -> ! {
    todo!("0x2e02d0 G3D::Sphere::~Sphere()")
}

// 0x2e02d4 — __GLOBAL__I_a_86
#[doc(alias = "global constructor keyed to_a_86")]
pub fn stub_2e02d4() -> ! {
    todo!("0x2e02d4 `global constructor keyed to'_a_86")
}

// 0x2e06d0 — __ZN3RBX8DragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
#[doc(alias = "RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_2e06d0() -> ! {
    todo!("0x2e06d0 RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x2e08bc — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_2e08bc() -> ! {
    todo!("0x2e08bc boost::shared_ptr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x2e09ec — __ZN5boost10shared_ptrIN3RBX11LuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_2e09ec() -> ! {
    todo!("0x2e09ec boost::shared_ptr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x2e0ab4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11LuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(rbx_core::SharedPtr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const")]
pub fn stub_2e0ab4() -> ! {
    todo!("0x2e0ab4 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(boost::shared_ptr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const")
}

// 0x2e0b98 — __ZN5boost6detail12shared_countC2IPN3RBX11LuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_2e0b98() -> ! {
    todo!("0x2e0b98 boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x2e0c90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2e0c90() -> ! {
    todo!("0x2e0c90 boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e0c94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2e0c94() -> ! {
    todo!("0x2e0c94 boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e0c98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_2e0c98() -> ! {
    todo!("0x2e0c98 boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x2e0ca8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2e0ca8() -> ! {
    todo!("0x2e0ca8 boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2e0cc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2e0cc0() -> ! {
    todo!("0x2e0cc0 boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x2e0cc4 — __GLOBAL__I_a_87
#[doc(alias = "global constructor keyed to_a_87")]
pub fn stub_2e0cc4() -> ! {
    todo!("0x2e0cc4 `global constructor keyed to'_a_87")
}

// 0x2e0f38 — __ZN3RBX13DragUtilities13safeMoveYDropERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKN3G3D7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::DragUtilities::safeMoveYDrop(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
pub fn stub_2e0f38() -> ! {
    todo!("0x2e0f38 RBX::DragUtilities::safeMoveYDrop(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")
}

// 0x2e10d8 — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")]
pub fn stub_2e10d8() -> ! {
    todo!("0x2e10d8 RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")
}

// 0x2e1308 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerEb
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,bool)")]
pub fn stub_2e1308() -> ! {
    todo!("0x2e1308 RBX::DragUtilities::hitObjectOrPlane(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,bool)")
}

// 0x2e13f0 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
pub fn stub_2e13f0() -> ! {
    todo!("0x2e13f0 RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")
}

// 0x2e1628 — __ZN3RBX13DragUtilities9hitObjectERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerERN3G3D7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObject(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")]
pub fn stub_2e1628() -> ! {
    todo!("0x2e1628 RBX::DragUtilities::hitObject(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")
}

// 0x2e1708 — __ZN3RBX13DragUtilities9hitObjectERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
pub fn stub_2e1708() -> ! {
    todo!("0x2e1708 RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")
}

// 0x2e1860 — __ZN3RBX13DragUtilities12anyPartAliveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::anyPartAlive(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1860() -> ! {
    todo!("0x2e1860 RBX::DragUtilities::anyPartAlive(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e195c — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERS1_IPNS_9PrimitiveESaISB_EE
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>)")]
pub fn stub_2e195c() -> ! {
    todo!("0x2e195c RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>)")
}

// 0x2e1b90 — __ZN3RBX13DragUtilities10pvsToPartsERKSt6vectorIPNS_10PVInstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
#[doc(alias = "RBX::DragUtilities::pvsToParts(std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
pub fn stub_2e1b90() -> ! {
    todo!("0x2e1b90 RBX::DragUtilities::pvsToParts(std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")
}

// 0x2e1bc0 — __ZN3RBX13DragUtilities16instancesToPartsERKSt6vectorIPNS_8InstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
#[doc(alias = "RBX::DragUtilities::instancesToParts(std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
pub fn stub_2e1bc0() -> ! {
    todo!("0x2e1bc0 RBX::DragUtilities::instancesToParts(std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")
}

// 0x2e1bf4 — __ZN3RBX13DragUtilities19unJoinFromOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::unJoinFromOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1bf4() -> ! {
    todo!("0x2e1bf4 RBX::DragUtilities::unJoinFromOutsiders(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e1cc0 — __ZN3RBX13DragUtilities15joinToOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::joinToOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1cc0() -> ! {
    todo!("0x2e1cc0 RBX::DragUtilities::joinToOutsiders(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e1d90 — __ZN3RBX13DragUtilities4joinERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::join(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1d90() -> ! {
    todo!("0x2e1d90 RBX::DragUtilities::join(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e1ed8 — __ZN3RBX13DragUtilities19joinWithInPartsOnlyERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::joinWithInPartsOnly(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1ed8() -> ! {
    todo!("0x2e1ed8 RBX::DragUtilities::joinWithInPartsOnly(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e1fa4 — __ZN3RBX13DragUtilities11setDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::setDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e1fa4() -> ! {
    todo!("0x2e1fa4 RBX::DragUtilities::setDragging(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e20f8 — __ZN3RBX13DragUtilities12stopDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::stopDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e20f8() -> ! {
    todo!("0x2e20f8 RBX::DragUtilities::stopDragging(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e2290 — __ZN3RBX13DragUtilities11alignToGridEPNS_12PartInstanceE
#[doc(alias = "RBX::DragUtilities::alignToGrid(RBX::PartInstance *)")]
pub fn stub_2e2290() -> ! {
    todo!("0x2e2290 RBX::DragUtilities::alignToGrid(RBX::PartInstance *)")
}

// 0x2e2300 — __ZN3RBX13DragUtilities12moveAndCleanEPNS_12PartInstanceERKN3G3D7Vector3E
#[doc(alias = "RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")]
pub fn stub_2e2300() -> ! {
    todo!("0x2e2300 RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")
}

// 0x2e23e4 — __ZN3RBX13DragUtilities5cleanEPNS_12PartInstanceE
#[doc(alias = "RBX::DragUtilities::clean(RBX::PartInstance *)")]
pub fn stub_2e23e4() -> ! {
    todo!("0x2e23e4 RBX::DragUtilities::clean(RBX::PartInstance *)")
}

// 0x2e2400 — __ZN3RBX13DragUtilities5cleanERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::clean(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e2400() -> ! {
    todo!("0x2e2400 RBX::DragUtilities::clean(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e24f0 — __ZN3RBX13DragUtilities4moveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EEN3G3D15CoordinateFrameESB_
#[doc(alias = "RBX::DragUtilities::move(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")]
pub fn stub_2e24f0() -> ! {
    todo!("0x2e24f0 RBX::DragUtilities::move(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")
}

// 0x2e26d4 — __ZN3RBX13DragUtilities6toGridERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_2e26d4() -> ! {
    todo!("0x2e26d4 RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x2e27ec — __ZN3RBX13DragUtilities14computeExtentsERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::DragUtilities::computeExtents(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_2e27ec() -> ! {
    todo!("0x2e27ec RBX::DragUtilities::computeExtents(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)")
}

// 0x2e28b8 — __ZN3RBX13DragUtilities13getPrimitivesEPKNS_8InstanceERSt6vectorIPNS_9PrimitiveESaIS6_EE
#[doc(alias = "RBX::DragUtilities::getPrimitives(RBX::Instance const*,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
pub fn stub_2e28b8() -> ! {
    todo!("0x2e28b8 RBX::DragUtilities::getPrimitives(RBX::Instance const*,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")
}

// 0x2e28bc — __ZN3RBX13DragUtilities18getPrimitivesConstEPKNS_8InstanceERSt6vectorIPKNS_9PrimitiveESaIS7_EE
#[doc(alias = "RBX::DragUtilities::getPrimitivesConst(RBX::Instance const*,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
pub fn stub_2e28bc() -> ! {
    todo!("0x2e28bc RBX::DragUtilities::getPrimitivesConst(RBX::Instance const*,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")
}

// 0x2e2948 — __GLOBAL__I_a_88
#[doc(alias = "global constructor keyed to_a_88")]
pub fn stub_2e2948() -> ! {
    todo!("0x2e2948 `global constructor keyed to'_a_88")
}

// 0x2e2cbc — __GLOBAL__I_a_89
#[doc(alias = "global constructor keyed to_a_89")]
pub fn stub_2e2cbc() -> ! {
    todo!("0x2e2cbc `global constructor keyed to'_a_89")
}

// 0x2e2f2c — __ZN3RBX8GameToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
pub fn stub_2e2f2c() -> ! {
    todo!("0x2e2f2c RBX::GameTool::GameTool(RBX::Workspace *)")
}

// 0x2e2f30 — __ZN3RBX8GameToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
pub fn stub_2e2f30() -> ! {
    todo!("0x2e2f30 RBX::GameTool::GameTool(RBX::Workspace *)")
}

// 0x2e3044 — __ZN3RBX8GameTool11onMouseIdleERKNS_7UIEventE
#[doc(alias = "RBX::GameTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_2e3044() -> ! {
    todo!("0x2e3044 RBX::GameTool::onMouseIdle(RBX::UIEvent const&)")
}

// 0x2e304c — __ZNK3RBX8GameTool13draggablePartEPKNS_12PartInstanceERKN3G3D7Vector3E
#[doc(alias = "RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")]
pub fn stub_2e304c() -> ! {
    todo!("0x2e304c RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")
}

// 0x2e3080 — __ZN3RBX8GameTool12onMouseHoverERKNS_7UIEventE
#[doc(alias = "RBX::GameTool::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_2e3080() -> ! {
    todo!("0x2e3080 RBX::GameTool::onMouseHover(RBX::UIEvent const&)")
}

// 0x2e30f4 — __ZN3RBX8GameTool11onMouseDownERKNS_7UIEventE
#[doc(alias = "RBX::GameTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_2e30f4() -> ! {
    todo!("0x2e30f4 RBX::GameTool::onMouseDown(RBX::UIEvent const&)")
}

// 0x2e3234 — __ZN3RBX8GameToolD0Ev
#[doc(alias = "RBX::GameTool::~GameTool()")]
pub fn stub_2e3234() -> ! {
    todo!("0x2e3234 RBX::GameTool::~GameTool()")
}

// 0x2e32d4 — __ZN3RBX8GameToolD1Ev
#[doc(alias = "RBX::GameTool::~GameTool()")]
pub fn stub_2e32d4() -> ! {
    todo!("0x2e32d4 RBX::GameTool::~GameTool()")
}

// 0x2e32d8 — __ZThn36_N3RBX8GameToolD0Ev
#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
pub fn stub_2e32d8() -> ! {
    todo!("0x2e32d8 `non-virtual thunk to'RBX::GameTool::~GameTool()")
}

// 0x2e32e0 — __ZN3RBX8GameToolD2Ev
#[doc(alias = "RBX::GameTool::~GameTool()")]
pub fn stub_2e32e0() -> ! {
    todo!("0x2e32e0 RBX::GameTool::~GameTool()")
}

// 0x2e33e4 — __ZThn36_N3RBX8GameToolD1Ev
#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
pub fn stub_2e33e4() -> ! {
    todo!("0x2e33e4 `non-virtual thunk to'RBX::GameTool::~GameTool()")
}

// 0x2e33ec — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")]
pub fn stub_2e33ec() -> ! {
    todo!("0x2e33ec __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")
}

// 0x2e3414 — __ZNK3RBX8GameTool8isStickyEv
#[doc(alias = "RBX::GameTool::isSticky(void)const")]
pub fn stub_2e3414() -> ! {
    todo!("0x2e3414 RBX::GameTool::isSticky(void)const")
}

// 0x2e34dc — __ZNK3RBX8GameTool14drawConnectorsEv
#[doc(alias = "RBX::GameTool::drawConnectors(void)const")]
pub fn stub_2e34dc() -> ! {
    todo!("0x2e34dc RBX::GameTool::drawConnectors(void)const")
}

// 0x2e34e0 — __ZNK3RBX8GameTool13getCursorNameEv
#[doc(alias = "RBX::GameTool::getCursorName(void)const")]
pub fn stub_2e34e0() -> ! {
    todo!("0x2e34e0 RBX::GameTool::getCursorName(void)const")
}

// 0x2e34ec — __GLOBAL__I_a_90
#[doc(alias = "global constructor keyed to_a_90")]
pub fn stub_2e34ec() -> ! {
    todo!("0x2e34ec `global constructor keyed to'_a_90")
}

// 0x2e37c4 — __ZN3RBX8GrabToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
pub fn stub_2e37c4() -> ! {
    todo!("0x2e37c4 RBX::GrabTool::GrabTool(RBX::Workspace *)")
}

// 0x2e37c8 — __ZN3RBX8GrabToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
pub fn stub_2e37c8() -> ! {
    todo!("0x2e37c8 RBX::GrabTool::GrabTool(RBX::Workspace *)")
}

// 0x2e38e8 — __ZN3RBX8GrabTool11onMouseIdleERKNS_7UIEventE
#[doc(alias = "RBX::GrabTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_2e38e8() -> ! {
    todo!("0x2e38e8 RBX::GrabTool::onMouseIdle(RBX::UIEvent const&)")
}

// 0x2e38f0 — __ZN3RBX8GrabTool12onMouseHoverERKNS_7UIEventE
#[doc(alias = "RBX::GrabTool::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_2e38f0() -> ! {
    todo!("0x2e38f0 RBX::GrabTool::onMouseHover(RBX::UIEvent const&)")
}

// 0x2e395c — __ZN3RBX8GrabTool11onMouseDownERKNS_7UIEventE
#[doc(alias = "RBX::GrabTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_2e395c() -> ! {
    todo!("0x2e395c RBX::GrabTool::onMouseDown(RBX::UIEvent const&)")
}

// 0x2e3aa8 — __ZN3RBX8GrabToolD0Ev
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
pub fn stub_2e3aa8() -> ! {
    todo!("0x2e3aa8 RBX::GrabTool::~GrabTool()")
}

// 0x2e3b48 — __ZN3RBX8GrabToolD1Ev
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
pub fn stub_2e3b48() -> ! {
    todo!("0x2e3b48 RBX::GrabTool::~GrabTool()")
}

// 0x2e3b4c — __ZThn36_N3RBX8GrabToolD0Ev
#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
pub fn stub_2e3b4c() -> ! {
    todo!("0x2e3b4c `non-virtual thunk to'RBX::GrabTool::~GrabTool()")
}

// 0x2e3b54 — __ZN3RBX8GrabToolD2Ev
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
pub fn stub_2e3b54() -> ! {
    todo!("0x2e3b54 RBX::GrabTool::~GrabTool()")
}

// 0x2e3c58 — __ZThn36_N3RBX8GrabToolD1Ev
#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
pub fn stub_2e3c58() -> ! {
    todo!("0x2e3c58 `non-virtual thunk to'RBX::GrabTool::~GrabTool()")
}

// 0x2e3c60 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")]
pub fn stub_2e3c60() -> ! {
    todo!("0x2e3c60 __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")
}

// 0x2e3c88 — __ZNK3RBX8GrabTool8isStickyEv
#[doc(alias = "RBX::GrabTool::isSticky(void)const")]
pub fn stub_2e3c88() -> ! {
    todo!("0x2e3c88 RBX::GrabTool::isSticky(void)const")
}

// 0x2e3d50 — __ZNK3RBX8GrabTool14drawConnectorsEv
#[doc(alias = "RBX::GrabTool::drawConnectors(void)const")]
pub fn stub_2e3d50() -> ! {
    todo!("0x2e3d50 RBX::GrabTool::drawConnectors(void)const")
}

// 0x2e3d54 — __ZNK3RBX8GrabTool13getCursorNameEv
#[doc(alias = "RBX::GrabTool::getCursorName(void)const")]
pub fn stub_2e3d54() -> ! {
    todo!("0x2e3d54 RBX::GrabTool::getCursorName(void)const")
}

// 0x2e3d60 — __GLOBAL__I_a_91
#[doc(alias = "global constructor keyed to_a_91")]
pub fn stub_2e3d60() -> ! {
    todo!("0x2e3d60 `global constructor keyed to'_a_91")
}

// 0x2e3fd0 — __GLOBAL__I_a_92
#[doc(alias = "global constructor keyed to_a_92")]
pub fn stub_2e3fd0() -> ! {
    todo!("0x2e3fd0 `global constructor keyed to'_a_92")
}

// 0x2e4240 — __GLOBAL__I_a_93
#[doc(alias = "global constructor keyed to_a_93")]
pub fn stub_2e4240() -> ! {
    todo!("0x2e4240 `global constructor keyed to'_a_93")
}

// 0x2e4518 — __ZN3RBX10HammerToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
pub fn stub_2e4518() -> ! {
    todo!("0x2e4518 RBX::HammerTool::HammerTool(RBX::Workspace *)")
}

// 0x2e451c — __ZN3RBX10HammerToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
pub fn stub_2e451c() -> ! {
    todo!("0x2e451c RBX::HammerTool::HammerTool(RBX::Workspace *)")
}

// 0x2e4624 — __ZN3RBX10HammerToolD0Ev
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
pub fn stub_2e4624() -> ! {
    todo!("0x2e4624 RBX::HammerTool::~HammerTool()")
}

// 0x2e46c4 — __ZN3RBX10HammerToolD1Ev
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
pub fn stub_2e46c4() -> ! {
    todo!("0x2e46c4 RBX::HammerTool::~HammerTool()")
}

// 0x2e46c8 — __ZThn36_N3RBX10HammerToolD0Ev
#[doc(alias = "non-virtual thunk toRBX::HammerTool::~HammerTool()")]
pub fn stub_2e46c8() -> ! {
    todo!("0x2e46c8 `non-virtual thunk to'RBX::HammerTool::~HammerTool()")
}

// 0x2e46d0 — __ZN3RBX10HammerToolD2Ev
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
pub fn stub_2e46d0() -> ! {
    todo!("0x2e46d0 RBX::HammerTool::~HammerTool()")
}

// 0x2e47ec — __ZThn36_N3RBX10HammerToolD1Ev
#[doc(alias = "non-virtual thunk toRBX::HammerTool::~HammerTool()")]
pub fn stub_2e47ec() -> ! {
    todo!("0x2e47ec `non-virtual thunk to'RBX::HammerTool::~HammerTool()")
}

// 0x2e47f4 — __ZN3RBX10HammerTool11onMouseIdleERKNS_7UIEventE
#[doc(alias = "RBX::HammerTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_2e47f4() -> ! {
    todo!("0x2e47f4 RBX::HammerTool::onMouseIdle(RBX::UIEvent const&)")
}

// 0x2e48cc — __ZN3RBX10HammerTool11onMouseDownERKNS_7UIEventE
#[doc(alias = "RBX::HammerTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_2e48cc() -> ! {
    todo!("0x2e48cc RBX::HammerTool::onMouseDown(RBX::UIEvent const&)")
}

// 0x2e4a2c — __ZNK3RBX10HammerTool13getCursorNameEv
#[doc(alias = "RBX::HammerTool::getCursorName(void)const")]
pub fn stub_2e4a2c() -> ! {
    todo!("0x2e4a2c RBX::HammerTool::getCursorName(void)const")
}

// 0x2e4a5c — __ZN3RBX10HammerTool13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_2e4a5c() -> ! {
    todo!("0x2e4a5c RBX::HammerTool::render3dAdorn(RBX::Adorn *)")
}

// 0x2e4a70 — __ZThn4_N3RBX10HammerTool13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_2e4a70() -> ! {
    todo!("0x2e4a70 `non-virtual thunk to'RBX::HammerTool::render3dAdorn(RBX::Adorn *)")
}

// 0x2e4a84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ExplosionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")]
pub fn stub_2e4a84() -> ! {
    todo!("0x2e4a84 boost::shared_ptr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")
}

// 0x2e4b34 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")]
pub fn stub_2e4b34() -> ! {
    todo!("0x2e4b34 __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")
}

// 0x2e4b5c — __ZNK3RBX10HammerTool8isStickyEv
#[doc(alias = "RBX::HammerTool::isSticky(void)const")]
pub fn stub_2e4b5c() -> ! {
    todo!("0x2e4b5c RBX::HammerTool::isSticky(void)const")
}

// 0x2e4c24 — __ZN5boost10shared_ptrIN3RBX9ExplosionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2e4c24() -> ! {
    todo!("0x2e4c24 boost::shared_ptr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e4cec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(rbx_core::SharedPtr<RBX::Explosion> const*,RBX::Explosion *)const")]
pub fn stub_2e4cec() -> ! {
    todo!("0x2e4cec void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(boost::shared_ptr<RBX::Explosion> const*,RBX::Explosion *)const")
}

// 0x2e4dd8 — __ZN5boost6detail12shared_countC2IPN3RBX9ExplosionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2e4dd8() -> ! {
    todo!("0x2e4dd8 boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e4ee0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2e4ee0() -> ! {
    todo!("0x2e4ee0 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e4ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2e4ee4() -> ! {
    todo!("0x2e4ee4 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2e4f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2e4f04() -> ! {
    todo!("0x2e4f04 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2e4f1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2e4f1c() -> ! {
    todo!("0x2e4f1c boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2e4f20 — __GLOBAL__I_a_94
#[doc(alias = "global constructor keyed to_a_94")]
pub fn stub_2e4f20() -> ! {
    todo!("0x2e4f20 `global constructor keyed to'_a_94")
}

// 0x2e51d0 — __ZN3RBX10LuaDragger15mouseDownPublicEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS2_IKSt6vectorIS4_SaIS4_EEEE
#[doc(alias = "RBX::LuaDragger::mouseDownPublic(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
pub fn stub_2e51d0() -> ! {
    todo!("0x2e51d0 RBX::LuaDragger::mouseDownPublic(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)")
}

// 0x2e56d4 — __ZN3RBX10LuaDragger9mouseMoveENS_6RbxRayE
#[doc(alias = "RBX::LuaDragger::mouseMove(RBX::RbxRay)")]
pub fn stub_2e56d4() -> ! {
    todo!("0x2e56d4 RBX::LuaDragger::mouseMove(RBX::RbxRay)")
}

// 0x2e59b4 — __ZN3RBX10LuaDragger7mouseUpEv
#[doc(alias = "RBX::LuaDragger::mouseUp(void)")]
pub fn stub_2e59b4() -> ! {
    todo!("0x2e59b4 RBX::LuaDragger::mouseUp(void)")
}

// 0x2e5b88 — __ZN3RBX10LuaDragger10axisRotateEN3G3D7Vector34AxisE
#[doc(alias = "RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")]
pub fn stub_2e5b88() -> ! {
    todo!("0x2e5b88 RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")
}

// 0x2e5c24 — __ZN3RBX10LuaDraggerC2Ev
#[doc(alias = "RBX::LuaDragger::LuaDragger(void)")]
pub fn stub_2e5c24() -> ! {
    todo!("0x2e5c24 RBX::LuaDragger::LuaDragger(void)")
}

// 0x2e5e10 — __ZN3RBX10LuaDraggerD0Ev
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e5e10() -> ! {
    todo!("0x2e5e10 RBX::LuaDragger::~LuaDragger()")
}

// 0x2e5eb0 — __ZN3RBX10LuaDraggerD1Ev
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e5eb0() -> ! {
    todo!("0x2e5eb0 RBX::LuaDragger::~LuaDragger()")
}

// 0x2e5eb4 — __ZThn32_N3RBX10LuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e5eb4() -> ! {
    todo!("0x2e5eb4 `non-virtual thunk to'RBX::LuaDragger::~LuaDragger()")
}

// 0x2e5ebc — __ZThn36_N3RBX10LuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e5ebc() -> ! {
    todo!("0x2e5ebc `non-virtual thunk to'RBX::LuaDragger::~LuaDragger()")
}

// 0x2e5ec4 — __ZN3RBX10LuaDraggerD2Ev
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e5ec4() -> ! {
    todo!("0x2e5ec4 RBX::LuaDragger::~LuaDragger()")
}

// 0x2e6060 — __ZThn32_N3RBX10LuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e6060() -> ! {
    todo!("0x2e6060 `non-virtual thunk to'RBX::LuaDragger::~LuaDragger()")
}

// 0x2e6068 — __ZThn36_N3RBX10LuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
pub fn stub_2e6068() -> ! {
    todo!("0x2e6068 `non-virtual thunk to'RBX::LuaDragger::~LuaDragger()")
}

// 0x2e6070 — __ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
#[doc(alias = "RBX::LuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
pub fn stub_2e6070() -> ! {
    todo!("0x2e6070 RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)")
}
