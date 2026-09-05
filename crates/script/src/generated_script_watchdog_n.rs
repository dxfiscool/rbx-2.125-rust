// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x463730..0x46a354 | script 25752->25852 distinct (filler 0x463730 asc, not-in-script 59793->59693)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x463730(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::CallbackDesc<bool ()>::clearCallback(RBX::Reflection::DescribedBase*) con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x4637f0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x463908() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple> (boost::shared_ptr<RBX~")
}

#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_0x463a5c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple> (boost::shared_ptr<RBX~")
}

#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_0x463b98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *) [0x463ce8]")]
pub fn stub_0x463ce8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p() [0x463dc0]")]
pub fn stub_0x463dc0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")]
pub fn stub_0x463dc8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")]
pub fn stub_0x463e70() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")]
pub fn stub_0x463e74() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "__ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x463f54() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x464030() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")]
pub fn stub_0x464110(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x464200(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x46421c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x464230(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x464310(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")]
pub fn stub_0x464408(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4644d8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
pub fn stub_0x46455c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc() [0x464654]")]
pub fn stub_0x464654(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::CallbackDescriptor::~CallbackDescriptor()")]
pub fn stub_0x464760(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::CallbackDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")]
pub fn stub_0x464788(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter() [0x46478c]")]
pub fn stub_0x46478c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::setCallback(RBX::Reflection::DescribedBase *,boost::function<bool ()(void)> const&)const")]
pub fn stub_0x464790(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")]
pub fn stub_0x464acc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl() [0x464bc4]")]
pub fn stub_0x464bc4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
pub fn stub_0x465010() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x465110() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4651e8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x465208(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x4652e0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x4653b0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,int>(char const*,char const*,bool (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x465474(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor() [0x465584]")]
pub fn stub_0x465584(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x4655b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x4655b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4655bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x4655e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>(char const*,char const*,bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x465700(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x465814(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x465818(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x46581c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x465840(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x465864() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 5)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x465b24() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 5)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc() [0x465bc4]")]
pub fn stub_0x465bc4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x465cb0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 5)
}

#[doc(alias = "RBX::Reflection::Call5Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),std::string,std::string,std::string,std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
pub fn stub_0x46602c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call5Helper<RBX::DataModel, void (RBX::DataModel::*)(std::string, std::st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x466388() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x4665dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x466830() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc() [0x466934]")]
pub fn stub_0x466934(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x4669e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::DataModel*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),RBX::Reflection::Variant&)")]
pub fn stub_0x466a0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,std::string,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x466af8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x466d48() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc() [0x466db0]")]
pub fn stub_0x466db0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x466e94() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 3)
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,std::string,bool),std::string,std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,std::string,bool),RBX::Reflection::Variant &,std::string const&,std::string const&,bool const&)")]
pub fn stub_0x467084(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call3Helper<RBX::DataModel, std::string (RBX::DataModel::*)(std::string, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x4672e4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x4674e0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc() [0x46752c]")]
pub fn stub_0x46752c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x467608() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "std::string", 2)
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,bool),std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,bool),RBX::Reflection::Variant &,std::string const&,bool const&)")]
pub fn stub_0x467768(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call2Helper<RBX::DataModel, std::string (RBX::DataModel::*)(std::string, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
pub fn stub_0x467ed0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x467fcc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4680a0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
pub fn stub_0x468174(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x468258(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
pub fn stub_0x468274(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x46828c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x468364(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x468434(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string) &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_0x4684f8(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x468658(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::BoundFuncDesc(void (RBX::DataModel::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x468c38() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x468db0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc() [0x468de0]")]
pub fn stub_0x468de0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x468eac() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string),std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x468fe8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::DataModel, void (RBX::DataModel::*)(std::string), std::s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::BoundFuncDesc(bool (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x469118() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "bool", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc() [0x46921c]")]
pub fn stub_0x46921c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x4692d0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "bool", 0)
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DataModel,bool (RBX::DataModel::*)(void),bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x4692f4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call0Helper<RBX::DataModel, bool (RBX::DataModel::*)(), bool>::call(RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::BoundFuncDesc(void (RBX::DataModel::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x469324() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x46949c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc() [0x4694cc]")]
pub fn stub_0x4694cc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x4695a0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x4695d8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x469750() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc() [0x469780]")]
pub fn stub_0x469780(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x46984c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
pub fn stub_0x469988(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::DataModel, void (RBX::DataModel::*)(RBX::ContentId), RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::BoundFuncDesc(void (RBX::DataModel::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x469ac4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x469c3c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc() [0x469c6c]")]
pub fn stub_0x469c6c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x469d40() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 1)
}

#[doc(alias = "RBX::Instance::SaveFilter RBX::Reflection::ArgHelper::getArg<RBX::Instance::SaveFilter,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Instance::SaveFilter> const&,boost::disable_if<boost::is_same<RBX::Instance::SaveFilter,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x46a1c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
pub fn stub_0x46a354() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}
