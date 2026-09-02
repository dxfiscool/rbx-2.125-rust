// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x2db7a8..0x302cdc | total filtered 10215, remaining 5986 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ' stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x2db7a8 — __ZN3RBX9CloneToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::CloneTool::CloneTool(RBX::Workspace *)")]
pub fn stub_0x2db7a8() -> ! {
    todo!("0x2db7a8 RBX::CloneTool::CloneTool(RBX::Workspace *)")
}

// 0x2dbe5c — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12PartDragToolEPNS_12PartInstanceEN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEEEENSC_IT_EET0_T1_T2_T3_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2dbe5c() -> ! {
    todo!("0x2dbe5c rbx_core::SharedPtr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2e06d0 — __ZN3RBX8DragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
#[doc(alias = "RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e06d0() -> ! {
    todo!("0x2e06d0 RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2e08bc — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e08bc() -> ! {
    todo!("0x2e08bc rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2e1bc0 — __ZN3RBX13DragUtilities16instancesToPartsERKSt6vectorIPNS_8InstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
#[doc(alias = "RBX::DragUtilities::instancesToParts(std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
pub fn stub_0x2e1bc0() -> ! {
    todo!("0x2e1bc0 RBX::DragUtilities::instancesToParts(std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")
}

// 0x2e28b8 — __ZN3RBX13DragUtilities13getPrimitivesEPKNS_8InstanceERSt6vectorIPNS_9PrimitiveESaIS6_EE
#[doc(alias = "RBX::DragUtilities::getPrimitives(RBX::Instance const*,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
pub fn stub_0x2e28b8() -> ! {
    todo!("0x2e28b8 RBX::DragUtilities::getPrimitives(RBX::Instance const*,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")
}

// 0x2e28bc — __ZN3RBX13DragUtilities18getPrimitivesConstEPKNS_8InstanceERSt6vectorIPKNS_9PrimitiveESaIS7_EE
#[doc(alias = "RBX::DragUtilities::getPrimitivesConst(RBX::Instance const*,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
pub fn stub_0x2e28bc() -> ! {
    todo!("0x2e28bc RBX::DragUtilities::getPrimitivesConst(RBX::Instance const*,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")
}

// 0x2e2f2c — __ZN3RBX8GameToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
pub fn stub_0x2e2f2c() -> ! {
    todo!("0x2e2f2c RBX::GameTool::GameTool(RBX::Workspace *)")
}

// 0x2e2f30 — __ZN3RBX8GameToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
pub fn stub_0x2e2f30() -> ! {
    todo!("0x2e2f30 RBX::GameTool::GameTool(RBX::Workspace *)")
}

// 0x2e37c4 — __ZN3RBX8GrabToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
pub fn stub_0x2e37c4() -> ! {
    todo!("0x2e37c4 RBX::GrabTool::GrabTool(RBX::Workspace *)")
}

// 0x2e37c8 — __ZN3RBX8GrabToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
pub fn stub_0x2e37c8() -> ! {
    todo!("0x2e37c8 RBX::GrabTool::GrabTool(RBX::Workspace *)")
}

// 0x2e4518 — __ZN3RBX10HammerToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
pub fn stub_0x2e4518() -> ! {
    todo!("0x2e4518 RBX::HammerTool::HammerTool(RBX::Workspace *)")
}

// 0x2e451c — __ZN3RBX10HammerToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
pub fn stub_0x2e451c() -> ! {
    todo!("0x2e451c RBX::HammerTool::HammerTool(RBX::Workspace *)")
}

// 0x2e4a84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ExplosionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")]
// was: boost::shared_ptr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)
pub fn stub_0x2e4a84() -> ! {
    todo!("0x2e4a84 rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")
}

// 0x2e4c24 — __ZN5boost10shared_ptrIN3RBX9ExplosionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2e4c24() -> ! {
    todo!("0x2e4c24 rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e4dd8 — __ZN5boost6detail12shared_countC2IPN3RBX9ExplosionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2e4dd8() -> ! {
    todo!("0x2e4dd8 boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e4ee0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2e4ee0() -> ! {
    todo!("0x2e4ee0 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e4ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2e4ee4() -> ! {
    todo!("0x2e4ee4 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2e4f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2e4f04() -> ! {
    todo!("0x2e4f04 boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2e4f1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2e4f1c() -> ! {
    todo!("0x2e4f1c boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2e51d0 — __ZN3RBX10LuaDragger15mouseDownPublicEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS2_IKSt6vectorIS4_SaIS4_EEEE
#[doc(alias = "RBX::LuaDragger::mouseDownPublic(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
// was: RBX::LuaDragger::mouseDownPublic(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
pub fn stub_0x2e51d0() -> ! {
    todo!("0x2e51d0 RBX::LuaDragger::mouseDownPublic(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")
}

// 0x2e6d94 — __ZN3RBXL7addPartEN5boost10shared_ptrINS_8InstanceEEEPSt6vectorINS0_8weak_ptrINS_12PartInstanceEEESaIS7_EE
#[doc(alias = "RBX::addPart(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *)")]
// was: RBX::addPart(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *)
pub fn stub_0x2e6d94() -> ! {
    todo!("0x2e6d94 RBX::addPart(rbx_core::SharedPtr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *)")
}

// 0x2e700c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
pub fn stub_0x2e700c() -> ! {
    todo!("0x2e700c RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x2e716c — __ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2e716c() -> ! {
    todo!("0x2e716c rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x2e74dc — __ZNK3RBX10LuaDragger12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::LuaDragger::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x2e74dc() -> ! {
    todo!("0x2e74dc RBX::LuaDragger::askSetParent(RBX::Instance const*)const")
}

// 0x2e776c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10LuaDraggerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")]
// was: boost::shared_ptr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)
pub fn stub_0x2e776c() -> ! {
    todo!("0x2e776c rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")
}

// 0x2e781c — __ZN5boost10shared_ptrIN3RBX10LuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2e781c() -> ! {
    todo!("0x2e781c rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e79cc — __ZN5boost6detail12shared_countC2IPN3RBX10LuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2e79cc() -> ! {
    todo!("0x2e79cc boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2e7ad4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2e7ad4() -> ! {
    todo!("0x2e7ad4 boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e7ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2e7ad8() -> ! {
    todo!("0x2e7ad8 boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2e7adc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2e7adc() -> ! {
    todo!("0x2e7adc boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2e7afc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2e7afc() -> ! {
    todo!("0x2e7afc boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2e8128 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SD_SD_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&,std::random_access_iterator_tag)
pub fn stub_0x2e8128() -> ! {
    todo!("0x2e8128 __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")
}

// 0x2e8ee8 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x2e8ee8() -> ! {
    todo!("0x2e8ee8 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x2e9144 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0x2e9144() -> ! {
    todo!("0x2e9144 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x2e91ac — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
pub fn stub_0x2e91ac() -> ! {
    todo!("0x2e91ac RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x2e924c — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0x2e924c() -> ! {
    todo!("0x2e924c RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x2e9388 — __ZN3RBX10Reflection11Call3HelperINS_10LuaDraggerEMS2_FvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEES6_S8_SD_vE4callEPS2_SF_RNS0_7VariantERKS6_RKS8_RKSD_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
// was: RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,G3D::Vector3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)
pub fn stub_0x2e9388() -> ! {
    todo!("0x2e9388 RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")
}

// 0x2e96ac — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_0x2e96ac() -> ! {
    todo!("0x2e96ac rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x2e9870 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_PS9_INS2_8weak_ptrINS4_12PartInstanceEEESaISH_EEENSD_5list2INS2_3argILi1EEENSD_5valueISK_EEEEEEET0_T_SV_SU_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)
pub fn stub_0x2e9870() -> ! {
    todo!("0x2e9870 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")
}

// 0x2e98b8 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorINS_8weak_ptrIN3RBX12PartInstanceEEESaIS9_EEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEESC_ENS0_5list1IRKSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0x2e98b8() -> ! {
    todo!("0x2e98b8 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x2e998c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
pub fn stub_0x2e998c() -> ! {
    todo!("0x2e998c RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x2e9f80 — __ZN3RBX11LuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e9f80() -> ! {
    todo!("0x2e9f80 RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2e9f84 — __ZN3RBX11LuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e9f84() -> ! {
    todo!("0x2e9f84 RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2eee88 — __ZN3RBX8NullToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
pub fn stub_0x2eee88() -> ! {
    todo!("0x2eee88 RBX::NullTool::NullTool(RBX::Workspace *)")
}

// 0x2eee8c — __ZN3RBX8NullToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
pub fn stub_0x2eee8c() -> ! {
    todo!("0x2eee8c RBX::NullTool::NullTool(RBX::Workspace *)")
}

// 0x2ef12c — __ZN3RBX11NewNullToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
pub fn stub_0x2ef12c() -> ! {
    todo!("0x2ef12c RBX::NewNullTool::NewNullTool(RBX::Workspace *)")
}

// 0x2ef130 — __ZN3RBX11NewNullToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
pub fn stub_0x2ef130() -> ! {
    todo!("0x2ef130 RBX::NewNullTool::NewNullTool(RBX::Workspace *)")
}

// 0x2f0948 — __ZN3RBX12PartDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2f0948() -> ! {
    todo!("0x2f0948 RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2f094c — __ZN3RBX12PartDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2f094c() -> ! {
    todo!("0x2f094c RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2f2bf0 — __ZN3RBX10RunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::initLocal(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
pub fn stub_0x2f2bf0() -> ! {
    todo!("0x2f2bf0 RBX::RunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")
}

// 0x2f2ff8 — __ZN3RBX10RunDragger4initEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::init(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
pub fn stub_0x2f2ff8() -> ! {
    todo!("0x2f2ff8 RBX::RunDragger::init(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")
}

// 0x2f6ff4 — __ZN3RBX16BoxSelectCommandC2EPNS_9WorkspaceE
#[doc(alias = "RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")]
pub fn stub_0x2f6ff4() -> ! {
    todo!("0x2f6ff4 RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")
}

// 0x2f732c — __ZN3RBX16BoxSelectCommand9selectAndERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
#[doc(alias = "RBX::BoxSelectCommand::selectAnd(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: RBX::BoxSelectCommand::selectAnd(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_0x2f732c() -> ! {
    todo!("0x2f732c RBX::BoxSelectCommand::selectAnd(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0x2f7394 — __ZN3RBX16BoxSelectCommand13selectReverseERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
#[doc(alias = "RBX::BoxSelectCommand::selectReverse(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: RBX::BoxSelectCommand::selectReverse(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_0x2f7394() -> ! {
    todo!("0x2f7394 RBX::BoxSelectCommand::selectReverse(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0x2f75e8 — __ZN3RBX16BoxSelectCommand17getMouseInstancesERSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EERKNS_7UIEventERN3G3D6Rect2DE
#[doc(alias = "RBX::BoxSelectCommand::getMouseInstances(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")]
// was: RBX::BoxSelectCommand::getMouseInstances(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)
pub fn stub_0x2f75e8() -> ! {
    todo!("0x2f75e8 RBX::BoxSelectCommand::getMouseInstances(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")
}

// 0x2f78d8 — __ZNK3RBX9Selection10isSelectedEPKNS_8InstanceE
#[doc(alias = "RBX::Selection::isSelected(RBX::Instance const*)const")]
pub fn stub_0x2f78d8() -> ! {
    todo!("0x2f78d8 RBX::Selection::isSelected(RBX::Instance const*)const")
}

// 0x2f79c8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
// was: boost::shared_ptr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0x2f79c8() -> ! {
    todo!("0x2f79c8 rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x2f7a7c — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection11AddIteratorEET1_T_SA_T0_SB_S9_
#[doc(alias = "RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
// was: RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::AddIterator)
pub fn stub_0x2f7a7c() -> ! {
    todo!("0x2f7a7c RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")
}

// 0x2f7bd4 — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14RemoveIteratorEET1_T_SA_T0_SB_S9_
#[doc(alias = "RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
// was: RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::RemoveIterator)
pub fn stub_0x2f7bd4() -> ! {
    todo!("0x2f7bd4 RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")
}

// 0x2f7d2c — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14ToggleIteratorEET1_T_SA_T0_SB_S9_
#[doc(alias = "RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
// was: RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::ToggleIterator)
pub fn stub_0x2f7d2c() -> ! {
    todo!("0x2f7d2c RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")
}

// 0x2f7e84 — __ZN3RBX8Instance15queryTypedChildINS_10SelectableEEEPT_i
#[doc(alias = "RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")]
pub fn stub_0x2f7e84() -> ! {
    todo!("0x2f7e84 RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")
}

// 0x2f7fcc — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_insert_unique(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2f7fcc() -> ! {
    todo!("0x2f7fcc std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x2f8034 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2f8034() -> ! {
    todo!("0x2f8034 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x2f8080 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_create_node(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2f8080() -> ! {
    todo!("0x2f8080 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x2f8164 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::Instance>> *)
pub fn stub_0x2f8164() -> ! {
    todo!("0x2f8164 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")
}

// 0x2f818c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<boost::shared_ptr<RBX::Instance>> *)
pub fn stub_0x2f818c() -> ! {
    todo!("0x2f818c std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")
}

// 0x2f81a8 — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14ToggleIteratorEEET0_T_SD_SC_
#[doc(alias = "RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
// was: RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::ToggleIterator)
pub fn stub_0x2f81a8() -> ! {
    todo!("0x2f81a8 RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")
}

// 0x2f829c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EEaSERKSA_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::operator=(std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_0x2f829c() -> ! {
    todo!("0x2f829c std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0x2f82e8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSC_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")]
// was: std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<boost::shared_ptr<RBX::Instance>> const*,std::_Rb_tree_node<boost::shared_ptr<RBX::Instance>>*)
pub fn stub_0x2f82e8() -> ! {
    todo!("0x2f82e8 std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")
}

// 0x2f843c — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14RemoveIteratorEEET0_T_SD_SC_
#[doc(alias = "RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
// was: RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::RemoveIterator)
pub fn stub_0x2f843c() -> ! {
    todo!("0x2f843c RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")
}

// 0x2f8530 — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection11AddIteratorEEET0_T_SD_SC_
#[doc(alias = "RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
// was: RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::AddIterator)
pub fn stub_0x2f8530() -> ! {
    todo!("0x2f8530 RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")
}

// 0x2f88fc — __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_8InstanceEEEE13createServiceEv
#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")]
pub fn stub_0x2f88fc() -> ! {
    todo!("0x2f88fc RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")
}

// 0x2f89dc — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEaSERKS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
// was: boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>>::operator=(boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> const&)
pub fn stub_0x2f89dc() -> ! {
    todo!("0x2f89dc rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")
}

// 0x2f8a14 — __ZN3RBX11shared_fromINS_17FilteredSelectionINS_8InstanceEEEEEN5boost10shared_ptrIT_EEPS6_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")]
// was: boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)
pub fn stub_0x2f8a14() -> ! {
    todo!("0x2f8a14 rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")
}

// 0x2f8b84 — __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_PKS3_
#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")]
pub fn stub_0x2f8b84() -> ! {
    todo!("0x2f8b84 RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")
}

// 0x2f8b9c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKS5_EEET_SF_SF_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance const> const&,std::random_access_iterator_tag)
pub fn stub_0x2f8b9c() -> ! {
    todo!("0x2f8b9c __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")
}

// 0x2faa84 — __ZN3RBX14AsyncHttpQueueC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEi
#[doc(alias = "RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")]
pub fn stub_0x2faa84() -> ! {
    todo!("0x2faa84 RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")
}

// 0x2fc440 — __ZN3RBX14AsyncHttpQueue23dispatchGenericCallbackEN5boost8functionIFvPNS_9DataModelEEEEPNS_8InstanceENS0_9ResultJobE
#[doc(alias = "RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_0x2fc440() -> ! {
    todo!("0x2fc440 RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fc6a8 — __ZN3RBX14AsyncHttpQueue16dispatchCallbackEN5boost8functionIFvNS0_13RequestResultEPSiNS1_10shared_ptrIKSsEEEEEPNS_8InstanceES3_S7_NS0_9ResultJobE
#[doc(alias = "RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")]
// was: RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,RBX::AsyncHttpQueue::ResultJob)
pub fn stub_0x2fc6a8() -> ! {
    todo!("0x2fc6a8 RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fcfb4 — __ZN3RBXL8callbackENS_14AsyncHttpQueue15CallbackWrapperEPNS_8InstanceENS0_13RequestResultEN5boost10shared_ptrISsEE
#[doc(alias = "RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
// was: RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)
pub fn stub_0x2fcfb4() -> ! {
    todo!("0x2fcfb4 RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")
}

// 0x2fdf08 — __ZN3RBX18HttpQueueStatsItem6createEPNS_14AsyncHttpQueueEPNS_8InstanceE
#[doc(alias = "RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_0x2fdf08() -> ! {
    todo!("0x2fdf08 RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x2fe884 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX14AsyncHttpQueue15CallbackWrapperESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_PNS2_8InstanceENS3_13RequestResultENSA_10shared_ptrISsEEENSB_5list4INSA_3argILi1EEENSB_5valueISE_EENSN_ISF_EENSN_ISH_EEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>)")]
// was: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>)
pub fn stub_0x2fe884() -> ! {
    todo!("0x2fe884 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>)")
}

// 0x2fe8f4 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue15CallbackWrapperEPNS1_8InstanceENS2_13RequestResultENS_10shared_ptrISsEENS_3argILi1EEES5_S6_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
// was: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)
pub fn stub_0x2fe8f4() -> ! {
    todo!("0x2fe8f4 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")
}

// 0x2ff470 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEclIPFvNS9_15CallbackWrapperES7_SA_SD_ENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)")]
// was: void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)
pub fn stub_0x2ff470() -> ! {
    todo!("0x2ff470 void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)")
}

// 0x2ff588 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEC2ES3_S8_SB_SE_
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>)")]
// was: boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>)
pub fn stub_0x2ff588() -> ! {
    todo!("0x2ff588 boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>)")
}

// 0x300230 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>)
pub fn stub_0x300230() -> ! {
    todo!("0x300230 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>)")
}

// 0x3003b8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESP_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x3003b8() -> ! {
    todo!("0x3003b8 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x3003d4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x3003d4() -> ! {
    todo!("0x3003d4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &)const")
}

// 0x300530 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x300530() -> ! {
    todo!("0x300530 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x300688 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x300688() -> ! {
    todo!("0x300688 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x300798 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEclIPFvSC_S6_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0x300798() -> ! {
    todo!("0x300798 void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0x302324 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")]
// was: boost::shared_ptr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)
pub fn stub_0x302324() -> ! {
    todo!("0x302324 rbx_core::SharedPtr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")
}

// 0x302418 — __ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE
#[doc(alias = "RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_0x302418() -> ! {
    todo!("0x302418 RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x3029fc — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x3029fc() -> ! {
    todo!("0x3029fc rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302bac — __ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x302bac() -> ! {
    todo!("0x302bac boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302cb4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x302cb4() -> ! {
    todo!("0x302cb4 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x302cb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x302cb8() -> ! {
    todo!("0x302cb8 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x302cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x302cbc() -> ! {
    todo!("0x302cbc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x302cdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x302cdc() -> ! {
    todo!("0x302cdc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}
