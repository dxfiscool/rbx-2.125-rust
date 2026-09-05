// Auto-generated skeletons for rbx-script — Lua/Script/CodeGen/Luau/Yield/lua batch
// Filter: Lua|Script|CodeGen|Luau|Yield|lua (case-sensitive)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2b6da4..0x2c9a68 EA-sorted asc next 100 not yet stubbed in script crate
// Filtered 5401, existing 2031, remaining 3370 -> 3270 after batch; global filler 5740 already in script
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2b6da4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2b6dac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2b6e50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2b6e58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2b7770() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor::Item const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor::Item const*)")]
pub fn stub_0x2b7b20(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::Item const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,RBX::Reflection::EnumDescriptor::Item const*)")]
pub fn stub_0x2b7bf8(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaEnumItem) -> crate::lua::LuaEnumItem {
// Bridge<EnumItem>::pushNewObject.
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(value.clone()) }));
value.clone()
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor const*)")]
pub fn stub_0x2b7c38(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "RBX::Reflection::EnumDescriptor const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor const*>(lua_State *,RBX::Reflection::EnumDescriptor const*)")]
pub fn stub_0x2b7d10(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(lua_State *)")]
pub fn stub_0x2b7e68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(lua_State *)")]
pub fn stub_0x2b7e9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_index(lua_State *)")]
pub fn stub_0x2b7ed0(value: &crate::lua::LuaEnumItem, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumItem>::on_index: Name/Value/EnumType; else invalid.
if key == "Value" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.value)));
} else if key == "EnumType" {
    thread.push(crate::lua::LuaStackValue::String(value.owner.clone()));
} else if key == "Name" {
    thread.push(crate::lua::LuaStackValue::String(format!("{}:{}", value.owner, value.value)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_newindex(lua_State *)")]
pub fn stub_0x2b7f04(key: &str) -> ! {
// Bridge<EnumItem>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_index(lua_State *)")]
pub fn stub_0x2b7f38(value: &crate::lua::LuaEnumDescriptor, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumDescriptor>::on_index — ordinal lookup by key.
match key.parse::<usize>() {
    Ok(i) if i < value.values.len() => {
        thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: value.name.clone(), value: value.values[i] }) }));
    }
    _ => panic!("{key} is not a valid member"),
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_newindex(lua_State *)")]
pub fn stub_0x2b7f6c(key: &str) -> ! {
// Bridge<EnumDesc>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings> RBX::Creatable<RBX::Instance>::create<RBX::LuaSettings>(void)")]
pub fn stub_0x2b8254() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings>::shared_ptr<RBX::LuaSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2b8304() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaSettings,RBX::LuaSettings>(rbx_core::SharedPtr<RBX::LuaSettings> const*,RBX::LuaSettings *)const")]
pub fn stub_0x2b83cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2b84b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b85c0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2b85c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b85e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2b8600() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2b8838() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc() [0x2b8a94]")]
pub fn stub_0x2b8a94(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x2b8b48() -> crate::slot::SlotConnection {
// IDA 0x2b8b48: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x2b8c9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x2b8f38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bada0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2baf4c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::~BoundFuncDesc() [0x2baf7c]")]
pub fn stub_0x2baf7c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bb050() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,int RBX::ScriptContext::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x2bb084() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isReadOnly(void)const")]
pub fn stub_0x2bb330() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isWriteOnly(void)const")]
pub fn stub_0x2bb334() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x2bb338() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x2bb344() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bb394() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc() [0x2bb498]")]
pub fn stub_0x2bb498(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bb54c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),RBX::Reflection::Variant&)")]
pub fn stub_0x2bb570() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bb720() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2bb8cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc() [0x2bb8fc]")]
pub fn stub_0x2bb8fc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bb9d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),RBX::Reflection::Variant &,bool const&)")]
pub fn stub_0x2bba10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bbafc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2bbc74() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::~BoundFuncDesc() [0x2bbca4]")]
pub fn stub_0x2bbca4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bbd78() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,bool RBX::ScriptContext::*,void (RBX::ScriptContext::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x2bbdb8() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isReadOnly(void)const")]
pub fn stub_0x2bc244() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isWriteOnly(void)const")]
pub fn stub_0x2bc248() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x2bc24c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x2bc258() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::BoundFuncDesc(void (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bccdc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::~BoundFuncDesc() [0x2bcde0]")]
pub fn stub_0x2bcde0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bce94() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bd744() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2bd90c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::~BoundFuncDesc() [0x2bd958]")]
pub fn stub_0x2bd958(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2bda2c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 2)
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,std::string),std::string,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
pub fn stub_0x2bdbf8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call2Helper<RBX::ScriptContext, void (RBX::ScriptContext::*)(std::string,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2bddc0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2bdf38() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::~BoundFuncDesc() [0x2bdf68]")]
pub fn stub_0x2bdf68(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x2be03c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2c2424() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2c252c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2c2530]")]
pub fn stub_0x2c2530(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2c2534() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c2554() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2c256c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::function1<void,lua_State *>::assign_to_own(boost::function1<void,lua_State *> const&)")]
pub fn stub_0x2c2778(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Lua::WeakThreadRef::addRef(lua_State *)")]
pub fn stub_0x2c96b4(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(RBX::Lua::WeakThreadRef const&)")]
pub fn stub_0x2c9800() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(RBX::Lua::WeakThreadRef const&) [0x2c9804]")]
pub fn stub_0x2c9804() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef()")]
pub fn stub_0x2c99c4(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef() [0x2c9a64]")]
pub fn stub_0x2c9a64(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef() [0x2c9a68]")]
pub fn stub_0x2c9a68(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}
