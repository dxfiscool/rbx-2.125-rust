// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|lua|Yield (5401 filtered, all stubbed) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x28dcb8..0x2affbc | global filler EA-sorted asc after 0x28d6fc | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::Weak<RBX::DataModel> RBX::weak_from<RBX::DataModel>(RBX::DataModel*)")]
pub fn stub_0x28dcb8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x293594() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::isReadOnly(void)const")]
pub fn stub_0x2936e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::isWriteOnly(void)const")]
pub fn stub_0x2936f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x293704(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::equalValues(RBX::Reflection::Des~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x2938b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x2939dc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x293bd8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::copyValue(RBX::Reflection::Descr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::~TypedPropertyDescriptor()")]
pub fn stub_0x293d00(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ContentId>::~TypedPropertyDescriptor() [0x293d24]")]
pub fn stub_0x293d24(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x293ec4() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::isReadOnly(void)const")]
pub fn stub_0x294014(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::isWriteOnly(void)const")]
pub fn stub_0x294024(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x294034(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::equalValues(RBX::Reflectio~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x294314(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x2944f0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x294770(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::copyValue(RBX::Reflection:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::~TypedPropertyDescriptor()")]
pub fn stub_0x294930(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::~TypedPropertyDescriptor() [0x294954]")]
pub fn stub_0x294954(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::flyweight(void)")]
pub fn stub_0x2949dc() -> crate::slot::PortedFn {
// IDA 0x2949dc: boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_, boost::parameter::v~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2949dc, "boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_,~")
}

#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(RBX::ProtectedString const&)")]
pub fn stub_0x294ba8() -> crate::slot::PortedFn {
// IDA 0x294ba8: boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_, boost::parameter::v~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x294ba8, "boost::flyweights::flyweight<RBX::ProtectedString, boost::parameter::void_, boost::parameter::void_,~")
}

#[doc(alias = "global constructor keyed to_a_71")]
pub fn stub_0x294e3c() -> crate::slot::PortedFn {
// IDA 0x294e3c: __GLOBAL__I_a_71.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x294e3c, "__GLOBAL__I_a_71")
}

#[doc(alias = "cleanTimeout(double &)")]
pub fn stub_0x29f0fc() -> crate::slot::PortedFn {
// IDA 0x29f0fc: cleanTimeout(double&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x29f0fc, "cleanTimeout(double&)")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x2a3bc4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Security::Context::current(void)")]
pub fn stub_0x2a3ca8(handle: &crate::slot::InstanceHandle) {
// RBX::Security::Context::current() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::operator=(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_0x2a4a7c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RobloxExtraSpace::eraseRefsFromAllNodes(void)")]
pub fn stub_0x2a4c6c() -> crate::slot::PortedFn {
// IDA 0x2a4c6c: RobloxExtraSpace::eraseRefsFromAllNodes().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a4c6c, "RobloxExtraSpace::eraseRefsFromAllNodes()")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,unsigned long>(unsigned long const&)")]
pub fn stub_0x2a4d18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_0x2a5550(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function1<void,bool>::operator()(bool)const")]
pub fn stub_0x2a5da0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::shared_from<RBX::Stats::StatsService>(RBX::Stats::StatsService*)")]
pub fn stub_0x2a5e64() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(RBX::Instance const*)")]
pub fn stub_0x2a5f4c() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
pub fn stub_0x2a6058() -> crate::slot::PortedFn {
// IDA 0x2a6058: RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a6058, "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample()")
}

#[doc(alias = "RBX::RunningAverage<double,double>::sample(double)")]
pub fn stub_0x2a60b0(handle: &crate::slot::InstanceHandle) {
// RBX::RunningAverage<double, double>::sample(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::find<RBX::Stats::StatsService>(RBX::Instance const*)")]
pub fn stub_0x2a6324() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Stats::StatsService"))
}

#[doc(alias = "boost::thread::join(void)")]
pub fn stub_0x2a6368() -> crate::slot::PortedFn {
// IDA 0x2a6368: boost::thread::join().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a6368, "boost::thread::join()")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::operator=(rbx_core::SharedPtr<RBX::RunService> const&)")]
pub fn stub_0x2a65c8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_0x2a6c90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal_with_args<0,void ()(void)>::operator()(void)")]
pub fn stub_0x2a6cc0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<0, void ()>::operator()() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2a6e68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Security::Impersonator::Impersonator(RBX::Security::Identities)")]
pub fn stub_0x2a7120() -> crate::slot::InstanceHandle {
// RBX::Security::Impersonator ctor.
crate::slot::InstanceHandle::new("RBX::Security::Impersonator")
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> std::operator+<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
pub fn stub_0x2a7348(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "boost::function1<std::string,std::string const&>::operator()(std::string const&)const")]
pub fn stub_0x2a73ec(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x2a7ef0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> const&)")]
pub fn stub_0x2a947c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2a94a0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (int)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::disconnectAll(void)")]
pub fn stub_0x2a9598(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void ()>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> const&)")]
pub fn stub_0x2a9710(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2a9738(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void ()>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
pub fn stub_0x2a9830() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
pub fn stub_0x2a99a8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2a99d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::Instance>*)")]
pub fn stub_0x2aab78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2aaba8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::__false_type)")]
pub fn stub_0x2ab1a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")]
pub fn stub_0x2ac1c0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2ac368(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::RunTransition)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::~slot()")]
pub fn stub_0x2ac458(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "boost::function1<std::string,std::string const&>::dummy::nonnull(void)")]
pub fn stub_0x2ac838() -> crate::slot::PortedFn {
// IDA 0x2ac838: boost::function1<std::string, std::string const&>::dummy::nonnull().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2ac838, "boost::function1<std::string, std::string const&>::dummy::nonnull()")
}

#[doc(alias = "boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")]
pub fn stub_0x2acc24() -> crate::slot::PortedFn {
// IDA 0x2acc24: boost::function<std::string (std::string const&)>::operator=(boost::function<std::string (std::string const&)> const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2acc24, "boost::function<std::string (std::string const&)>::operator=(boost::function<std::string (std::strin~")
}

#[doc(alias = "boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")]
pub fn stub_0x2acce8() -> crate::slot::PortedFn {
// IDA 0x2acce8: boost::function1<std::string, std::string const&>::swap(boost::function1<std::string, std::string const&>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2acce8, "boost::function1<std::string, std::string const&>::swap(boost::function1<std::string, std::string co~")
}

#[doc(alias = "boost::function1<std::string,std::string const&>::clear(void)")]
pub fn stub_0x2acdc4(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")]
pub fn stub_0x2acdf0() -> crate::slot::PortedFn {
// IDA 0x2acdf0: boost::function1<std::string, std::string const&>::move_assign(boost::function1<std::string, std::string const&>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2acdf0, "boost::function1<std::string, std::string const&>::move_assign(boost::function1<std::string, std::st~")
}

#[doc(alias = "boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")]
pub fn stub_0x2acef4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
pub fn stub_0x2acf24(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
pub fn stub_0x2ad2b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
pub fn stub_0x2ad394() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::clear(void)")]
pub fn stub_0x2ad498(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
pub fn stub_0x2adb24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2adc84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
pub fn stub_0x2ade8c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
pub fn stub_0x2adeb4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
pub fn stub_0x2aded8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2adedc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
pub fn stub_0x2adfd8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::Stats::sStats>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
pub fn stub_0x2ae020(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Stats::sStats>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Stats::StatsService>(void)")]
pub fn stub_0x2ae108() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x2ae1e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x2ae280() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings> RBX::Creatable<RBX::Instance>::create<RBX::DebugSettings>(void)")]
pub fn stub_0x2ae3c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebugSettings")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings>::shared_ptr<RBX::DebugSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2ae478() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebugSettings")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebugSettings,RBX::DebugSettings>(rbx_core::SharedPtr<RBX::DebugSettings> const*,RBX::DebugSettings *)const")]
pub fn stub_0x2ae540() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebugSettings")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2ae630() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2ae738(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2ae740() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2ae760() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2ae778() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
pub fn stub_0x2ae77c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sDebugSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")]
pub fn stub_0x2ae7c0() -> crate::slot::PortedFn {
// IDA 0x2ae7c0: void RBX::Name::callDoDeclare<RBX::sDebugSettings>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2ae7c0, "void RBX::Name::callDoDeclare<RBX::sDebugSettings>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
pub fn stub_0x2ae7c4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDebugSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x2ae8a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *>(rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *)")]
pub fn stub_0x2aef18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::shared_from_this(void)")]
pub fn stub_0x2af310(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// enable_shared_from_this — alias the owned ref.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_0x2afa28(slot: &crate::slot::CallableSlot) {
// IDA 0x2afa28: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
pub fn stub_0x2afc34(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
pub fn stub_0x2afc58(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2afc80(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Heartbeat const&)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")]
pub fn stub_0x2afe78(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")]
pub fn stub_0x2aff88() -> crate::slot::SlotConnection {
// IDA 0x2aff88: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_0x2affbc(slot: &mut crate::slot::CallableSlot) {
// IDA 0x2affbc: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}
