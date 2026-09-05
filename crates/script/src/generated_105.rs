// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf2dd04..0xf2ff84 | 5211->5311 covered (filtered exhausted, global filler EA-sorted asc after 0xf2dcf4), rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&) [0xf2dd04]")]
pub fn stub_0xf2dd04(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&) [0xf2dd14]")]
pub fn stub_0xf2dd14() -> crate::slot::InstanceHandle {
// std::vector ctor.
crate::slot::InstanceHandle::new("std::vector")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector() [0xf2dd24]")]
pub fn stub_0xf2dd24(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&) [0xf2dd34]")]
pub fn stub_0xf2dd34(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::Callb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
pub fn stub_0xf2de34(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::le~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
pub fn stub_0xf2de44(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::generatePaletteMap(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
pub fn stub_0xf2de64(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
pub fn stub_0xf2de74(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
pub fn stub_0xf2de84() -> crate::slot::InstanceHandle {
// RBX::BrickColor::BrickMap ctor.
crate::slot::InstanceHandle::new("RBX::BrickColor::BrickMap")
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
pub fn stub_0xf2de94(handle: crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap dtor.
drop(handle);
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
pub fn stub_0xf2dea4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
pub fn stub_0xf2deb4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
pub fn stub_0xf2dec4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
pub fn stub_0xf2ded4(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_itera~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
pub fn stub_0xf2dee4(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
pub fn stub_0xf2def4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
pub fn stub_0xf2df04(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
pub fn stub_0xf2df14(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
pub fn stub_0xf2df24(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
pub fn stub_0xf2df34(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
pub fn stub_0xf2df44(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
pub fn stub_0xf2df54(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
pub fn stub_0xf2df64(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
pub fn stub_0xf2df74() -> crate::slot::PortedFn {
// IDA 0xf2df74: std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::Bri~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2df74, "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std~")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
pub fn stub_0xf2df84(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
pub fn stub_0xf2df94(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
pub fn stub_0xf2dfa4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
pub fn stub_0xf2dfb4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
pub fn stub_0xf2dfc4() -> crate::slot::PortedFn {
// IDA 0xf2dfc4: void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::Col~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2dfc4, "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::Brick~")
}

#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
pub fn stub_0xf2dfd4() -> crate::slot::PortedFn {
// IDA 0xf2dfd4: void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::C~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2dfd4, "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::Bri~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*)")]
pub fn stub_0xf2e044() -> crate::slot::PortedFn {
// IDA 0xf2e044: rbx_core::WeakPtr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e044, "rbx_core::WeakPtr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter>(rbx_core::WeakPtr<RBX::ContentFilter> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0xf2e054() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
pub fn stub_0xf2e064() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
pub fn stub_0xf2e084() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
pub fn stub_0xf2e0a4() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_0xf2e0b4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2e0b4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
pub fn stub_0xf2e0c4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
pub fn stub_0xf2e0d4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
pub fn stub_0xf2e0e4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
pub fn stub_0xf2e0f4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool>(void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool)")]
pub fn stub_0xf2e104() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
pub fn stub_0xf2e114() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2e124(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2e134(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2e144(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS7_5list2INS7_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e154() -> crate::slot::PortedFn {
// IDA 0xf2e154: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS7_5list2INS7_5v~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e154, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEE~")
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS7_5list3INS7_5valueISB_EENSF_ISsEENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e164() -> crate::slot::PortedFn {
// IDA 0xf2e164: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS7_5list3INS7_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e164, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEE~")
}

#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS7_5list4INS_3argILi1EEENSG_ILi2EEENS7_5valueISC_EENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e174() -> crate::slot::PortedFn {
// IDA 0xf2e174: j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS7_5list4I~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e174, "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFil~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ContentFilter>::weak_ptr<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&,boost::detail::sp_enable_if_convertible<RBX::ContentFilter,RBX::ContentFilter>::type)")]
pub fn stub_0xf2e184() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e1b4() -> crate::slot::PortedFn {
// IDA 0xf2e1b4: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5va~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e1b4, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEES~")
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e1c4() -> crate::slot::PortedFn {
// IDA 0xf2e1c4: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5v~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e1c4, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEES~")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")]
pub fn stub_0xf2e1d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e1e4() -> crate::slot::PortedFn {
// IDA 0xf2e1e4: j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4IN~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e1e4, "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilt~")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf2e254(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf2e264(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf2e274(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
pub fn stub_0xf2e284(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
pub fn stub_0xf2e294() -> crate::slot::PortedFn {
// IDA 0xf2e294: std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::l~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e294, "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::alloca~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
pub fn stub_0xf2e2a4() -> crate::slot::PortedFn {
// IDA 0xf2e2a4: std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::u~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e2a4, "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::alloca~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
pub fn stub_0xf2e2b4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
pub fn stub_0xf2e2c4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
pub fn stub_0xf2e2d4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
pub fn stub_0xf2e2e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
pub fn stub_0xf2e2f4() -> crate::slot::PortedFn {
// IDA 0xf2e2f4: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::st~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e2f4, "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
pub fn stub_0xf2e304() -> crate::slot::PortedFn {
// IDA 0xf2e304: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::st~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e304, "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
pub fn stub_0xf2e314(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
pub fn stub_0xf2e324(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
pub fn stub_0xf2e334(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
pub fn stub_0xf2e344(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
pub fn stub_0xf2e354(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
pub fn stub_0xf2e364(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *) [0xf2e374]")]
pub fn stub_0xf2e374(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void) [0xf2e384]")]
pub fn stub_0xf2e384(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::filesystem::directory_iterator::directory_iterator(boost::filesystem::path const&) [0xf2e394]")]
pub fn stub_0xf2e394() -> crate::slot::PortedFn {
// IDA 0xf2e394: boost::filesystem::directory_iterator::directory_iterator(boost::filesystem::path const&) [0xf2e394].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e394, "boost::filesystem::directory_iterator::directory_iterator(boost::filesystem::path const&) [0xf2e394]")
}

#[doc(alias = "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<char *>::type>,boost::filesystem::path&>::type boost::filesystem::path::operator=<char *>(char * const&) [0xf2e3a4]")]
pub fn stub_0xf2e3a4() -> crate::slot::PortedFn {
// IDA 0xf2e3a4: boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<char *>::type>,boost::filesystem::path&>::type~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e3a4, "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<char *>::type>,boost::file~")
}

#[doc(alias = "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<std::string>::type>,boost::filesystem::path&>::type boost::filesystem::path::operator=<std::string>(std::string const&) [0xf2e3b4]")]
pub fn stub_0xf2e3b4() -> crate::slot::PortedFn {
// IDA 0xf2e3b4: boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<std::string>::type>,boost::filesystem::path&>:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e3b4, "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<std::string>::type>,boost:~")
}

#[doc(alias = "boost::filesystem::detail::dir_itr_imp::~dir_itr_imp() [0xf2e3c4]")]
pub fn stub_0xf2e3c4() -> crate::slot::PortedFn {
// IDA 0xf2e3c4: boost::filesystem::detail::dir_itr_imp::~dir_itr_imp() [0xf2e3c4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2e3c4, "boost::filesystem::detail::dir_itr_imp::~dir_itr_imp() [0xf2e3c4]")
}

#[doc(alias = "rbx_core::SharedPtr<boost::filesystem::detail::dir_itr_imp>::shared_ptr<boost::filesystem::detail::dir_itr_imp>(boost::filesystem::detail::dir_itr_imp *) [0xf2e3d4]")]
pub fn stub_0xf2e3d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::filesystem::detail::dir_itr_imp")
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e594() -> crate::slot::PortedFn {
// IDA 0xf2e594: j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e594, "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPS~")
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e5a4() -> crate::slot::PortedFn {
// IDA 0xf2e5a4: j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e5a4, "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9ex~")
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e5b4() -> crate::slot::PortedFn {
// IDA 0xf2e5b4: j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3IN~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e5b4, "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9excep~")
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e5f4() -> crate::slot::PortedFn {
// IDA 0xf2e5f4: j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptio~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e5f4, "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functi~")
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e604() -> crate::slot::PortedFn {
// IDA 0xf2e604: j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e604, "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPS~")
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2e614() -> crate::slot::PortedFn {
// IDA 0xf2e614: j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2e614, "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPS~")
}

#[doc(alias = "j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS4_5list1INS4_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ef04() -> crate::slot::PortedFn {
// IDA 0xf2ef04: j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7Varia~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ef04, "j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3R~")
}

#[doc(alias = "j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ef44() -> crate::slot::PortedFn {
// IDA 0xf2ef44: j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflectio~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ef44, "j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3ma~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf2f6d4() -> crate::slot::PortedFn {
// IDA 0xf2f6d4: j___ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServ~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f6d4, "j___ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8Inst~")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v")]
pub fn stub_0xf2f744() -> crate::slot::PortedFn {
// IDA 0xf2f744: j___ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f744, "j___ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v")]
pub fn stub_0xf2f754() -> crate::slot::PortedFn {
// IDA 0xf2f754: j___ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f754, "j___ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v")
}

#[doc(alias = "j___ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2fc04() -> crate::slot::PortedFn {
// IDA 0xf2fc04: j___ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5v~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2fc04, "j___ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERK~")
}

#[doc(alias = "j___ZN5boost8functionIFvddEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2fc14() -> crate::slot::PortedFn {
// IDA 0xf2fc14: j___ZN5boost8functionIFvddEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSC_EENS4_5list3INS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2fc14, "j___ZN5boost8functionIFvddEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperER~")
}

#[doc(alias = "j___ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2fc54() -> crate::slot::PortedFn {
// IDA 0xf2fc54: j___ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5va~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2fc54, "j___ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKd~")
}

#[doc(alias = "j___ZN5boost9function2IvddEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2fc94() -> crate::slot::PortedFn {
// IDA 0xf2fc94: j___ZN5boost9function2IvddEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2fc94, "j___ZN5boost9function2IvddEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERK~")
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7Creator12getClassNameEv")]
pub fn stub_0xf2fca4() -> crate::slot::PortedFn {
// IDA 0xf2fca4: j___ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7Creator12getClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2fca4, "j___ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7Creator12getClassNameEv")
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list_av_5<RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::type> boost::bind<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>(boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool) [0xf2ff34]")]
pub fn stub_0xf2ff34() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list_av_4<rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>(void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>) [0xf2ff44]")]
pub fn stub_0xf2ff44() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *) [0xf2ff54]")]
pub fn stub_0xf2ff54() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2ff64]")]
pub fn stub_0xf2ff64(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2ff74]")]
pub fn stub_0xf2ff74(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ff84() -> crate::slot::PortedFn {
// IDA 0xf2ff84: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ff84, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}
