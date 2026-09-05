// Auto-generated skeletons for rbx-script — Lua/Script/CodeGen/Luau/RBX::Script batch (filler cont.)
// Filter: Lua|Script|CodeGen|Luau|RBX::Script (case-sensitive)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Note: task filter yields 4456 funcs, all already stubbed (5551 existing via broader filter); this batch appends next 120 EA-sorted funcs not yet stubbed (global filler 0xf6a8d4..0xf6b044)
// Previous max script EA 0xf6a8c4, filtered remaining 0, filler from 0xf6a8d4 onward (EA-sorted, not yet in any crate).
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MaterialPtr>,std::_Select1st<std::pair<std::string const,Ogre::MaterialPtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MaterialPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MaterialPtr> const&)")]
pub fn stub_0xf6a8d4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::map<Ogre::GLES2FBOManager::RBFormat,Ogre::GLES2FBOManager::RBRef,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::GLES2FBOManager::RBFormat const&)")]
pub fn stub_0xf6a8e4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GLES2FBOManager::FormatProperties::Mode*,std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GLES2FBOManager::FormatProperties::Mode const&)")]
pub fn stub_0xf6a8f4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
pub fn stub_0xf6a904(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
pub fn stub_0xf6a914(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(Ogre::GLES2FBOManager::RBFormat const&)")]
pub fn stub_0xf6a924(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>> *)")]
pub fn stub_0xf6a934(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
pub fn stub_0xf6a944(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6a954(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>> *)")]
pub fn stub_0xf6a964(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::destroy(void)")]
pub fn stub_0xf6a974() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::RenderToVertexBuffer")
}

#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareIndexBuffer * const&)")]
pub fn stub_0xf6a984(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareVertexBuffer * const&)")]
pub fn stub_0xf6a994(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderTexture **,std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderTexture * const&)")]
pub fn stub_0xf6a9a4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::RenderTexture *,Ogre::STLAllocator<Ogre::RenderTexture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
pub fn stub_0xf6a9b4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()")]
pub fn stub_0xf6a9c4() -> crate::slot::PortedFn {
// IDA 0xf6a9c4: Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6a9c4, "Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *,boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *)")]
pub fn stub_0xf6a9d4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,unsigned int>>(unsigned int const&,std::pair<unsigned int const,unsigned int> &&)")]
pub fn stub_0xf6a9e4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
pub fn stub_0xf6a9f4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0xf6aa04(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned int const&)")]
pub fn stub_0xf6aa14(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
pub fn stub_0xf6aa24(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::string const&)")]
pub fn stub_0xf6aa34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6aa44(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6aa54(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwarePixelBuffer> const&)")]
pub fn stub_0xf6aa64(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xf6aa74() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPol~")
}

#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xf6aa84(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::HardwarePixelBufferSharedPtr*,std::vector<Ogre::HardwarePixelBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwarePixelBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::HardwarePixelBufferSharedPtr const&)")]
pub fn stub_0xf6aa94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::HardwarePixelBufferSharedPtr")
}

#[doc(alias = "std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image*,std::vector<Ogre::Image,Ogre::STLAllocator<Ogre::Image,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const&)")]
pub fn stub_0xf6aaa4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "Ogre::GLES2Support::~GLES2Support()")]
pub fn stub_0xf6aab4() -> crate::slot::PortedFn {
// IDA 0xf6aab4: Ogre::GLES2Support::~GLES2Support().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6aab4, "Ogre::GLES2Support::~GLES2Support()")
}

#[doc(alias = "Ogre::_ConfigOption::~_ConfigOption()")]
pub fn stub_0xf6aac4() -> crate::slot::PortedFn {
// IDA 0xf6aac4: Ogre::_ConfigOption::~_ConfigOption().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6aac4, "Ogre::_ConfigOption::~_ConfigOption()")
}

#[doc(alias = "std::map<std::string,Ogre::_ConfigOption,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6aad4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::pair<std::string const,Ogre::_ConfigOption>::pair(std::pair<std::string const,Ogre::_ConfigOption> const&)")]
pub fn stub_0xf6aae4() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xf6aaf4() -> crate::slot::PortedFn {
// IDA 0xf6aaf4: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(s~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6aaf4, "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCat~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::_ConfigOption> const&)")]
pub fn stub_0xf6ab04(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::_ConfigOption>>,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
pub fn stub_0xf6ab14(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::_ConfigOption>> *)")]
pub fn stub_0xf6ab24(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
pub fn stub_0xf6ab34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6ab44(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned int>> *)")]
pub fn stub_0xf6ab54(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
pub fn stub_0xf6ab64(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
pub fn stub_0xf6ab74(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>> *)")]
pub fn stub_0xf6ab84(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::map<std::string,Ogre::VertexElementSemantic,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf6ab94(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
pub fn stub_0xf6aba4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
pub fn stub_0xf6abb4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexElementSemantic>> *)")]
pub fn stub_0xf6abc4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
pub fn stub_0xf6abd4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *> const&)")]
pub fn stub_0xf6abe4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *> const&)")]
pub fn stub_0xf6abf4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESProgramPipeline *>> *)")]
pub fn stub_0xf6ac04(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::CPreprocessor::Macro::~Macro()")]
pub fn stub_0xf6ac14() -> crate::slot::PortedFn {
// IDA 0xf6ac14: Ogre::CPreprocessor::Macro::~Macro().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6ac14, "Ogre::CPreprocessor::Macro::~Macro()")
}

#[doc(alias = "std::vector<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GLUniformReference*,std::vector<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GLUniformReference const&)")]
pub fn stub_0xf6ac24(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,unsigned int> const&)")]
pub fn stub_0xf6ac34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf6ac44(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned int> const&)")]
pub fn stub_0xf6ac54(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleEmitterFactory **,std::vector<Ogre::ParticleEmitterFactory *,Ogre::STLAllocator<Ogre::ParticleEmitterFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleEmitterFactory * const&)")]
pub fn stub_0xf6ac64(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleAffectorFactory **,std::vector<Ogre::ParticleAffectorFactory *,Ogre::STLAllocator<Ogre::ParticleAffectorFactory *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleAffectorFactory * const&)")]
pub fn stub_0xf6ac74(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,RBX::FileMeshFace const&)")]
pub fn stub_0xf6ac84(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,unsigned long,RBX::FileMeshFace const&)")]
pub fn stub_0xf6ac94(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::reserve(unsigned long)")]
pub fn stub_0xf6aca4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,RBX::FileMeshVertexNormalTexture3d const&)")]
pub fn stub_0xf6acb4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,unsigned long,RBX::FileMeshVertexNormalTexture3d const&)")]
pub fn stub_0xf6acc4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::reserve(unsigned long)")]
pub fn stub_0xf6acd4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,char *,char *,std::forward_iterator_tag)")]
pub fn stub_0xf6ace4() -> crate::slot::PortedFn {
// IDA 0xf6ace4: void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<char *,std::vector<cha~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6ace4, "void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<ch~")
}

#[doc(alias = "RBX::WindowAverage<double,double>::getStats(unsigned long)const")]
pub fn stub_0xf6acf4(avg: &crate::slot::WindowAverage, stat: u32) -> f64 {
// WindowAverage::getStats — 0 selects the mean, else the
// window variance; empty windows yield zero.
avg.get_stats(stat)
}

#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>> const&)const")]
pub fn stub_0xf6ad04() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::append(RBX::IAdornable * const&)")]
pub fn stub_0xf6ad14(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::resize(int,bool)")]
pub fn stub_0xf6ad24(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::realloc(int)")]
pub fn stub_0xf6ad34(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::~Array()")]
pub fn stub_0xf6ad44(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc2d>::fastRemove(RBX::IAdornable*)")]
pub fn stub_0xf6ad54(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3d>::fastRemove(RBX::IAdornable*)")]
pub fn stub_0xf6ad64(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3dSorted>::fastRemove(RBX::IAdornable*)")]
pub fn stub_0xf6ad74(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable * const&)")]
pub fn stub_0xf6ad84(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "_SCNetworkReachabilityCreateWithAddress")]
pub fn stub_0xf6ad94() -> crate::slot::PortedFn {
// IDA 0xf6ad94: _SCNetworkReachabilityCreateWithAddress.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ad94, "_SCNetworkReachabilityCreateWithAddress")
}

#[doc(alias = "_SCNetworkReachabilityCreateWithName")]
pub fn stub_0xf6ada4() -> crate::slot::PortedFn {
// IDA 0xf6ada4: _SCNetworkReachabilityCreateWithName.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ada4, "_SCNetworkReachabilityCreateWithName")
}

#[doc(alias = "_SCNetworkReachabilityGetFlags")]
pub fn stub_0xf6adb4() -> crate::slot::PortedFn {
// IDA 0xf6adb4: _SCNetworkReachabilityGetFlags.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6adb4, "_SCNetworkReachabilityGetFlags")
}

#[doc(alias = "_SCNetworkReachabilityScheduleWithRunLoop")]
pub fn stub_0xf6adc4() -> crate::slot::PortedFn {
// IDA 0xf6adc4: _SCNetworkReachabilityScheduleWithRunLoop.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6adc4, "_SCNetworkReachabilityScheduleWithRunLoop")
}

#[doc(alias = "_SCNetworkReachabilitySetCallback")]
pub fn stub_0xf6add4() -> crate::slot::PortedFn {
// IDA 0xf6add4: _SCNetworkReachabilitySetCallback.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6add4, "_SCNetworkReachabilitySetCallback")
}

#[doc(alias = "_SCNetworkReachabilityUnscheduleFromRunLoop")]
pub fn stub_0xf6ade4() -> crate::slot::PortedFn {
// IDA 0xf6ade4: _SCNetworkReachabilityUnscheduleFromRunLoop.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ade4, "_SCNetworkReachabilityUnscheduleFromRunLoop")
}

#[doc(alias = "_class_addMethod")]
pub fn stub_0xf6adf4() -> crate::slot::PortedFn {
// IDA 0xf6adf4: _class_addMethod.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6adf4, "_class_addMethod")
}

#[doc(alias = "_class_getInstanceMethod")]
pub fn stub_0xf6ae04() -> crate::slot::PortedFn {
// IDA 0xf6ae04: _class_getInstanceMethod.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae04, "_class_getInstanceMethod")
}

#[doc(alias = "_class_getInstanceSize")]
pub fn stub_0xf6ae14() -> crate::slot::PortedFn {
// IDA 0xf6ae14: _class_getInstanceSize.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae14, "_class_getInstanceSize")
}

#[doc(alias = "_class_getInstanceVariable")]
pub fn stub_0xf6ae24() -> crate::slot::PortedFn {
// IDA 0xf6ae24: _class_getInstanceVariable.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae24, "_class_getInstanceVariable")
}

#[doc(alias = "_class_getIvarLayout")]
pub fn stub_0xf6ae34() -> crate::slot::PortedFn {
// IDA 0xf6ae34: _class_getIvarLayout.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae34, "_class_getIvarLayout")
}

#[doc(alias = "_class_getSuperclass")]
pub fn stub_0xf6ae44() -> crate::slot::PortedFn {
// IDA 0xf6ae44: _class_getSuperclass.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae44, "_class_getSuperclass")
}

#[doc(alias = "_ivar_getName")]
pub fn stub_0xf6ae54() -> crate::slot::PortedFn {
// IDA 0xf6ae54: _ivar_getName.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae54, "_ivar_getName")
}

#[doc(alias = "_ivar_getOffset")]
pub fn stub_0xf6ae64() -> crate::slot::PortedFn {
// IDA 0xf6ae64: _ivar_getOffset.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae64, "_ivar_getOffset")
}

#[doc(alias = "_objc_autorelease")]
pub fn stub_0xf6ae74(obj: &mut crate::slot::ObjcRef) -> usize {
// _objc_autorelease — pool drain is host-side; the count
// is unchanged here.
obj.retain()
}

#[doc(alias = "_objc_autoreleaseReturnValue")]
pub fn stub_0xf6ae84(obj: &mut crate::slot::ObjcRef) -> usize {
// _objc_autorelease — pool drain is host-side; the count
// is unchanged here.
obj.retain()
}

#[doc(alias = "_objc_begin_catch")]
pub fn stub_0xf6ae94() -> crate::slot::PortedFn {
// IDA 0xf6ae94: _objc_begin_catch.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6ae94, "_objc_begin_catch")
}

#[doc(alias = "_objc_end_catch")]
pub fn stub_0xf6aea4() -> crate::slot::PortedFn {
// IDA 0xf6aea4: _objc_end_catch.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aea4, "_objc_end_catch")
}

#[doc(alias = "_objc_enumerationMutation")]
pub fn stub_0xf6aeb4() -> crate::slot::PortedFn {
// IDA 0xf6aeb4: _objc_enumerationMutation.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aeb4, "_objc_enumerationMutation")
}

#[doc(alias = "_objc_exception_rethrow")]
pub fn stub_0xf6aec4() -> crate::slot::PortedFn {
// IDA 0xf6aec4: _objc_exception_rethrow.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aec4, "_objc_exception_rethrow")
}

#[doc(alias = "_objc_getAssociatedObject")]
pub fn stub_0xf6aed4() -> crate::slot::PortedFn {
// IDA 0xf6aed4: _objc_getAssociatedObject.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aed4, "_objc_getAssociatedObject")
}

#[doc(alias = "_objc_getClass")]
pub fn stub_0xf6aee4() -> crate::slot::PortedFn {
// IDA 0xf6aee4: _objc_getClass.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aee4, "_objc_getClass")
}

#[doc(alias = "_objc_getProperty")]
pub fn stub_0xf6aef4() -> crate::slot::PortedFn {
// IDA 0xf6aef4: _objc_getProperty.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aef4, "_objc_getProperty")
}

#[doc(alias = "_objc_msgSend")]
pub fn stub_0xf6af04() -> crate::slot::PortedFn {
// IDA 0xf6af04: _objc_msgSend.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af04, "_objc_msgSend")
}

#[doc(alias = "_objc_msgSendSuper2")]
pub fn stub_0xf6af14() -> crate::slot::PortedFn {
// IDA 0xf6af14: _objc_msgSendSuper2.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af14, "_objc_msgSendSuper2")
}

#[doc(alias = "_objc_msgSend_stret")]
pub fn stub_0xf6af24() -> crate::slot::PortedFn {
// IDA 0xf6af24: _objc_msgSend_stret.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af24, "_objc_msgSend_stret")
}

#[doc(alias = "_objc_release")]
pub fn stub_0xf6af34(obj: &mut crate::slot::ObjcRef) -> usize {
// _objc_release — zero means the last release freed it.
obj.release()
}

#[doc(alias = "_objc_retain")]
pub fn stub_0xf6af44(obj: &mut crate::slot::ObjcRef) -> usize {
// _objc_retain — bump the retain count.
obj.retain()
}

#[doc(alias = "_objc_retainAutorelease")]
pub fn stub_0xf6af54() -> crate::slot::PortedFn {
// IDA 0xf6af54: _objc_retainAutorelease.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af54, "_objc_retainAutorelease")
}

#[doc(alias = "_objc_retainAutoreleaseReturnValue")]
pub fn stub_0xf6af64() -> crate::slot::PortedFn {
// IDA 0xf6af64: _objc_retainAutoreleaseReturnValue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af64, "_objc_retainAutoreleaseReturnValue")
}

#[doc(alias = "_objc_retainAutoreleasedReturnValue")]
pub fn stub_0xf6af74() -> crate::slot::PortedFn {
// IDA 0xf6af74: _objc_retainAutoreleasedReturnValue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af74, "_objc_retainAutoreleasedReturnValue")
}

#[doc(alias = "_objc_retainBlock")]
pub fn stub_0xf6af84(obj: &mut crate::slot::ObjcRef) -> usize {
// _objc_retain — bump the retain count.
obj.retain()
}

#[doc(alias = "_objc_setAssociatedObject")]
pub fn stub_0xf6af94() -> crate::slot::PortedFn {
// IDA 0xf6af94: _objc_setAssociatedObject.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6af94, "_objc_setAssociatedObject")
}

#[doc(alias = "_objc_setProperty")]
pub fn stub_0xf6afa4() -> crate::slot::PortedFn {
// IDA 0xf6afa4: _objc_setProperty.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6afa4, "_objc_setProperty")
}

#[doc(alias = "_objc_storeStrong")]
pub fn stub_0xf6afb4() -> crate::slot::PortedFn {
// IDA 0xf6afb4: _objc_storeStrong.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6afb4, "_objc_storeStrong")
}

#[doc(alias = "_objc_sync_enter")]
pub fn stub_0xf6afc4() -> crate::slot::PortedFn {
// IDA 0xf6afc4: _objc_sync_enter.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6afc4, "_objc_sync_enter")
}

#[doc(alias = "_objc_sync_exit")]
pub fn stub_0xf6afd4() -> crate::slot::PortedFn {
// IDA 0xf6afd4: _objc_sync_exit.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6afd4, "_objc_sync_exit")
}

#[doc(alias = "_object_getClass")]
pub fn stub_0xf6afe4() -> crate::slot::PortedFn {
// IDA 0xf6afe4: _object_getClass.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6afe4, "_object_getClass")
}

#[doc(alias = "_object_setIvar")]
pub fn stub_0xf6aff4() -> crate::slot::PortedFn {
// IDA 0xf6aff4: _object_setIvar.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6aff4, "_object_setIvar")
}

#[doc(alias = "_protocol_getMethodDescription")]
pub fn stub_0xf6b004() -> crate::slot::PortedFn {
// IDA 0xf6b004: _protocol_getMethodDescription.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6b004, "_protocol_getMethodDescription")
}

#[doc(alias = "_sel_getUid")]
pub fn stub_0xf6b014() -> crate::slot::PortedFn {
// IDA 0xf6b014: _sel_getUid.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6b014, "_sel_getUid")
}

#[doc(alias = "std::string::find_last_of(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b024() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::find_first_of(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b034() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::find_last_not_of(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b044() -> String {
// std::string ctor — empty.
String::new()
}
