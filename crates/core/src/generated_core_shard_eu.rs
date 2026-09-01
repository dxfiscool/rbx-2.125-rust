//! core shard EU — 100 core stubs EA-sorted, lowest uncovered 0xb8c808..0xbdadb8 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after ET 0xb8c644).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xb8c644.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "void std::__adjust_heap<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,int,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator)")]
// 0xb8c808 — __ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_
pub fn stub_b8c808() -> ! {
    todo!("0xb8c808 __ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_")
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")]
// 0xb8cc38 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_
pub fn stub_b8cc38() -> ! {
    todo!("0xb8cc38 __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")]
// 0xb8cd88 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_b8cd88() -> ! {
    todo!("0xb8cd88 __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")]
// 0xb8d500 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm
pub fn stub_b8d500() -> ! {
    todo!("0xb8d500 __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm")
}

#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")]
// 0xb8d608 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_
pub fn stub_b8d608() -> ! {
    todo!("0xb8d608 __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_")
}

#[doc(alias = "RBX::AdornRbxGfx::getFreeSubEntity(void)")]
// 0xb8edc8 — __ZN3RBX11AdornRbxGfx16getFreeSubEntityEv
pub fn stub_b8edc8() -> ! {
    todo!("0xb8edc8 __ZN3RBX11AdornRbxGfx16getFreeSubEntityEv")
}

#[doc(alias = "RBX::AdornRbxGfx::findMesh(std::string)")]
// 0xb8ef38 — __ZN3RBX11AdornRbxGfx8findMeshESs
pub fn stub_b8ef38() -> ! {
    todo!("0xb8ef38 __ZN3RBX11AdornRbxGfx8findMeshESs")
}

#[doc(alias = "RBX::AdornRbxGfx::getViewport(void)const")]
// 0xb8f338 — __ZNK3RBX11AdornRbxGfx11getViewportEv
pub fn stub_b8f338() -> ! {
    todo!("0xb8f338 __ZNK3RBX11AdornRbxGfx11getViewportEv")
}

#[doc(alias = "RBX::AdornRbxGfx::getCamera(void)const")]
// 0xb8f3f8 — __ZNK3RBX11AdornRbxGfx9getCameraEv
pub fn stub_b8f3f8() -> ! {
    todo!("0xb8f3f8 __ZNK3RBX11AdornRbxGfx9getCameraEv")
}

#[doc(alias = "RBX::AdornRbxGfx::setTexture(int,rbx_core::SharedPtr<RBX::TextureProxyBase> const&)")]
// 0xb8f414 — __ZN3RBX11AdornRbxGfx10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// was: RBX::AdornRbxGfx::setTexture(int,boost::shared_ptr<RBX::TextureProxyBase> const&)
pub fn stub_b8f414() -> ! {
    todo!("0xb8f414 __ZN3RBX11AdornRbxGfx10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE")
}

#[doc(alias = "RBX::AdornRbxGfx::getTextureSize(rbx_core::SharedPtr<RBX::TextureProxyBase> const&)const")]
// 0xb8f560 — __ZNK3RBX11AdornRbxGfx14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// was: RBX::AdornRbxGfx::getTextureSize(boost::shared_ptr<RBX::TextureProxyBase> const&)const
pub fn stub_b8f560() -> ! {
    todo!("0xb8f560 __ZNK3RBX11AdornRbxGfx14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE")
}

#[doc(alias = "RBX::AdornRbxGfx::createTextureProxy(RBX::ContentId const&,bool &,bool)")]
// 0xb90148 — __ZN3RBX11AdornRbxGfx18createTextureProxyERKNS_9ContentIdERbb
pub fn stub_b90148() -> ! {
    todo!("0xb90148 __ZN3RBX11AdornRbxGfx18createTextureProxyERKNS_9ContentIdERbb")
}

#[doc(alias = "RBX::AdornRbxGfx::finishRenderPass(void)")]
// 0xb91764 — __ZN3RBX11AdornRbxGfx16finishRenderPassEv
pub fn stub_b91764() -> ! {
    todo!("0xb91764 __ZN3RBX11AdornRbxGfx16finishRenderPassEv")
}

