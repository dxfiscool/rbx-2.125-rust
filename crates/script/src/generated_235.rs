// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x402e18..0x40ad04 | script 24055->24155 distinct (filler 0x402e18 asc, not-in-script 61493->61393)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x402e18() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x402e30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::PVInstance>>::createService(void)const")]
pub fn stub_0x402e34(handle: &crate::slot::InstanceHandle) {
// RBX::ServiceClient<RBX::FilteredSelection<RBX::PVInstance>>::createService() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> const&)")]
pub fn stub_0x402f14(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> RBX::shared_from<RBX::FilteredSelection<RBX::PVInstance>>(RBX::FilteredSelection<RBX::PVInstance>*)")]
pub fn stub_0x402f4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::PVInstance>")
}

#[doc(alias = "RBX::BoolPropertyVerbSetIt::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x403034() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>,std::random_access_iterator_tag)")]
pub fn stub_0x403278() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "bool boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>::operator()<bool,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<bool>,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,long)")]
pub fn stub_0x40336c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x40336c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")]
pub fn stub_0x403820() -> crate::slot::InstanceHandle {
// RBX::CommonVerbs ctor.
crate::slot::InstanceHandle::new("RBX::CommonVerbs")
}

#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *) [0x403824]")]
pub fn stub_0x403824() -> crate::slot::InstanceHandle {
// RBX::CommonVerbs ctor.
crate::slot::InstanceHandle::new("RBX::CommonVerbs")
}

#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x404394() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HammerTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HammerTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4047d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HammerTool")
}

#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x404c8c() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CloneTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::CloneTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4050c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CloneTool")
}

#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x405580() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GrabTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GrabTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4059bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GrabTool")
}

#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x405e74() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GameTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4062b0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GameTool")
}

#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x406768() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NullTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x406ba4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NullTool,RBX::NullTool>(rbx_core::SharedPtr<RBX::NullTool> const*,RBX::NullTool *)const")]
pub fn stub_0x406d20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x406e04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x406efc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x406f00]")]
pub fn stub_0x406f00(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x406f04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x406f14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x406f2c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x40705c() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::~TToolVerb() [0x4071e0]")]
pub fn stub_0x4071e0(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x407280(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x4072b8(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::DropperTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x4073cc(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::DropperTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DropperTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DropperTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x407498() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DropperTool")
}

#[doc(alias = "RBX::DropperTool::getCursorName(void)const")]
pub fn stub_0x407574(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DropperTool getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DropperTool>::shared_ptr<RBX::DropperTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x407590() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DropperTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DropperTool,RBX::DropperTool>(rbx_core::SharedPtr<RBX::DropperTool> const*,RBX::DropperTool *)const")]
pub fn stub_0x407658() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DropperTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40773c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x407834(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x407838]")]
pub fn stub_0x407838(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x40783c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x40784c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x407864() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x407994() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb() [0x407b18]")]
pub fn stub_0x407b18(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x407bb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x407bf0(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::MaterialTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x407d04(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::MaterialTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x407dd0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MaterialTool")
}

#[doc(alias = "RBX::MaterialTool::isSticky(void)const")]
pub fn stub_0x407eac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MaterialTool getter.
cell.get()
}

#[doc(alias = "RBX::MaterialTool::getCursorName(void)const")]
pub fn stub_0x407f74(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MaterialTool getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x407f90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MaterialTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MaterialTool,RBX::MaterialTool>(rbx_core::SharedPtr<RBX::MaterialTool> const*,RBX::MaterialTool *)const")]
pub fn stub_0x408058() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MaterialTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40813c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x408234(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x408238]")]
pub fn stub_0x408238(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x40823c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x40824c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x408264() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x408394() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FillTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FillTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4087d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FillTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FillTool>::shared_ptr<RBX::FillTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x408990() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FillTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FillTool,RBX::FillTool>(rbx_core::SharedPtr<RBX::FillTool> const*,RBX::FillTool *)const")]
pub fn stub_0x408a58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FillTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x408b3c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x408c34(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x408c38]")]
pub fn stub_0x408c38(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x408c3c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x408c4c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x408c64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x408d94() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LockTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4091d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LockTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)")]
pub fn stub_0x409374() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LockTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool>::shared_ptr<RBX::LockTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x4094dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LockTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LockTool,RBX::LockTool>(rbx_core::SharedPtr<RBX::LockTool> const*,RBX::LockTool *)const")]
pub fn stub_0x4095a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LockTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x409688() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x409780(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x409784]")]
pub fn stub_0x409784(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x409788() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x409798() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4097b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x4098e0() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x409d1c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnchorTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")]
pub fn stub_0x409ec8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnchorTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40a030() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnchorTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AnchorTool,RBX::AnchorTool>(rbx_core::SharedPtr<RBX::AnchorTool> const*,RBX::AnchorTool *)const")]
pub fn stub_0x40a0f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnchorTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40a1dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x40a2d4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x40a2d8]")]
pub fn stub_0x40a2d8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x40a2dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x40a2ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x40a304() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x40a434() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x40a870() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SmoothNoOutlinesTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40aa30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SmoothNoOutlinesTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::SmoothNoOutlinesTool,RBX::SmoothNoOutlinesTool>(rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> const*,RBX::SmoothNoOutlinesTool *)const")]
pub fn stub_0x40aaf8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SmoothNoOutlinesTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x40abdc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x40acd4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x40acd8]")]
pub fn stub_0x40acd8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x40acdc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x40acec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x40ad04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
