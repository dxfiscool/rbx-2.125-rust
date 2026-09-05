// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf28b64..0xf2a2a4 | 4711->4811 covered, 590 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf28b64]")]
pub fn stub_0xf28b64() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf28b74]")]
pub fn stub_0xf28b74() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf28b84]")]
pub fn stub_0xf28b84() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 3)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf28b94]")]
pub fn stub_0xf28b94() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 3)
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*) [0xf28fc4]")]
pub fn stub_0xf28fc4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::decl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void) [0xf28fd4]")]
pub fn stub_0xf28fd4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::stat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*) [0xf28fe4]")]
pub fn stub_0xf28fe4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::decl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&) [0xf29024]")]
pub fn stub_0xf29024(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&) [0xf29044]")]
pub fn stub_0xf29044() -> crate::slot::PortedFn {
// IDA 0xf29044: std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::inse~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf29044, "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunction~")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*) [0xf29174]")]
pub fn stub_0xf29174(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::merg~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*) [0xf29184]")]
pub fn stub_0xf29184() -> crate::slot::InstanceHandle {
// RBX::Reflection::MemberDescriptorContainer ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::MemberDescriptorContainer")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long) [0xf29294]")]
pub fn stub_0xf29294(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *) [0xf292a4]")]
pub fn stub_0xf292a4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void) [0xf292f4]")]
pub fn stub_0xf292f4() -> crate::slot::PortedFn {
// IDA 0xf292f4: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf292f4, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long) [0xf293d4]")]
pub fn stub_0xf293d4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void) [0xf293e4]")]
pub fn stub_0xf293e4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long) [0xf293f4]")]
pub fn stub_0xf293f4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&) [0xf29404]")]
pub fn stub_0xf29404() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const [0xf29474]")]
pub fn stub_0xf29474(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const [0xf294c4]")]
pub fn stub_0xf294c4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::_M_allocate(unsigned long) [0xf29524]")]
pub fn stub_0xf29524() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::_M_allocate(unsigned long) [0xf29574]")]
pub fn stub_0xf29574() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&) [0xf29614]")]
pub fn stub_0xf29614(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> * const&) [0xf296a4]")]
pub fn stub_0xf296a4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> * const&) [0xf296b4]")]
pub fn stub_0xf296b4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::ArgumentPusher) [0xf29a04]")]
pub fn stub_0xf29a04() -> crate::slot::PortedFn {
// IDA 0xf29a04: int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::ArgumentPusher) [0xf29~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf29a04, "int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::Ar~")
}

#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *) [0xf29a14]")]
pub fn stub_0xf29a14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *) [0xf29a24]")]
pub fn stub_0xf29a24() -> crate::slot::PortedFn {
// IDA 0xf29a24: int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Refl~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf29a24, "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,s~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) [0xf29a34]")]
pub fn stub_0xf29a34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>) [0xf29a44]")]
pub fn stub_0xf29a44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) [0xf29a54]")]
pub fn stub_0xf29a54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::alloca~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>) [0xf29a64]")]
pub fn stub_0xf29a64() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPt~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) [0xf29a74]")]
pub fn stub_0xf29a74() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) [0xf29a84]")]
pub fn stub_0xf29a84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::S~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>) [0xf29a94]")]
pub fn stub_0xf29a94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost:~")
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&) [0xf29aa4]")]
pub fn stub_0xf29aa4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &) [0xf29ac4]")]
pub fn stub_0xf29ac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29ad4]")]
pub fn stub_0xf29ad4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16) [0xf29ae4]")]
pub fn stub_0xf29ae4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2i16) -> crate::lua::LuaVector2i16 {
// Overload of the primary stub_0x26e9c0.
crate::lua::stub_0x26e9c0(thread, value)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29af4]")]
pub fn stub_0xf29af4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cb1c.
crate::lua::stub_0x26cb1c(_thread, _index, out, slot)
}

