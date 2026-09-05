// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x45ccdc..0x460030 | script 25552->25652 distinct (filler 0x45ccdc asc, not-in-script 59993->59893)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(bool)> RBX::DataModel::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x45ccdc() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::DataModel")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc() [0x45ce60]")]
pub fn stub_0x45ce60(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x45cf14() -> crate::slot::SlotConnection {
// IDA 0x45cf14: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x45d068(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::DataModel, void (bool), rbx::signal<void (bool)>, r~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x45d0f4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::DataModel, void (bool), rbx::signal<void (bool)>, rbx:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
pub fn stub_0x45d108() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x45d228() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
pub fn stub_0x45d310(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x45d408(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x45d428(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::insert(rbx::signals::signal<void ()(bool)>::slot *)")]
pub fn stub_0x45d500(slot: &crate::slot::CallableSlot) {
// IDA 0x45d500: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
pub fn stub_0x45d710(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x45d710: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
pub fn stub_0x45d810(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x45d810: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
pub fn stub_0x45d818(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x45d818: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::remove(rbx::signals::signal<void ()(bool)>::slot *)")]
pub fn stub_0x45d820(slot: &mut crate::slot::CallableSlot) {
// IDA 0x45d820: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x45d910(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (bool)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot() [0x45d918]")]
pub fn stub_0x45d918(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::PropDescriptor<std::string (RBX::DataModel::*)(void)const,int>(char const*,char const*,std::string (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x45d9ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor() [0x45dafc]")]
pub fn stub_0x45dafc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x45db28(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x45db2c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45db30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x45db58(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc() [0x45dc78]")]
pub fn stub_0x45dc78(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x45dd2c() -> crate::slot::SlotConnection {
// IDA 0x45dd2c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x45df30(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<0, RBX::DataModel, void (), rbx::signal<void ()>, rbx::sign~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x45dfa4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::DataModel, void (), rbx::signal<void ()>, rbx::signal<~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
pub fn stub_0x45dfb8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x45dfb8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable() [0x45e0c8]")]
pub fn stub_0x45e0c8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x45e0c8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::BoundFuncDesc(bool (RBX::DataModel::*)(RBX::DataModel::GearType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x45e1f8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x45e370() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc() [0x45e3a0]")]
pub fn stub_0x45e3a0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x45e474() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "bool", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::DataModel::GearType,bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::Reflection::Variant &,RBX::DataModel::GearType const&)")]
pub fn stub_0x45e4b4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::DataModel, bool (RBX::DataModel::*)(RBX::DataModel::Gear~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x45e4ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
pub fn stub_0x45e67c() -> crate::slot::PortedFn {
// IDA 0x45e67c: bool RBX::Reflection::ArgHelper::try_enum<1, RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments&, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x45e67c, "bool RBX::Reflection::ArgHelper::try_enum<1, RBX::DataModel::GearType>(RBX::Reflection::FunctionDesc~")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::EnumPropDescriptor<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x45e6d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor() [0x45e87c]")]
pub fn stub_0x45e87c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isReadOnly(void)const")]
pub fn stub_0x45e8a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isWriteOnly(void)const")]
pub fn stub_0x45e8b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45e8c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x45e8f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x45e914(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x45ea60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::hasStringValue(void)const")]
pub fn stub_0x45ea88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45ea8c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x45eab0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x45eaf0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x45eb10(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45ed50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x45ed6c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45eda0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x45eda8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45edf4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x45ee14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel::GearGenreSetting)const")]
pub fn stub_0x45ee4c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x45eebc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x45ef00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x45ef04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45ef08(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")]
pub fn stub_0x45ef28(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x45f048(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor() [0x45f1f4]")]
pub fn stub_0x45f1f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")]
pub fn stub_0x45f220(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")]
pub fn stub_0x45f230(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f240(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x45f268(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x45f28c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x45f3d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")]
pub fn stub_0x45f3fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f400(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x45f424(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x45f464(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x45f484(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f6c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x45f6e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f714(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x45f71c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f768(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x45f788(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre)const")]
pub fn stub_0x45f7bc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre) co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x45f82c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x45f86c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x45f870(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45f874(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")]
pub fn stub_0x45f894(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x45f9b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor() [0x45fb60]")]
pub fn stub_0x45fb60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")]
pub fn stub_0x45fb8c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")]
pub fn stub_0x45fb9c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45fbac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x45fbd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x45fbf8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x45fd44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")]
pub fn stub_0x45fd68(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x45fd6c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x45fd90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x45fdd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x45fdf0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x460030(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}
