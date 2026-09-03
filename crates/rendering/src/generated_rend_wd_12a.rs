//! rendering shard rend_wd_12a — 120 stubs 0xb86c9c..0xf22d20 EA-sorted asc rendering-filtered gap filler (Ogre/G3D/Render/Adorn/MeshContent/TextureContent/Material/Shader/Texture) [skeleton batch rend_wd_12a]
//! Source: ida/export.json (85545 funcs) EA asc rendering-filtered gap filler not yet in crates/rendering/src — next 120 uncovered rendering sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb86c9c — __ZN3RBX17MaterialGenerator18createBaseMaterialEj
// type: void __fastcall(RBX::MaterialGenerator *this, unsigned int, unsigned int)
#[doc(alias = "RBX::MaterialGenerator::createBaseMaterial(unsigned int)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator18createBaseMaterialEj")]
// IDA 0xb86c9c: 333 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b86c9c() {
}

// 0xb87024 — __ZN12_GLOBAL__N_115getMaterialNameEj
// type: void __fastcall(_anonymous_namespace_ *this, char)
#[doc(alias = "anonymous namespace::getMaterialName(unsigned int)")]
#[doc(alias = "__ZN12_GLOBAL__N_115getMaterialNameEj")]
// IDA 0xb87024: 142 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b87024() {
}

// 0xb8754c — __ZN12_GLOBAL__N_115getMaterialNameEjib
// type: void __fastcall(_anonymous_namespace_ *this, char, unsigned int, int)
#[doc(alias = "anonymous namespace::getMaterialName(unsigned int,int,bool)")]
#[doc(alias = "__ZN12_GLOBAL__N_115getMaterialNameEjib")]
// IDA 0xb8754c: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8754c() {
}

// 0xb87db8 — __ZN3RBX17MaterialGenerator12fetchTextureERKNS_9TextureIdEiPNS_8InstanceEPNS_11AsyncResultE
// type: void __fastcall(_DWORD *, int, const std::string *, __int32, int, _DWORD *)
#[doc(alias = "RBX::MaterialGenerator::fetchTexture(RBX::TextureId const&,int,RBX::Instance *,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator12fetchTextureERKNS_9TextureIdEiPNS_8InstanceEPNS_11AsyncResultE")]
// IDA 0xb87db8: 585 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b87db8() {
}

// 0xb883dc — __ZN3RBX17MaterialGenerator24createMaterialForTextureERKNS_9TextureIdEiPNS_8InstanceEjPNS_11AsyncResultE
// type: void __fastcall(RBX::MaterialGenerator *, const Ogre::TexturePtr *, const std::string *, __int32, int, unsigned int, _DWORD *)
#[doc(alias = "RBX::MaterialGenerator::createMaterialForTexture(RBX::TextureId const&,int,RBX::Instance *,unsigned int,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator24createMaterialForTextureERKNS_9TextureIdEiPNS_8InstanceEjPNS_11AsyncResultE")]
// IDA 0xb883dc: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b883dc() {
}

// 0xb8859c — __ZN3RBX17MaterialGenerator21createDefaultMaterialEPNS_12PartInstanceEjb
// type: void __fastcall(RBX::MaterialGenerator *this, RBX::PartInstance *, RBX::PartInstance *, unsigned int, int)
#[doc(alias = "RBX::MaterialGenerator::createDefaultMaterial(RBX::PartInstance *,unsigned int,bool)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator21createDefaultMaterialEPNS_12PartInstanceEjb")]
// IDA 0xb8859c: 1066 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8859c() {
}

// 0xb890f0 — __ZN3RBX17MaterialGenerator21createMaterialForPartEPNS_12PartInstanceEPKNS_18HumanoidIdentifierEjPNS_11AsyncResultE
// type: int __fastcall(int, RBX::PartInstance *, int, int, unsigned int, _DWORD *)
#[doc(alias = "RBX::MaterialGenerator::createMaterialForPart(RBX::PartInstance *,RBX::HumanoidIdentifier const*,unsigned int,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator21createMaterialForPartEPNS_12PartInstanceEPKNS_18HumanoidIdentifierEjPNS_11AsyncResultE")]
// IDA 0xb890f0: 2354 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b890f0() {
}