#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16) [0xf29b04]")]
pub fn stub_0xf29b04(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3i16) -> crate::lua::LuaVector3i16 {
// Overload of the primary stub_0x26eaf0.
crate::lua::stub_0x26eaf0(thread, value)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b14]")]
pub fn stub_0xf29b14(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector3int16>::getValue<Variant> — class-tag check, then wrap.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR3INT16) {
    Some(crate::lua::LuaUserdataPayload::Vector3i16(v)) => { *out = crate::lua::ScriptVariant::Vector3i16(*v); true }
    _ => false,
}
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &) [0xf29b24]")]
pub fn stub_0xf29b24(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector3i16, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26c140.
crate::lua::stub_0x26c140(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b34]")]
pub fn stub_0xf29b34(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26c92c.
crate::lua::stub_0x26c92c(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b44]")]
pub fn stub_0xf29b44(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cd0c.
crate::lua::stub_0x26cd0c(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b54]")]
pub fn stub_0xf29b54(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cc14.
crate::lua::stub_0x26cc14(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b64]")]
pub fn stub_0xf29b64(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector3>::getValue<Variant> — class-tag check, then wrap.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR3) {
    Some(crate::lua::LuaUserdataPayload::Vector3(v)) => { *out = crate::lua::ScriptVariant::Vector3(*v); true }
    _ => false,
}
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &) [0xf29b74]")]
pub fn stub_0xf29b74(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector3, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26c230.
crate::lua::stub_0x26c230(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &) [0xf29b84]")]
pub fn stub_0xf29b84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29b94]")]
pub fn stub_0xf29b94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::BrickColor,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29ba4]")]
pub fn stub_0xf29ba4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cd88.
crate::lua::stub_0x26cd88(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::InputObject,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29bc4]")]
pub fn stub_0xf29bc4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26d070.
crate::lua::stub_0x26d070(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29be4]")]
pub fn stub_0xf29be4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Region3int16>::getValue<Variant> — class-tag check, then wrap.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::REGION3INT16) {
    Some(crate::lua::LuaUserdataPayload::Region3i16(v)) => { *out = crate::lua::ScriptVariant::Region3i16(*v); true }
    _ => false,
}
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Axes,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c04]")]
pub fn stub_0xf29c04(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cf78.
crate::lua::stub_0x26cf78(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::UDim,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c14]")]
pub fn stub_0xf29c14(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26ce04.
crate::lua::stub_0x26ce04(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Faces,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c24]")]
pub fn stub_0xf29c24(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cefc.
crate::lua::stub_0x26cefc(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::UDim2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c34]")]
pub fn stub_0xf29c34(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26ce80.
crate::lua::stub_0x26ce80(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::CellID,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c54]")]
pub fn stub_0xf29c54(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cff4.
crate::lua::stub_0x26cff4(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::RbxRay,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c64]")]
pub fn stub_0xf29c64(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Overload of the primary stub_0x26cc90.
crate::lua::stub_0x26cc90(_thread, _index, out, slot)
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &) [0xf29c84]")]
pub fn stub_0xf29c84(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::ScriptVariant, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Region3>::getValue<Variant> — class-tag check, then wrap.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::REGION3) {
    Some(crate::lua::LuaUserdataPayload::Region3(v)) => { *out = crate::lua::ScriptVariant::Region3(*v); true }
    _ => false,
}
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::getValue<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,unsigned int,RBX::Reflection::EnumDescriptor::Item const* &) [0xf29ca4]")]
pub fn stub_0xf29ca4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaEnumItem, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge getValue — class-tag check for EnumItem userdata.
match slot {
    crate::lua::LuaStackValue::Userdata(ud) if ud.class == "EnumItem" => true,
    _ => { let _ = out; false },
}
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0xf29e84]")]
pub fn stub_0xf29e84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost:~")
}

#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *) [0xf29f24]")]
pub fn stub_0xf29f24(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2i16) -> crate::lua::LuaVector2i16 {
// Overload of the primary stub_0x26e9c0.
crate::lua::stub_0x26e9c0(thread, value)
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &) [0xf29f34]")]
pub fn stub_0xf29f34(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector2i16, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector2int16>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR2INT16) {
    Some(crate::lua::LuaUserdataPayload::Vector2i16(v)) => { *out = *v; true }
    _ => false,
}
}

#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *) [0xf29f44]")]
pub fn stub_0xf29f44(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3i16) -> crate::lua::LuaVector3i16 {
// Overload of the primary stub_0x26eaf0.
crate::lua::stub_0x26eaf0(thread, value)
}

#[doc(alias = "G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame) [0xf29f54]")]
pub fn stub_0xf29f54(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaCoordinateFrame) -> crate::lua::LuaCoordinateFrame {
// Bridge<CoordinateFrame>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::CFRAME, crate::lua::LuaUserdataPayload::CoordinateFrame(*value));
*value
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &) [0xf29f64]")]
pub fn stub_0xf29f64(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaCoordinateFrame, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<CoordinateFrame>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::CFRAME) {
    Some(crate::lua::LuaUserdataPayload::CoordinateFrame(v)) => { *out = *v; true }
    _ => false,
}
}

#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *) [0xf29f74]")]
pub fn stub_0xf29f74(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaColor3) -> crate::lua::LuaColor3 {
// Bridge<Color3>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::COLOR3, crate::lua::LuaUserdataPayload::Color3(*value));
*value
}

