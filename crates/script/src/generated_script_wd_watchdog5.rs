// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (filtered 4818 exhausted) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x5333a8..0x5392f4 | EA-sorted asc distinct not yet in script (remaining 55270->55150, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x5333a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE — RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_0x5333a8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiButton")
}

// 0x5333b8 — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE — RBX::Reflection::EventDescBase<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x5333b8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::GuiButton, void (), rbx::remote_signal<void ()>, rbx::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5333cc — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiButton::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiButton::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x5333cc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiButton", "void", 1)
}

// 0x533548 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE — RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x533548() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiButton", "void", 1)
}

// 0x533578 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED0Ev — RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::~BoundFuncDesc() [0x533578]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED0Ev")]
pub fn stub_0x533578(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x533680 — __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x533680() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiButton", "void", 1)
}

// 0x5337bc — __ZN3RBX10Reflection11Call1HelperINS_9GuiButtonEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs — RBX::Reflection::Call1Helper<RBX::GuiButton,void (RBX::GuiButton::*)(std::string),std::string,void>::call(RBX::GuiButton*,void (RBX::GuiButton::*)(std::string),RBX::Reflection::Variant &,std::string const&)
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiButton,void (RBX::GuiButton::*)(std::string),std::string,void>::call(RBX::GuiButton*,void (RBX::GuiButton::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9GuiButtonEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_0x5337bc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::GuiButton, void (RBX::GuiButton::*)(std::string), std::s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5338ec — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE — rbx::signals::signal<void ()(RBX::UDim2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
pub fn stub_0x5338ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::UDim2)>::slot")
}

// 0x533a4c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception — rbx::signals::signal<void ()(RBX::UDim2)>::on_error(std::exception &)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception")]
pub fn stub_0x533a4c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

// 0x533a74 — __ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE — rbx::signals::signal<void ()(int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
pub fn stub_0x533a74() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (int, int)>::slot")
}

// 0x533bd4 — __ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception — rbx::signals::signal<void ()(int,int)>::on_error(std::exception &)
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception")]
pub fn stub_0x533bd4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

