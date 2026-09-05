// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x391848..0x3969dc | script 22052->22202 distinct (filler 0x391848 asc, not-in-script 63493->63343)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x391848() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hat")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hat,RBX::Hat>(rbx_core::SharedPtr<RBX::Hat> const*,RBX::Hat *)const")]
pub fn stub_0x391910() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hat")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3919f8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x391b00(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x391b04]")]
pub fn stub_0x391b04(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x391b08() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x391b28() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x391b40() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x391b44() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev")]
pub fn stub_0x391d88() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv")]
pub fn stub_0x391e24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv")]
pub fn stub_0x391eac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")]
pub fn stub_0x391ff0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Accoutrement")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3920a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Accoutrement")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Accoutrement,RBX::Accoutrement>(rbx_core::SharedPtr<RBX::Accoutrement> const*,RBX::Accoutrement *)const")]
pub fn stub_0x392168() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Accoutrement")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x392250() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x392358(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x39235c]")]
pub fn stub_0x39235c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x392360() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x392380() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x392398() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv")]
pub fn stub_0x39239c() -> crate::slot::PortedFn {
// IDA 0x39239c: void RBX::Name::callDoDeclare<RBX::sAccoutrement>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39239c, "void RBX::Name::callDoDeclare<RBX::sAccoutrement>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")]
pub fn stub_0x3923a0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAccoutrement>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev")]
pub fn stub_0x392480() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv")]
pub fn stub_0x3926c4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>>::~vector()")]
pub fn stub_0x392738(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x392804(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot() [0x392830]")]
pub fn stub_0x392830(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x392904(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x392904: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x392920(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x392920: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x39293c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x39293c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Accoutrement*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x392a14() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x392afc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x392afc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x392b28]")]
pub fn stub_0x392b28(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x392b28: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x392bfc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x392c5c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")]
pub fn stub_0x392c78(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>() con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x392ce0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x392ce4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x392d84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x392d8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x392e30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x392e38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x392edc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x392ee0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x392f80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x392f88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39302c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x393034(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::PropDescriptor<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>(char const*,char const*,int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3930d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor() [0x3931ec]")]
pub fn stub_0x3931ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isReadOnly(void)const")]
pub fn stub_0x393218(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isWriteOnly(void)const")]
pub fn stub_0x39321c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x393220(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x393240(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x393264(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::~PropDescriptor() [0x393378]")]
pub fn stub_0x393378(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x3933a4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x3933a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3933ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x3933d4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3933f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::~PropDescriptor() [0x39350c]")]
pub fn stub_0x39350c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
pub fn stub_0x393538(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
pub fn stub_0x39353c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x393540(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
pub fn stub_0x39357c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "global constructor keyed to_a_152")]
pub fn stub_0x3935a0() -> crate::slot::PortedFn {
// IDA 0x3935a0: __GLOBAL__I_a_152.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3935a0, "__GLOBAL__I_a_152")
}

#[doc(alias = "RBX::PartAdornment::setAdornee(RBX::PartInstance *)")]
pub fn stub_0x393b34(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::PartAdornment setter.
cell.set(value)
}

#[doc(alias = "RBX::PartAdornment::PartAdornment(char const*)")]
pub fn stub_0x393c44() -> crate::slot::InstanceHandle {
// RBX::PartAdornment ctor.
crate::slot::InstanceHandle::new("RBX::PartAdornment")
}

#[doc(alias = "RBX::PVAdornment::setAdornee(RBX::PVInstance *)")]
pub fn stub_0x393dd0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::PVAdornment setter.
cell.set(value)
}

#[doc(alias = "RBX::PVAdornment::PVAdornment(char const*)")]
pub fn stub_0x393ee0() -> crate::slot::InstanceHandle {
// RBX::PVAdornment ctor.
crate::slot::InstanceHandle::new("RBX::PVAdornment")
}

#[doc(alias = "RBX::PartAdornment::getAdorneeDangerous(void)const")]
pub fn stub_0x39406c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartAdornment getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
pub fn stub_0x394090(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::PVAdornment::getAdorneeDangerous(void)const")]
pub fn stub_0x3940bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PVAdornment getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()")]
pub fn stub_0x3940e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")]
pub fn stub_0x39410c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PVInstance")
}

#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
pub fn stub_0x39427c(handle: crate::slot::InstanceHandle) {
// RBX::PVAdornment dtor.
drop(handle);
}

#[doc(alias = "RBX::PVAdornment::~PVAdornment() [0x3943c4]")]
pub fn stub_0x3943c4(handle: crate::slot::InstanceHandle) {
// RBX::PVAdornment dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
pub fn stub_0x394464() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment()")]
pub fn stub_0x39448c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment() [0x3945d4]")]
pub fn stub_0x3945d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
pub fn stub_0x394730() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment() [0x394758]")]
pub fn stub_0x394758(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment() [0x3948a0]")]
pub fn stub_0x3948a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")]
pub fn stub_0x3949fc() -> crate::slot::PortedFn {
// IDA 0x3949fc: void RBX::Name::callDoDeclare<RBX::sPVAdornment>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3949fc, "void RBX::Name::callDoDeclare<RBX::sPVAdornment>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
pub fn stub_0x394a00(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPVAdornment>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x394ae0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x394b9c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x394c68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x394d20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x394df0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x394ea8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::RefPropDescriptor<RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*)>(char const*,char const*,RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x394f78(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::singleton(void)")]
pub fn stub_0x39501c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RefType<RBX::PVInstance*>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor() [0x395114]")]
pub fn stub_0x395114(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isReadOnly(void)const")]
pub fn stub_0x395144(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isWriteOnly(void)const")]
pub fn stub_0x395154(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x395164(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x39518c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x3952a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x39536c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x395390(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x395464(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x395488(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x39549c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x395518(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x395538(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x395618(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isReadOnly(void)const")]
pub fn stub_0x395620(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isWriteOnly(void)const")]
pub fn stub_0x395624(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x395628(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PVInstance * const&)const")]
pub fn stub_0x395648(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::~RefType()")]
pub fn stub_0x39566c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::RefType dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::PVInstance *>(char const*,char const*,RBX::PVInstance * *)")]
pub fn stub_0x395670() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::~RefType() [0x39571c]")]
pub fn stub_0x39571c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::RefType dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x395720(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3957dc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3958a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x395960(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x395a30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x395ae8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x395bb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor() [0x395c5c]")]
pub fn stub_0x395c5c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isReadOnly(void)const")]
pub fn stub_0x395c8c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isWriteOnly(void)const")]
pub fn stub_0x395c9c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x395cac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x395cd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x395dec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x395eb4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x395ed8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x395fac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x395fd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x395fe4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x396060(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x396080(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x396160(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartAdornment,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
pub fn stub_0x396168(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartAdornment,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
pub fn stub_0x39616c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartAdornment,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x396170(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartAdornment,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
pub fn stub_0x396190(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance>::shared_ptr<RBX::PVInstance>(rbx_core::Weak<RBX::PVInstance> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x3961b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PVInstance")
}

#[doc(alias = "global constructor keyed to_a_153")]
pub fn stub_0x396230() -> crate::slot::PortedFn {
// IDA 0x396230: __GLOBAL__I_a_153.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x396230, "__GLOBAL__I_a_153")
}

#[doc(alias = "RBX::AnimatableRootJoint::getParentName(void)")]
pub fn stub_0x396554(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AnimatableRootJoint getter.
cell.get()
}

#[doc(alias = "RBX::AnimatableRootJoint::getPartName(void)")]
pub fn stub_0x396564(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AnimatableRootJoint getter.
cell.get()
}

#[doc(alias = "RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&)")]
pub fn stub_0x396574(handle: &crate::slot::InstanceHandle) {
// RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_154")]
pub fn stub_0x39672c() -> crate::slot::PortedFn {
// IDA 0x39672c: __GLOBAL__I_a_154.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x39672c, "__GLOBAL__I_a_154")
}

#[doc(alias = "RBX::Animation::setAssetId(RBX::AnimationId)")]
pub fn stub_0x39699c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Animation setter.
cell.set(value)
}

#[doc(alias = "RBX::Animation::Animation(void)")]
pub fn stub_0x3969d8() -> crate::slot::InstanceHandle {
// RBX::Animation ctor.
crate::slot::InstanceHandle::new("RBX::Animation")
}

#[doc(alias = "RBX::Animation::Animation(void) [0x3969dc]")]
pub fn stub_0x3969dc() -> crate::slot::InstanceHandle {
// RBX::Animation ctor.
crate::slot::InstanceHandle::new("RBX::Animation")
}
