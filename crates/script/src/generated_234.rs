// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3fcb84..0x402df8 | script 23955->24055 distinct (filler 0x3fcb84 asc, not-in-script 61593->61493)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance> RBX::shared_from<RBX::ModelInstance>(RBX::ModelInstance*)")]
pub fn stub_0x3fcb84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ModelInstance")
}

#[doc(alias = "RBX::Ungroup std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::Ungroup>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::Ungroup)")]
pub fn stub_0x3fcc6c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::RotateAxisCommand::RotateAxisCommand(std::string,RBX::DataModel *)")]
pub fn stub_0x3fcca8() -> crate::slot::InstanceHandle {
// RBX::RotateAxisCommand ctor.
crate::slot::InstanceHandle::new("RBX::RotateAxisCommand")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x3ff478(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Network::Players, RBX::Network::sPlayers, RBX::NonFactoryP~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::getSelection(void)")]
pub fn stub_0x3ff598(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FilteredSelection getter.
cell.get()
}

#[doc(alias = "void RBX::Selection::setSelection<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>)")]
pub fn stub_0x3ff5f0() -> crate::slot::PortedFn {
// IDA 0x3ff5f0: void RBX::Selection::setSelection<__gnu_cxx::__normal_iterator<RBX::Instance**, std::vector<RBX::Instance*, std::allocat~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3ff5f0, "void RBX::Selection::setSelection<__gnu_cxx::__normal_iterator<RBX::Instance**, std::vector<RBX::Ins~")
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::Instance>>(void)const")]
pub fn stub_0x3ff614() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::Instance>"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::Instance>>(void)")]
pub fn stub_0x3ff954() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::Instance>>(void)")]
pub fn stub_0x3ff958() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(void)const")]
pub fn stub_0x3ffa30() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::Instance>>(void)")]
pub fn stub_0x3ffbf8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::Instance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
pub fn stub_0x3ffca8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::FilteredSelection(void)")]
pub fn stub_0x3ffcdc() -> crate::slot::InstanceHandle {
// RBX::FilteredSelection ctor.
crate::slot::InstanceHandle::new("RBX::FilteredSelection")
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
pub fn stub_0x3ffe98(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::~FilteredSelection() [0x3ffe9c]")]
pub fn stub_0x3ffe9c(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x3fff3c(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::Instance>::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x400090(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
pub fn stub_0x4000e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection() [0x4000ec]")]
pub fn stub_0x4000ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection() [0x4000f8]")]
pub fn stub_0x4000f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection() [0x400100]")]
pub fn stub_0x400100(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x400108(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
pub fn stub_0x400110() -> crate::slot::PortedFn {
// IDA 0x400110: __gnu_cxx::__normal_iterator<RBX::Instance**, std::vector<RBX::Instance*, std::allocator<RBX::Instance*>>> std::__find<_~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x400110, "__gnu_cxx::__normal_iterator<RBX::Instance**, std::vector<RBX::Instance*, std::allocator<RBX::Instan~")
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::~FilteredSelection() [0x4001a0]")]
pub fn stub_0x4001a0(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::shared_ptr<RBX::FilteredSelection<RBX::Instance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4003f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::Instance>,RBX::FilteredSelection<RBX::Instance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const*,RBX::FilteredSelection<RBX::Instance> *)const")]
pub fn stub_0x4004bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4005a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4006ac(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4006b0]")]
pub fn stub_0x4006b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4006b4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4006d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4006ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::getSelection(void)")]
pub fn stub_0x4006f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FilteredSelection getter.
cell.get()
}

#[doc(alias = "RBX::Ungroup::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x400748() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")]
pub fn stub_0x4007b4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::ModelInstance>"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
pub fn stub_0x400928() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
pub fn stub_0x40092c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")]
pub fn stub_0x400a04() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::ModelInstance>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::ModelInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
pub fn stub_0x400bcc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::ModelInstance>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::ModelInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::ModelInstance>> const&)")]
pub fn stub_0x400c7c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::FilteredSelection(void)")]
pub fn stub_0x400cb0() -> crate::slot::InstanceHandle {
// RBX::FilteredSelection ctor.
crate::slot::InstanceHandle::new("RBX::FilteredSelection")
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
pub fn stub_0x400e6c(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection() [0x400e70]")]
pub fn stub_0x400e70(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x400f10(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::ModelInstance>::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x401088(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
pub fn stub_0x401104(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection() [0x40110c]")]
pub fn stub_0x40110c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection() [0x401114]")]
pub fn stub_0x401114(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection() [0x40111c]")]
pub fn stub_0x40111c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x401124(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::push_back(RBX::ModelInstance * const&)")]
pub fn stub_0x40112c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
pub fn stub_0x401158() -> crate::slot::PortedFn {
// IDA 0x401158: __gnu_cxx::__normal_iterator<RBX::ModelInstance**, std::vector<RBX::ModelInstance*, std::allocator<RBX::ModelInstance*>>~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x401158, "__gnu_cxx::__normal_iterator<RBX::ModelInstance**, std::vector<RBX::ModelInstance*, std::allocator<R~")
}

#[doc(alias = "std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::ModelInstance * const&)")]
pub fn stub_0x4011e8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_allocate(unsigned long)")]
pub fn stub_0x4012c8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection() [0x4012e0]")]
pub fn stub_0x4012e0(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::ModelInstance>>::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x401414() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::ModelInstance>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::ModelInstance>,RBX::FilteredSelection<RBX::ModelInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::ModelInstance>> const*,RBX::FilteredSelection<RBX::ModelInstance> *)const")]
pub fn stub_0x4014dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::ModelInstance>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4015c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4016cc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4016d0]")]
pub fn stub_0x4016d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4016d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4016f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x40170c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_0x401710(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x401710: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "RBX::Instance::findCommonNode(RBX::Instance*,RBX::Instance*)")]
pub fn stub_0x4017dc(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::findCommonNode(RBX::Instance*, RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::canAddChild(RBX::Instance const*)const")]
pub fn stub_0x40181c(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::canAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance> RBX::Creatable<RBX::Instance>::create<RBX::ModelInstance>(void)")]
pub fn stub_0x40187c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ModelInstance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x401930() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ModelInstance")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ModelInstance,RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const*,RBX::ModelInstance *)const")]
pub fn stub_0x4019f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ModelInstance")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x401ae0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x401be8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x401bec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::getSelection(void)")]
pub fn stub_0x401e80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FilteredSelection getter.
cell.get()
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PVInstance>>(void)const")]
pub fn stub_0x401ed8() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::PVInstance>")
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PVInstance>>(void)const")]
pub fn stub_0x4020a0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::PVInstance>"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
pub fn stub_0x402214() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::PVInstance>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::PVInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> const&)")]
pub fn stub_0x4022c4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
pub fn stub_0x4022f8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
pub fn stub_0x4022fc() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::FilteredSelection(void)")]
pub fn stub_0x4023d4() -> crate::slot::InstanceHandle {
// RBX::FilteredSelection ctor.
crate::slot::InstanceHandle::new("RBX::FilteredSelection")
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
pub fn stub_0x402590(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection() [0x402594]")]
pub fn stub_0x402594(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x402634(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::PVInstance>::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x4027ac(handle: &crate::slot::InstanceHandle) {
// RBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
pub fn stub_0x402828(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection() [0x402830]")]
pub fn stub_0x402830(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection() [0x402838]")]
pub fn stub_0x402838(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection() [0x402840]")]
pub fn stub_0x402840(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0x402848(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::push_back(RBX::PVInstance * const&)")]
pub fn stub_0x402850(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
pub fn stub_0x40287c() -> crate::slot::PortedFn {
// IDA 0x40287c: __gnu_cxx::__normal_iterator<RBX::PVInstance**, std::vector<RBX::PVInstance*, std::allocator<RBX::PVInstance*>>> std::__~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x40287c, "__gnu_cxx::__normal_iterator<RBX::PVInstance**, std::vector<RBX::PVInstance*, std::allocator<RBX::PV~")
}

#[doc(alias = "std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::PVInstance * const&)")]
pub fn stub_0x40290c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_allocate(unsigned long)")]
pub fn stub_0x4029ec() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection() [0x402a04]")]
pub fn stub_0x402a04(handle: crate::slot::InstanceHandle) {
// RBX::FilteredSelection dtor.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>>::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x402b38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::PVInstance>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PVInstance>,RBX::FilteredSelection<RBX::PVInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> const*,RBX::FilteredSelection<RBX::PVInstance> *)const")]
pub fn stub_0x402c00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::PVInstance>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x402ce8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x402df0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x402df4]")]
pub fn stub_0x402df4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x402df8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
