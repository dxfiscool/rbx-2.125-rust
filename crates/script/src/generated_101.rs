// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf2a2b4..0xf2a9c4 | 4811->4911 covered, 490 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a2b4]")]
pub fn stub_0xf2a2b4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a2b4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *)) [0xf2a2c4]")]
pub fn stub_0xf2a2c4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2a2c4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const [0xf2a2d4]")]
pub fn stub_0xf2a2d4(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const [0xf2a2d4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "YieldFunctionStateObject::execute(void) [0xf2a314]")]
pub fn stub_0xf2a314() -> crate::slot::PortedFn {
// IDA 0xf2a314: YieldFunctionStateObject::execute(void) [0xf2a314].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2a314, "YieldFunctionStateObject::execute(void) [0xf2a314]")
}

#[doc(alias = "YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,lua_State *) [0xf2a324]")]
pub fn stub_0xf2a324() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<YieldFunctionStateObject> RBX::shared_from<YieldFunctionStateObject>(YieldFunctionStateObject*) [0xf2a334]")]
pub fn stub_0xf2a334() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("YieldFunctionStateObject")
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int) [0xf2a344]")]
pub fn stub_0xf2a344() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &) [0xf2a354]")]
pub fn stub_0xf2a354() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &) [0xf2a364]")]
pub fn stub_0xf2a364() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::EventInstance* RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::pushNewObject<RBX::Lua::EventInstance>(lua_State *,RBX::Lua::EventInstance) [0xf2a374]")]
pub fn stub_0xf2a374(thread: &mut crate::lua::LuaThreadState, value: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>) -> Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>> {
// SharedPtrBridge::pushNewObject<Instance> — pushes the
// Instance-class userdata (or nil) and returns the ref.
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::INSTANCE.to_owned(), payload: crate::lua::LuaUserdataPayload::Instance(value.clone()) }));
value.clone()
}

#[doc(alias = "rbx_core::SharedPtr<YieldFunctionStateObject>::shared_ptr<YieldFunctionStateObject>(YieldFunctionStateObject *) [0xf2a384]")]
pub fn stub_0xf2a384() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("YieldFunctionStateObject")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>) [0xf2a3a4]")]
pub fn stub_0xf2a3a4() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list1<RBX::Reflection::Variant&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant> &,boost::_bi::list1<RBX::Reflection::Variant&> &,int) [0xf2a3b4]")]
pub fn stub_0xf2a3b4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2a3b4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string> &,boost::_bi::list1<std::string &> &,int) [0xf2a3c4]")]
pub fn stub_0xf2a3c4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2a3c4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>) [0xf2a3d4]")]
pub fn stub_0xf2a3d4() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int) [0xf2a3e4]")]
pub fn stub_0xf2a3e4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2a3e4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>::list3(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>) [0xf2a3f4]")]
pub fn stub_0xf2a3f4() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple> boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<rbx_core::SharedPtr<RBX::Reflection::Tuple>>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,long) [0xf2a404]")]
pub fn stub_0xf2a404(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2a404: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage2(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>) [0xf2a414]")]
pub fn stub_0xf2a414() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>) [0xf2a424]")]
pub fn stub_0xf2a424() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>) [0xf2a434]")]
pub fn stub_0xf2a434() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>::storage3(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>) [0xf2a444]")]
pub fn stub_0xf2a444() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list_av_3<RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>::type> boost::bind<rbx_core::SharedPtr<RBX::Reflection::Tuple>,RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>(rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>) [0xf2a454]")]
pub fn stub_0xf2a454() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list_av_2<rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,RBX::Reflection::Variant,rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(RBX::Reflection::Variant),rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>) [0xf2a464]")]
pub fn stub_0xf2a464() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list_av_2<rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,std::string,rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(std::string),rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>) [0xf2a474]")]
pub fn stub_0xf2a474() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string>::type> boost::bind<void,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string>(void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string) [0xf2a484]")]
pub fn stub_0xf2a484() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::shared_count::shared_count<YieldFunctionStateObject>(YieldFunctionStateObject *) [0xf2a494]")]
pub fn stub_0xf2a494() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2a4a4]")]
pub fn stub_0xf2a4a4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2a4b4]")]
pub fn stub_0xf2a4b4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2a4c4]")]
pub fn stub_0xf2a4c4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type) [0xf2a4d4]")]
pub fn stub_0xf2a4d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSD_13WeakThreadRefEEEENSB_5list3INSB_5valueISE_EENS_3argILi1EEENSL_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a4e4() -> crate::slot::PortedFn {
// IDA 0xf2a4e4: j___ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a4e4, "j___ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_P~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_5list2INS7_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a4f4() -> crate::slot::PortedFn {
// IDA 0xf2a4f4: j___ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a4f4, "j___ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunction~")
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5list2INS7_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a504() -> crate::slot::PortedFn {
// IDA 0xf2a504: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5li~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a504, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakTh~")
}

#[doc(alias = "j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS4_5list2INS4_5valueINS_10shared_ptrIS8_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a514() -> crate::slot::PortedFn {
// IDA 0xf2a514: j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS4_5list2INS4_5valueINS_10sh~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a514, "j___ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS4_5list~")
}