// 0x533bfc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x533bfc() -> crate::slot::SlotConnection {
// IDA 0x533bfc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x533c70 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv — RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv")]
pub fn stub_0x533c70(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::GuiObject, void (RBX::UDim2)>::listenerConnectionAdded() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x533cbc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
pub fn stub_0x533cbc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x533ce8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot() [0x533ce8]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
pub fn stub_0x533ce8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x533dbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x533dbc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x533dbc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x533dc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x533dc4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x533dc4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x533dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv — boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
pub fn stub_0x533dcc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x533de4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x533de4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x533de4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x533e10 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable() [0x533e10]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x533e10(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x533e10: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x533ee4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x533ee4() -> crate::slot::SlotConnection {
// IDA 0x533ee4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x533f58 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv — RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv")]
pub fn stub_0x533f58(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::GuiObject, void (int, int)>::listenerConnectionAdded() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x533fa4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev")]
pub fn stub_0x533fa4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x533fd0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot() [0x533fd0]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev")]
pub fn stub_0x533fd0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x5340a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x5340a4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5340a4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5340ac — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x5340ac(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5340ac: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5340b4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv — boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
pub fn stub_0x5340b4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x5340cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x5340cc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5340cc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5340f8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable() [0x5340f8]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x5340f8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5340f8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5341cc — __ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<void (RBX::GuiObject::*)(RBX::UDim2)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<void (RBX::GuiObject::*)(RBX::UDim2)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x5341cc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x53422c — __ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_ — boost::detail::function::function_void_mem_invoker2<void (RBX::GuiObject::*)(RBX::UDim2),void,RBX::GuiObject*,RBX::UDim2>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject*,RBX::UDim2)
#[doc(alias = "boost::detail::function::function_void_mem_invoker2<void (RBX::GuiObject::*)(RBX::UDim2),void,RBX::GuiObject*,RBX::UDim2>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject*,RBX::UDim2)")]
#[doc(alias = "__ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_")]
pub fn stub_0x53422c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x534260 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_ — boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_")]
pub fn stub_0x534260() -> crate::slot::PortedFn {
// IDA 0x534260: boost::function1<void, RBX::GuiObject::TweenStatus>::swap(boost::function1<void, RBX::GuiObject::TweenStatus>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x534260, "boost::function1<void, RBX::GuiObject::TweenStatus>::swap(boost::function1<void, RBX::GuiObject::Twe~")
}

// 0x53433c — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_ — boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_")]
pub fn stub_0x53433c() -> crate::slot::PortedFn {
// IDA 0x53433c: boost::function1<void, RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void, RBX::GuiObject::TweenStatus>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x53433c, "boost::function1<void, RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void, RBX::GuiObje~")
}

// 0x535248 — __ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v — RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v")]
pub fn stub_0x535248() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::TweenService")
}

// 0x535410 — __ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v — RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v")]
pub fn stub_0x535410() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::TweenService"))
}

// 0x535584 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::TweenService> RBX::Creatable<RBX::Instance>::create<RBX::TweenService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService> RBX::Creatable<RBX::Instance>::create<RBX::TweenService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x535584() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TweenService")
}

// 0x535634 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE — boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::TweenService>(boost::shared_ptr<RBX::TweenService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TweenService>(rbx_core::SharedPtr<RBX::TweenService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_0x535634(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v")]
pub fn stub_0x535668(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sTweenService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sTweenServiceEEEEvv")]
pub fn stub_0x5356ac() -> crate::slot::PortedFn {
// IDA 0x5356ac: void RBX::Name::callDoDeclare<RBX::sTweenService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5356ac, "void RBX::Name::callDoDeclare<RBX::sTweenService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v")]
pub fn stub_0x5356b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sTweenService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x535794 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv")]
pub fn stub_0x535794() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

// 0x535798 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv")]
pub fn stub_0x535798() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

// 0x535870 — __ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x535870() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TweenService")
}

// 0x535938 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TweenService,RBX::TweenService>(boost::shared_ptr<RBX::TweenService> const*,RBX::TweenService *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TweenService,RBX::TweenService>(rbx_core::SharedPtr<RBX::TweenService> const*,RBX::TweenService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x535938() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TweenService")
}

// 0x535a20 — __ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x535a20() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x535b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x535b28(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x535b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x535b2c]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x535b2c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x535b30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x535b30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x535b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x535b50() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x535b68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x535b68() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE15isNullClassNameEv")]
pub fn stub_0x535b6c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x536b60() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x536c38() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x536d10 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_ — void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_")]
pub fn stub_0x536d10(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x536df8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x536df8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x536e14 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_ — boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_")]
pub fn stub_0x536e14(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x536e30 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE — bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x536e30(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x536f0c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE — bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x536f0c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x536fe0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE — void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x536fe0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x5370ac — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x5370ac(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x537170 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x537170(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x5372bc — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_ — boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_")]
pub fn stub_0x5372bc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x5372ec — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_ — boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
pub fn stub_0x5372ec() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

// 0x5373b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_ — boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
pub fn stub_0x5373b4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

// 0x537484 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv — boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)
// type: void()
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv")]
pub fn stub_0x537484() -> crate::slot::PortedFn {
// IDA 0x537484: boost::function1<void, RBX::GuiObject::TweenStatus>::dummy::nonnull().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x537484, "boost::function1<void, RBX::GuiObject::TweenStatus>::dummy::nonnull()")
}

// 0x537488 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev — rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")]
pub fn stub_0x537488() -> crate::slot::PortedFn {
// IDA 0x537488: rbx::remote_signal<void (RBX::UDim2)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x537488, "rbx::remote_signal<void (RBX::UDim2)>::remote_signal()")
}

// 0x5375e4 — __ZN3rbx13remote_signalIFviiEEC2Ev — rbx::remote_signal<void ()(int,int)>::remote_signal(void)
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,int)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFviiEEC2Ev")]
pub fn stub_0x5375e4() -> crate::slot::PortedFn {
// IDA 0x5375e4: rbx::remote_signal<void (int, int)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5375e4, "rbx::remote_signal<void (int, int)>::remote_signal()")
}

// 0x537740 — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv — RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
pub fn stub_0x537740(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<1, RBX::GuiObject, void (RBX::UDim2)>::connectSignalListener() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x537834 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE — RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::getSignalPtr(RBX::Reflection::EventSource *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_0x537834() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiObject")
}

// 0x53789c — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_ — RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_")]
pub fn stub_0x53789c(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<1, RBX::GuiObject, void (RBX::UDim2)>::signalProducedIncremented(~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5378c4 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_ — RBX::Reflection::RemoteEventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::UDim2)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::UDim2)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")]
pub fn stub_0x5378c4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::GuiObject, void (RBX::UDim2), rbx::remote_sig~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x537a18 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x537a18() -> crate::slot::SlotConnection {
// IDA 0x537a18: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x537a8c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE — rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")]
pub fn stub_0x537a8c(slot: &crate::slot::CallableSlot) {
// IDA 0x537a8c: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

// 0x537c98 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_")]
pub fn stub_0x537c98(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x537cbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x537cbc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x537ce8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot() [0x537ce8]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x537ce8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x537dbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv — rbx::signals::signal<void ()(RBX::UDim2)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv")]
pub fn stub_0x537dbc(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

// 0x537ecc — __ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv — rbx::signals::signal<void ()(RBX::UDim2)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv")]
pub fn stub_0x537ecc() -> crate::slot::SlotConnection {
// IDA 0x537ecc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x537ed8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_ — rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_0x537ed8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x537ed8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x537f00 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_ — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_0x537f00(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x537f00: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x537f28 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x537f28(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x537f28: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x537f5c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE — rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")]
pub fn stub_0x537f5c(slot: &mut crate::slot::CallableSlot) {
// IDA 0x537f5c: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

// 0x53804c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x53804c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::UDim2)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x538050 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x538050(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::UDim2)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x538140 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev — rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev")]
pub fn stub_0x538140(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x53816c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev — rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot() [0x53816c]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev")]
pub fn stub_0x53816c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x538240 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
pub fn stub_0x538240(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x538240: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x53826c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable() [0x53826c]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
pub fn stub_0x53826c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x53826c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x538340 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv — RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
pub fn stub_0x538340(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::GuiObject, void (RBX::UDim2)>::connectSignalListener() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x538344 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv — RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv")]
pub fn stub_0x538344(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<2, RBX::GuiObject, void (int, int)>::connectSignalListener() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x538438 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE — RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_0x538438() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::GuiObject")
}

// 0x5384a0 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii — RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii")]
pub fn stub_0x5384a0(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<2, RBX::GuiObject, void (int, int)>::signalProducedIncremented(in~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5384b8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii — RBX::Reflection::RemoteEventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::replicateEvent(RBX::Reflection::EventSource *,int,int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::replicateEvent(RBX::Reflection::EventSource *,int,int)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii")]
pub fn stub_0x5384b8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<2, RBX::GuiObject, void (int, int), rbx::remote_signa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x538624 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x538624() -> crate::slot::SlotConnection {
// IDA 0x538624: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x538698 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev — rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
pub fn stub_0x538698(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x5386c4 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev — rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x5386c4]")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
pub fn stub_0x5386c4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x538798 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii — rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
pub fn stub_0x538798(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x538798: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5387c0 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
pub fn stub_0x5387c0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5387c0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5387e8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x5387e8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x5387e8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x538810 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev")]
pub fn stub_0x538810(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x538810: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x53883c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable() [0x53883c]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev")]
pub fn stub_0x53883c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x53883c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x538910 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv — RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv")]
pub fn stub_0x538910(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::GuiObject, void (int, int)>::connectSignalListener() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x538914(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5389d0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x538a9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x538b54(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x538c24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x538cdc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x538dac — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_")]
pub fn stub_0x538dac(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

// 0x538de0 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")]
pub fn stub_0x538de0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x538e08 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x538e08(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x538e60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x538e60(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x538f14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x538f14(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x538f6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x538f6c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x538fd4 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x538fd4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x5390b8 — __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm")]
pub fn stub_0x5390b8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5390d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_ — RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)
#[doc(alias = "RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_")]
pub fn stub_0x5390d0(handle: &crate::slot::InstanceHandle) {
// RBX::GuiObject::TweenStatus* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x53910c — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x53910c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

// 0x53929c — __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
#[doc(alias = "RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0x53929c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x5392f4 — __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x5392f4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}
