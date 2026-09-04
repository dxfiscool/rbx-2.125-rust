//! rendering — Ogre|G3D|Gfx|Render substr 15058 total
//! This shard: 0xf65404..0xf66214 (100 stubs, EA-sorted asc, 14023->14123 covered, 1132 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf65404 — j___ZNSt6vectorIPN4Ogre14ParticleSystemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleSystem **,std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>>,Ogre::ParticleSystem * const&)")]
// was: std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleSystem **,std::vector<Ogre::ParticleSystem *,std::allocator<Ogre::ParticleSystem *>>>,Ogre::ParticleSystem * const&)
// IDA 0xf65404: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65404() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65414 — j___ZNSt6vectorISt4pairIPN4Ogre14ParticleSystemEfESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)")]
// was: std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ParticleSystem *,float>*,std::vector<std::pair<Ogre::ParticleSystem *,float>,std::allocator<std::pair<Ogre::ParticleSystem *,float>>>>,std::pair<Ogre::ParticleSystem *,float> const&)
// IDA 0xf65414: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65414() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65424 — j___ZN4Ogre9SharedPtrINS_6RbxSkyEED1Ev
// type: int __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::RbxSky>::~SharedPtr()
// IDA 0xf65424: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65424() {
}

// 0xf65434 — j___ZN5boost12scoped_arrayIN4Ogre11MaterialPtrEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()")]
// was: boost::scoped_array<Ogre::MaterialPtr>::~scoped_array()
// IDA 0xf65434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65434() {
}

// 0xf65444 — j___ZNSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)")]
// was: std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)
// IDA 0xf65444: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65444() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65454 — j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::ShadowRenderable *,bool>*,std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>>,std::pair<Ogre::ShadowRenderable *,bool> const&)
// IDA 0xf65454: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65454() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65464 — j___ZNSt6vectorISt4pairIPN4Ogre16ShadowRenderableEbESaIS4_EE7reserveEm
// type: int()
#[doc(alias = "std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)")]
// was: std::vector<std::pair<Ogre::ShadowRenderable *,bool>,std::allocator<std::pair<Ogre::ShadowRenderable *,bool>>>::reserve(unsigned long)
// IDA 0xf65464: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65464() {
}

// 0xf65474 — j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_insert_unique(std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *> const&)
// IDA 0xf65474: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65474() {
}

// 0xf65484 — j___ZNSt8_Rb_treeIPKN4Ogre10RenderableESt4pairIKS3_PNS0_7Matrix4EESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)")]
// was: std::_Rb_tree<Ogre::Renderable const*,std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>,std::_Select1st<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>,std::less<Ogre::Renderable const*>,std::allocator<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Renderable const* const,Ogre::Matrix4 *>> *)
// IDA 0xf65484: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65484() {
}

// 0xf65494 — j___ZN3RBX11SpatialGridINS_11FastClusterEEC2ERKN3G3D7Vector3Ef
#[doc(alias = "RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)")]
// was: RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)
// IDA 0xf65494: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65494() {
}

// 0xf655d4 — j___ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)")]
// was: std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,unsigned long,Ogre::Vector3 const&)
// IDA 0xf655d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f655d4() {
}

// 0xf655e4 — j___ZNSt6vectorIPN4Ogre12ManualObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)")]
// was: std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject **,std::vector<Ogre::ManualObject *,std::allocator<Ogre::ManualObject *>>>,Ogre::ManualObject * const&)
// IDA 0xf655e4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f655e4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf655f4 — j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_insert_unique(std::pair<Ogre::SubMesh const* const,Ogre::Sphere> const&)
// IDA 0xf655f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f655f4() {
}

// 0xf65604 — j___ZNSt8_Rb_treeIPKN4Ogre7SubMeshESt4pairIKS3_NS0_6SphereEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)")]
// was: std::_Rb_tree<Ogre::SubMesh const*,std::pair<Ogre::SubMesh const* const,Ogre::Sphere>,std::_Select1st<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>,std::less<Ogre::SubMesh const*>,std::allocator<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh const* const,Ogre::Sphere>> *)
// IDA 0xf65604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65604() {
}

// 0xf65694 — j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre10TexturePtrES4_EET0_T_S6_S5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)")]
// was: Ogre::TexturePtr * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::TexturePtr *,Ogre::TexturePtr *>(Ogre::TexturePtr *,Ogre::TexturePtr *,Ogre::TexturePtr *)
// IDA 0xf65694: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f65694() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf65704 — j___ZNSt6vectorIN4Ogre10TexturePtrESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)")]
// was: std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,std::allocator<Ogre::TexturePtr>>>,Ogre::TexturePtr const&)
// IDA 0xf65704: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65704() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65874 — j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEE9classNameEv
#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEE9classNameEv")]
// was: j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEE9classNameEv
// IDA 0xf65874: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65874() {
}