// 0xb8aa38 — __ZN3RBX17MaterialGenerator22createMaterialForDecalEPNS_5DecalEjPNS_11AsyncResultE
// type: Ogre::NedPoolingImpl *__fastcall(_DWORD *, const Ogre::TexturePtr *, int, unsigned int, _DWORD *)
#[doc(alias = "RBX::MaterialGenerator::createMaterialForDecal(RBX::Decal *,unsigned int,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator22createMaterialForDecalEPNS_5DecalEjPNS_11AsyncResultE")]
// IDA 0xb8aa38: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8aa38() {
}

// 0xb8ab2c — __ZN3RBX17MaterialGenerator14createMaterialEPNS_12PartInstanceEPNS_5DecalEPKNS_18HumanoidIdentifierEjPNS_11AsyncResultE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::MaterialGenerator::createMaterial(RBX::PartInstance *,RBX::Decal *,RBX::HumanoidIdentifier const*,unsigned int,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX17MaterialGenerator14createMaterialEPNS_12PartInstanceEPNS_5DecalEPKNS_18HumanoidIdentifierEjPNS_11AsyncResultE")]
// IDA 0xb8ab2c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8ab2c() {
}

// 0xb8bed8 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_10BrickColorE
// type: int __fastcall(std::string *, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::BrickColor const&)")]
#[doc(alias = "__ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_10BrickColorE")]
// IDA 0xb8bed8: 193 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8bed8() {
}

// 0xb8c314 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE
// type: int __fastcall(std::string *, const std::string *, const std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)")]
#[doc(alias = "__ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE")]
// IDA 0xb8c314: 287 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c314() {
}

// 0xb8cc38 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_
// type: void __fastcall(int, _DWORD *, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_")]
// IDA 0xb8cc38: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_b8cc38() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xb8cd88 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: void __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0xb8cd88: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_b8cd88() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xb8d500 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int *, unsigned int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm")]
// IDA 0xb8d500: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8d500() {
}

// 0xb8d608 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_
// type: void *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_")]
// IDA 0xb8d608: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_b8d608() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xb994b4 — __ZNK5boost23enable_shared_from_thisIN3RBX16TextureProxyBaseEE22_internal_accept_ownerIS2_NS1_15RbxTextureProxyEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TextureProxyBase>::_internal_accept_owner<RBX::TextureProxyBase,RBX::RbxTextureProxy>(boost::shared_ptr<RBX::TextureProxyBase> const*,RBX::RbxTextureProxy *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX16TextureProxyBaseEE22_internal_accept_ownerIS2_NS1_15RbxTextureProxyEEEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xb994b4: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b994b4() {
}

// 0xb9960c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED1Ev")]
// IDA 0xb9960c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b9960c() {
}

// 0xb99610 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEED0Ev")]
// IDA 0xb99610: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b99610() {
}

// 0xb99614 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE7disposeEv")]
// IDA 0xb99614: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99614() {
}

// 0xb99624 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE11get_deleterERKSt9type_info")]
// IDA 0xb99624: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99624() {
}

// 0xb99628 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RbxTextureProxy>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15RbxTextureProxyEE19get_untyped_deleterEv")]
// IDA 0xb99628: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99628() {
}

// 0xbb3ff4 — __ZNK3RBX15ServiceProvider6createINS_19MeshContentProviderEEEPT_v
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::create<RBX::MeshContentProvider>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_19MeshContentProviderEEEPT_v")]
// IDA 0xbb3ff4: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb3ff4() {
}

// 0xbb4280 — __ZNK3RBX15ServiceProvider4findINS_19MeshContentProviderEEEPT_v
// type: struct _Unwind_Exception *__fastcall(int, int, int, int)
#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::find<RBX::MeshContentProvider>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_19MeshContentProviderEEEPT_v")]
// IDA 0xbb4280: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4280() {
}

// 0xbb45a8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19MeshContentProviderEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(RBX::MeshContentProvider **, int, int, int, void *, int)
#[doc(alias = "boost::shared_ptr<RBX::MeshContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::MeshContentProvider>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19MeshContentProviderEEEN5boost10shared_ptrIT_EEv")]
// IDA 0xbb45a8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb45a8() {
}

// 0xbb46f4 — __ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv")]
// IDA 0xbb46f4: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb46f4() {
}

// 0xbb4810 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19MeshContentProviderEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::MeshContentProvider>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19MeshContentProviderEEEvv")]
// IDA 0xbb4810: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4810() {
}

// 0xbb48d8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19MeshContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::MeshContentProvider,RBX::MeshContentProvider>(boost::shared_ptr<RBX::MeshContentProvider> const*,RBX::MeshContentProvider *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19MeshContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xbb48d8: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb48d8() {
}

// 0xbb4a3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MeshContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0xbb4a3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bb4a3c() {
}