#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// 0xb95acc — __ZN3RBX11AdornRbxGfxD0Ev
pub fn stub_b95acc() -> ! {
    todo!("0xb95acc __ZN3RBX11AdornRbxGfxD0Ev")
}

#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// 0xb95b6c — __ZN3RBX11AdornRbxGfxD1Ev
pub fn stub_b95b6c() -> ! {
    todo!("0xb95b6c __ZN3RBX11AdornRbxGfxD1Ev")
}

#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// 0xb95b70 — __ZN3RBX11AdornRbxGfxD2Ev
pub fn stub_b95b70() -> ! {
    todo!("0xb95b70 __ZN3RBX11AdornRbxGfxD2Ev")
}

#[doc(alias = "RBX::AdornRbxGfx::destroyAll(void)")]
// 0xb96238 — __ZN3RBX11AdornRbxGfx10destroyAllEv
pub fn stub_b96238() -> ! {
    todo!("0xb96238 __ZN3RBX11AdornRbxGfx10destroyAllEv")
}

#[doc(alias = "RBX::AdornRbxGfx::preSubmitPass(void)")]
// 0xb9834c — __ZN3RBX11AdornRbxGfx13preSubmitPassEv
pub fn stub_b9834c() -> ! {
    todo!("0xb9834c __ZN3RBX11AdornRbxGfx13preSubmitPassEv")
}

#[doc(alias = "RBX::AdornRbxGfx::postSubmitPass(void)")]
// 0xb98388 — __ZN3RBX11AdornRbxGfx14postSubmitPassEv
pub fn stub_b98388() -> ! {
    todo!("0xb98388 __ZN3RBX11AdornRbxGfx14postSubmitPassEv")
}

#[doc(alias = "RBX::AdornRbxGfx::prepareRenderPass(void)")]
// 0xb983c8 — __ZN3RBX11AdornRbxGfx17prepareRenderPassEv
pub fn stub_b983c8() -> ! {
    todo!("0xb983c8 __ZN3RBX11AdornRbxGfx17prepareRenderPassEv")
}

#[doc(alias = "RBX::AdornRbxGfx::getRenderCaps(void)const")]
// 0xb985fc — __ZNK3RBX11AdornRbxGfx13getRenderCapsEv
pub fn stub_b985fc() -> ! {
    todo!("0xb985fc __ZNK3RBX11AdornRbxGfx13getRenderCapsEv")
}

