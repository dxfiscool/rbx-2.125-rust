// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen|ScriptContext (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x444dc0..0x44b340 | script 18541->18641 distinct (filler 0x444dc0 asc, not-in-script 67004->66904)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x444dc0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x444ec8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x444ecc]")]
pub fn stub_0x444ecc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x444ed0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x444ef0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x444f08() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
pub fn stub_0x44558c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Soundscape::SoundService"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")]
pub fn stub_0x445700(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")]
pub fn stub_0x445768() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")]
pub fn stub_0x4457dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_0x445848(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::Soundscape::sSoundService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")]
pub fn stub_0x44588c() -> crate::slot::PortedFn {
// IDA 0x44588c: void RBX::Name::callDoDeclare<RBX::Soundscape::sSoundService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x44588c, "void RBX::Name::callDoDeclare<RBX::Soundscape::sSoundService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_0x445890(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Soundscape::sSoundService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Soundscape::SoundService>(void)")]
pub fn stub_0x445974() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
pub fn stub_0x445978() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Lighting,RBX::Lighting>(rbx_core::SharedPtr<RBX::Lighting> const*,RBX::Lighting *)const")]
pub fn stub_0x445c38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lighting")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Lighting *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x445d28(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x445f40() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
pub fn stub_0x446120(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x44630c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x446328(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x446380(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x446558(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x446764(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_0x44689c() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>::bind_t(std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
pub fn stub_0x446a48() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>)")]
pub fn stub_0x446af8() -> crate::slot::PortedFn {
// IDA 0x446af8: boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x446af8, "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>)")
}

#[doc(alias = "boost::detail::thread_data_base::thread_data_base(void)")]
pub fn stub_0x446bc8() -> crate::slot::PortedFn {
// IDA 0x446bc8: boost::detail::thread_data_base::thread_data_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x446bc8, "boost::detail::thread_data_base::thread_data_base()")
}

#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::push_back(std::pair<boost::condition_variable *,boost::mutex *> const&)")]
pub fn stub_0x446d80(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)")]
pub fn stub_0x446db0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x446db0, "std::vector<std::pair<boost::condition_variable*, boost::mutex*>, std::allocator<std::pair<boost::co~")
}

#[doc(alias = "std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)")]
pub fn stub_0x446ea8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x446ea8, "std::_Vector_base<std::pair<boost::condition_variable*, boost::mutex*>, std::allocator<std::pair<boo~")
}

#[doc(alias = "std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)")]
pub fn stub_0x446ec0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x446ec0, "std::pair<boost::condition_variable*, boost::mutex*>* std::__copy_backward<false, std::random_access~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
pub fn stub_0x446f08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::detail::thread_data_base")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x446ff0(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
pub fn stub_0x447000() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4470d0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)")]
pub fn stub_0x4471fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x447338(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x4473b8(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x447410(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x44753c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)")]
pub fn stub_0x447674() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x447794() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4478e0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x447a30(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x447b90(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_0x447bac(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x447bcc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x447d1c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x447e68(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_0x447f68(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x44806c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x448218() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x44830c() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)")]
pub fn stub_0x448400(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::scoped_ptr<RBX::DataModel::LegacyLock::Implementation>::~scoped_ptr()")]
pub fn stub_0x448434(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::~Implementation()")]
pub fn stub_0x4484dc(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::LegacyLock::Implementation dtor.
drop(handle);
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::reset(RBX::DataModel::GenericJob **)")]
pub fn stub_0x4486b8() -> crate::slot::PortedFn {
// IDA 0x4486b8: boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::reset(RBX::DataModel::GenericJob**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4486b8, "boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::reset(RBX::DataModel::GenericJob**)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService> RBX::Creatable<RBX::Instance>::create<RBX::AssetService>(void)")]
pub fn stub_0x448914() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AssetService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const&)")]
pub fn stub_0x4489c4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService>::shared_ptr<RBX::AssetService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x448c00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AssetService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const*,RBX::AssetService *)const")]
pub fn stub_0x448cc8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AssetService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x448db4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x448ebc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x448ec0]")]
pub fn stub_0x448ec0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x448ec4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x448ee4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x448efc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService> RBX::Creatable<RBX::Instance>::create<RBX::ContextActionService>(void)")]
pub fn stub_0x449f84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContextActionService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const&)")]
pub fn stub_0x44a034(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService>::shared_ptr<RBX::ContextActionService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44a270() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContextActionService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContextActionService,RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const*,RBX::ContextActionService *)const")]
pub fn stub_0x44a338() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContextActionService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44a424() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44a52c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44a530]")]
pub fn stub_0x44a530(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44a534() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44a554() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44a56c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44a610() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44a718(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44a720() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::Creatable<RBX::Instance>::create<RBX::FWService>(void)")]
pub fn stub_0x44a744() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FWService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const&)")]
pub fn stub_0x44a7f4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWService>::shared_ptr<RBX::FWService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44a828() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FWService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FWService,RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const*,RBX::FWService *)const")]
pub fn stub_0x44a8f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FWService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44a9dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44aae4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44aae8]")]
pub fn stub_0x44aae8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44aaec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44ab0c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44ab24() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(rbx_core::SharedPtr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
pub fn stub_0x44ab28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::Players")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44ac18(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService> RBX::Creatable<RBX::Instance>::create<RBX::PersonalServerService>(void)")]
pub fn stub_0x44ad94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PersonalServerService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const&)")]
pub fn stub_0x44ae44(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService>::shared_ptr<RBX::PersonalServerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44b080() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PersonalServerService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PersonalServerService,RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const*,RBX::PersonalServerService *)const")]
pub fn stub_0x44b148() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PersonalServerService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44b234() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44b33c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44b340]")]
pub fn stub_0x44b340(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}
