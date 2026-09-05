// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x77ffec..0x78593c | remaining 1890 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x77ffec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x78000c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x78012c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc() [0x7802b0]")]
pub fn stub_0x7802b0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x780364() -> crate::slot::SlotConnection {
// IDA 0x780364: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x7804b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x780618() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc() [0x780744]")]
pub fn stub_0x780744(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x7807f8() -> crate::slot::SlotConnection {
// IDA 0x7807f8: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x7809fc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<0, RBX::Scripting::ScriptDebugger, void (), rbx::signal<voi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x780a70(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger, void (), rbx::signal<void (~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x780a84() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc() [0x780c08]")]
pub fn stub_0x780c08(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x780cbc() -> crate::slot::SlotConnection {
// IDA 0x780cbc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x780e10(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::Scripting::ScriptDebugger, void (int), rbx::signal<~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x780e9c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger, void (int), rbx::signal<voi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::PropDescriptor<int (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x781ac4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor() [0x781bd0]")]
pub fn stub_0x781bd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x781bfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x781c00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x781c04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x781c24(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::PropDescriptor<bool (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x781d44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor() [0x781e50]")]
pub fn stub_0x781e50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x781e7c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x781e80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x781e84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x781ea8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::PropDescriptor<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>(char const*,char const*,std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x781fc8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::~PropDescriptor() [0x7820dc]")]
pub fn stub_0x7820dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::isReadOnly(void)const")]
pub fn stub_0x782108(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::isWriteOnly(void)const")]
pub fn stub_0x78210c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x782110(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::GetSetImpl<std::string (RBX::Scripting::ScriptDebugger::*)(void)const,void (RBX::Scripting::ScriptDebugger::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x782138(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::RefPropDescriptor<RBX::Script* (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,RBX::Script* (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x78227c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::singleton(void)")]
pub fn stub_0x782320(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RefType<RBX::Script*>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::~RefPropDescriptor() [0x782418]")]
pub fn stub_0x782418(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::isReadOnly(void)const")]
pub fn stub_0x782448(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::isWriteOnly(void)const")]
pub fn stub_0x782458(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x782468(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x782490(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x7825a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x782670(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x782694(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x782768(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x78278c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x7827a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x78281c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x78283c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x78291c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x7829a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x7829a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x7829ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script *>::GetImpl<RBX::Script * (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Script * const&)const")]
pub fn stub_0x7829cc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::~RefType()")]
pub fn stub_0x782aec(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::RefType dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Script *>(char const*,char const*,RBX::Script * *)")]
pub fn stub_0x782af0() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Script *>::~RefType() [0x782b9c]")]
pub fn stub_0x782b9c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::RefType dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x782ba4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x782d74() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::~BoundFuncDesc() [0x782e70]")]
pub fn stub_0x782e70(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x782f98() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 2)
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Scripting::ScriptDebugger,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),std::string,RBX::Reflection::Variant,void>::call(RBX::Scripting::ScriptDebugger*,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant),RBX::Reflection::Variant&,std::string const&,RBX::Reflection::Variant const&)")]
pub fn stub_0x783148(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call2Helper<RBX::Scripting::ScriptDebugger, void (RBX::Scripting::ScriptD~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),char const*,char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x783510() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x783768() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::~BoundFuncDesc() [0x7837d0]")]
pub fn stub_0x7837d0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x783904() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 3)
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Scripting::ScriptDebugger,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),std::string,RBX::Reflection::Variant,int,void>::call(RBX::Scripting::ScriptDebugger*,void (RBX::Scripting::ScriptDebugger::*)(std::string,RBX::Reflection::Variant,int),RBX::Reflection::Variant&,std::string const&,RBX::Reflection::Variant const&,int const&)")]
pub fn stub_0x783ad4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call3Helper<RBX::Scripting::ScriptDebugger, void (RBX::Scripting::ScriptD~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x783c90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc() [0x783d94]")]
pub fn stub_0x783d94(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x783e48() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant&)")]
pub fn stub_0x783e6c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::BoundFuncDesc(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x783f58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x784104() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::~BoundFuncDesc() [0x784134]")]
pub fn stub_0x784134(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x784208() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),int,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::Scripting::ScriptDebugger::*)(int),RBX::Reflection::Variant&,int const&)")]
pub fn stub_0x784248() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x784338() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc() [0x78443c]")]
pub fn stub_0x78443c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x7844f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant&)")]
pub fn stub_0x784514() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x78467c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::~BoundFuncDesc() [0x784780]")]
pub fn stub_0x784780(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x784834() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::ScriptDebugger", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x784854() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x7849d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x784a04]")]
pub fn stub_0x784a04(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x784b0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::Variant>::call(RBX::Scripting::ScriptDebugger*,RBX::Reflection::Variant (RBX::Scripting::ScriptDebugger::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x784bf4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x784f5c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x7850d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc() [0x785108]")]
pub fn stub_0x785108(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x785210() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x785350() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x7854d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc() [0x7855d4]")]
pub fn stub_0x7855d4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x785688() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Scripting::ScriptDebugger*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Scripting::ScriptDebugger::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x7856ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Scripting::ScriptDebugger::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x785794() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x78590c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc() [0x78593c]")]
pub fn stub_0x78593c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}
