// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x785a10..0x817ef4 | remaining 1790 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x785a10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),int,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),RBX::Reflection::Variant &,int const&)")]
pub fn stub_0x785a50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x785b3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::~EventDesc() [0x785cc0]")]
pub fn stub_0x785cc0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x785d74() -> crate::slot::SlotConnection {
// IDA 0x785d74: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x785ec8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x786028() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x78603c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc() [0x786140]")]
pub fn stub_0x786140(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x7861f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Scripting::DebuggerManager*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::DebuggerManager::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x786218() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::PropDescriptor<bool (RBX::Scripting::DebuggerManager::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::DebuggerManager::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x786300(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::~PropDescriptor() [0x78640c]")]
pub fn stub_0x78640c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x786438(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x78643c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x786440(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::GetImpl<bool (RBX::Scripting::DebuggerManager::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x786464(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x786584() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::DebuggerManager", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::~BoundFuncDesc() [0x786688]")]
pub fn stub_0x786688(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x78673c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::DebuggerManager", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x78675c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x7868d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x786908]")]
pub fn stub_0x786908(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x786a10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Scripting::DebuggerManager*,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::DebuggerManager::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x786af8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_buckets(void)")]
pub fn stub_0x786c28(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::clear(void)")]
pub fn stub_0x786e50(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::clear(void)")]
pub fn stub_0x786e7c(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
pub fn stub_0x786ea8(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::dummy::nonnull(void)")]
pub fn stub_0x786f70() -> crate::slot::PortedFn {
// IDA 0x786f70: boost::function2<void, lua_State*, lua_Debug*>::dummy::nonnull().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x786f70, "boost::function2<void, lua_State*, lua_Debug*>::dummy::nonnull()")
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::clear(void)")]
pub fn stub_0x786f74(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerManager> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerManager>(void)")]
pub fn stub_0x787034() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerManager")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerManager>::shared_ptr<RBX::Scripting::DebuggerManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x7870e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerManager")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerManager,RBX::Scripting::DebuggerManager>(rbx_core::SharedPtr<RBX::Scripting::DebuggerManager> const*,RBX::Scripting::DebuggerManager *)const")]
pub fn stub_0x7871ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerManager")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x787294() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x78739c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x7873a0]")]
pub fn stub_0x7873a0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x7873a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x7873c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x7873dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Humanoid,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::isScriptable(void)const")]
pub fn stub_0x7c486c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::TestService::countScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x801fe0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::TestService::startScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x802008() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::TestService::stopScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x802690() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::~BoundYieldFuncDesc()")]
pub fn stub_0x80372c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")]
pub fn stub_0x80b454() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")]
pub fn stub_0x80ce90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::TestService::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x810768() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::TestService", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::~BoundYieldFuncDesc() [0x81086c]")]
pub fn stub_0x81086c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x810920() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::TestService", "void", 0)
}
