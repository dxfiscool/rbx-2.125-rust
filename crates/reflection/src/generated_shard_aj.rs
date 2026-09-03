// Auto-generated shard AJ — next 100 RBX::Reflection stubs — EA-sorted after 0x897a00
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 strict, 10791 prior -> 10891 total, EA 0x897aac..0x8c1588, rbx_core::SharedPtr not boost)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") with rbx_core::SharedPtr
#![allow(unused_imports)]
use rbx_core::SharedPtr;

// 0x897aac — __ZN3RBX10Reflection5TTypeINS_12Region3int16EED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Region3int16>::~TType()")]
pub fn stub_897aac() {
    // IDA 0x897aac: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x89f0a8 — __ZN3RBX16BindableFunction6invokeEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS1_8functionIFvS6_EEENS7_IFvSsEEE
// was: RBX::BindableFunction::invoke(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "RBX::BindableFunction::invoke(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX16BindableFunction6invokeEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS1_8functionIFvS6_EEENS7_IFvSsEEE")]
pub fn stub_89f0a8() -> ! {
    todo!("0x89f0a8 RBX::BindableFunction::invoke(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x89f768 — __ZN3RBX13BindableEvent4fireEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// was: RBX::BindableEvent::fire(boost::shared_ptr<RBX::Reflection::Tuple const>)
#[doc(alias = "RBX::BindableEvent::fire(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN3RBX13BindableEvent4fireEN5boost10shared_ptrIKNS_10Reflection5TupleEEE")]
pub fn stub_89f768() -> ! {
    todo!("0x89f768 RBX::BindableEvent::fire(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")
}

// 0x89f9a4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED1Ev")]
pub fn stub_89f9a4() {
    // IDA 0x89f9a4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x89fa98 — __ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED1Ev")]
pub fn stub_89fa98() {
    // IDA 0x89fa98: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x89fb8c — __ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_ED1Ev")]
pub fn stub_89fb8c() {
    // IDA 0x89fb8c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8a0f84 — __ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_8a0f84() -> ! {
    todo!("0x8a0f84 RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8a1108 — __ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_ED0Ev")]
pub fn stub_8a1108() {
    // IDA 0x8a1108: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8a11bc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_8a11bc() -> ! {
    todo!("0x8a11bc RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8a1310 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
pub fn stub_8a1310() -> ! {
    todo!("0x8a1310 RBX::Reflection::EventDescImpl<1,RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8a1470 — __ZNK3RBX10Reflection13EventDescBaseINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx6signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_8a1470() -> ! {
    todo!("0x8a1470 RBX::Reflection::EventDescBase<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8a1484 — __ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EEC2EMS2_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::BindableEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EEC2EMS2_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_8a1484() -> ! {
    todo!("0x8a1484 RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8a1600 — __ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_8a1600() -> ! {
    todo!("0x8a1600 RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x8a1630 — __ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED0Ev")]
pub fn stub_8a1630() {
    // IDA 0x8a1630: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8a1738 — __ZNK3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13BindableEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_8a1738() -> ! {
    todo!("0x8a1738 RBX::Reflection::BoundFuncDesc<RBX::BindableEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8a1818 — __ZN3RBX10Reflection11Call1HelperINS_13BindableEventEMS2_FvN5boost10shared_ptrIKNS0_5TupleEEEES7_vE4callEPS2_S9_RNS0_7VariantERKS7_
// was: RBX::Reflection::Call1Helper<RBX::BindableEvent,void (RBX::BindableEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,void>::call(RBX::BindableEvent*,void (RBX::BindableEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Reflection::Tuple const> const&)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::BindableEvent,void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::BindableEvent*,void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13BindableEventEMS2_FvN5boost10shared_ptrIKNS0_5TupleEEEES7_vE4callEPS2_S9_RNS0_7VariantERKS7_")]
pub fn stub_8a1818() -> ! {
    todo!("0x8a1818 RBX::Reflection::Call1Helper<RBX::BindableEvent,void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::BindableEvent*,void (RBX::BindableEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")
}

// 0x8a1900 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EEC2INS_16BindableFunctionEEEPKcMT_NS2_8functionIS7_EESC_MSD_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::BindableFunction>(char const*,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::BindableFunction::*,char const*,void (RBX::BindableFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::BindableFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableFunction::*,char const*,void (RBX::BindableFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EEC2INS_16BindableFunctionEEEPKcMT_NS2_8functionIS7_EESC_MSD_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_8a1900() -> ! {
    todo!("0x8a1900 RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::BindableFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::BindableFunction::*,char const*,void (RBX::BindableFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8a1a88 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEED1Ev
// was: RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::~Setter()
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::~Setter()")]
#[doc(alias = "__ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEED1Ev")]
pub fn stub_8a1a88() {
    // IDA 0x8a1a88: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8a1a8c — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEED0Ev
// was: RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::~Setter()
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::~Setter()")]
#[doc(alias = "__ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEED0Ev")]
pub fn stub_8a1a8c() {
    // IDA 0x8a1a8c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8a1a90 — __ZNK3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEE11setCallbackEPNS0_13DescribedBaseERKNS2_8functionIS7_EE
// was: RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::setCallback(RBX::Reflection::DescribedBase *,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)const
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::setCallback(RBX::Reflection::DescribedBase *,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_16BindableFunctionEE11setCallbackEPNS0_13DescribedBaseERKNS2_8functionIS7_EE")]
pub fn stub_8a1a90() -> ! {
    todo!("0x8a1a90 RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::BindableFunction>::setCallback(RBX::Reflection::DescribedBase *,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)const")
}

// 0x8a1acc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_8a1acc() -> ! {
    todo!("0x8a1acc RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8a1c48 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_8a1c48() -> ! {
    todo!("0x8a1c48 RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x8a1c78 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev")]
pub fn stub_8a1c78() {
    // IDA 0x8a1c78: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8a1d80 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE")]
pub fn stub_8a1d80() -> ! {
    todo!("0x8a1d80 RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x8ad0a4 — __ZN3RBX10Reflection13BoundFuncDescINS_15GamePassServiceEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_8ad0a4() {
    // IDA 0x8ad0a4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8ad0e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev")]
pub fn stub_8ad0e4() {
    // IDA 0x8ad0e4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8ada28 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_8ada28() -> ! {
    todo!("0x8ada28 RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8adc10 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")]
pub fn stub_8adc10() -> ! {
    todo!("0x8adc10 RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8adc5c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev")]
pub fn stub_8adc5c() {
    // IDA 0x8adc5c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8add88 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE")]
pub fn stub_8add88() -> ! {
    todo!("0x8add88 RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x8adf60 — __ZN3RBX10Reflection13BoundFuncDescINS_15GamePassServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::BoundFuncDesc(void (RBX::GamePassService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8adf60() -> ! {
    todo!("0x8adf60 RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::BoundFuncDesc(void (RBX::GamePassService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8ae0d8 — __ZN3RBX10Reflection13BoundFuncDescINS_15GamePassServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_8ae0d8() -> ! {
    todo!("0x8ae0d8 RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x8ae108 — __ZN3RBX10Reflection13BoundFuncDescINS_15GamePassServiceEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_8ae108() {
    // IDA 0x8ae108: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8ae1d4 — __ZNK3RBX10Reflection13BoundFuncDescINS_15GamePassServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_8ae1d4() -> ! {
    todo!("0x8ae1d4 RBX::Reflection::BoundFuncDesc<RBX::GamePassService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8ae310 — __ZN3RBX10Reflection11Call1HelperINS_15GamePassServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GamePassService,void (RBX::GamePassService::*)(std::string),std::string,void>::call(RBX::GamePassService*,void (RBX::GamePassService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_8ae310() -> ! {
    todo!("0x8ae310 RBX::Reflection::Call1Helper<RBX::GamePassService,void (RBX::GamePassService::*)(std::string),std::string,void>::call(RBX::GamePassService*,void (RBX::GamePassService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x8ae6ec — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::EnumDesc(void)")]
pub fn stub_8ae6ec() -> ! {
    todo!("0x8ae6ec RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::EnumDesc(void)")
}

// 0x8ae6f0 — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::EnumDesc(void)")]
pub fn stub_8ae6f0() -> ! {
    todo!("0x8ae6f0 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::EnumDesc(void)")
}

// 0x8b1c10 — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::addPair(RBX::UserInputService::SwipeDirection,char const*)")]
pub fn stub_8b1c10() -> ! {
    todo!("0x8b1c10 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::addPair(RBX::UserInputService::SwipeDirection,char const*)")
}

// 0x8b1f70 — __ZN3RBX10Reflection14PropDescriptorINS_16UserInputServiceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::UserInputService,bool>::~PropDescriptor()")]
pub fn stub_8b1f70() {
    // IDA 0x8b1f70: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b1f94 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8b1f94() {
    // IDA 0x8b1f94: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b1fb8 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8b1fb8() {
    // IDA 0x8b1fb8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b1fdc — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8b1fdc() {
    // IDA 0x8b1fdc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b2000 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8b2000() {
    // IDA 0x8b2000: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b2024 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8b2024() {
    // IDA 0x8b2024: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b2f24 — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::~EnumDesc()")]
pub fn stub_8b2f24() {
    // IDA 0x8b2f24: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8b2f28 — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::~EnumDesc()")]
pub fn stub_8b2f28() {
    // IDA 0x8b2f28: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8b2fc8 — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::lookup(char const*)const")]
pub fn stub_8b2fc8() -> ! {
    todo!("0x8b2fc8 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::lookup(char const*)const")
}

// 0x8b2ff8 — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_8b2ff8() -> ! {
    todo!("0x8b2ff8 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x8b3018 — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_8b3018() -> ! {
    todo!("0x8b3018 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x8b304c — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToString(unsigned long,std::string &)const")]
pub fn stub_8b304c() -> ! {
    todo!("0x8b304c RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToString(unsigned long,std::string &)const")
}

// 0x8b3190 — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToString(RBX::UserInputService::SwipeDirection const&)const")]
pub fn stub_8b3190() -> ! {
    todo!("0x8b3190 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToString(RBX::UserInputService::SwipeDirection const&)const")
}

// 0x8b33fc — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToItem(RBX::UserInputService::SwipeDirection const&)const")]
pub fn stub_8b33fc() -> ! {
    todo!("0x8b33fc RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToItem(RBX::UserInputService::SwipeDirection const&)const")
}

// 0x8b35b8 — __ZNK3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToValue(RBX::Name const&,RBX::UserInputService::SwipeDirection&)const")]
pub fn stub_8b35b8() -> ! {
    todo!("0x8b35b8 RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::convertToValue(RBX::Name const&,RBX::UserInputService::SwipeDirection&)const")
}

// 0x8b3634 — __ZN3RBX10Reflection8EnumDescINS_16UserInputService14SwipeDirectionEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection>::~EnumDesc()")]
pub fn stub_8b3634() {
    // IDA 0x8b3634: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x8b3bd0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ClickDetectorES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ClickDetector,RBX::ClickDetector>(boost::shared_ptr<RBX::ClickDetector> const*,RBX::ClickDetector *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ClickDetector,RBX::ClickDetector>(rbx_core::SharedPtr<RBX::ClickDetector> const*,RBX::ClickDetector *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ClickDetectorES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_8b3bd0() -> ! {
    todo!("0x8b3bd0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ClickDetector,RBX::ClickDetector>(rbx_core::SharedPtr<RBX::ClickDetector> const*,RBX::ClickDetector *)const")
}

// 0x8b6a30 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16UserInputServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::UserInputService,RBX::UserInputService>(boost::shared_ptr<RBX::UserInputService> const*,RBX::UserInputService *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::UserInputService,RBX::UserInputService>(rbx_core::SharedPtr<RBX::UserInputService> const*,RBX::UserInputService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16UserInputServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_8b6a30() -> ! {
    todo!("0x8b6a30 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::UserInputService,RBX::UserInputService>(rbx_core::SharedPtr<RBX::UserInputService> const*,RBX::UserInputService *)const")
}

// 0x8bbf98 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8bbf98() -> ! {
    todo!("0x8bbf98 RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8bc11c — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8bc11c() {
    // IDA 0x8bc11c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8bc1d0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_8bc1d0() -> ! {
    todo!("0x8bc1d0 RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8bc324 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8bc324() -> ! {
    todo!("0x8bc324 RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8bc3c4 — __ZNK3RBX10Reflection13EventDescBaseINS_16UserInputServiceEFvNS_11InputObjectEEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8bc3c4() -> ! {
    todo!("0x8bc3c4 RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(RBX::InputObject),rbx::signal<void ()(RBX::InputObject)>,rbx::signal<void ()(RBX::InputObject)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8bca58 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_11InputObjectENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::InputObject const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::InputObject const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_11InputObjectENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_")]
pub fn stub_8bca58() -> ! {
    todo!("0x8bca58 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::InputObject const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x8bcb74 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_11InputObjectEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::InputObject>(RBX::InputObject const&)")]
pub fn stub_8bcb74() -> ! {
    todo!("0x8bcb74 void RBX::Reflection::GenericSlotWrapper::execute1<RBX::InputObject>(RBX::InputObject const&)")
}

// 0x8bcf28 — __ZN5boost9function1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
#[doc(alias = "void boost::function1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_8bcf28() -> ! {
    todo!("0x8bcf28 void boost::function1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x8bd020 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_8bd020() -> ! {
    todo!("0x8bd020 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8bd03c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::InputObject>::invoke(boost::detail::function::function_buffer &,RBX::InputObject)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::InputObject>::invoke(boost::detail::function::function_buffer &,RBX::InputObject)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
pub fn stub_8bd03c() -> ! {
    todo!("0x8bd03c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::InputObject>::invoke(boost::detail::function::function_buffer &,RBX::InputObject)")
}

// 0x8bd044 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_8bd044() -> ! {
    todo!("0x8bd044 bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x8bd12c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_8bd12c() -> ! {
    todo!("0x8bd12c bool boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x8bd210 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX11InputObjectEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_8bd210() -> ! {
    todo!("0x8bd210 void boost::detail::function::basic_vtable1<void,RBX::InputObject>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x8bd2e4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_11InputObjectEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::InputObject>(RBX::InputObject &)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::InputObject>(RBX::InputObject &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_11InputObjectEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_")]
pub fn stub_8bd2e4() -> ! {
    todo!("0x8bd2e4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::InputObject>(RBX::InputObject &)")
}

// 0x8bd2fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_11InputObjectEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_8bd2fc() -> ! {
    todo!("0x8bd2fc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::InputObject const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x8be270 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8be270() -> ! {
    todo!("0x8be270 RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8be3f4 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8be3f4() {
    // IDA 0x8be3f4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8be4a8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_8be4a8() -> ! {
    todo!("0x8be4a8 RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8be5fc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8be5fc() -> ! {
    todo!("0x8be5fc RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8be688 — __ZNK3RBX10Reflection13EventDescBaseINS_16UserInputServiceEFvNS2_14SwipeDirectionEEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8be688() -> ! {
    todo!("0x8be688 RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(RBX::UserInputService::SwipeDirection),rbx::signal<void ()(RBX::UserInputService::SwipeDirection)>,rbx::signal<void ()(RBX::UserInputService::SwipeDirection)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8bec00 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_16UserInputService14SwipeDirectionENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::UserInputService::SwipeDirection const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::UserInputService::SwipeDirection const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_16UserInputService14SwipeDirectionENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_")]
pub fn stub_8bec00() -> ! {
    todo!("0x8bec00 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::UserInputService::SwipeDirection const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x8bed1c — __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_16UserInputService14SwipeDirectionEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection const&)")]
pub fn stub_8bed1c() -> ! {
    todo!("0x8bed1c void RBX::Reflection::GenericSlotWrapper::execute1<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection const&)")
}

// 0x8bf058 — __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
#[doc(alias = "void boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_8bf058() -> ! {
    todo!("0x8bf058 void boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x8bf150 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")]
pub fn stub_8bf150() -> ! {
    todo!("0x8bf150 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8bf16c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::UserInputService::SwipeDirection>::invoke(boost::detail::function::function_buffer &,RBX::UserInputService::SwipeDirection)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::UserInputService::SwipeDirection>::invoke(boost::detail::function::function_buffer &,RBX::UserInputService::SwipeDirection)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_8bf16c() -> ! {
    todo!("0x8bf16c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::UserInputService::SwipeDirection>::invoke(boost::detail::function::function_buffer &,RBX::UserInputService::SwipeDirection)")
}

// 0x8bf180 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_8bf180() -> ! {
    todo!("0x8bf180 bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x8bf268 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_8bf268() -> ! {
    todo!("0x8bf268 bool boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x8bf34c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX16UserInputService14SwipeDirectionEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_8bf34c() -> ! {
    todo!("0x8bf34c void boost::detail::function::basic_vtable1<void,RBX::UserInputService::SwipeDirection>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x8bf420 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_16UserInputService14SwipeDirectionEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection &)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_16UserInputService14SwipeDirectionEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_")]
pub fn stub_8bf420() -> ! {
    todo!("0x8bf420 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection &)")
}

// 0x8bf438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_16UserInputService14SwipeDirectionEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_8bf438() -> ! {
    todo!("0x8bf438 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UserInputService::SwipeDirection const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x8c0334 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(float,float)> RBX::UserInputService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8c0334() -> ! {
    todo!("0x8c0334 RBX::Reflection::EventDesc<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(float,float)> RBX::UserInputService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8c0524 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::~EventDesc()")]
pub fn stub_8c0524() {
    // IDA 0x8c0524: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8c05d8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_8c05d8() -> ! {
    todo!("0x8c05d8 RBX::Reflection::EventDescImpl<2,RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8c072c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8c072c() -> ! {
    todo!("0x8c072c RBX::Reflection::EventDescImpl<2,RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8c07c8 — __ZNK3RBX10Reflection13EventDescBaseINS_16UserInputServiceEFvffEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8c07c8() -> ! {
    todo!("0x8c07c8 RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(float,float),rbx::signal<void ()(float,float)>,rbx::signal<void ()(float,float)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8c0d54 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_")]
pub fn stub_8c0d54() -> ! {
    todo!("0x8c0d54 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x8c0e70 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IffEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<float,float>(float const&,float const&)")]
pub fn stub_8c0e70() -> ! {
    todo!("0x8c0e70 void RBX::Reflection::GenericSlotWrapper::execute2<float,float>(float const&,float const&)")
}

// 0x8c1008 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_
// was: boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_")]
pub fn stub_8c1008() -> ! {
    todo!("0x8c1008 boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")
}

// 0x8c10d8 — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_")]
pub fn stub_8c10d8() -> ! {
    todo!("0x8c10d8 boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")
}

// 0x8c1374 — __ZN5boost9function2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
#[doc(alias = "void boost::function2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_")]
pub fn stub_8c1374() -> ! {
    todo!("0x8c1374 void boost::function2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x8c146c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_8c146c() -> ! {
    todo!("0x8c146c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8c1488 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvffE6invokeERNS1_15function_bufferEff
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,float,float>::invoke(boost::detail::function::function_buffer &,float,float)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,float,float>::invoke(boost::detail::function::function_buffer &,float,float)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvffE6invokeERNS1_15function_bufferEff")]
pub fn stub_8c1488() -> ! {
    todo!("0x8c1488 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,float,float>::invoke(boost::detail::function::function_buffer &,float,float)")
}

// 0x8c14a0 — __ZNK5boost6detail8function13basic_vtable2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_8c14a0() -> ! {
    todo!("0x8c14a0 bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x8c1588 — __ZNK5boost6detail8function13basic_vtable2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKfSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_8c1588() -> ! {
    todo!("0x8c1588 bool boost::detail::function::basic_vtable2<void,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
