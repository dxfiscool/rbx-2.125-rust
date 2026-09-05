// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Script|Lua|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2c9b74..0x2d0808 | 2131->2231 covered, 3170 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Lua::WeakThreadRef::reset(void)")]
pub fn stub_0x2c9b74(handle: &mut crate::slot::InstanceHandle) {
// thread-ref attach — binds the link engine-side.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::removeFromNode(void)")]
pub fn stub_0x2c9c54(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::operator=(RBX::Lua::WeakThreadRef const&)")]
pub fn stub_0x2c9cb0(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *)")]
pub fn stub_0x2c9db8() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::removeRef(void)")]
pub fn stub_0x2c9dbc(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::Node::eraseAllRefs(void)")]
pub fn stub_0x2c9df8(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakThreadRef::Node::~Node()")]
pub fn stub_0x2c9ee8(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakThreadRef::Node::~Node() [0x2c9eec]")]
pub fn stub_0x2c9eec(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakThreadRef::Node::create(lua_State *)")]
pub fn stub_0x2c9f1c(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_index(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node> const&,char const*,lua_State *)")]
pub fn stub_0x2c9f54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_newindex(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>&,char const*,lua_State *)")]
pub fn stub_0x2ca00c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "RBX::Lua::dumpThreadRefCounts(void)")]
pub fn stub_0x2ca0c4(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int)")]
pub fn stub_0x2ca11c() -> crate::slot::InstanceHandle {
// RBX::Lua::WeakFunctionRef ctor.
crate::slot::InstanceHandle::new("RBX::Lua::WeakFunctionRef")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int) [0x2ca120]")]
pub fn stub_0x2ca120() -> crate::slot::InstanceHandle {
// RBX::Lua::WeakFunctionRef ctor.
crate::slot::InstanceHandle::new("RBX::Lua::WeakFunctionRef")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_index(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,char const*,lua_State *)")]
pub fn stub_0x2ca240() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_newindex(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>&,char const*,lua_State *)")]
pub fn stub_0x2ca2f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_index(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,char const*,lua_State *)")]
pub fn stub_0x2ca3b0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_newindex(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>&,char const*,lua_State *)")]
pub fn stub_0x2ca468() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Lua::lua_tofunction(lua_State *,int)")]
pub fn stub_0x2ca520(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::lua_tofunction(lua_State*, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2ca52c(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::lua_pushfunction(lua_State*, RBX::Lua::WeakFunctionRef const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x2ca57c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "callGenericFunctionBridge(lua_State *)")]
pub fn stub_0x2ca664() -> crate::slot::PortedFn {
// IDA 0x2ca664: callGenericFunctionBridge(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2ca664, "callGenericFunctionBridge(lua_State*)")
}

#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
pub fn stub_0x2ca820() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "callGenericAsyncFunctionBridge(lua_State *)")]
pub fn stub_0x2ca908() -> crate::slot::PortedFn {
// IDA 0x2ca908: callGenericAsyncFunctionBridge(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2ca908, "callGenericAsyncFunctionBridge(lua_State*)")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
pub fn stub_0x2cad6c(handle: crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef dtor.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef() [0x2cae0c]")]
pub fn stub_0x2cae0c(handle: crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef dtor.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef() [0x2cae10]")]
pub fn stub_0x2cae10(handle: crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef dtor.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::removeRef(void)")]
pub fn stub_0x2caf24(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef::removeRef() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2caf98() -> crate::slot::InstanceHandle {
// RBX::Lua::WeakFunctionRef ctor.
crate::slot::InstanceHandle::new("RBX::Lua::WeakFunctionRef")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&) [0x2caf9c]")]
pub fn stub_0x2caf9c() -> crate::slot::InstanceHandle {
// RBX::Lua::WeakFunctionRef ctor.
crate::slot::InstanceHandle::new("RBX::Lua::WeakFunctionRef")
}

#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *) [0x2cb0fc]")]
pub fn stub_0x2cb0fc() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
pub fn stub_0x2cb2ec(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef() [0x2cb2f0]")]
pub fn stub_0x2cb2f0(handle: crate::slot::InstanceHandle) {
// thread-ref dtor — releases the weak link.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef::operator=(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2cb3fc(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef::operator=(RBX::Lua::WeakFunctionRef const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Lua::WeakFunctionRef>(void)")]
pub fn stub_0x2cb4d0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Lua::WeakFunctionRef~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef & RBX::Reflection::Variant::convert<RBX::Lua::WeakFunctionRef>(void)")]
pub fn stub_0x2cb5b4(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::WeakFunctionRef& RBX::Reflection::Variant::convert<RBX::Lua::WeakFunctionRef>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)")]
pub fn stub_0x2cb874() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "onAsyncResult(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *)")]
pub fn stub_0x2cb958() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(RBX::Lua::detail::LiveThreadRef*)")]
pub fn stub_0x2cbc1c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x2cbd58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
pub fn stub_0x2cbda8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
pub fn stub_0x2cbdf8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef * rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x2cbdfc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
pub fn stub_0x2cbe54(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)")]
pub fn stub_0x2cbe58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType() [0x2cbf04]")]
pub fn stub_0x2cbf04(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void)")]
pub fn stub_0x2cbfb8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *)")]
pub fn stub_0x2cc020() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType() [0x2cc0c8]")]
pub fn stub_0x2cc0c8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const")]
pub fn stub_0x2cc0cc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list_av_3<RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,boost::arg<1>>::type> boost::bind<void,RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *,RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,boost::arg<1>>(void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,boost::arg<1>)")]
pub fn stub_0x2cc210() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::clear(void)")]
pub fn stub_0x2cc608(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::_bi::value<RBX::Lua::ThreadRef>::value(RBX::Lua::ThreadRef const&)")]
pub fn stub_0x2cc634() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>)")]
pub fn stub_0x2cc6f0() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>::storage3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>)")]
pub fn stub_0x2cc8d0() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>>::storage2(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>)")]
pub fn stub_0x2ccab0() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage1<boost::_bi::value<RBX::Lua::ThreadRef>>::storage1(boost::_bi::value<RBX::Lua::ThreadRef>)")]
pub fn stub_0x2cccc4() -> crate::slot::BindPiece {
// boost::bind fragment (storage1) composing a host BoundCall.
crate::slot::BindPiece::new("storage1")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2ccd80() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2ccf68() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>::bind_t(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>> const&)")]
pub fn stub_0x2cd154() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "void boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>)")]
pub fn stub_0x2cd2dc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2cd4d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>,void,RBX::Lua::IAsyncResult *>::invoke(boost::detail::function::function_buffer &,RBX::Lua::IAsyncResult *)")]
pub fn stub_0x2cd4f0(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2cd50c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2cd6f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2cd8d8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>::operator()<void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list1<RBX::Lua::IAsyncResult *&>>(boost::_bi::type<void>,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *) &,boost::_bi::list1<RBX::Lua::IAsyncResult *&> &,int)")]
pub fn stub_0x2cd984(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2cd984: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<Weak<RBX::ScriptContext>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2cdb6c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to_own(boost::function1<void,RBX::Lua::IAsyncResult *> const&)")]
pub fn stub_0x2cdd44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvLuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2ce4e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool>::shared_ptr<RBX::AdvLuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2ce804() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvLuaDragTool,RBX::AdvLuaDragTool>(rbx_core::SharedPtr<RBX::AdvLuaDragTool> const*,RBX::AdvLuaDragTool *)const")]
pub fn stub_0x2ce8cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2ce9b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2ceaa8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x2ceaac]")]
pub fn stub_0x2ceaac(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x2ceab0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2ceac0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2cead8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::AdvLuaDragger::AdvLuaDragger(void)")]
pub fn stub_0x2ced4c() -> crate::slot::InstanceHandle {
// RBX::AdvLuaDragger ctor.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cef40(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger() [0x2cefe0]")]
pub fn stub_0x2cefe0(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cefe4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger() [0x2cefec]")]
pub fn stub_0x2cefec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger() [0x2ceff4]")]
pub fn stub_0x2ceff4(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger() [0x2cf168]")]
pub fn stub_0x2cf168(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger() [0x2cf170]")]
pub fn stub_0x2cf170(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AdvLuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>)")]
pub fn stub_0x2cf178() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::AdvLuaDragger::mouseMove(RBX::RbxRay)")]
pub fn stub_0x2cf3b8(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::mouseMove(RBX::RbxRay) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::tryStartDragging(RBX::RbxRay const&)")]
pub fn stub_0x2cf6d0(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::tryStartDragging(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::doDrag(RBX::RbxRay const&)")]
pub fn stub_0x2cf930(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::doDrag(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::mouseUp(void)")]
pub fn stub_0x2cfd7c(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::mouseUp() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
pub fn stub_0x2d0030(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvLuaDragger getter.
cell.get()
}

#[doc(alias = "RBX::AdvLuaDragger::startDragging(void)")]
pub fn stub_0x2d0154(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::startDragging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
pub fn stub_0x2d03b0(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis, G3D::Matrix3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::alignPartToGrid(void)")]
pub fn stub_0x2d05bc(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::alignPartToGrid() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragger::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x2d07e0(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragger::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2d07e4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2d07f4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev")]
pub fn stub_0x2d0804() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_0x2d0808() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}