#[doc(alias = "void boost::function1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>) [0xf2a524]")]
pub fn stub_0xf2a524(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a534() -> crate::slot::PortedFn {
// IDA 0xf2a534: j___ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFun~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a534, "j___ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PF~")
}

#[doc(alias = "void boost::function1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>) [0xf2a544]")]
pub fn stub_0xf2a544(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a554() -> crate::slot::PortedFn {
// IDA 0xf2a554: j___ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a554, "j___ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionS~")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>) [0xf2a564]")]
pub fn stub_0xf2a564(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a574() -> crate::slot::PortedFn {
// IDA 0xf2a574: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5lis~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a574, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThr~")
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>) [0xf2a584]")]
pub fn stub_0xf2a584(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2a594() -> crate::slot::PortedFn {
// IDA 0xf2a594: j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10sha~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a594, "j___ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2~")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::findDescriptor(char const*)const [0xf2a5e4]")]
pub fn stub_0xf2a5e4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::find~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void boost::enable_shared_from_this<YieldFunctionStateObject>::_internal_accept_owner<YieldFunctionStateObject,YieldFunctionStateObject>(rbx_core::SharedPtr<YieldFunctionStateObject> const*,YieldFunctionStateObject *)const [0xf2a5f4]")]
pub fn stub_0xf2a5f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("YieldFunctionStateObject")
}

#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>::call<rbx_core::SharedPtr<YieldFunctionStateObject>,RBX::Reflection::Variant>(rbx_core::SharedPtr<YieldFunctionStateObject> &,void const*,RBX::Reflection::Variant &)const [0xf2a604]")]
pub fn stub_0xf2a604() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>::call<rbx_core::SharedPtr<YieldFunctionStateObject>,std::string>(rbx_core::SharedPtr<YieldFunctionStateObject> &,void const*,std::string &)const [0xf2a614]")]
pub fn stub_0xf2a614() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2a624]")]
pub fn stub_0xf2a624(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &)const [0xf2a634]")]
pub fn stub_0xf2a634(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2a644]")]
pub fn stub_0xf2a644(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2a654]")]
pub fn stub_0xf2a654(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf2a664]")]
pub fn stub_0xf2a664(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2a674]")]
pub fn stub_0xf2a674(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const [0xf2a684]")]
pub fn stub_0xf2a684(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2a694]")]
pub fn stub_0xf2a694(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2a6a4]")]
pub fn stub_0xf2a6a4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf2a6b4]")]
pub fn stub_0xf2a6b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2a6c4]")]
pub fn stub_0xf2a6c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,bool RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a714]")]
pub fn stub_0xf2a714() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,double RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a724]")]
pub fn stub_0xf2a724() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "double")
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,float RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a734]")]
pub fn stub_0xf2a734() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,int RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a744]")]
pub fn stub_0xf2a744() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf2a754() -> crate::slot::PortedFn {
// IDA 0xf2a754: j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE17static~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a754, "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS~")
}

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev")]
pub fn stub_0xf2a764() -> crate::slot::PortedFn {
// IDA 0xf2a764: j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a764, "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev")
}

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub fn stub_0xf2a774() -> crate::slot::PortedFn {
// IDA 0xf2a774: j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a774, "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")
}

#[doc(alias = "RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>::TGenericSlotWrapper(RBX::Lua::WaitScriptSlot const&) [0xf2a784]")]
pub fn stub_0xf2a784() -> crate::slot::FnSlot {
// script-slot wrapper ctor — empty.
crate::slot::FnSlot::new()
}

#[doc(alias = "RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>::TGenericSlotWrapper(RBX::Lua::FunctionScriptSlot const&) [0xf2a794]")]
pub fn stub_0xf2a794() -> crate::slot::FnSlot {
// script-slot wrapper ctor — empty.
crate::slot::FnSlot::new()
}

#[doc(alias = "RBX::Lua::WaitScriptSlot::WaitScriptSlot(lua_State *) [0xf2a7a4]")]
pub fn stub_0xf2a7a4() -> crate::slot::FnSlot {
// script-slot wrapper ctor — empty.
crate::slot::FnSlot::new()
}

#[doc(alias = "RBX::Lua::WaitScriptSlot::operator()(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&) [0xf2a7b4]")]
pub fn stub_0xf2a7b4(slot: &crate::slot::FnSlot) {
// script-slot wrapper dispatch.
slot.invoke();
}

#[doc(alias = "RBX::Lua::FunctionScriptSlot::FunctionScriptSlot(lua_State *,int) [0xf2a7c4]")]
pub fn stub_0xf2a7c4() -> crate::slot::FnSlot {
// script-slot wrapper ctor — empty.
crate::slot::FnSlot::new()
}

#[doc(alias = "RBX::Lua::FunctionScriptSlot::operator()(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&) [0xf2a7d4]")]
pub fn stub_0xf2a7d4(slot: &crate::slot::FnSlot) {
// script-slot wrapper dispatch.
slot.invoke();
}

