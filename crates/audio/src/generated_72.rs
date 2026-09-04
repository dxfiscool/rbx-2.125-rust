//! audio generated_72 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x6b94fc..0x6c61e0 EA-sorted asc filler after 0x6a632c, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x6b94fc — __ZN3RBX10Reflection9EventDescINS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::StringValue::*,char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_6b94fc() -> ! {
    todo!("0x6b94fc RBX::Reflection::EventDesc<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::StringValue::*,char const*,char const*,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6b9680 — __ZN3RBX10Reflection9EventDescINS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::~EventDesc()")]
pub fn stub_6b9680() {
    // IDA 0x6b9680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6b9734 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_6b9734() -> ! {
    todo!("0x6b9734 RBX::Reflection::EventDescImpl<1,RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x6b9888 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_6b9888() -> ! {
    todo!("0x6b9888 RBX::Reflection::EventDescImpl<1,RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x6b9a2c — __ZNK3RBX10Reflection13EventDescBaseINS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_6b9a2c() -> ! {
    todo!("0x6b9a2c RBX::Reflection::EventDescBase<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x6b9a40 — __ZN3RBX10Reflection9EventDescINS_11StringValueEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::StringValue::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_6b9a40() -> ! {
    todo!("0x6b9a40 RBX::Reflection::EventDesc<RBX::StringValue,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::StringValue::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::StringValue::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6b9bc4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_11StringValueEEEPKcS7_MT_SsMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::StringValue>(char const*,char const*,std::string  RBX::StringValue::*,void (RBX::StringValue::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6b9bc4() -> ! {
    todo!("0x6b9bc4 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::StringValue>(char const*,char const*,std::string  RBX::StringValue::*,void (RBX::StringValue::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6b9d58 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11StringValueEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::isReadOnly(void)const")]
pub fn stub_6b9d58() -> ! {
    todo!("0x6b9d58 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::isReadOnly(void)const")
}

// 0x6b9d5c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11StringValueEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::isWriteOnly(void)const")]
pub fn stub_6b9d5c() -> ! {
    todo!("0x6b9d5c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::isWriteOnly(void)const")
}

// 0x6b9d60 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11StringValueEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6b9d60() -> ! {
    todo!("0x6b9d60 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6b9d78 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11StringValueEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_6b9d78() -> ! {
    todo!("0x6b9d78 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::StringValue>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x6bec2c — __ZN3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6bec2c() {
    // IDA 0x6bec2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bec40 — __ZN3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6bec40() {
    // IDA 0x6bec40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6becf4 — __ZThn132_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6becf4() {
    // IDA 0x6becf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bed08 — __ZThn132_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6bed08() {
    // IDA 0x6bed08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bedbc — __ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6bedbc() {
    // IDA 0x6bedbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bedd0 — __ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6bedd0() {
    // IDA 0x6bedd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bee80 — __ZThn132_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6bee80() {
    // IDA 0x6bee80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bee94 — __ZThn132_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6bee94() {
    // IDA 0x6bee94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bef48 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_6bef48() {
    // IDA 0x6bef48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bef5c — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_6bef5c() {
    // IDA 0x6bef5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bf00c — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_6bf00c() {
    // IDA 0x6bf00c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bf020 — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_6bf020() {
    // IDA 0x6bf020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bf288 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_6bf288() {
    // IDA 0x6bf288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bf804 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_6bf804() {
    // IDA 0x6bf804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6bf8a0 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_6bf8a0() -> ! {
    todo!("0x6bf8a0 __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x6bf928 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator6createEv")]
pub fn stub_6bf928() -> ! {
    todo!("0x6bf928 __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator6createEv")
}

// 0x6bfbe8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11VehicleSeatES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VehicleSeat,RBX::VehicleSeat>(rbx_core::SharedPtr<RBX::VehicleSeat> const*,RBX::VehicleSeat *)const")]
pub fn stub_6bfbe8() {
    // IDA 0x6bfbe8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x6bfe20 — __ZN3RBX4Name13callDoDeclareILZNS_12sVehicleSeatEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sVehicleSeatEEEEvv")]
pub fn stub_6bfe20() -> ! {
    todo!("0x6bfe20 __ZN3RBX4Name13callDoDeclareILZNS_12sVehicleSeatEEEEvv")
}

// 0x6bfe24 — __ZN3RBX4Name9doDeclareILZNS_12sVehicleSeatEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sVehicleSeatEEEERKS0_v")]
pub fn stub_6bfe24() -> ! {
    todo!("0x6bfe24 __ZN3RBX4Name9doDeclareILZNS_12sVehicleSeatEEEERKS0_v")
}

// 0x6bff04 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_6bff04() -> ! {
    todo!("0x6bff04 __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorC2Ev")
}

// 0x6c0148 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_6c0148() -> ! {
    todo!("0x6c0148 __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE17static_getCreatorEv")
}

// 0x6c01bc — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")]
pub fn stub_6c01bc() -> ! {
    todo!("0x6c01bc void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")
}

// 0x6c04ac — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")]
pub fn stub_6c04ac() -> ! {
    todo!("0x6c04ac void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")
}

// 0x6c0dcc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4WeldES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Weld,RBX::Weld>(rbx_core::SharedPtr<RBX::Weld> const*,RBX::Weld *)const")]
pub fn stub_6c0dcc() {
    // IDA 0x6c0dcc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x6c13fc — __ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::Weak<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_6c13fc() -> ! {
    todo!("0x6c13fc boost::shared_ptr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(boost::weak_ptr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")
}

// 0x6c1478 — __ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_6c1478() {
    // IDA 0x6c1478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c148c — __ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_6c148c() {
    // IDA 0x6c148c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c14a0 — __ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_6c14a0() {
    // IDA 0x6c14a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c14a8 — __ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_6c14a8() {
    // IDA 0x6c14a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1bac — __ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_6c1bac() -> ! {
    todo!("0x6c1bac __ZN3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")
}

// 0x6c1dc8 — __ZThn32_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6c1dc8() {
    // IDA 0x6c1dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1ddc — __ZThn32_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6c1ddc() {
    // IDA 0x6c1ddc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1e90 — __ZThn36_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6c1e90() {
    // IDA 0x6c1e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1ea4 — __ZThn36_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6c1ea4() {
    // IDA 0x6c1ea4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1f58 — __ZThn32_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6c1f58() {
    // IDA 0x6c1f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c1f6c — __ZThn32_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6c1f6c() {
    // IDA 0x6c1f6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c2020 — __ZThn36_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6c2020() {
    // IDA 0x6c2020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c2034 — __ZThn36_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11VehicleSeatELZNS_12sVehicleSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6c2034() {
    // IDA 0x6c2034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c2324 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::PropDescriptor<int (RBX::VehicleSeat::*)(void)const,int>(char const*,char const*,int (RBX::VehicleSeat::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6c2324() -> ! {
    todo!("0x6c2324 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::PropDescriptor<int (RBX::VehicleSeat::*)(void)const,int>(char const*,char const*,int (RBX::VehicleSeat::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6c2430 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::~PropDescriptor()")]
pub fn stub_6c2430() {
    // IDA 0x6c2430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c245c — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::isReadOnly(void)const")]
pub fn stub_6c245c() -> ! {
    todo!("0x6c245c RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::isReadOnly(void)const")
}

// 0x6c2460 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_6c2460() -> ! {
    todo!("0x6c2460 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::isWriteOnly(void)const")
}

// 0x6c2464 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6c2464() -> ! {
    todo!("0x6c2464 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6c2484 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_6c2484() -> ! {
    todo!("0x6c2484 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetImpl<int (RBX::VehicleSeat::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x6c25a4 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::PropDescriptor<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>(char const*,char const*,bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6c25a4() -> ! {
    todo!("0x6c25a4 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::PropDescriptor<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>(char const*,char const*,bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6c26b8 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::~PropDescriptor()")]
pub fn stub_6c26b8() {
    // IDA 0x6c26b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c26e4 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::isReadOnly(void)const")]
pub fn stub_6c26e4() -> ! {
    todo!("0x6c26e4 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::isReadOnly(void)const")
}

// 0x6c26e8 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_6c26e8() -> ! {
    todo!("0x6c26e8 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::isWriteOnly(void)const")
}

// 0x6c26ec — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6c26ec() -> ! {
    todo!("0x6c26ec RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6c2710 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_6c2710() -> ! {
    todo!("0x6c2710 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::GetSetImpl<bool (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x6c2734 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::PropDescriptor<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>(char const*,char const*,float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6c2734() -> ! {
    todo!("0x6c2734 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::PropDescriptor<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>(char const*,char const*,float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6c2848 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::~PropDescriptor()")]
pub fn stub_6c2848() {
    // IDA 0x6c2848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c2874 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::isReadOnly(void)const")]
pub fn stub_6c2874() -> ! {
    todo!("0x6c2874 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::isReadOnly(void)const")
}

// 0x6c2878 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::isWriteOnly(void)const")]
pub fn stub_6c2878() -> ! {
    todo!("0x6c2878 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::isWriteOnly(void)const")
}

// 0x6c287c — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6c287c() -> ! {
    todo!("0x6c287c RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6c289c — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_6c289c() -> ! {
    todo!("0x6c289c RBX::Reflection::PropDescriptor<RBX::VehicleSeat,float>::GetSetImpl<float (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x6c28c0 — __ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::PropDescriptor<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>(char const*,char const*,int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6c28c0() -> ! {
    todo!("0x6c28c0 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::PropDescriptor<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>(char const*,char const*,int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6c29d4 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::isReadOnly(void)const")]
pub fn stub_6c29d4() -> ! {
    todo!("0x6c29d4 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::isReadOnly(void)const")
}

// 0x6c29d8 — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::isWriteOnly(void)const")]
pub fn stub_6c29d8() -> ! {
    todo!("0x6c29d8 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::isWriteOnly(void)const")
}

// 0x6c29dc — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6c29dc() -> ! {
    todo!("0x6c29dc RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6c29fc — __ZNK3RBX10Reflection14PropDescriptorINS_11VehicleSeatEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_6c29fc() -> ! {
    todo!("0x6c29fc RBX::Reflection::PropDescriptor<RBX::VehicleSeat,int>::GetSetImpl<int (RBX::VehicleSeat::*)(void)const,void (RBX::VehicleSeat::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x6c2cc0 — __GLOBAL__I_a_282
#[doc(alias = "global constructor keyed to_a_282")]
pub fn stub_6c2cc0() -> ! {
    todo!("0x6c2cc0 global constructor keyed to_a_282")
}

// 0x6c4978 — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_6c4978() {
    // IDA 0x6c4978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c499c — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFSsvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,std::string ()(void),0>::~BoundFuncDesc()")]
pub fn stub_6c499c() {
    // IDA 0x6c499c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c49c0 — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_6c49c0() {
    // IDA 0x6c49c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c4ab4 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UIEvent const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>> const&)")]
pub fn stub_6c4ab4() -> ! {
    todo!("0x6c4ab4 rbx::signals::connection rbx::signals::signal<void ()(RBX::UIEvent const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>> const&)")
}

// 0x6c4d20 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv")]
pub fn stub_6c4d20() -> ! {
    todo!("0x6c4d20 __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv")
}

// 0x6c4ff0 — __ZThn32_NK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv")]
pub fn stub_6c4ff0() {
    // IDA 0x6c4ff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c52c0 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD1Ev")]
pub fn stub_6c52c0() {
    // IDA 0x6c52c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c52c4 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD2Ev")]
pub fn stub_6c52c4() {
    // IDA 0x6c52c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c5360 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator12getClassNameEv")]
pub fn stub_6c5360() -> ! {
    todo!("0x6c5360 __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator12getClassNameEv")
}

// 0x6c53e8 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator6createEv")]
pub fn stub_6c53e8() -> ! {
    todo!("0x6c53e8 __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator6createEv")
}

// 0x6c56a8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11VirtualUserES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VirtualUser,RBX::VirtualUser>(rbx_core::SharedPtr<RBX::VirtualUser> const*,RBX::VirtualUser *)const")]
pub fn stub_6c56a8() {
    // IDA 0x6c56a8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x6c58dc — __ZN3RBX4Name13callDoDeclareILZNS_12sVirtualUserEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sVirtualUserEEEEvv")]
pub fn stub_6c58dc() -> ! {
    todo!("0x6c58dc __ZN3RBX4Name13callDoDeclareILZNS_12sVirtualUserEEEEvv")
}

// 0x6c58e0 — __ZN3RBX4Name9doDeclareILZNS_12sVirtualUserEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sVirtualUserEEEERKS0_v")]
pub fn stub_6c58e0() -> ! {
    todo!("0x6c58e0 __ZN3RBX4Name9doDeclareILZNS_12sVirtualUserEEEERKS0_v")
}

// 0x6c59c0 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorC2Ev")]
pub fn stub_6c59c0() -> ! {
    todo!("0x6c59c0 __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorC2Ev")
}

// 0x6c5c04 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E17static_getCreatorEv")]
pub fn stub_6c5c04() -> ! {
    todo!("0x6c5c04 __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E17static_getCreatorEv")
}

// 0x6c5c78 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::insert(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")]
pub fn stub_6c5c78() -> ! {
    todo!("0x6c5c78 rbx::signals::signal<void ()(RBX::UIEvent const&)>::insert(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")
}

// 0x6c5e84 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)")]
pub fn stub_6c5e84() -> ! {
    todo!("0x6c5e84 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)")
}

// 0x6c5ea8 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_6c5ea8() {
    // IDA 0x6c5ea8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c5ed4 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_6c5ed4() {
    // IDA 0x6c5ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c5fa8 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::disconnect(void)")]
pub fn stub_6c5fa8() -> ! {
    todo!("0x6c5fa8 rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::disconnect(void)")
}

// 0x6c60b8 — __ZNK3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::connected(void)const")]
pub fn stub_6c60b8() -> ! {
    todo!("0x6c60b8 rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::connected(void)const")
}

// 0x6c60c4 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)")]
pub fn stub_6c60c4() -> ! {
    todo!("0x6c60c4 rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)")
}

// 0x6c60cc — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)")]
pub fn stub_6c60cc() {
    // IDA 0x6c60cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6c60d4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>::operator()<RBX::UIEvent>(RBX::UIEvent const&)")]
pub fn stub_6c60d4() -> ! {
    todo!("0x6c60d4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>::operator()<RBX::UIEvent>(RBX::UIEvent const&)")
}

// 0x6c60ec — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::remove(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")]
pub fn stub_6c60ec() -> ! {
    todo!("0x6c60ec rbx::signals::signal<void ()(RBX::UIEvent const&)>::remove(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)")
}

// 0x6c61dc — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_6c61dc() -> ! {
    todo!("0x6c61dc rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_init_mutex(void)")
}

// 0x6c61e0 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_6c61e0() -> ! {
    todo!("0x6c61e0 rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_do_get_mutex(void)")
}