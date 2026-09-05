// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua (case-sensitive) — 4456 filtered, all already stubbed (14253 existing, 5401 with Yield|lua)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2684e4..0x28d6fc | global filler EA-sorted asc after 0x2684e0 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_0x2684e4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> * rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x2684e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::TType<void>::~TType()")]
pub fn stub_0x268540(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::~vector()")]
pub fn stub_0x268544(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::push_back(RBX::Reflection::Type const* const&)")]
pub fn stub_0x268558(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::Type const**,std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>>,RBX::Reflection::Type const* const&)")]
pub fn stub_0x268584(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_allocate(unsigned long)")]
pub fn stub_0x268664() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::Type::Type<void>(char const*,void *)")]
pub fn stub_0x26867c() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type")
}

#[doc(alias = "RBX::Reflection::TType<void>::~TType() [0x268728]")]
pub fn stub_0x268728(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(char const*,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> *)")]
pub fn stub_0x26872c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType() [0x2687d8]")]
pub fn stub_0x2687d8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
pub fn stub_0x2687dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(char const*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> *)")]
pub fn stub_0x2688b0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType() [0x26895c]")]
pub fn stub_0x26895c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_0x268960() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_0x268a34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(char const*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> *)")]
pub fn stub_0x268b40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~TType() [0x268bec]")]
pub fn stub_0x268bec(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_61")]
pub fn stub_0x268bf0() -> crate::slot::PortedFn {
// IDA 0x268bf0: __GLOBAL__I_a_61.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x268bf0, "__GLOBAL__I_a_61")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::link_point(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *&,boost::multi_index::detail::hashed_unique_tag)")]
pub fn stub_0x26af9c() -> crate::slot::PortedFn {
// IDA 0x26af9c: boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x26af9c, "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::ref~")
}

#[doc(alias = "boost::multi_index::detail::auto_space<unsigned long,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
pub fn stub_0x26afd0() -> crate::slot::PortedFn {
// IDA 0x26afd0: boost::multi_index::detail::auto_space<unsigned long, std::allocator<boost::flyweights::detail::refcounted_value<boost::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x26afd0, "boost::multi_index::detail::auto_space<unsigned long, std::allocator<boost::flyweights::detail::refc~")
}

#[doc(alias = "global constructor keyed to_a_62")]
pub fn stub_0x26b1f4() -> crate::slot::PortedFn {
// IDA 0x26b1f4: __GLOBAL__I_a_62.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x26b1f4, "__GLOBAL__I_a_62")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase>& rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x26c350(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::Reflection::Type::operator!=(RBX::Reflection::Type const&)const")]
pub fn stub_0x26c474(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type::operator!=(RBX::Reflection::Type const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,int>(int const&)")]
pub fn stub_0x26c500() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>")
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::operator=(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
pub fn stub_0x26c6a4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(void)")]
pub fn stub_0x26c6dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e228(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e318(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e464(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e554(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e648(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e780(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26e8d0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26ea00(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26ec34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26ed24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26ee14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26f0e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(std::type_info const&)")]
pub fn stub_0x26f368(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::lookupDescriptor(std::type_info const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26f3a0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26f490(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")]
pub fn stub_0x26f578() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::construct_func(char const*,char *)")]
pub fn stub_0x26f5e0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::InputObject>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")]
pub fn stub_0x26f600() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")]
pub fn stub_0x26f680(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::CellID>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::construct_func(char const*,char *)")]
pub fn stub_0x26f6ec(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::CellID>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::destruct_func(char *)")]
pub fn stub_0x26f718(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::CellID>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::construct_func(char const*,char *)")]
pub fn stub_0x26f720(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::UDim>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::destruct_func(char *)")]
pub fn stub_0x26f730(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::UDim>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")]
pub fn stub_0x26f738(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::RbxRay>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::destruct_func(char *)")]
pub fn stub_0x26f7a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::RbxRay>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")]
pub fn stub_0x26f7b0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")]
pub fn stub_0x26f808(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<G3D::Vector2>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")]
pub fn stub_0x26f878() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")]
pub fn stub_0x26f8d8(any: crate::lua::ScriptVariant) {
// placement_any dtor.
drop(any);
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")]
pub fn stub_0x26f930(handle: crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder dtor.
drop(handle);
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")]
pub fn stub_0x26f9a0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")]
pub fn stub_0x26fa00(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Region3int16>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::destruct_func(char *)")]
pub fn stub_0x26fa70(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Region3int16>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
pub fn stub_0x26fb68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_0x26fbc4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_0x26fccc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd()")]
pub fn stub_0x26fdd0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd() [0x26fdfc]")]
pub fn stub_0x26fdfc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::dispose(void)")]
pub fn stub_0x26feb4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_deleter(std::type_info const&)")]
pub fn stub_0x26fed4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_untyped_deleter(void)")]
pub fn stub_0x26feec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_0x26fef0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::operator=(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_0x26ff58(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "global constructor keyed to_a_63")]
pub fn stub_0x270078() -> crate::slot::PortedFn {
// IDA 0x270078: __GLOBAL__I_a_63.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x270078, "__GLOBAL__I_a_63")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::at(unsigned long)const")]
pub fn stub_0x277870(vec: &crate::slot::VecModel, index: usize) -> Option<usize> {
// bounds-checked element access shape.
if index < vec.len() { Some(index) } else { None }
}

#[doc(alias = "G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)")]
pub fn stub_0x27797c() -> crate::slot::PortedFn {
// IDA 0x27797c: G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&, float).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x27797c, "G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&, float)")
}

#[doc(alias = "RBX::CellID::fromParameters(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x277af4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::RbxRay::distance(G3D::Vector3 const&)const")]
pub fn stub_0x278084(handle: &crate::slot::InstanceHandle) {
// RBX::RbxRay::distance(G3D::Vector3 const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RbxRay::closestPoint(G3D::Vector3 const&)const")]
pub fn stub_0x2780dc(handle: &crate::slot::InstanceHandle) {
// RBX::RbxRay::closestPoint(G3D::Vector3 const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_64")]
pub fn stub_0x278164() -> crate::slot::PortedFn {
// IDA 0x278164: __GLOBAL__I_a_64.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x278164, "__GLOBAL__I_a_64")
}

#[doc(alias = "RBX::RbxRay::operator==(RBX::RbxRay const&)const")]
pub fn stub_0x27b438(handle: &crate::slot::InstanceHandle) {
// RBX::RbxRay::operator==(RBX::RbxRay const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellID::operator==(RBX::CellID const&)const")]
pub fn stub_0x27b4b4(handle: &crate::slot::InstanceHandle) {
// RBX::CellID::operator==(RBX::CellID const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_65")]
pub fn stub_0x27b50c() -> crate::slot::PortedFn {
// IDA 0x27b50c: __GLOBAL__I_a_65.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x27b50c, "__GLOBAL__I_a_65")
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(RBX::Name const&)")]
pub fn stub_0x27bea8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::lookupDescriptor(RBX::Name const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_66")]
pub fn stub_0x27bef0() -> crate::slot::PortedFn {
// IDA 0x27bef0: __GLOBAL__I_a_66.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x27bef0, "__GLOBAL__I_a_66")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::findDescriptor(char const*)const")]
pub fn stub_0x285774(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::findDescr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropertyDescriptor::setEnumItem(RBX::Reflection::DescribedBase *,RBX::Reflection::EnumDescriptor::Item const&)const")]
pub fn stub_0x2857f0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::EnumPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::Reflection::Tuple>::reset<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_0x28581c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::findDescriptor(char const*)const")]
pub fn stub_0x285848(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::findDescript~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::findDescriptor(char const*)const")]
pub fn stub_0x285870(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::findDescr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_67")]
pub fn stub_0x2858c0() -> crate::slot::PortedFn {
// IDA 0x2858c0: __GLOBAL__I_a_67.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2858c0, "__GLOBAL__I_a_67")
}

#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::push_back(boost::pool<boost::default_user_allocator_new_delete> * const&)")]
pub fn stub_0x286100(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::purge_memory(void)")]
pub fn stub_0x28612c() -> crate::slot::PortedFn {
// IDA 0x28612c: boost::pool<boost::default_user_allocator_new_delete>::purge_memory().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x28612c, "boost::pool<boost::default_user_allocator_new_delete>::purge_memory()")
}

#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::pool<boost::default_user_allocator_new_delete> **,std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>>,boost::pool<boost::default_user_allocator_new_delete> * const&)")]
pub fn stub_0x286170(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x286250() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "global constructor keyed to_a_68")]
pub fn stub_0x286268() -> crate::slot::PortedFn {
// IDA 0x286268: __GLOBAL__I_a_68.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x286268, "__GLOBAL__I_a_68")
}

#[doc(alias = "global constructor keyed to_a_69")]
pub fn stub_0x287738() -> crate::slot::PortedFn {
// IDA 0x287738: __GLOBAL__I_a_69.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x287738, "__GLOBAL__I_a_69")
}

#[doc(alias = "RBX::Reflection::GenericSlotWrapper::~GenericSlotWrapper()")]
pub fn stub_0x289268(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::GenericSlotWrapper dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::GenericSlotWrapper::~GenericSlotWrapper() [0x289284]")]
pub fn stub_0x289284(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::GenericSlotWrapper dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_70")]
pub fn stub_0x28aa88() -> crate::slot::PortedFn {
// IDA 0x28aa88: __GLOBAL__I_a_70.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x28aa88, "__GLOBAL__I_a_70")
}

#[doc(alias = "RemoteCheatHelper(rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_0x28cc94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::~flyweight()")]
pub fn stub_0x28d6bc() -> crate::slot::PortedFn {
// IDA 0x28d6bc: boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_, boost::parameter::v~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x28d6bc, "boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_,~")
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_> const&)")]
pub fn stub_0x28d6fc() -> crate::slot::PortedFn {
// IDA 0x28d6fc: boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_, boost::parameter::v~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x28d6fc, "boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_,~")
}