// 0xf65904 — j___ZN4Ogre9SharedPtrINS_8MaterialEED1Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Material>::~SharedPtr()
// IDA 0xf65904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65904() {
}

// 0xf65914 — j___ZN4Ogre9SharedPtrINS_8ResourceEED1Ev
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Resource>::~SharedPtr()
// IDA 0xf65914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65914() {
}

// 0xf659d4 — j___ZNK3RBX15ServiceProvider4findINS_18RenderHooksServiceEEEPT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::find<RBX::RenderHooksService>(void)const")]
// was: RBX::RenderHooksService * RBX::ServiceProvider::find<RBX::RenderHooksService>(void)const
// IDA 0xf659d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f659d4() {
}

// 0xf65a34 — j___ZN4Ogre13RbxTypesetter12getCharWidthERKNS_7FontPtrEfcf
#[doc(alias = "Ogre::RbxTypesetter::getCharWidth(Ogre::FontPtr const&,float,char,float)")]
// was: Ogre::RbxTypesetter::getCharWidth(Ogre::FontPtr const&,float,char,float)
// IDA 0xf65a34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65a34() {
}

// 0xf65a64 — j___ZNSt6vectorISt4pairIiN4Ogre13RbxTypesetter7SpacingEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing>*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,std::pair<int,Ogre::RbxTypesetter::Spacing> const&)")]
// was: std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<int,Ogre::RbxTypesetter::Spacing>*,std::vector<std::pair<int,Ogre::RbxTypesetter::Spacing>,std::allocator<std::pair<int,Ogre::RbxTypesetter::Spacing>>>>,std::pair<int,Ogre::RbxTypesetter::Spacing> const&)
// IDA 0xf65a64: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65a64() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65a74 — j___ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::Array(void)")]
// was: G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::Array(void)
// IDA 0xf65a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65a74() {
}

// 0xf65a84 — j___ZN3G3D5ArrayIPN4Ogre20RbxCullableSceneNodeELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::~Array()")]
// was: G3D::Array<Ogre::RbxCullableSceneNode *,10,32ul>::~Array()
// IDA 0xf65a84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65a84() {
}

// 0xf65a94 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS5_11SpatialNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)
// IDA 0xf65a94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65a94() {
}

// 0xf65aa4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS5_11SpatialNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)
// IDA 0xf65aa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65aa4() {
}

// 0xf65ab4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)
// IDA 0xf65ab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ab4() {
}

// 0xf65ac4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)
// IDA 0xf65ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ac4() {
}

// 0xf65ad4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS2_b
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)
// IDA 0xf65ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ad4() {
}

// 0xf65ae4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS5_8TreeNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)
// IDA 0xf65ae4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ae4() {
}

// 0xf65af4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS2_b
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)
// IDA 0xf65af4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65af4() {
}

// 0xf65b04 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS2_
// type: int(void)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)
// IDA 0xf65b04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b04() {
}

// 0xf65b14 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// type: int __fastcall(int, RBX::SpatialHashStatic *this, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)
// IDA 0xf65b14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b14() {
}

// 0xf65b24 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE22visitPrimitivesInSpaceEPNS5_11SpaceFilterE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::visitPrimitivesInSpace(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpaceFilter *)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::visitPrimitivesInSpace(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpaceFilter *)
// IDA 0xf65b24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b24() {
}

// 0xf65b34 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS2_RKNS_7ExtentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(Ogre::RbxCullableSceneNode*,RBX::Extents const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(Ogre::RbxCullableSceneNode*,RBX::Extents const&)
// IDA 0xf65b34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b34() {
}

// 0xf65b44 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS2_
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(Ogre::RbxCullableSceneNode*)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(Ogre::RbxCullableSceneNode*)
// IDA 0xf65b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b44() {
}

// 0xf65b54 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE5setupEv
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::setup(void)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::setup(void)
// IDA 0xf65b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b54() {
}

// 0xf65b64 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS2_RKNS_12Vector3int32Eb
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::addNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,bool)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::addNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,bool)
// IDA 0xf65b64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b64() {
}

// 0xf65b74 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS4_i
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)
// IDA 0xf65b74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65b74() {
}

// 0xf65b84 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()
// IDA 0xf65b84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65b84() {
}

// 0xf65b94 — j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEED2Ev
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::~object_pool()")]
// was: boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::~object_pool()
// IDA 0xf65b94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65b94() {
}