// 0xbb4a40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MeshContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0xbb4a40: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bb4a40() {
}

// 0xbb4a44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MeshContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0xbb4a44: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4a44() {
}

// 0xbb4a64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MeshContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0xbb4a64: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4a64() {
}

// 0xbb4a7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MeshContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19MeshContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0xbb4a7c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb4a7c() {
}

// 0xbd7b3c — __ZN3RBX20TextureCompositorJob6updateEv
// type: _DWORD __fastcall(RBX::TextureCompositorJob *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::update(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob6updateEv")]
// IDA 0xbd7b3c: 2547 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd7b3c() {
}

// 0xbda090 — __ZN3RBX17TextureCompositor21prepareDefaultTextureEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::prepareDefaultTexture(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor21prepareDefaultTextureEv")]
// IDA 0xbda090: 217 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bda090() {
}

// 0xbda2bc — __ZN3RBX17TextureCompositorD0Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD0Ev")]
// IDA 0xbda2bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bda2bc() {
}

// 0xbda35c — __ZN3RBX17TextureCompositorD1Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD1Ev")]
// IDA 0xbda35c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bda35c() {
}

// 0xbda360 — __ZN3RBX17TextureCompositorD2Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD2Ev")]
// IDA 0xbda360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bda360() {
}

// 0xbdad0c — __ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE
// type: _UNKNOWN **__fastcall(_DWORD *, int, int *)
#[doc(alias = "RBX::TextureCompositor::getTexture(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE")]
// IDA 0xbdad0c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdad0c() {
}

// 0xbdadb8 — __ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::TextureCompositor::getTextureId(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE")]
// IDA 0xbdadb8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdadb8() {
}

// 0xbdaf9c — __ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::attachInstance(boost::shared_ptr<RBX::TextureCompositor::Job> const&,boost::shared_ptr<RBX::PartInstance> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE")]
// IDA 0xbdaf9c: 324 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdaf9c() {
}

// 0xbdb320 — __ZNK3RBX17TextureCompositor12isQueueEmptyEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::isQueueEmpty(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor12isQueueEmptyEv")]
// IDA 0xbdb320: 15 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb320() {
}

// 0xbdb8d8 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_")]
// IDA 0xbdb8d8: 276 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb8d8() {
}

// 0xbdbb84 — __ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::garbageCollectOrphanedJobs(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv")]
// IDA 0xbdbb84: 506 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdbb84() {
}

// 0xbdc080 — __ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::findRebakeTargetAndEnqueue(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv")]
// IDA 0xbdc080: 313 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc080() {
}

// 0xbdc398 — __ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
// IDA 0xbdc398: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc398() {
}

// 0xbdc888 — __ZN3RBX17TextureCompositor9updateJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::updateJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor9updateJobERNS0_3JobE")]
// IDA 0xbdc888: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc888() {
}

// 0xbde4fc — __ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::orphanTextureFromJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE")]
// IDA 0xbde4fc: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde4fc() {
}

// 0xbde708 — __ZN3RBX17TextureCompositor18getOrCreateTextureEj
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, unsigned int)
#[doc(alias = "RBX::TextureCompositor::getOrCreateTexture(unsigned int)")]
#[doc(alias = "__ZN3RBX17TextureCompositor18getOrCreateTextureEj")]
// IDA 0xbde708: 727 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde708() {
}

// 0xbdf32c — __ZNK3RBX17TextureCompositor13getStatisticsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::getStatistics(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor13getStatisticsEv")]
// IDA 0xbdf32c: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf32c() {
}

// 0xbdf498 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_")]
// IDA 0xbdf498: 285 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf498() {
}

// 0xbdf798 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
// IDA 0xbdf798: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf798() {
}

// 0xbdf9e4 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_")]
// IDA 0xbdf9e4: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf9e4() {
}

// 0xbdfae0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_")]
// IDA 0xbdfae0: 323 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdfae0() {
}

// 0xbdfe68 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_")]
// IDA 0xbdfe68: 321 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdfe68() {
}

// 0xbe01e4 — __ZNK12_GLOBAL__N_121MaterialDeadPredicateclERKy
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "anonymous namespace::MaterialDeadPredicate::operator()(unsigned long long const&)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_121MaterialDeadPredicateclERKy")]
// IDA 0xbe01e4: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be01e4() {
}

// 0xbe04f0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_")]
// IDA 0xbe04f0: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be04f0() {
}

// 0xbe075c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_
// type: int __fastcall(int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_")]
// IDA 0xbe075c: 223 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be075c() {
}

