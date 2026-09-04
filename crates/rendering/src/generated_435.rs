//! rendering shard 435 — 100 stubs 0x6828b4..0x6df6dc EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x6828b4..0x6df6dc (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x6828b4 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED1Ev
// IDA 0x6828b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6828b4() {
}

// 0x6828e8 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED1Ev
// IDA 0x6828e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6828e8() {
}

// 0x682930 — __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x682930: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_682930() {
}

// 0x683430 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Tool,RBX::Tool>(boost::shared_ptr<RBX::Tool> const*,RBX::Tool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x683430: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683430() {
}

// 0x683b2c — __ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
#[doc(alias = "boost::shared_ptr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x683b2c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683b2c() {
}

// 0x683bf4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(boost::shared_ptr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x683bf4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683bf4() {
}

// 0x683cd8 — __ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x683cd8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683cd8() {
}

// 0x683dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x683dd0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_683dd0() {
}

// 0x683dd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x683dd4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_683dd4() {
}

// 0x683dd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x683dd8: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683dd8() {
}

// 0x683de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x683de8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683de8() {
}

// 0x683e00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x683e00: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_683e00() {
}

// 0x6857a0 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")]
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// IDA 0x6857a0: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6857a0() {
}

// 0x685870 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// IDA 0x685870: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_685870() {
}

// 0x685a18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Mouse,RBX::Mouse>(boost::shared_ptr<RBX::Mouse> const*,RBX::Mouse *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x685a18: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_685a18() {
}

// 0x686f7c — __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x686f7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_686f7c() {
}

// 0x687030 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x687030: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687030() {
}

// 0x687234 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x687234: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687234() {
}

// 0x6872a8 — __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x6872a8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6872a8() {
}

// 0x68788c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Tool>(char const*,char const*,bool RBX::Tool::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x68788c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_68788c() {
}

// 0x687a1c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv
// IDA 0x687a1c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687a1c() {
}

// 0x687a20 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv
// IDA 0x687a20: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687a20() {
}

// 0x687a24 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x687a24: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687a24() {
}

// 0x687a30 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x687a30: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687a30() {
}

// 0x687a80 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::PropDescriptor<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>(char const*,char const*,bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x687a80: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687a80() {
}

// 0x687b94 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED0Ev
// IDA 0x687b94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_687b94() {
}

// 0x687bc0 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x687bc0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687bc0() {
}

// 0x687bc4 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x687bc4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687bc4() {
}

// 0x687bc8 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x687bc8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687bc8() {
}

// 0x687bec — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x687bec: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687bec() {
}

// 0x687c10 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::PropDescriptor<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>(char const*,char const*,std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x687c10: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687c10() {
}

// 0x687d24 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED0Ev
// IDA 0x687d24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_687d24() {
}

// 0x687d50 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
// IDA 0x687d50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687d50() {
}

// 0x687d54 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
// IDA 0x687d54: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687d54() {
}

// 0x687d58 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x687d58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687d58() {
}

// 0x687d80 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// IDA 0x687d80: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_687d80() {
}

// 0x6891ec — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
// IDA 0x6891ec: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6891ec() {
}

// 0x689420 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
// IDA 0x689420: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_689420() {
}

// 0x68944c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// IDA 0x68944c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_68944c() {
}

// 0x689520 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: int()
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// IDA 0x689520: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_689520() {
}

// 0x689528 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// IDA 0x689528: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_689528() {
}

// 0x689530 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// IDA 0x689530: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_689530() {
}

// 0x689548 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
// IDA 0x689548: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_689548() {
}

// 0x689574 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
// IDA 0x689574: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_689574() {
}

// 0x68c5bc — __ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "__ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "boost::shared_ptr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)")]
// was: __ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x68c5bc: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_68c5bc() {
}

// 0x68e938 — __ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev")]
#[doc(alias = "boost::scoped_ptr<RBX::TouchDebouncer>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev
// IDA 0x68e938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_68e938() {
}

// 0x690298 — __ZN3RBX10Controller10bindButtonENS0_6ButtonESs
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZN3RBX10Controller10bindButtonENS0_6ButtonESs")]
#[doc(alias = "RBX::Controller::bindButton(RBX::Controller::Button,std::string)")]
// was: __ZN3RBX10Controller10bindButtonENS0_6ButtonESs
// IDA 0x690298: 456 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_690298() {
}

// 0x6907a0 — __ZN3RBX10Controller12unbindButtonENS0_6ButtonE
#[doc(alias = "__ZN3RBX10Controller12unbindButtonENS0_6ButtonE")]
#[doc(alias = "RBX::Controller::unbindButton(RBX::Controller::Button)")]
// was: __ZN3RBX10Controller12unbindButtonENS0_6ButtonE
// IDA 0x6907a0: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6907a0() {
}

