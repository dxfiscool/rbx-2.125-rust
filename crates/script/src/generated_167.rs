// Auto-generated skeletons for rbx-script — Lua|Script batch
// Filter: Lua|Script (case-sensitive) (4456 filtered, 1934 remaining before, 1784 after)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x449bd8..0x586e58 EA-sorted asc next 150 Lua|Script gaps not yet in script crate (script 11121 -> 11271 distinct, global 32656->32751 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const*,RBX::ScriptService *)const")]
pub fn stub_0x449bd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x449cc4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x449dcc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x449dd0]")]
pub fn stub_0x449dd0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x449dd4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x449df4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x449e0c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")]
pub fn stub_0x44c420(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sScriptInformationProvider>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")]
pub fn stub_0x44c468(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sScriptInformationProvider>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void)")]
pub fn stub_0x44c550() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44c628() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::DebugSettings::getLuaRamLimit(void)const")]
pub fn stub_0x47c5a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setLuaRamLimit(int)")]
pub fn stub_0x47c5b8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isScriptable(void)const")]
pub fn stub_0x498af4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0x4d6218(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Scripting::DebuggerManager, RBX::Scripting::sDebuggerManag~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4db678(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::ScriptInformationProvider, RBX::sScriptInformationProvider~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4db798(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::RuntimeScriptService, RBX::sRuntimeScriptService, RBX::Non~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4dce18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::LuaWebService, RBX::sLuaWebService, RBX::NonFactoryProduct~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11LuaSettingsELZNS_12sLuaSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4defd8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::LuaSettings, RBX::sLuaSettings, RBX::FactoryProduct<RBX::L~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11LocalScriptELZNS_12sLocalScriptEENS_14FactoryProductIS2_NS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4df218(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::LocalScript, RBX::sLocalScript, RBX::FactoryProduct<RBX::L~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4e0bf8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::AdvLuaDragger, RBX::sAdvLuaDragger, RBX::FactoryProduct<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4e0d18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::LuaDragger, RBX::sLuaDragger, RBX::FactoryProduct<RBX::Lua~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(void)const")]
pub fn stub_0x4fcb3c() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::Creatable<RBX::Instance>::create<RBX::ScriptInformationProvider>(void)")]
pub fn stub_0x4fed3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&)")]
pub fn stub_0x4fedec(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4fee20() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4fef28(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4fef2c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::GuiBuilder::buildLuaGui(void)")]
pub fn stub_0x51dbc4(handle: &crate::slot::InstanceHandle) {
// RBX::GuiBuilder::buildLuaGui(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::tweenSizeAndPosition(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x525cc8(handle: &crate::slot::InstanceHandle) {
// RBX::GuiObject::tweenSizeAndPosition(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDire~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x525e50(handle: &crate::slot::InstanceHandle) {
// RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObje~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::tweenSize(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x526058(handle: &crate::slot::InstanceHandle) {
// RBX::GuiObject::tweenSize(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::InvokeTweenStatusCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus)")]
pub fn stub_0x528060() -> crate::slot::PortedFn {
// IDA 0x528060: RBX::InvokeTweenStatusCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x528060, "RBX::InvokeTweenStatusCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiO~")
}

#[doc(alias = "RBX::InvokeRemoveOnTweenEnd(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x528c4c() -> crate::slot::PortedFn {
// IDA 0x528c4c: RBX::InvokeRemoveOnTweenEnd(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x528c4c, "RBX::InvokeRemoveOnTweenEnd(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc()")]
pub fn stub_0x52bdd0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc()")]
pub fn stub_0x52bdd4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>)")]
pub fn stub_0x52cfc0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x52d230() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isScriptable(void)const")]
pub fn stub_0x531b90() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiButton")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0x533324() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiButton")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x534440() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x5345d8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)")]
pub fn stub_0x534774(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x534920(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
pub fn stub_0x53493c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x534958(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x534af4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x534c8c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list1<RBX::GuiObject::TweenStatus &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) &,boost::_bi::list1<RBX::GuiObject::TweenStatus &> &,int)")]
pub fn stub_0x534dd0(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x534dd0: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x534f14(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
pub fn stub_0x535104() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x535c0c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x535da4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>)")]
pub fn stub_0x535f40(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x5360ec(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
pub fn stub_0x536108(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x536124(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x5362c0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x536458(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::GuiObject::TweenStatus&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::GuiObject::TweenStatus&> &,int)")]
pub fn stub_0x53659c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x53659c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x5366e8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
pub fn stub_0x5368d8() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
pub fn stub_0x536a1c() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::isScriptable(void)const")]
pub fn stub_0x53a318() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiObject")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isScriptable(void)const")]
pub fn stub_0x53ba7c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiObject")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x53d538() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 6)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x53d980() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 6)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc() [0x53da40]")]
pub fn stub_0x53da40(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x53dae0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 6)
}

#[doc(alias = "RBX::Reflection::Call6Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x53dc24(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call6Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiOb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x53e574() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x53e7f4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 7)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x53ec8c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 7)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc() [0x53ed6c]")]
pub fn stub_0x53ed6c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x53ee0c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 7)
}

#[doc(alias = "RBX::Reflection::Call7Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x53ef64(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call7Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,7>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x53f8dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc() [0x54033c]")]
pub fn stub_0x54033c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc() [0x5404ac]")]
pub fn stub_0x5404ac(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::GuiService::addNotification(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x541f8c(handle: &crate::slot::InstanceHandle) {
// RBX::GuiService::addNotification(std::string,std::string,std::string,int,RBX::Lua::WeakFun~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiService::addCenterDialog(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x5424e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::InvokeCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool)")]
pub fn stub_0x543c90() -> crate::slot::PortedFn {
// IDA 0x543c90: RBX::InvokeCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x543c90, "RBX::InvokeCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool)")
}

#[doc(alias = "RBX::InvokeNotificationCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x5446a8() -> crate::slot::PortedFn {
// IDA 0x5446a8: RBX::InvokeNotificationCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5446a8, "RBX::InvokeNotificationCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")
}

#[doc(alias = "RBX::GuiService::getUseLuaChat(void)const")]
pub fn stub_0x544a54(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiService getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::~BoundFuncDesc()")]
pub fn stub_0x544b48(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()")]
pub fn stub_0x544b4c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "__ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESM_")]
pub fn stub_0x545ab8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool)")]
pub fn stub_0x545c48() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef)")]
pub fn stub_0x5465b8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEENS4_5list2INS4_5valueINS6_INS7_18NotificationObjectEEEEENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x54819c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x5482f4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)")]
pub fn stub_0x54849c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x548608(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x548624(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x548638(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x548794(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x5488ec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) &,boost::_bi::list0 &,int)")]
pub fn stub_0x5489f4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x5489f4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x548b94(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
pub fn stub_0x548d44() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
pub fn stub_0x548e44() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x54a644() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x54a7a4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>)")]
pub fn stub_0x54a908(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x54aa80(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x54aa9c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x54aab0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x54ac14(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x54ad74(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool) &,boost::_bi::list0 &,int)")]
pub fn stub_0x54ae84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x54ae84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x54af8c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
pub fn stub_0x54b148() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
pub fn stub_0x54b250() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
pub fn stub_0x54b35c() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::BoundFuncDesc(void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x54d9e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x54dc5c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc() [0x54dcdc]")]
pub fn stub_0x54dcdc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x54dd7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call4Helper<RBX::GuiService,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,RBX::GuiService::CenterDialogType const&,RBX::Lua::WeakFunctionRef const&,RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x54dee8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x54e1dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x54e3b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::BoundFuncDesc(void (RBX::GuiService::*)(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x54e5e0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiService", "void", 5)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x54e8a4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiService", "void", 5)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::~BoundFuncDesc() [0x54e944]")]
pub fn stub_0x54e944(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x54e9e4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiService", "void", 5)
}

#[doc(alias = "RBX::Reflection::Call5Helper<RBX::GuiService,void (RBX::GuiService::*)(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&,int const&,RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x54ec9c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call5Helper<RBX::GuiService,void (RBX::GuiService::*)(std::string,std::st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x54f17c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(RBX::Instance const*)")]
pub fn stub_0x55483c() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "RBX::Lua::ThreadRef::ThreadRef(RBX::Lua::detail::LiveThreadRef *)")]
pub fn stub_0x5548d0() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc() [0x554b04]")]
pub fn stub_0x554b04(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::~BoundFuncDesc() [0x554c64]")]
pub fn stub_0x554c64(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0x55f6ac() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::BodyPosition")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0x561318() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Rocket")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isScriptable(void)const")]
pub fn stub_0x56b7b8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Handles")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isScriptable(void)const")]
pub fn stub_0x56cea0() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Handles")
}

#[doc(alias = "RBX::HopperBin::onSelectScript(void)")]
pub fn stub_0x572714(handle: &crate::slot::InstanceHandle) {
// RBX::HopperBin::onSelectScript(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ScriptMouseCommand,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x573890() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand>::shared_ptr<RBX::ScriptMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x575fd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ScriptMouseCommand,RBX::ScriptMouseCommand>(rbx_core::SharedPtr<RBX::ScriptMouseCommand> const*,RBX::ScriptMouseCommand *)const")]
pub fn stub_0x5760a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x576184() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x57627c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x576280]")]
pub fn stub_0x576280(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x576284() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x576294() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x5762ac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0x577f00() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::HopperBin")
}

#[doc(alias = "RBX::UnsafeScriptStripperCollector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x5841e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::unsafeScriptStripper(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x584390() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::handleScriptInfoResponse(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x585108() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::shared_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
pub fn stub_0x586e58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}