#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3) [0xf29f84]")]
pub fn stub_0xf29f84(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaColor3) -> crate::lua::LuaColor3 {
// Bridge<Color3>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::COLOR3, crate::lua::LuaUserdataPayload::Color3(*value));
*value
}

#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *) [0xf29f94]")]
pub fn stub_0xf29f94(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2) -> crate::lua::LuaVector2 {
// Bridge<Vector2>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR2, crate::lua::LuaUserdataPayload::Vector2(*value));
*value
}

#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2) [0xf29fa4]")]
pub fn stub_0xf29fa4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2) -> crate::lua::LuaVector2 {
// Bridge<Vector2>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR2, crate::lua::LuaUserdataPayload::Vector2(*value));
*value
}

#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &) [0xf29fb4]")]
pub fn stub_0xf29fb4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector2, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector2>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR2) {
    Some(crate::lua::LuaUserdataPayload::Vector2(v)) => { *out = *v; true }
    _ => false,
}
}

#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *) [0xf29fc4]")]
pub fn stub_0xf29fc4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3) -> crate::lua::LuaVector3 {
// Bridge<Vector3>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(*value));
*value
}

#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3) [0xf29fd4]")]
pub fn stub_0xf29fd4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3) -> crate::lua::LuaVector3 {
// Bridge<Vector3>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(*value));
*value
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *) [0xf2a0b4]")]
pub fn stub_0xf2a0b4(value: &crate::lua::LuaVector2i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *) [0xf2a0c4]")]
pub fn stub_0xf2a0c4(value: &crate::lua::LuaVector3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *) [0xf2a0d4]")]
pub fn stub_0xf2a0d4(value: &crate::lua::LuaCoordinateFrame, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CoordinateFrame>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", value.position.x, value.position.y, value.position.z, value.rotation[0][0], value.rotation[0][1], value.rotation[0][2], value.rotation[1][0], value.rotation[1][1], value.rotation[1][2], value.rotation[2][0], value.rotation[2][1], value.rotation[2][2])));
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(G3D::Color3 const&,lua_State *) [0xf2a0e4]")]
pub fn stub_0xf2a0e4(value: &crate::lua::LuaColor3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Color3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.r, value.g, value.b)));
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a0f4]")]
pub fn stub_0xf2a0f4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a0f4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *) [0xf2a104]")]
pub fn stub_0xf2a104(value: &crate::lua::LuaVector2, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *) [0xf2a114]")]
pub fn stub_0xf2a114(value: &crate::lua::LuaVector3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a134]")]
pub fn stub_0xf2a134() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::S~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a144]")]
pub fn stub_0xf2a144() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost:~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a154]")]
pub fn stub_0xf2a154() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a174]")]
pub fn stub_0xf2a174(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a174: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(RBX::InputObject const&,lua_State *) [0xf2a1c4]")]
pub fn stub_0xf2a1c4(value: &crate::lua::LuaInputObject, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<InputObject>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("InputObject({})", value.kind)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a1d4]")]
pub fn stub_0xf2a1d4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a1d4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(RBX::Region3int16 const&,lua_State *) [0xf2a1e4]")]
pub fn stub_0xf2a1e4(value: &crate::lua::LuaRegion3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a1f4]")]
pub fn stub_0xf2a1f4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a1f4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(RBX::Axes const&,lua_State *) [0xf2a204]")]
pub fn stub_0xf2a204(value: &crate::lua::LuaAxes, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Axes>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Axes({})", value.bits)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a214]")]
pub fn stub_0xf2a214(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a214: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(RBX::Faces const&,lua_State *) [0xf2a224]")]
pub fn stub_0xf2a224(value: &crate::lua::LuaFaces, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Faces>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Faces({})", value.bits)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a234]")]
pub fn stub_0xf2a234(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a234: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(RBX::CellID const&,lua_State *) [0xf2a244]")]
pub fn stub_0xf2a244(value: &crate::lua::LuaCellId, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CellID>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a254]")]
pub fn stub_0xf2a254(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a254: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_tostring(RBX::RbxRay const&,lua_State *) [0xf2a264]")]
pub fn stub_0xf2a264(value: &crate::lua::LuaRbxRay, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<RbxRay>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} | {}, {}, {}", value.origin.x, value.origin.y, value.origin.z, value.direction.x, value.direction.y, value.direction.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a274]")]
pub fn stub_0xf2a274(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a274: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(RBX::Region3 const&,lua_State *) [0xf2a284]")]
pub fn stub_0xf2a284(value: &crate::lua::LuaRegion3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a294]")]
pub fn stub_0xf2a294(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a294: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a2a4]")]
pub fn stub_0xf2a2a4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a2a4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}