// 0x6907f4 — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEEC2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEEC2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEEC2Ev
// IDA 0x6907f4: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6907f4() {
}

// 0x692234 — __ZN3RBX17VehicleController22onSteppedKeyboardInputEN5boost10shared_ptrINS_11VehicleSeatEEE
#[doc(alias = "__ZN3RBX17VehicleController22onSteppedKeyboardInputEN5boost10shared_ptrINS_11VehicleSeatEEE")]
#[doc(alias = "RBX::VehicleController::onSteppedKeyboardInput(boost::shared_ptr<RBX::VehicleSeat>)")]
// was: __ZN3RBX17VehicleController22onSteppedKeyboardInputEN5boost10shared_ptrINS_11VehicleSeatEEE
// IDA 0x692234: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_692234() {
}

// 0x692320 — __ZN3RBX17VehicleController19onSteppedTouchInputEN5boost10shared_ptrINS_11VehicleSeatEEE
#[doc(alias = "__ZN3RBX17VehicleController19onSteppedTouchInputEN5boost10shared_ptrINS_11VehicleSeatEEE")]
#[doc(alias = "RBX::VehicleController::onSteppedTouchInput(boost::shared_ptr<RBX::VehicleSeat>)")]
// was: __ZN3RBX17VehicleController19onSteppedTouchInputEN5boost10shared_ptrINS_11VehicleSeatEEE
// IDA 0x692320: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_692320() {
}

// 0x692d40 — __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button,std::string),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EED1Ev
// IDA 0x692d40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_692d40() {
}

// 0x692e4c — __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EED1Ev
// IDA 0x692e4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_692e4c() {
}

// 0x692e8c — __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,bool ()(RBX::Controller::Button),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EED1Ev
// IDA 0x692e8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_692e8c() {
}

// 0x692ecc — __ZN3RBX10Reflection9EventDescINS_10ControllerEFvNS2_6ButtonEEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ControllerEFvNS2_6ButtonEEN3rbx6signalIS4_EEMS2_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Controller,void ()(RBX::Controller::Button),rbx::signal<void ()(RBX::Controller::Button)>,rbx::signal<void ()(RBX::Controller::Button)> RBX::Controller::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10ControllerEFvNS2_6ButtonEEN3rbx6signalIS4_EEMS2_S7_ED1Ev
// IDA 0x692ecc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_692ecc() {
}

// 0x692ef0 — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEE7addPairES3_PKc
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::addPair(RBX::Controller::Button,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEE7addPairES3_PKc
// IDA 0x692ef0: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_692ef0() {
}

// 0x693250 — __ZN3RBX10Reflection7Variant14genericConvertINS_10Controller6ButtonEEERT_v
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_10Controller6ButtonEEERT_v")]
#[doc(alias = "RBX::Controller::Button & RBX::Reflection::Variant::genericConvert<RBX::Controller::Button>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10Controller6ButtonEEERT_v
// IDA 0x693250: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_693250() {
}

// 0x693454 — __ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "__ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "boost::shared_ptr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)")]
// was: __ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x693454: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_693454() {
}

// 0x6936bc — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_")]
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_
// IDA 0x6936bc: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6936bc() {
}

// 0x693800 — __ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::ButtonBindingWidget>::operator=(boost::shared_ptr<RBX::ButtonBindingWidget> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_
// IDA 0x693800: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_693800() {
}

// 0x6938f0 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToString(RBX::Controller::Button const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringERKS3_
// IDA 0x6938f0: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6938f0() {
}

// 0x693a90 — __ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "__ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "boost::shared_ptr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)")]
// was: __ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x693a90: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_693a90() {
}

// 0x69519c — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED1Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED1Ev
// IDA 0x69519c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_69519c() {
}

// 0x6951a0 — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED0Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED0Ev
// IDA 0x6951a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6951a0() {
}

// 0x695240 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupEPKc
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupEPKc
// IDA 0x695240: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_695240() {
}

// 0x695270 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE6lookupERKNS0_7VariantE
// IDA 0x695270: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_695270() {
}

// 0x695290 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueEmRNS0_7VariantE
// IDA 0x695290: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_695290() {
}

// 0x6952c4 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringEmRSs
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringEmRSs")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringEmRSs
// IDA 0x6952c4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6952c4() {
}

// 0x695720 — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE13convertToItemERKS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE13convertToItemERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToItem(RBX::Controller::Button const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE13convertToItemERKS3_
// IDA 0x695720: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_695720() {
}

// 0x6958dc — __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToValue(RBX::Name const&,RBX::Controller::Button&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueERKNS_4NameERS3_
// IDA 0x6958dc: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6958dc() {
}