// 0xbe09c8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_
// type: int(void)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::push_back(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_")]
// IDA 0xbe09c8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_be09c8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xbe0b20 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm")]
// IDA 0xbe0b20: 51 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be0b20() {
}

// 0xbe1144 — __ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_")]
// IDA 0xbe1144: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1144() {
}

// 0xbe1250 — __ZN3RBX17TextureCompositor3JobD2Ev
// type: void __fastcall(RBX::TextureCompositor::Job *__hidden this)
#[doc(alias = "RBX::TextureCompositor::Job::~Job()")]
#[doc(alias = "__ZN3RBX17TextureCompositor3JobD2Ev")]
// IDA 0xbe1250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be1250() {
}

// 0xbe1550 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev")]
// IDA 0xbe1550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be1550() {
}

// 0xbe15fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev")]
// IDA 0xbe15fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be15fc() {
}

// 0xbe1600 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev")]
// IDA 0xbe1600: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be1600() {
}

// 0xbe1604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv")]
// IDA 0xbe1604: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1604() {
}

// 0xbe16a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info")]
// IDA 0xbe16a8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16a8() {
}

// 0xbe16ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv")]
// IDA 0xbe16ac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16ac() {
}

// 0xbe16b0 — __ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_")]
// IDA 0xbe16b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16b0() {
}

// 0xbe1818 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev")]
// IDA 0xbe1818: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be1818() {
}

// 0xbe181c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev")]
// IDA 0xbe181c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be181c() {
}

// 0xbe1820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv")]
// IDA 0xbe1820: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1820() {
}

// 0xbe192c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info")]
// IDA 0xbe192c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be192c() {
}

// 0xbe1930 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv")]
// IDA 0xbe1930: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1930() {
}

// 0xbe1934 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag
// type: int(void)
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag")]
// IDA 0xbe1934: 894 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1934() {
}

// 0xbe21d4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
// IDA 0xbe21d4: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be21d4() {
}

// 0xbe2288 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
// IDA 0xbe2288: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_be2288() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xbe2340 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job>* std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<boost::shared_ptr<RBX::TextureCompositor::Job>*>(unsigned long,boost::shared_ptr<RBX::TextureCompositor::Job>*,boost::shared_ptr<RBX::TextureCompositor::Job>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_")]
// IDA 0xbe2340: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_be2340() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xbe2524 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag")]
// IDA 0xbe2524: 907 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2524() {
}

// 0xbe2dec — __ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>)")]
#[doc(alias = "__ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_")]
// IDA 0xbe2dec: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2dec() {
}

// 0xbe2e74 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E")]
// IDA 0xbe2e74: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2e74() {
}

// 0xbe2fa8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")]
// IDA 0xbe2fa8: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2fa8() {
}

// 0xbe305c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_create_node(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")]
// IDA 0xbe305c: 103 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be305c() {
}

// 0xbe3278 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
// IDA 0xbe3278: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_be3278() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbe380c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_")]
// IDA 0xbe380c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_be380c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xbe38f8 — __ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type")]
// IDA 0xbe38f8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be38f8() {
}

// 0xbe3b44 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type")]
// IDA 0xbe3b44: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be3b44() {
}

// 0xbe3d90 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// IDA 0xbe3d90: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be3d90() {
}

// 0xbe40d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// IDA 0xbe40d8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be40d8() {
}

// 0xbe414c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0xbe414c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be414c() {
}

// 0xbe4230 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_")]
// IDA 0xbe4230: 93 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4230() {
}

// 0xbe43b4 — __ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_
#[doc(alias = "std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_")]
// IDA 0xbe43b4: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be43b4() {
}

// 0xbe4530 — __ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_")]
// IDA 0xbe4530: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4530() {
}

// 0xbe46e4 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TextureCompositor::Job>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xbe46e4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be46e4() {
}

// 0xbe4714 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0xbe4714: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4714() {
}

// 0xbe4854 — __ZN3RBX20TextureCompositorJob9LayerDataD2Ev
// type: void __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::~LayerData()")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataD2Ev")]
// IDA 0xbe4854: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be4854() {
}

// 0xbe4b50 — __ZN3RBX20TextureCompositorJob9LayerDataC2Ev
// type: _DWORD __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2Ev")]
// IDA 0xbe4b50: 246 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4b50() {
}

// 0xbe57cc — __ZN3RBX15RbxTextureProxyD0Ev
// type: void __fastcall(RBX::RbxTextureProxy *__hidden this)
#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
#[doc(alias = "__ZN3RBX15RbxTextureProxyD0Ev")]
// IDA 0xbe57cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be57cc() {
}

