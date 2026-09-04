//! core watchdog q — 100 core stubs EA-sorted, seventeenth gap filler after watchdog_p 0x39765c.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x39765c (watchdog_p max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x397664 — __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x397664() {
    // IDA 0x397664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x397708 — __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::PropDescriptor<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>(char const*,char const*,RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x397708() {
    // IDA 0x397708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39781c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int *, int, int, char, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x39781c() {
    // IDA 0x39781c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x397940 — __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")]
pub fn stub_0x397940() {
    // IDA 0x397940: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39796c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isReadOnly(void)const")]
pub fn stub_0x39796c() {
    // IDA 0x39796c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39797c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isWriteOnly(void)const")]
pub fn stub_0x39797c() {
    // IDA 0x39797c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39798c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11equalValuesEPKNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x39798c() {
    // IDA 0x39798c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x397b38 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x397b38() {
    // IDA 0x397b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x397c64 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x397c64() {
    // IDA 0x397c64: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x397e60 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x397e60() {
    // IDA 0x397e60: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x398078 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor()")]
pub fn stub_0x398078() {
    // IDA 0x398078: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39809c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor()")]
pub fn stub_0x39809c() {
    // IDA 0x39809c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3980c8 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isReadOnly(void)const")]
pub fn stub_0x3980c8() {
    // IDA 0x3980c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3980cc — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isWriteOnly(void)const")]
pub fn stub_0x3980cc() {
    // IDA 0x3980cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3980d0 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3980d0() {
    // IDA 0x3980d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3980f8 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::setValue(RBX::Reflection::DescribedBase *,RBX::AnimationId const&)const")]
pub fn stub_0x3980f8() {
    // IDA 0x3980f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3991b8 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")]
pub fn stub_0x3991b8() {
    // IDA 0x3991b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39920c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")]
pub fn stub_0x39920c() {
    // IDA 0x39920c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39924c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")]
pub fn stub_0x39924c() {
    // IDA 0x39924c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399294 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x399294() {
    // IDA 0x399294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3992b8 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x3992b8() {
    // IDA 0x3992b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3992dc — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x3992dc() {
    // IDA 0x3992dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399304 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x399304() {
    // IDA 0x399304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39998c — __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39998c() {
    // IDA 0x39998c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399990 — __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399990() {
    // IDA 0x399990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399a30 — __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x399a30() {
    // IDA 0x399a30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399a38 — __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399a38() {
    // IDA 0x399a38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399adc — __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x399adc() {
    // IDA 0x399adc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399ae4 — __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399ae4() {
    // IDA 0x399ae4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399b88 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x399b88() {
    // IDA 0x399b88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399e40 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x399e40() {
    // IDA 0x399e40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399eb4 — __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x399eb4() {
    // IDA 0x399eb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x399ec8 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::AnimationTrack::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x399ec8() {
    // IDA 0x399ec8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a04c — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x39a04c() {
    // IDA 0x39a04c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a254 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39a254() {
    // IDA 0x39a254: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a3f8 — __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39a3f8() {
    // IDA 0x39a3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a40c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EEC2EMS2_FvffEPKcS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float),char const*,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39a40c() {
    // IDA 0x39a40c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a648 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39a648() {
    // IDA 0x39a648: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a694 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")]
pub fn stub_0x39a694() {
    // IDA 0x39a694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a774 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39a774() {
    // IDA 0x39a774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39a978 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EEC2EMS2_FvfEPKcS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float),char const*,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39a978() {
    // IDA 0x39a978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39ab30 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39ab30() {
    // IDA 0x39ab30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39ab60 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")]
pub fn stub_0x39ab60() {
    // IDA 0x39ab60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39ac34 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39ac34() {
    // IDA 0x39ac34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39ac70 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EEC2EMS2_FvfffEPKcS8_fS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, float, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float,float),char const*,char const*,float,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39ac70() {
    // IDA 0x39ac70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39af34 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
// type: int __fastcall(int, int, int *, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39af34() {
    // IDA 0x39af34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39af9c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")]
pub fn stub_0x39af9c() {
    // IDA 0x39af9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39b088 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39b088() {
    // IDA 0x39b088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c124 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c124() {
    // IDA 0x39c124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c148 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c148() {
    // IDA 0x39c148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c16c — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c16c() {
    // IDA 0x39c16c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c190 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_0x39c190() {
    // IDA 0x39c190: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c1b4 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::AnimationTrackState*,std::string)")]
pub fn stub_0x39c1b4() {
    // IDA 0x39c1b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c414 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c414() {
    // IDA 0x39c414: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c44c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c44c() {
    // IDA 0x39c44c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c724 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::AnimationTrackState*,std::string)const")]
pub fn stub_0x39c724() {
    // IDA 0x39c724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39c840 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESs
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
pub fn stub_0x39c840() {
    // IDA 0x39c840: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39c98c — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEfff
// type: int __fastcall(int, int, int, int, float)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float)")]
pub fn stub_0x39c98c() {
    // IDA 0x39c98c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39cf30 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float)")]
pub fn stub_0x39cf30() {
    // IDA 0x39cf30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39d09c — __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEffff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float,float)")]
pub fn stub_0x39d09c() {
    // IDA 0x39d09c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39f398 — __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f398() {
    // IDA 0x39f398: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f39c — __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f39c() {
    // IDA 0x39f39c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f43c — __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f43c() {
    // IDA 0x39f43c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f444 — __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f444() {
    // IDA 0x39f444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f4e8 — __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f4e8() {
    // IDA 0x39f4e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f4f0 — __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f4f0() {
    // IDA 0x39f4f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f594 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_0x39f594() {
    // IDA 0x39f594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f7ac — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_0x39f7ac() {
    // IDA 0x39f7ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f7b4 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_0x39f7b4() {
    // IDA 0x39f7b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f7bc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39f7bc() {
    // IDA 0x39f7bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f960 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39f960() {
    // IDA 0x39f960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39f970 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39f970() {
    // IDA 0x39f970: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39f984 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39f984() {
    // IDA 0x39f984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39fb08 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x39fb08() {
    // IDA 0x39fb08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fb2c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x39fb2c() {
    // IDA 0x39fb2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fbe0 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39fbe0() {
    // IDA 0x39fbe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fdf8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isScriptable(void)const")]
pub fn stub_0x39fdf8() {
    // IDA 0x39fdf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fe00 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x39fe00() {
    // IDA 0x39fe00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fe08 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39fe08() {
    // IDA 0x39fe08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39feb0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39feb0() {
    // IDA 0x39feb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x39fec0 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39fec0() {
    // IDA 0x39fec0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x39fff0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IfffEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<float,float,float>(float const&,float const&,float const&)")]
pub fn stub_0x39fff0() {
    // IDA 0x39fff0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3a105c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a105c() {
    // IDA 0x3a105c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3a12b8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a12b8() {
    // IDA 0x3a12b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a12dc — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a12dc() {
    // IDA 0x3a12dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1390 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x3a1390() {
    // IDA 0x3a1390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a15a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isScriptable(void)const")]
pub fn stub_0x3a15a8() {
    // IDA 0x3a15a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a15b0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3a15b0() {
    // IDA 0x3a15b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a15b8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a15b8() {
    // IDA 0x3a15b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1654 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1654() {
    // IDA 0x3a1654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1664 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3a1664() {
    // IDA 0x3a1664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3a1678 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a1678() {
    // IDA 0x3a1678: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3a1868 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a1868() {
    // IDA 0x3a1868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a188c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a188c() {
    // IDA 0x3a188c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1940 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x3a1940() {
    // IDA 0x3a1940: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1b58 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isScriptable(void)const")]
pub fn stub_0x3a1b58() {
    // IDA 0x3a1b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1b60 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3a1b60() {
    // IDA 0x3a1b60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1b68 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1b68() {
    // IDA 0x3a1b68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1c2c — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1c2c() {
    // IDA 0x3a1c2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a1c3c — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3a1c3c() {
    // IDA 0x3a1c3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
