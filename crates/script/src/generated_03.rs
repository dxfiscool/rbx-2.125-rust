// Auto-generated skeletons for rbx-script — Lua/Script/CodeGen/Luau/RBX::Script batch (filler)
// Filter: Lua|Script|CodeGen|Luau|RBX::Script (case-sensitive)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Note: task filter yields 4456 funcs, all already stubbed in crates/script/src/*.rs (5401 existing via broader Script|Lua|Yield|lua filter).
// This batch appends next 150 EA-sorted funcs not yet stubbed in script crate (global high-EA filler 0xf69f74..0xf6a8c4) to satisfy skeleton continuation.
// Previous max script EA 0xf69f64, filtered remaining 0, filler from 0xf69f74 onward (EA-sorted, not yet in any crate).
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")]
pub fn stub_0xf69f74(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)")]
pub fn stub_0xf69f84(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")]
pub fn stub_0xf69f94(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")]
pub fn stub_0xf69fa4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")]
pub fn stub_0xf69fb4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")]
pub fn stub_0xf69fc4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")]
pub fn stub_0xf69fd4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")]
pub fn stub_0xf69fe4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")]
pub fn stub_0xf69ff4(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")]
pub fn stub_0xf6a004() -> crate::slot::PortedFn {
// IDA 0xf6a004: std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllo~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a004, "std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,O~")
}

#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")]
pub fn stub_0xf6a014(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a024() -> crate::slot::PortedFn {
// IDA 0xf6a024: std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a024, "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogr~")
}

#[doc(alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")]
pub fn stub_0xf6a034(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")]
pub fn stub_0xf6a044(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")]
pub fn stub_0xf6a054() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuProgram")
}

#[doc(alias = "std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")]
pub fn stub_0xf6a064(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)")]
pub fn stub_0xf6a074() -> crate::slot::PortedFn {
// IDA 0xf6a074: Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(O~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a074, "Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCat~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
pub fn stub_0xf6a084(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a094(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)")]
pub fn stub_0xf6a0a4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)")]
pub fn stub_0xf6a0b4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)")]
pub fn stub_0xf6a0c4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)")]
pub fn stub_0xf6a0d4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)")]
pub fn stub_0xf6a0e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)")]
pub fn stub_0xf6a0f4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_0xf6a104(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_0xf6a114(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a124(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)")]
pub fn stub_0xf6a134(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_0xf6a144(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a154(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)")]
pub fn stub_0xf6a164(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")]
pub fn stub_0xf6a174() -> crate::slot::PortedFn {
// IDA 0xf6a174: Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a174, "Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")
}

#[doc(alias = "Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")]
pub fn stub_0xf6a184() -> crate::slot::PortedFn {
// IDA 0xf6a184: Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a184, "Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")
}

#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a194(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a1a4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)")]
pub fn stub_0xf6a1b4() -> crate::slot::PortedFn {
// IDA 0xf6a1b4: void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCate~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a1b4, "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPoli~")
}

#[doc(alias = "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a1c4() -> crate::slot::PortedFn {
// IDA 0xf6a1c4: std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a1c4, "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(O~")
}

#[doc(alias = "std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")]
pub fn stub_0xf6a1d4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")]
pub fn stub_0xf6a1e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)")]
pub fn stub_0xf6a1f4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")]
pub fn stub_0xf6a204(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")]
pub fn stub_0xf6a214(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)")]
pub fn stub_0xf6a224(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a234(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a244(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")]
pub fn stub_0xf6a254(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xf6a264(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xf6a274(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
pub fn stub_0xf6a284(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf6a294(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf6a2a4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a2b4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)")]
pub fn stub_0xf6a2c4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf6a2d4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf6a2e4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf6a2f4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a304(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)")]
pub fn stub_0xf6a314(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf6a324(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
pub fn stub_0xf6a334(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
pub fn stub_0xf6a344(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)")]
pub fn stub_0xf6a354(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)")]
pub fn stub_0xf6a364(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)")]
pub fn stub_0xf6a374(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> * std::__uninitialized_copy_a<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a384(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
pub fn stub_0xf6a394(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
pub fn stub_0xf6a3a4(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)")]
pub fn stub_0xf6a3b4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a3c4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)")]
pub fn stub_0xf6a3d4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")]
pub fn stub_0xf6a3e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a3f4() -> crate::slot::PortedFn {
// IDA 0xf6a3f4: std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolic~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a3f4, "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::C~")
}

#[doc(alias = "std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule const&)")]
pub fn stub_0xf6a404(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a414() -> crate::slot::PortedFn {
// IDA 0xf6a414: std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAl~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a414, "std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule~")
}

#[doc(alias = "std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")]
pub fn stub_0xf6a424(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")]
pub fn stub_0xf6a434(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "Ogre::Technique::GPUDeviceNameRule* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a444(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "Ogre::Technique::GPUDeviceNameRule * std::__uninitialized_copy_a<Ogre::Technique::GPUDeviceNameRule *,Ogre::Technique::GPUDeviceNameRule *,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::Technique::GPUDeviceNameRule *,Ogre::Technique::GPUDeviceNameRule *,Ogre::Technique::GPUDeviceNameRule *,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a454(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::TexturePtr const&)")]
pub fn stub_0xf6a464(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a474() -> crate::slot::PortedFn {
// IDA 0xf6a474: std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a474, "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre:~")
}

#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,std::string const&)")]
pub fn stub_0xf6a484(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>)")]
pub fn stub_0xf6a494(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> const*,std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>*)")]
pub fn stub_0xf6a4a4() -> crate::slot::PortedFn {
// IDA 0xf6a4a4: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a4a4, "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEff~")
}

#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> *)")]
pub fn stub_0xf6a4b4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "void std::__uninitialized_fill_n_a<std::string *,unsigned long,std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,unsigned long,std::string const&,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a4c4() -> crate::slot::PortedFn {
// IDA 0xf6a4c4: void std::__uninitialized_fill_n_a<std::string *,unsigned long,std::string,Ogre::STLAllocator<std::string,Ogre::Categori~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a4c4, "void std::__uninitialized_fill_n_a<std::string *,unsigned long,std::string,Ogre::STLAllocator<std::s~")
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::operator=(Ogre::SharedPtr<Ogre::HighLevelGpuProgram> const&)")]
pub fn stub_0xf6a4d4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Any>,std::_Select1st<std::pair<std::string const,Ogre::Any>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Any>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Any>> *)")]
pub fn stub_0xf6a4e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_Rep::_M_clone(std::allocator<unsigned short> const&,unsigned long)")]
pub fn stub_0xf6a4f4(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::append(unsigned short const*,unsigned long)")]
pub fn stub_0xf6a504(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_M_mutate(unsigned long,unsigned long,unsigned long)")]
pub fn stub_0xf6a514(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAnimationData,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexData::HardwareAnimationData*,std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAnimationData,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexData::HardwareAnimationData const&)")]
pub fn stub_0xf6a524(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAnimationData,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAnimationData,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a534() -> crate::slot::PortedFn {
// IDA 0xf6a534: std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAnimationData,Ogre::Cat~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a534, "std::vector<Ogre::VertexData::HardwareAnimationData,Ogre::STLAllocator<Ogre::VertexData::HardwareAni~")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,unsigned short>> *)")]
pub fn stub_0xf6a544(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(unsigned short const&)")]
pub fn stub_0xf6a554(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::RenderWindow *,std::pair<Ogre::RenderWindow * const,Ogre::WindowEventListener *>,std::_Select1st<std::pair<Ogre::RenderWindow * const,Ogre::WindowEventListener *>>,std::less<Ogre::RenderWindow *>,Ogre::STLAllocator<std::pair<Ogre::RenderWindow * const,Ogre::WindowEventListener *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::RenderWindow * const,Ogre::WindowEventListener *>> *)")]
pub fn stub_0xf6a564(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::destroy(void)")]
pub fn stub_0xf6a574() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
pub fn stub_0xf6a584() -> crate::slot::PortedFn {
// IDA 0xf6a584: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a584, "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::Cate~")
}

#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
pub fn stub_0xf6a594() -> crate::slot::PortedFn {
// IDA 0xf6a594: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a594, "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::Ca~")
}

#[doc(alias = "std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
pub fn stub_0xf6a5a4(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list(std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a5b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0xf6a5c4() -> crate::slot::PortedFn {
// IDA 0xf6a5c4: std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a5c4, "std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::Categorise~")
}

#[doc(alias = "std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
pub fn stub_0xf6a5d4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0xf6a5e4() -> crate::slot::PortedFn {
// IDA 0xf6a5e4: std::deque<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a5e4, "std::deque<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::Categori~")
}

#[doc(alias = "std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
pub fn stub_0xf6a5f4(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
pub fn stub_0xf6a604(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::find(std::string const&)")]
pub fn stub_0xf6a614(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
pub fn stub_0xf6a624(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
pub fn stub_0xf6a634(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a644() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const*,std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>*)")]
pub fn stub_0xf6a654() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
pub fn stub_0xf6a664() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a674() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::DefaultWorkQueueBase::RequestHandlerHolder")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a684() -> crate::slot::PortedFn {
// IDA 0xf6a684: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAlloca~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a684, "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandl~")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a694(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
pub fn stub_0xf6a6a4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*)")]
pub fn stub_0xf6a6b4() -> crate::slot::PortedFn {
// IDA 0xf6a6b4: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a6b4, "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Og~")
}

#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xf6a6c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAl~")
}

#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xf6a6d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPol~")
}

#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo const&)")]
pub fn stub_0xf6a6e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::FileInfo const&)")]
pub fn stub_0xf6a6f4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a704(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "Ogre::FileInfo * std::__uninitialized_copy_a<Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::FileInfo *,Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_0xf6a714(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::map<std::string,Ogre::InstanceManager::BatchSettings,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a724(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::map<std::string,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a734(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstanceBatch **,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstanceBatch * const&)")]
pub fn stub_0xf6a744(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6a754() -> crate::slot::PortedFn {
// IDA 0xf6a754: std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCat~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a754, "std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPol~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
pub fn stub_0xf6a764(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
pub fn stub_0xf6a774(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>> *)")]
pub fn stub_0xf6a784(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
pub fn stub_0xf6a794(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a7a4() -> crate::slot::PortedFn {
// IDA 0xf6a7a4: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::Instanc~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a7a4, "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAll~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a7b4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a7c4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
pub fn stub_0xf6a7d4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6a7e4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::RenderTarget * const&)")]
pub fn stub_0xf6a7f4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::RenderTarget *> *)")]
pub fn stub_0xf6a804(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::vector<Ogre::InstancedEntity *,Ogre::STLAllocator<Ogre::InstancedEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedEntity **,std::vector<Ogre::InstancedEntity *,Ogre::STLAllocator<Ogre::InstancedEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedEntity * const&)")]
pub fn stub_0xf6a814(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::InstancedEntity *,Ogre::STLAllocator<Ogre::InstancedEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
pub fn stub_0xf6a824(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::MaterialPtr>*)")]
pub fn stub_0xf6a834() -> crate::slot::PortedFn {
// IDA 0xf6a834: Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a834, "Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre~")
}

#[doc(alias = "std::map<std::string,Ogre::MaterialPtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6a844(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned char *,std::vector<unsigned char,Ogre::STLAllocator<unsigned char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned char const&)")]
pub fn stub_0xf6a854(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Matrix4 * const,unsigned long> const&)")]
pub fn stub_0xf6a864(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::Matrix4 *,std::pair<Ogre::Matrix4 * const,unsigned long>,std::_Select1st<std::pair<Ogre::Matrix4 * const,unsigned long>>,std::less<Ogre::Matrix4 *>,Ogre::STLAllocator<std::pair<Ogre::Matrix4 * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Matrix4 * const,unsigned long>> *)")]
pub fn stub_0xf6a874(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
pub fn stub_0xf6a884() -> crate::slot::PortedFn {
// IDA 0xf6a884: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogr~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a884, "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<s~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MaterialPtr> const&)")]
pub fn stub_0xf6a894(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MaterialPtr>>,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
pub fn stub_0xf6a8a4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a8b4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MaterialPtr>> *)")]
pub fn stub_0xf6a8c4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}
