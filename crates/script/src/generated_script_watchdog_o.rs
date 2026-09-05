// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x46a454..0x46f034 | script 25852->25952 distinct (filler 0x46a454 asc, not-in-script 59693->59593)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x46a454() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x46a528() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
pub fn stub_0x46a5fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x46a6e0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_0x46a6fc(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x46a714(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x46a7ec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x46a8bc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,int)")]
pub fn stub_0x46a980(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x46aa8c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Instance::SaveFilter &,boost::enable_if<boost::is_enum<RBX::Instance::SaveFilter>,void>::type *)")]
pub fn stub_0x46abd4() -> crate::slot::PortedFn {
// IDA 0x46abd4: bool RBX::Reflection::ArgHelper::try_enum<1, RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments&,~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x46abd4, "bool RBX::Reflection::ArgHelper::try_enum<1, RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDes~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x46ac28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x46ada0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc() [0x46add0]")]
pub fn stub_0x46add0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x46ae9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::DataModel*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
pub fn stub_0x46afdc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x46b164() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc() [0x46b354]")]
pub fn stub_0x46b354(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x46b408() -> crate::slot::SlotConnection {
// IDA 0x46b408: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x46b55c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x46b6cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")]
pub fn stub_0x46b6e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x46b858() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&)")]
pub fn stub_0x46b974() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
pub fn stub_0x46badc(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x46bb0c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x46bbf0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x46bcd8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x46bdd0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x46bdec(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x46be00(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x46bee8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x46bfcc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(rbx_core::SharedPtr<RBX::Instance> &,RBX::Reflection::PropertyDescriptor const* &)")]
pub fn stub_0x46c0a0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x46c0bc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&)")]
pub fn stub_0x46c214() -> crate::slot::SlotConnection {
// IDA 0x46c214: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
pub fn stub_0x46c308(slot: &crate::slot::CallableSlot) {
// IDA 0x46c308: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot*)")]
pub fn stub_0x46c514(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*)")]
pub fn stub_0x46c538(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x46c538: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")]
pub fn stub_0x46c634(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot() [0x46c744]")]
pub fn stub_0x46c744(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x46c878(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x46c878: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x46c950(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x46c950: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_0x46c958(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x46ca70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x46cb60(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x46cb60: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable() [0x46cc70]")]
pub fn stub_0x46cc70(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x46cc70: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot() [0x46cda0]")]
pub fn stub_0x46cda0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> const&)")]
pub fn stub_0x46cdd0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::BoundFuncDesc(void (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x46ce00() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc() [0x46cf04]")]
pub fn stub_0x46cf04(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x46cfb8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 0)
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(RBX::DataModel *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x46cfd8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_invoker1<void (*)(RBX::DataModel *),void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x46d034(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const")]
pub fn stub_0x46d0e4() -> crate::slot::WindowAverage {
// WindowAverage ctor — empty window.
crate::slot::WindowAverage::new()
}

#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
pub fn stub_0x46d128(avg: &crate::slot::WindowAverage, stat: u32) -> f64 {
// WindowAverage::getStats — 0 selects the mean, else the
// window variance; empty windows yield zero.
avg.get_stats(stat)
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
pub fn stub_0x46d1b0(avg: &crate::slot::WindowAverage, stat: u32) -> f64 {
// WindowAverage::getStats — 0 selects the mean, else the
// window variance; empty windows yield zero.
avg.get_stats(stat)
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const")]
pub fn stub_0x46d208() -> crate::slot::WindowAverage {
// WindowAverage ctor — empty window.
crate::slot::WindowAverage::new()
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Implementation(RBX::DataModel*,rbx_core::SharedPtr<RBX::DataModel::GenericJob>,rbx_core::SharedPtr<RBX::Limits::Counter>)")]
pub fn stub_0x46d248() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::GenericJob")
}

#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::pop_if_present(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&)")]
pub fn stub_0x46d698() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::reset<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
pub fn stub_0x46d778() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>(void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>)")]
pub fn stub_0x46d7a4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::task(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>)")]
pub fn stub_0x46d8bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::push(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
pub fn stub_0x46d908() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::push_back(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
pub fn stub_0x46d9cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_push_back_aux(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
pub fn stub_0x46da0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x46db60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x46db7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x46dc54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>)")]
pub fn stub_0x46dc6c() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>)")]
pub fn stub_0x46df08(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x46dff8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_0x46e014(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x46e030(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x46e110(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_0x46e208(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x46e208: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x46e2d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")]
pub fn stub_0x46e358() -> crate::slot::InstanceHandle {
// RBX::DataModel::LegacyLock::Implementation::Events ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
pub fn stub_0x46e404() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
pub fn stub_0x46e4d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")]
pub fn stub_0x46e5d8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p() [0x46e5dc]")]
pub fn stub_0x46e5dc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::dispose(void)")]
pub fn stub_0x46e5e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_deleter(std::type_info const&)")]
pub fn stub_0x46e604() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_untyped_deleter(void)")]
pub fn stub_0x46e608() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
pub fn stub_0x46e60c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)")]
pub fn stub_0x46e644() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)")]
pub fn stub_0x46e670() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")]
pub fn stub_0x46e69c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")]
pub fn stub_0x46e6a0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::~safe_queue()")]
pub fn stub_0x46e808(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()")]
pub fn stub_0x46e8cc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()")]
pub fn stub_0x46e9b4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")]
pub fn stub_0x46e9e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x46eb20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**)")]
pub fn stub_0x46ec78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)")]
pub fn stub_0x46ed6c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")]
pub fn stub_0x46ee90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock::Implementation::Events")
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")]
pub fn stub_0x46f030(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")]
pub fn stub_0x46f034(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob() — engine-side; linkage preserved via the alias.
let _ = handle;
}