// 0xf65ba4 — j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::~object_pool()")]
// was: boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::~object_pool()
// IDA 0xf65ba4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f65ba4() {
}

// 0xf65bc4 — j___ZNSt6vectorIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
#[doc(alias = "std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)")]
// was: std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)
// IDA 0xf65bc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65bc4() {
}

// 0xf65be4 — j___ZNSt6vectorIPN4Ogre20RbxCullableSceneNodeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RbxCullableSceneNode **,std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>>,Ogre::RbxCullableSceneNode * const&)")]
// was: std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RbxCullableSceneNode **,std::vector<Ogre::RbxCullableSceneNode *,std::allocator<Ogre::RbxCullableSceneNode *>>>,Ogre::RbxCullableSceneNode * const&)
// IDA 0xf65be4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f65be4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf65c54 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65c54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65c54() {
}

// 0xf65c64 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65c64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65c64() {
}

// 0xf65c74 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65c74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65c74() {
}

// 0xf65c84 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_17WaterFaceRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65c84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65c84() {
}

// 0xf65c94 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65c94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65c94() {
}

// 0xf65ca4 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65ca4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ca4() {
}

// 0xf65cb4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65cb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65cb4() {
}

// 0xf65cc4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65cc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65cc4() {
}

// 0xf65cd4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65cd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65cd4() {
}

// 0xf65ce4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_20SolidTerrainRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65ce4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ce4() {
}

// 0xf65cf4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65cf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65cf4() {
}

// 0xf65d04 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xf65d04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65d04() {
}

// 0xf65e04 — j___ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65e04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e04() {
}

// 0xf65e14 — j___ZN3RBX17WaterFaceRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65e14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e14() {
}

// 0xf65e24 — j___ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65e24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e24() {
}

// 0xf65e34 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12renderHelperENS_5Voxel4CellENS4_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS4_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)
// IDA 0xf65e34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e34() {
}

// 0xf65e44 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65e44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e44() {
}

// 0xf65e54 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65e54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e54() {
}

// 0xf65e64 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65e64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e64() {
}

// 0xf65e74 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65e74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e74() {
}

// 0xf65e84 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65e84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e84() {
}

// 0xf65e94 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12renderHelperENS_5Voxel4CellENS3_12CellMaterialERKN3G3D12Vector3int16EbRKNS6_7Vector3ENS3_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)
// IDA 0xf65e94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65e94() {
}

// 0xf65ea4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12wedgeUpEmptyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xf65ea4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ea4() {
}

// 0xf65eb4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE14detectOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65eb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65eb4() {
}

// 0xf65ec4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xf65ec4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ec4() {
}

// 0xf65ed4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65ed4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ed4() {
}

// 0xf65ee4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE9wedgeFaceERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xf65ee4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ee4() {
}

// 0xf65ef4 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12renderHelperENS1_4CellENS1_12CellMaterialERKN3G3D12Vector3int16EbRKNS7_7Vector3ENS1_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)
// IDA 0xf65ef4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65ef4() {
}

// 0xf65f04 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65f04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f04() {
}

// 0xf65f14 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65f14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f14() {
}

// 0xf65f24 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65f24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f24() {
}

// 0xf65f34 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xf65f34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f34() {
}

// 0xf65f44 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xf65f44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f44() {
}

// 0xf65f54 — j___ZN3RBX26MaterialTextureCoordinatesC2ERKN3G3D12Vector2int16ES4_S4_fb
// type: _DWORD __fastcall(RBX::MaterialTextureCoordinates *__hidden this, const G3D::Vector2int16 *, const G3D::Vector2int16 *, const G3D::Vector2int16 *, float, bool)
#[doc(alias = "RBX::MaterialTextureCoordinates::MaterialTextureCoordinates(G3D::Vector2int16 const&,G3D::Vector2int16 const&,G3D::Vector2int16 const&,float,bool)")]
// was: RBX::MaterialTextureCoordinates::MaterialTextureCoordinates(G3D::Vector2int16 const&,G3D::Vector2int16 const&,G3D::Vector2int16 const&,float,bool)
// IDA 0xf65f54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f54() {
}

// 0xf65f84 — j___ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk8loadDataINS0_4GridEEEvPKT_RKN3G3D12Vector3int16E
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::loadData<RBX::Voxel::Grid>(RBX::Voxel::Grid const*,G3D::Vector3int16 const&)")]
// was: void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::loadData<RBX::Voxel::Grid>(RBX::Voxel::Grid const*,G3D::Vector3int16 const&)
// IDA 0xf65f84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f84() {
}