#[doc(alias = "RBX::Adorn::~Adorn()")]
// 0xb98660 — __ZN3RBX5AdornD2Ev
pub fn stub_b98660() -> ! {
    todo!("0xb98660 __ZN3RBX5AdornD2Ev")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TextureProxyBase>::_internal_accept_owner<RBX::TextureProxyBase,RBX::RbxTextureProxy>(rbx_core::SharedPtr<RBX::TextureProxyBase> const*,RBX::RbxTextureProxy *)const")]
// 0xb994b4 — __ZNK5boost23enable_shared_from_thisIN3RBX16TextureProxyBaseEE22_internal_accept_ownerIS2_NS1_15RbxTextureProxyEEEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TextureProxyBase>::_internal_accept_owner<RBX::TextureProxyBase,RBX::RbxTextureProxy>(boost::shared_ptr<RBX::TextureProxyBase> const*,RBX::RbxTextureProxy *)const
pub fn stub_b994b4() -> ! {
    todo!("0xb994b4 __ZNK5boost23enable_shared_from_thisIN3RBX16TextureProxyBaseEE22_internal_accept_ownerIS2_NS1_15RbxTextureProxyEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::~sp_counted_impl_p()")]
// 0xb9960c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED1Ev
pub fn stub_b9960c() -> ! {
    todo!("0xb9960c __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::~sp_counted_impl_p()")]
// 0xb99610 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED0Ev
pub fn stub_b99610() -> ! {
    todo!("0xb99610 __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::dispose(void)")]
// 0xb99614 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE7disposeEv
pub fn stub_b99614() -> ! {
    todo!("0xb99614 __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::get_deleter(std::type_info const&)")]
// 0xb99624 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE11get_deleterERKSt9type_info
pub fn stub_b99624() -> ! {
    todo!("0xb99624 __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::get_untyped_deleter(void)")]
// 0xb99628 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE19get_untyped_deleterEv
pub fn stub_b99628() -> ! {
    todo!("0xb99628 __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::MeshGen::addRefVertex(int)")]
// 0xb9a3e4 — __ZN3RBX7MeshGen12addRefVertexEi
pub fn stub_b9a3e4() -> ! {
    todo!("0xb9a3e4 __ZN3RBX7MeshGen12addRefVertexEi")
}

#[doc(alias = "RBX::MeshGen::releaseVertex(int)")]
// 0xb9a3e8 — __ZN3RBX7MeshGen13releaseVertexEi
pub fn stub_b9a3e8() -> ! {
    todo!("0xb9a3e8 __ZN3RBX7MeshGen13releaseVertexEi")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getVertex(int)")]
// 0xb9a3ec — __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi
pub fn stub_b9a3ec() -> ! {
    todo!("0xb9a3ec __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)")]
// 0xb9a50c — __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm
pub fn stub_b9a50c() -> ! {
    todo!("0xb9a50c __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)")]
// 0xb9a524 — __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE
pub fn stub_b9a524() -> ! {
    todo!("0xb9a524 __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)")]
// 0xb9a558 — __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm
pub fn stub_b9a558() -> ! {
    todo!("0xb9a558 __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)")]
// 0xb9a568 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii
pub fn stub_b9a568() -> ! {
    todo!("0xb9a568 __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)")]
// 0xb9a580 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii
pub fn stub_b9a580() -> ! {
    todo!("0xb9a580 __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)")]
// 0xb9a5b0 — __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii
pub fn stub_b9a5b0() -> ! {
    todo!("0xb9a5b0 __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getIndexCount(void)")]
// 0xb9a6d0 — __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv
pub fn stub_b9a6d0() -> ! {
    todo!("0xb9a6d0 __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)")]
// 0xb9a6d4 — __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv
pub fn stub_b9a6d4() -> ! {
    todo!("0xb9a6d4 __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)")]
// 0xb9a7f4 — __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv
pub fn stub_b9a7f4() -> ! {
    todo!("0xb9a7f4 __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv")
}

#[doc(alias = "RBX::MeshGen::popVerticesTransform(void)")]
// 0xb9a918 — __ZN3RBX7MeshGen20popVerticesTransformEv
pub fn stub_b9a918() -> ! {
    todo!("0xb9a918 __ZN3RBX7MeshGen20popVerticesTransformEv")
}

#[doc(alias = "RBX::Adorn::~Adorn()")]
// 0xb9aa18 — __ZN3RBX5AdornD1Ev
pub fn stub_b9aa18() -> ! {
    todo!("0xb9aa18 __ZN3RBX5AdornD1Ev")
}

#[doc(alias = "RBX::Adorn::finishRenderPass(void)")]
// 0xb9aa20 — __ZN3RBX5Adorn16finishRenderPassEv
pub fn stub_b9aa20() -> ! {
    todo!("0xb9aa20 __ZN3RBX5Adorn16finishRenderPassEv")
}

#[doc(alias = "RBX::Adorn::postSubmitPass(void)")]
// 0xb9aa28 — __ZN3RBX5Adorn14postSubmitPassEv
pub fn stub_b9aa28() -> ! {
    todo!("0xb9aa28 __ZN3RBX5Adorn14postSubmitPassEv")
}

#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::create<RBX::MeshContentProvider>(void)const")]
// 0xbb3ff4 — __ZNK3RBX15ServiceProvider6createINS_19MeshContentProviderEEEPT_v
pub fn stub_bb3ff4() -> ! {
    todo!("0xbb3ff4 __ZNK3RBX15ServiceProvider6createINS_19MeshContentProviderEEEPT_v")
}

#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::find<RBX::MeshContentProvider>(void)const")]
// 0xbb4280 — __ZNK3RBX15ServiceProvider4findINS_19MeshContentProviderEEEPT_v
pub fn stub_bb4280() -> ! {
    todo!("0xbb4280 __ZNK3RBX15ServiceProvider4findINS_19MeshContentProviderEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::MeshContentProvider>(void)")]
// 0xbb4810 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19MeshContentProviderEEEvv
pub fn stub_bb4810() -> ! {
    todo!("0xbb4810 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19MeshContentProviderEEEvv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Image *,RBX::Image * (*)(std::istream &,std::string const&,int,bool),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<int>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xbb4a80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIPN3RBX5ImageEPFS7_RSiRKSsibENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueIiEENSH_IbEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
pub fn stub_bb4a80() -> ! {
    todo!("0xbb4a80 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIPN3RBX5ImageEPFS7_RSiRKSsibENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueIiEENSH_IbEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Image *,RBX::Image * (*)(std::istream &,std::string const&,int,bool),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<int>,boost::_bi::value<bool>>>,RBX::Image *,std::istream &,std::string const&>::invoke(boost::detail::function::function_buffer &,std::istream &,std::string const&)")]
// 0xbb4ae0 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIPN3RBX5ImageEPFS7_RSiRKSsibENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueIiEENSH_IbEEEEEES7_S8_SA_E6invokeERNS1_15function_bufferES8_SA_
pub fn stub_bb4ae0() -> ! {
    todo!("0xbb4ae0 __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIPN3RBX5ImageEPFS7_RSiRKSsibENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueIiEENSH_IbEEEEEES7_S8_SA_E6invokeERNS1_15function_bufferES8_SA_")
}

#[doc(alias = "RBX::RbxParticleEmitter::~RbxParticleEmitter()")]
// 0xbbe678 — __ZN3RBX18RbxParticleEmitterD0Ev
pub fn stub_bbe678() -> ! {
    todo!("0xbbe678 __ZN3RBX18RbxParticleEmitterD0Ev")
}

#[doc(alias = "RBX::RbxParticleEmitter::~RbxParticleEmitter()")]
// 0xbbe7ac — __ZN3RBX18RbxParticleEmitterD1Ev
pub fn stub_bbe7ac() -> ! {
    todo!("0xbbe7ac __ZN3RBX18RbxParticleEmitterD1Ev")
}

#[doc(alias = "RBX::RbxParticleEmitter::onSleepingChanged(bool)")]
// 0xbbe8c4 — __ZN3RBX18RbxParticleEmitter17onSleepingChangedEb
pub fn stub_bbe8c4() -> ! {
    todo!("0xbbe8c4 __ZN3RBX18RbxParticleEmitter17onSleepingChangedEb")
}

#[doc(alias = "non-virtual thunk toRBX::RbxParticleEmitter::onSleepingChanged(bool)")]
// 0xbbe9e8 — __ZThn392_N3RBX18RbxParticleEmitter17onSleepingChangedEb
pub fn stub_bbe9e8() -> ! {
    todo!("0xbbe9e8 __ZThn392_N3RBX18RbxParticleEmitter17onSleepingChangedEb")
}

#[doc(alias = "RBX::RbxParticleEmitter::updateCoordinateFrame(bool)")]
// 0xbbe9f0 — __ZN3RBX18RbxParticleEmitter21updateCoordinateFrameEb
pub fn stub_bbe9f0() -> ! {
    todo!("0xbbe9f0 __ZN3RBX18RbxParticleEmitter21updateCoordinateFrameEb")
}

#[doc(alias = "non-virtual thunk toRBX::RbxParticleEmitter::updateCoordinateFrame(bool)")]
// 0xbbea40 — __ZThn392_N3RBX18RbxParticleEmitter21updateCoordinateFrameEb
pub fn stub_bbea40() -> ! {
    todo!("0xbbea40 __ZThn392_N3RBX18RbxParticleEmitter21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::RbxParticleEmitter::onAncestorChangedEx(void)")]
// 0xbbeab4 — __ZN3RBX18RbxParticleEmitter19onAncestorChangedExEv
pub fn stub_bbeab4() -> ! {
    todo!("0xbbeab4 __ZN3RBX18RbxParticleEmitter19onAncestorChangedExEv")
}

#[doc(alias = "RBX::RbxParticleEmitter::unbind(void)")]
// 0xbbf390 — __ZN3RBX18RbxParticleEmitter6unbindEv
pub fn stub_bbf390() -> ! {
    todo!("0xbbf390 __ZN3RBX18RbxParticleEmitter6unbindEv")
}

#[doc(alias = "non-virtual thunk toRBX::RbxParticleEmitter::unbind(void)")]
// 0xbbf398 — __ZThn392_N3RBX18RbxParticleEmitter6unbindEv
pub fn stub_bbf398() -> ! {
    todo!("0xbbf398 __ZThn392_N3RBX18RbxParticleEmitter6unbindEv")
}

#[doc(alias = "RBX::RbxParticleEmitter::invalidateEntity(void)")]
// 0xbbf39c — __ZN3RBX18RbxParticleEmitter16invalidateEntityEv
pub fn stub_bbf39c() -> ! {
    todo!("0xbbf39c __ZN3RBX18RbxParticleEmitter16invalidateEntityEv")
}

#[doc(alias = "non-virtual thunk toRBX::RbxParticleEmitter::invalidateEntity(void)")]
// 0xbbf3d0 — __ZThn392_N3RBX18RbxParticleEmitter16invalidateEntityEv
pub fn stub_bbf3d0() -> ! {
    todo!("0xbbf3d0 __ZThn392_N3RBX18RbxParticleEmitter16invalidateEntityEv")
}

#[doc(alias = "RBX::RbxParticleEmitter::updateEntity(bool)")]
// 0xbbf404 — __ZN3RBX18RbxParticleEmitter12updateEntityEb
pub fn stub_bbf404() -> ! {
    todo!("0xbbf404 __ZN3RBX18RbxParticleEmitter12updateEntityEb")
}

#[doc(alias = "non-virtual thunk toRBX::RbxParticleEmitter::updateEntity(bool)")]
// 0xbbf8b0 — __ZThn392_N3RBX18RbxParticleEmitter12updateEntityEb
pub fn stub_bbf8b0() -> ! {
    todo!("0xbbf8b0 __ZThn392_N3RBX18RbxParticleEmitter12updateEntityEb")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RbxParticleEmitter,bool>,boost::_bi::list2<boost::_bi::value<RBX::RbxParticleEmitter*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbbfa58 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
pub fn stub_bbfa58() -> ! {
    todo!("0xbbfa58 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RbxParticleEmitter,bool>,boost::_bi::list2<boost::_bi::value<RBX::RbxParticleEmitter*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbbfab4 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
pub fn stub_bbfab4() -> ! {
    todo!("0xbbfab4 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RbxParticleEmitter,bool>,boost::_bi::list2<boost::_bi::value<RBX::RbxParticleEmitter*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xbbfbbc — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
pub fn stub_bbfbbc() -> ! {
    todo!("0xbbfbbc __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RbxParticleEmitter,bool>,boost::_bi::list2<boost::_bi::value<RBX::RbxParticleEmitter*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xbbfbd4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
pub fn stub_bbfbd4() -> ! {
    todo!("0xbbfbd4 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX18RbxParticleEmitterEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::RbxParticleEmitter>,boost::_bi::list1<boost::_bi::value<RBX::RbxParticleEmitter*>>>>::~callable_slot()")]
// 0xbbfbec — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_bbfbec() -> ! {
    todo!("0xbbfbec __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::RbxParticleEmitter>,boost::_bi::list1<boost::_bi::value<RBX::RbxParticleEmitter*>>>>::~callable_slot()")]
// 0xbbfc48 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_bbfc48() -> ! {
    todo!("0xbbfc48 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::RbxParticleEmitter>,boost::_bi::list1<boost::_bi::value<RBX::RbxParticleEmitter*>>>,0,void ()(void)>::call(void)")]
// 0xbbfd50 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_bbfd50() -> ! {
    todo!("0xbbfd50 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::RbxParticleEmitter>,boost::_bi::list1<boost::_bi::value<RBX::RbxParticleEmitter*>>>,0,void ()(void)>::call(void)")]
// 0xbbfd68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_bbfd68() -> ! {
    todo!("0xbbfd68 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX18RbxParticleEmitterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "RBX::RbxParticleFactory::~RbxParticleFactory()")]
// 0xbc0438 — __ZN3RBX18RbxParticleFactoryD1Ev
pub fn stub_bc0438() -> ! {
    todo!("0xbc0438 __ZN3RBX18RbxParticleFactoryD1Ev")
}

#[doc(alias = "RBX::RbxParticleFactory::clearParticleSystems(void)")]
// 0xbc1d70 — __ZN3RBX18RbxParticleFactory20clearParticleSystemsEv
pub fn stub_bc1d70() -> ! {
    todo!("0xbc1d70 __ZN3RBX18RbxParticleFactory20clearParticleSystemsEv")
}

#[doc(alias = "RBX::RbxParticleFactory::getParticleSystemsBegin(void)")]
// 0xbc1d78 — __ZN3RBX18RbxParticleFactory23getParticleSystemsBeginEv
pub fn stub_bc1d78() -> ! {
    todo!("0xbc1d78 __ZN3RBX18RbxParticleFactory23getParticleSystemsBeginEv")
}

#[doc(alias = "RBX::RbxParticleFactory::getParticleSystemsEnd(void)")]
// 0xbc1d7c — __ZN3RBX18RbxParticleFactory21getParticleSystemsEndEv
pub fn stub_bc1d7c() -> ! {
    todo!("0xbc1d7c __ZN3RBX18RbxParticleFactory21getParticleSystemsEndEv")
}

#[doc(alias = "RBX::RbxParticleManager::Throttle(void)")]
// 0xbc44b8 — __ZN3RBX18RbxParticleManager8ThrottleEv
pub fn stub_bc44b8() -> ! {
    todo!("0xbc44b8 __ZN3RBX18RbxParticleManager8ThrottleEv")
}

#[doc(alias = "std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)")]
// 0xbca954 — __ZNSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_bca954() -> ! {
    todo!("0xbca954 __ZNSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::erase_key(RBX::SpatialGridIndex const&)")]
// 0xbce674 — __ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE9erase_keyERKS6_
pub fn stub_bce674() -> ! {
    todo!("0xbce674 __ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE9erase_keyERKS6_")
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>)")]
// 0xbce818 — __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
pub fn stub_bce818() -> ! {
    todo!("0xbce818 __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Humanoid * const,RBX::FastCluster *>> *)")]
// 0xbce87c — __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_bce87c() -> ! {
    todo!("0xbce87c __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::operator[](RBX::SpatialGridIndex const&)")]
// 0xbce8a4 — __ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEEixERKS6_
pub fn stub_bce8a4() -> ! {
    todo!("0xbce8a4 __ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEEixERKS6_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::reserve_for_insert(unsigned long)")]
// 0xbceb68 — __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_bceb68() -> ! {
    todo!("0xbceb68 __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::create_buckets(unsigned long)")]
// 0xbced10 — __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_bced10() -> ! {
    todo!("0xbced10 __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::destroy_buckets(void)")]
// 0xbcee54 — __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE15destroy_bucketsEv
pub fn stub_bcee54() -> ! {
    todo!("0xbcee54 __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE15destroy_bucketsEv")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,20u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::malloc(void)")]
// 0xbceec8 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_bceec8() -> ! {
    todo!("0xbceec8 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::pair<RBX::Humanoid * const,RBX::FastCluster *> const&)")]
// 0xbcef38 — __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_bcef38() -> ! {
    todo!("0xbcef38 __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_insert_unique(std::pair<RBX::Humanoid * const,RBX::FastCluster *> const&)")]
// 0xbcf0ec — __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_bcf0ec() -> ! {
    todo!("0xbcf0ec __ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex*,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex const&)")]
// 0xbcf1dc — __ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_bcf1dc() -> ! {
    todo!("0xbcf1dc __ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex>(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex const&,std::random_access_iterator_tag)")]
// 0xbcf30c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN3RBX16SpatialGridIndexESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_RKT0_St26random_access_iterator_tag
pub fn stub_bcf30c() -> ! {
    todo!("0xbcf30c __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN3RBX16SpatialGridIndexESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex*,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,unsigned long,RBX::SpatialGridIndex const&)")]
// 0xbcf460 — __ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_bcf460() -> ! {
    todo!("0xbcf460 __ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "boost::unordered::unordered_map<RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>,boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~unordered_map()")]
// 0xbcf80c — __ZN5boost9unordered13unordered_mapIN3RBX16SpatialGridIndexENS2_11SpatialGridINS2_11FastClusterEE4CellENS_4hashIS3_EESt8equal_toIS3_ENS_19fast_pool_allocatorIS3_NS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED1Ev
pub fn stub_bcf80c() -> ! {
    todo!("0xbcf80c __ZN5boost9unordered13unordered_mapIN3RBX16SpatialGridIndexENS2_11SpatialGridINS2_11FastClusterEE4CellENS_4hashIS3_EESt8equal_toIS3_ENS_19fast_pool_allocatorIS3_NS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED1Ev")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,8u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xbcf8b8 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj8ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_bcf8b8() -> ! {
    todo!("0xbcf8b8 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj8ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,20u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xbcf940 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_bcf940() -> ! {
    todo!("0xbcf940 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "boost::scoped_ptr<RBX::SpatialGrid<RBX::FastCluster>>::~scoped_ptr()")]
// 0xbcf9c8 — __ZN5boost10scoped_ptrIN3RBX11SpatialGridINS1_11FastClusterEEEED2Ev
pub fn stub_bcf9c8() -> ! {
    todo!("0xbcf9c8 __ZN5boost10scoped_ptrIN3RBX11SpatialGridINS1_11FastClusterEEEED2Ev")
}

#[doc(alias = "RBX::TextureCompositorJob::update(void)")]
// 0xbd7b3c — __ZN3RBX20TextureCompositorJob6updateEv
pub fn stub_bd7b3c() -> ! {
    todo!("0xbd7b3c __ZN3RBX20TextureCompositorJob6updateEv")
}

#[doc(alias = "RBX::TextureCompositor::prepareDefaultTexture(void)")]
// 0xbda090 — __ZN3RBX17TextureCompositor21prepareDefaultTextureEv
pub fn stub_bda090() -> ! {
    todo!("0xbda090 __ZN3RBX17TextureCompositor21prepareDefaultTextureEv")
}

#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
// 0xbda2bc — __ZN3RBX17TextureCompositorD0Ev
pub fn stub_bda2bc() -> ! {
    todo!("0xbda2bc __ZN3RBX17TextureCompositorD0Ev")
}

#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
// 0xbda35c — __ZN3RBX17TextureCompositorD1Ev
pub fn stub_bda35c() -> ! {
    todo!("0xbda35c __ZN3RBX17TextureCompositorD1Ev")
}

#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
// 0xbda360 — __ZN3RBX17TextureCompositorD2Ev
pub fn stub_bda360() -> ! {
    todo!("0xbda360 __ZN3RBX17TextureCompositorD2Ev")
}

#[doc(alias = "RBX::TextureCompositor::getTexture(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbdad0c — __ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE
// was: RBX::TextureCompositor::getTexture(boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_bdad0c() -> ! {
    todo!("0xbdad0c __ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::TextureCompositor::getTextureId(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xbdadb8 — __ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE
// was: RBX::TextureCompositor::getTextureId(boost::shared_ptr<RBX::TextureCompositor::Job> const&)
pub fn stub_bdadb8() -> ! {
    todo!("0xbdadb8 __ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE")
}