#[doc(alias = "rbx::signals::connection* RBX::Lua::Bridge<rbx::signals::connection,true>::pushNewObject<rbx::signals::connection>(lua_State *,rbx::signals::connection) [0xf2a7e4]")]
pub fn stub_0xf2a7e4() -> crate::slot::SlotConnection {
// IDA 0xf2a7e4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,RBX::Lua::WaitScriptSlot>(RBX::Lua::WaitScriptSlot const&) [0xf2a7f4]")]
pub fn stub_0xf2a7f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,RBX::Lua::FunctionScriptSlot>(RBX::Lua::FunctionScriptSlot const&) [0xf2a804]")]
pub fn stub_0xf2a804() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>) [0xf2a814]")]
pub fn stub_0xf2a814() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>) [0xf2a824]")]
pub fn stub_0xf2a824() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>")
}

#[doc(alias = "rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>(rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> const&) [0xf2a834]")]
pub fn stub_0xf2a834() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>")
}

#[doc(alias = "rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>(rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> const&) [0xf2a844]")]
pub fn stub_0xf2a844() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>) [0xf2a854]")]
pub fn stub_0xf2a854() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>) [0xf2a864]")]
pub fn stub_0xf2a864() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Script,std::string (RBX::Script::*)(void),std::string>::call(RBX::Script*,std::string (RBX::Script::*)(void),RBX::Reflection::Variant &) [0xf2a874]")]
pub fn stub_0xf2a874(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call0Helper<RBX::Script,std::string (RBX::Script::*)(void),std::string>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Script,std::string ()(void),0>::BoundFuncDesc(std::string (RBX::Script::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2a884]")]
pub fn stub_0xf2a884() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Script", "std::string", 0)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BaseScript,RBX::ContentId>::PropDescriptor<RBX::ContentId const& (RBX::BaseScript::*)(void)const,void (RBX::BaseScript::*)(RBX::ContentId const&)>(char const*,char const*,RBX::ContentId const& (RBX::BaseScript::*)(void)const,void (RBX::BaseScript::*)(RBX::ContentId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a894]")]
pub fn stub_0xf2a894(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BaseScript,bool>::PropDescriptor<bool (RBX::BaseScript::*)(void)const,void (RBX::BaseScript::*)(bool)>(char const*,char const*,bool (RBX::BaseScript::*)(void)const,void (RBX::BaseScript::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a8a4]")]
pub fn stub_0xf2a8a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Script,RBX::ProtectedString>::PropDescriptor<RBX::ProtectedString const& (RBX::Script::*)(void)const,void (RBX::Script::*)(RBX::ProtectedString const&)>(char const*,char const*,RBX::ProtectedString const& (RBX::Script::*)(void)const,void (RBX::Script::*)(RBX::ProtectedString const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a8b4]")]
pub fn stub_0xf2a8b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::LocalScript::~LocalScript() [0xf2a8e4]")]
pub fn stub_0xf2a8e4(handle: crate::slot::InstanceHandle) {
// RBX::LocalScript dtor.
drop(handle);
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf2a8f4() -> crate::slot::PortedFn {
// IDA 0xf2a8f4: j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a8f4, "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE17static_g~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf2a904() -> crate::slot::PortedFn {
// IDA 0xf2a904: j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a904, "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE17static_getCreat~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2a914() -> crate::slot::PortedFn {
// IDA 0xf2a914: j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a914, "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xf2a924() -> crate::slot::PortedFn {
// IDA 0xf2a924: j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a924, "j___ZN3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RuntimeScriptService>(void) [0xf2a934]")]
pub fn stub_0xf2a934() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*) [0xf2a944]")]
pub fn stub_0xf2a944() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*) [0xf2a954]")]
pub fn stub_0xf2a954() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE15isNullClassNameEv")]
pub fn stub_0xf2a964() -> crate::slot::PortedFn {
// IDA 0xf2a964: j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE15isNullClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a964, "j___ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE15isNullClassNameEv")
}

#[doc(alias = "RBX::RuntimeScriptService::RuntimeScriptService(void) [0xf2a974]")]
pub fn stub_0xf2a974() -> crate::slot::InstanceHandle {
// RBX::RuntimeScriptService ctor.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_21sRuntimeScriptServiceEEEERKS0_v")]
pub fn stub_0xf2a984() -> crate::slot::PortedFn {
// IDA 0xf2a984: j___ZN3RBX4Name7declareILZNS_21sRuntimeScriptServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a984, "j___ZN3RBX4Name7declareILZNS_21sRuntimeScriptServiceEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v")]
pub fn stub_0xf2a994() -> crate::slot::PortedFn {
// IDA 0xf2a994: j___ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a994, "j___ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v")]
pub fn stub_0xf2a9a4() -> crate::slot::PortedFn {
// IDA 0xf2a9a4: j___ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a9a4, "j___ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void) [0xf2a9b4]")]
pub fn stub_0xf2a9b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void) [0xf2a9c4]")]
pub fn stub_0xf2a9c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}