// 0xf65f94 — j___ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk9fillEmptyERKN3G3D12Vector3int16ES7_
#[doc(alias = "RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillEmpty(G3D::Vector3int16 const&,G3D::Vector3int16 const&)")]
// was: RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillEmpty(G3D::Vector3int16 const&,G3D::Vector3int16 const&)
// IDA 0xf65f94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65f94() {
}

// 0xf65fa4 — j___ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xf65fa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65fa4() {
}

// 0xf65fb4 — j___ZNK3RBX20WaterRenderPredicateINS_19MegaClusterInstanceEE8internalERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionE
// type: int(void)
#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xf65fb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65fb4() {
}

// 0xf65fc4 — j___ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// was: RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xf65fc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65fc4() {
}

// 0xf65fd4 — j___ZNK3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS8_13LocalAreaInfoE
#[doc(alias = "RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const")]
// was: RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const
// IDA 0xf65fd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65fd4() {
}

// 0xf66024 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE5_copyERKS2_
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)")]
// was: G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)
// IDA 0xf66024: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66024() {
}

// 0xf66034 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::realloc(int)")]
// was: G3D::Array<G3D::Plane,10,32ul>::realloc(int)
// IDA 0xf66034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66034() {
}

// 0xf66044 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::~Array()")]
// was: G3D::Array<G3D::Plane,10,32ul>::~Array()
// IDA 0xf66044: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f66044() {
}

// 0xf66084 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS2_N5boost4hashIS8_EESaIS8_EEEEEvRKNS_7ExtentsERT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>>>(RBX::Extents const&,RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>> &)")]
// was: void RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>>>(RBX::Extents const&,RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>> &)
// IDA 0xf66084: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66084() {
}

// 0xf66094 — j___ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EE6insertERKS3_
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::insert(Ogre::RbxCullableSceneNode * const&)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::insert(Ogre::RbxCullableSceneNode * const&)
// IDA 0xf66094: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66094() {
}

// 0xf660a4 — j___ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EE6rehashEv
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::rehash(void)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::rehash(void)
// IDA 0xf660a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f660a4() {
}

// 0xf660b4 — j___ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EEC2ERKS3_mRKS6_
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::DenseHashSet(Ogre::RbxCullableSceneNode * const&,unsigned long,boost::hash<Ogre::RbxCullableSceneNode *> const&)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::DenseHashSet(Ogre::RbxCullableSceneNode * const&,unsigned long,boost::hash<Ogre::RbxCullableSceneNode *> const&)
// IDA 0xf660b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f660b4() {
}

// 0xf66134 — j___ZN3RBX9LightGrid15occupancyFillDFINS_21TorsoDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::TorsoDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::TorsoDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::TorsoDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::TorsoDistanceFunction &)
// IDA 0xf66134: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66134() {
}

// 0xf66144 — j___ZN3RBX9LightGrid15occupancyFillDFINS_21WedgeDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::WedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::WedgeDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::WedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::WedgeDistanceFunction &)
// IDA 0xf66144: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66144() {
}

// 0xf66154 — j___ZN3RBX9LightGrid15occupancyFillDFINS_22SphereDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::SphereDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::SphereDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::SphereDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::SphereDistanceFunction &)
// IDA 0xf66154: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66154() {
}

// 0xf66164 — j___ZN3RBX9LightGrid15occupancyFillDFINS_25CylinderXDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CylinderXDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderXDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CylinderXDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderXDistanceFunction &)
// IDA 0xf66164: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66164() {
}

// 0xf66174 — j___ZN3RBX9LightGrid15occupancyFillDFINS_25CylinderYDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CylinderYDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderYDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CylinderYDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderYDistanceFunction &)
// IDA 0xf66174: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66174() {
}

// 0xf66184 — j___ZN3RBX9LightGrid15occupancyFillDFINS_25EllipsoidDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::EllipsoidDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::EllipsoidDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::EllipsoidDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::EllipsoidDistanceFunction &)
// IDA 0xf66184: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66184() {
}

// 0xf66194 — j___ZN3RBX9LightGrid15occupancyFillDFINS_27CornerWedgeDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
// type: int __fastcall(int, int, int, int, int, float, int)
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CornerWedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CornerWedgeDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CornerWedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CornerWedgeDistanceFunction &)
// IDA 0xf66194: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66194() {
}

// 0xf66204 — j___ZN3RBX9LightGrid26lightingComputeShadowMaskZILb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66204: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66204() {
}

// 0xf66214 — j___ZN3RBX9LightGrid26lightingComputeShadowMaskZILb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66214: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66214() {
}