// 0x695958 — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev
// IDA 0x695958: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_695958() {
}

// 0x696518 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17VehicleControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17VehicleControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VehicleController,RBX::VehicleController>(boost::shared_ptr<RBX::VehicleController> const*,RBX::VehicleController *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17VehicleControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x696518: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_696518() {
}

// 0x696ca0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HumanoidControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HumanoidControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HumanoidController,RBX::HumanoidController>(boost::shared_ptr<RBX::HumanoidController> const*,RBX::HumanoidController *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HumanoidControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x696ca0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_696ca0() {
}

// 0x6dd740 — __ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
#[doc(alias = "boost::shared_ptr<RBX::AdvArrowTool>::shared_ptr<RBX::AdvArrowTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x6dd740: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dd740() {
}

// 0x6dd808 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x6dd808: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dd808() {
}

// 0x6dd8ec — __ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x6dd8ec: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dd8ec() {
}

// 0x6dd9e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x6dd9e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6dd9e4() {
}

// 0x6dd9e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x6dd9e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6dd9e8() {
}

// 0x6dd9ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x6dd9ec: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dd9ec() {
}

// 0x6dd9fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x6dd9fc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dd9fc() {
}

// 0x6dda14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x6dda14: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dda14() {
}

// 0x6dda18 — __ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
#[doc(alias = "boost::shared_ptr<RBX::NewNullTool>::shared_ptr<RBX::NewNullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x6dda18: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dda18() {
}

// 0x6ddae0 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const*,RBX::NewNullTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x6ddae0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ddae0() {
}

// 0x6ddbc4 — __ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x6ddbc4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ddbc4() {
}

// 0x6ddcbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x6ddcbc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6ddcbc() {
}

// 0x6ddcc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x6ddcc0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6ddcc0() {
}

// 0x6ddcc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x6ddcc4: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ddcc4() {
}

// 0x6ddcd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x6ddcd4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ddcd4() {
}

// 0x6ddcec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x6ddcec: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6ddcec() {
}

// 0x6de388 — __ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE
#[doc(alias = "__ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE")]
#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<double>(char const*,boost::function0<double>)")]
// was: __ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE
// IDA 0x6de388: 178 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6de388() {
}

// 0x6de56c — __ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE
#[doc(alias = "__ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE")]
#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<float>(char const*,boost::function0<float>)")]
// was: __ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE
// IDA 0x6de56c: 178 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6de56c() {
}

// 0x6de770 — __ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE
#[doc(alias = "__ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE")]
#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<int>(char const*,boost::function0<int>)")]
// was: __ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE
// IDA 0x6de770: 178 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6de770() {
}

// 0x6dea24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// IDA 0x6dea24: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dea24() {
}

// 0x6dea84 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEiE6invokeERNS1_15function_bufferE
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEiE6invokeERNS1_15function_bufferE")]
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEiE6invokeERNS1_15function_bufferE
// IDA 0x6dea84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6dea84() {
}

// 0x6dea88 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
// IDA 0x6dea88: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6dea88() {
}

// 0x6deb38 — __ZN5boost9function0IiE5clearEv
#[doc(alias = "__ZN5boost9function0IiE5clearEv")]
#[doc(alias = "boost::function0<int>::clear(void)")]
// was: __ZN5boost9function0IiE5clearEv
// IDA 0x6deb38: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6deb38() {
}

// 0x6df2a0 — __ZNK5boost9function0IiEclEv
#[doc(alias = "__ZNK5boost9function0IiEclEv")]
#[doc(alias = "boost::function0<int>::operator()(void)const")]
// was: __ZNK5boost9function0IiEclEv
// IDA 0x6df2a0: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6df2a0() {
}

// 0x6df430 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats14TypedStatsItemIiEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats14TypedStatsItemIiEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::TypedStatsItem<int>,RBX::Stats::TypedStatsItem<int>>(boost::shared_ptr<RBX::Stats::TypedStatsItem<int>> const*,RBX::Stats::TypedStatsItem<int> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats14TypedStatsItemIiEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x6df430: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6df430() {
}

// 0x6df6ac — __ZN5boost9function0IfE13assign_to_ownERKS1_
#[doc(alias = "__ZN5boost9function0IfE13assign_to_ownERKS1_")]
#[doc(alias = "boost::function0<float>::assign_to_own(boost::function0<float> const&)")]
// was: __ZN5boost9function0IfE13assign_to_ownERKS1_
// IDA 0x6df6ac: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6df6ac() {
}

// 0x6df6dc — __ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE
#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE")]
#[doc(alias = "RBX::Stats::TypedStatsItem<float>::TypedStatsItem(boost::function0<float>)")]
// was: __ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE
// IDA 0x6df6dc: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6df6dc() {
}