// 0xbe586c — __ZN3RBX15RbxTextureProxyD1Ev
// type: void __fastcall(RBX::RbxTextureProxy *__hidden this)
#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
#[doc(alias = "__ZN3RBX15RbxTextureProxyD1Ev")]
// IDA 0xbe586c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be586c() {
}

// 0xbe5870 — __ZN3RBX15RbxTextureProxyD2Ev
// type: void __fastcall(RBX::RbxTextureProxy *__hidden this)
#[doc(alias = "RBX::RbxTextureProxy::~RbxTextureProxy()")]
#[doc(alias = "__ZN3RBX15RbxTextureProxyD2Ev")]
// IDA 0xbe5870: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be5870() {
}

// 0xbe5a84 — __ZN3RBX15RbxTextureProxy15getOriginalSizeEv
// type: _DWORD __fastcall(RBX::RbxTextureProxy *__hidden this)
#[doc(alias = "RBX::RbxTextureProxy::getOriginalSize(void)")]
#[doc(alias = "__ZN3RBX15RbxTextureProxy15getOriginalSizeEv")]
// IDA 0xbe5a84: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be5a84() {
}

// 0xbf2dc0 — __ZNK3RBX15ServiceProvider6createINS_22TextureContentProviderEEEPT_v
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::create<RBX::TextureContentProvider>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_22TextureContentProviderEEEPT_v")]
// IDA 0xbf2dc0: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf2dc0() {
}

// 0xbf304c — __ZNK3RBX15ServiceProvider4findINS_22TextureContentProviderEEEPT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::find<RBX::TextureContentProvider>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_22TextureContentProviderEEEPT_v")]
// IDA 0xbf304c: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf304c() {
}

// 0xbf3374 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_22TextureContentProviderEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::shared_ptr<RBX::TextureContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::TextureContentProvider>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_22TextureContentProviderEEEN5boost10shared_ptrIT_EEv")]
// IDA 0xbf3374: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf3374() {
}

// 0xbf34c0 — __ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv")]
// IDA 0xbf34c0: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf34c0() {
}

// 0xbf35dc — __ZN3RBX4Name13callDoDeclareILZNS_23sTextureContentProviderEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_23sTextureContentProviderEEEEvv")]
// IDA 0xbf35dc: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf35dc() {
}

// 0xbf36b0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_22TextureContentProviderEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TextureContentProvider>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_22TextureContentProviderEEEvv")]
// IDA 0xbf36b0: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf36b0() {
}

// 0xbf3778 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22TextureContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextureContentProvider,RBX::TextureContentProvider>(boost::shared_ptr<RBX::TextureContentProvider> const*,RBX::TextureContentProvider *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22TextureContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xbf3778: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf3778() {
}

// 0xbf38dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextureContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0xbf38dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_bf38dc() {
}

// 0xbf38e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextureContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0xbf38e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bf38e0() {
}

// 0xbf38e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextureContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22TextureContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0xbf38e4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf38e4() {
}

// 0xbf5a34 — __ZN3RBX11FontFactory10getTextureENS_4Text4FontEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::FontFactory::getTexture(RBX::Text::Font,float)")]
#[doc(alias = "__ZN3RBX11FontFactory10getTextureENS_4Text4FontEf")]
// IDA 0xbf5a34: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5a34() {
}

// 0xf20950 — __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v$shim
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v$shim")]
// IDA 0xf20950: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f20950() {
}

// 0xf2095c — __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v$shim")]
// IDA 0xf2095c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2095c() {
}

// 0xf21478 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev$shim
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf21478: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21478() {
}

// 0xf2149c — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv$shim
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv$shim")]
// IDA 0xf2149c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2149c() {
}

// 0xf214a8 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v$shim")]
// IDA 0xf214a8: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f214a8() {
}

// 0xf218e0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv$shim")]
// IDA 0xf218e0: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f218e0() {
}

// 0xf218ec — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev$shim")]
// IDA 0xf218ec: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f218ec() {
}

// 0xf22c3c — __ZN3RBX10Reflection8EnumDescINS_8MaterialEED2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_8MaterialEED2Ev$shim")]
// IDA 0xf22c3c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f22c3c() {
}

// 0xf22d20 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE14convertToIndexES2_$shim
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8MaterialEE14convertToIndexES2_$shim")]
// IDA 0xf22d20: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f22d20() {
}

