// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x76fbfc..0x7750d8 | remaining 2190 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint() [0x770804]")]
pub fn stub_0x770804() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint() [0x770818]")]
pub fn stub_0x770818() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::~EventDesc()")]
pub fn stub_0x771218(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
pub fn stub_0x77123c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x77127c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_0x7712a0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_0x771394(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x771488(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x7714ac(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::~BoundFuncDesc()")]
pub fn stub_0x7714d0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x771510(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::~BoundFuncDesc()")]
pub fn stub_0x771534(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::~BoundFuncDesc()")]
pub fn stub_0x771658(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::~RefPropDescriptor()")]
pub fn stub_0x771778(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::~PropDescriptor()")]
pub fn stub_0x7717a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor()")]
pub fn stub_0x7717e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor()")]
pub fn stub_0x771824(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
pub fn stub_0x771848(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
pub fn stub_0x77186c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
pub fn stub_0x771890(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")]
pub fn stub_0x7718b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x771908(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::ScriptDebugger>(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const&)")]
pub fn stub_0x771ab8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::shared_from<RBX::Scripting::ScriptDebugger>(RBX::Scripting::ScriptDebugger*)")]
pub fn stub_0x771aec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerManager>::~shared_ptr()")]
pub fn stub_0x771c5c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::operator=(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const&)")]
pub fn stub_0x771c70(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger,boost::reference_wrapper<RBX::Script>>(boost::reference_wrapper<RBX::Script>)")]
pub fn stub_0x771ca8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::DebuggerBreakpoint>(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const&)")]
pub fn stub_0x771d60(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::shared_from<RBX::Scripting::DebuggerBreakpoint>(RBX::Scripting::DebuggerBreakpoint*)")]
pub fn stub_0x771d94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch,std::string>(std::string)")]
pub fn stub_0x771f04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::DebuggerWatch>(rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> const&)")]
pub fn stub_0x77205c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::shared_from<RBX::Scripting::DebuggerWatch>(RBX::Scripting::DebuggerWatch*)")]
pub fn stub_0x772090() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint>::operator=(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const&)")]
pub fn stub_0x772ed4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint,int>(int)")]
pub fn stub_0x772f0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::shared_from<RBX::Script>(RBX::Script*)")]
pub fn stub_0x773018() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>> const&)")]
pub fn stub_0x7731fc() -> crate::slot::SlotConnection {
// IDA 0x7731fc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::verifyAddChild(RBX::Instance const*)const")]
pub fn stub_0x774460(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerWatch::verifyAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::askForbidChild(RBX::Instance const*)const")]
pub fn stub_0x774580(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerWatch::askForbidChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::verifyAddChild(RBX::Instance const*)const")]
pub fn stub_0x774988(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerBreakpoint::verifyAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::askForbidChild(RBX::Instance const*)const")]
pub fn stub_0x774aa8(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerBreakpoint::askForbidChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint>(void)")]
pub fn stub_0x774d40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint>::shared_ptr<RBX::Scripting::DebuggerBreakpoint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x774df0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerBreakpoint,RBX::Scripting::DebuggerBreakpoint>(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const*,RBX::Scripting::DebuggerBreakpoint *)const")]
pub fn stub_0x774ebc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x774fa4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x7750ac(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x7750b0]")]
pub fn stub_0x7750b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x7750b4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x7750d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
