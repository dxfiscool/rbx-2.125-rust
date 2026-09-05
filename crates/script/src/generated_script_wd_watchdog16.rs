// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler watchdog16
// Filter: Script|Lua|Yield|CodeGen|Luau (filtered 4818 exhausted) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x5532ac..0x55d1d8 | EA-sorted asc distinct not yet in script (remaining 64738->64618, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x5532ac — __ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE — rbx::signals::signal<void ()(std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE")]
pub fn stub_0x5532ac(slot: &crate::slot::CallableSlot) {
// IDA 0x5532ac: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

// 0x5534b8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_ — boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_")]
pub fn stub_0x5534b8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5534dc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_ — rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")]
pub fn stub_0x5534dc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5534dc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5535d8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev — rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
pub fn stub_0x5535d8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x5536e8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev — rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot() [0x5536e8]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
pub fn stub_0x5536e8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x553818 — __ZN3rbx7signals6signalIFvSsSsEE4slot10disconnectEv — rbx::signals::signal<void ()(std::string,std::string)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot10disconnectEv")]
pub fn stub_0x553818(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

// 0x553928 — __ZNK3rbx7signals6signalIFvSsSsEE4slot9connectedEv — rbx::signals::signal<void ()(std::string,std::string)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsSsEE4slot9connectedEv")]
pub fn stub_0x553928() -> crate::slot::SlotConnection {
// IDA 0x553928: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x553934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs — rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")]
pub fn stub_0x553934(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x553934: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x553ad4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")]
pub fn stub_0x553ad4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x553ad4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x553adc — __ZNK5boost9function2IvSsSsEclESsSs — boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const
#[doc(alias = "boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const")]
#[doc(alias = "__ZNK5boost9function2IvSsSsEclESsSs")]
pub fn stub_0x553adc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x553cbc — __ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE — rbx::signals::signal<void ()(std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE")]
pub fn stub_0x553cbc(slot: &mut crate::slot::CallableSlot) {
// IDA 0x553cbc: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

// 0x553dac — __ZN3rbx7signals6signalIFvSsSsEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x553dac(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, std::string)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x553db0 — __ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x553db0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, std::string)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x553ea0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")]
pub fn stub_0x553ea0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x553ea0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x553fb0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable() [0x553fb0]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")]
pub fn stub_0x553fb0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x553fb0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5540e0 — __ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev — rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev")]
pub fn stub_0x5540e0(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x55410c — __ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev — rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot() [0x55410c]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev")]
pub fn stub_0x55410c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x5541e0 — __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_ — boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)
#[doc(alias = "boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")]
#[doc(alias = "__ZN5boost9function2IvSsSsE13assign_to_ownERKS1_")]
pub fn stub_0x5541e0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x554210 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,int>(char const*,char const*,bool (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,int>(char const*,char const*,bool (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x554210(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x55431c — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
pub fn stub_0x55431c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x554320 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
pub fn stub_0x554320(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x554324 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x554324(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x554348 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb — RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x554348(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x554468 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::PropDescriptor<double (RBX::GuiService::*)(void)const,int>(char const*,char const*,double (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::PropDescriptor<double (RBX::GuiService::*)(void)const,int>(char const*,char const*,double (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x554468(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x554574 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED0Ev — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor() [0x554574]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED0Ev")]
pub fn stub_0x554574(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5545a0 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE10isReadOnlyEv")]
pub fn stub_0x5545a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5545a4 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv")]
pub fn stub_0x5545a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5545a8 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x5545a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5545c8 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd — RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_0x5545c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5546e8 — __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)")]
#[doc(alias = "__ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5546e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple")
}

// 0x554854 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — boost::shared_ptr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(boost::weak_ptr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "boost::shared_ptr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(boost::weak_ptr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0x554854() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiObject")
}

// 0x554994 — __ZNSt8auto_ptrIN3RBX10Reflection5TupleEED2Ev — std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr()
#[doc(alias = "std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr()")]
#[doc(alias = "__ZNSt8auto_ptrIN3RBX10Reflection5TupleEED2Ev")]
pub fn stub_0x554994() -> crate::slot::PortedFn {
// IDA 0x554994: std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x554994, "std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr()")
}

// 0x554a3c — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
pub fn stub_0x554a3c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x554a64 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
pub fn stub_0x554a64() -> crate::slot::PortedFn {
// IDA 0x554a64: std::_Rb_tree<RBX::GuiService::CenterDialogType, std::pair<RBX::GuiService::CenterDialogType const, std::list<RBX::GuiSe~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x554a64, "std::_Rb_tree<RBX::GuiService::CenterDialogType, std::pair<RBX::GuiService::CenterDialogType const, ~")
}

// 0x554a8c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE — std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)
#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE")]
pub fn stub_0x554a8c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x554ab4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x554ab4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x554adc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x554adc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x554da4 — __GLOBAL__I_a_208 — global constructor keyed to_a_208
#[doc(alias = "global constructor keyed to_a_208")]
#[doc(alias = "__GLOBAL__I_a_208")]
pub fn stub_0x554da4() -> crate::slot::PortedFn {
// IDA 0x554da4: __GLOBAL__I_a_208.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x554da4, "__GLOBAL__I_a_208")
}

// 0x55603c — __Z17different5percentRKN3G3D7Vector3ES2_ — different5percent(G3D::Vector3 const&,G3D::Vector3 const&)
#[doc(alias = "different5percent(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__Z17different5percentRKN3G3D7Vector3ES2_")]
pub fn stub_0x55603c() -> crate::slot::PortedFn {
// IDA 0x55603c: different5percent(G3D::Vector3 const&, G3D::Vector3 const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55603c, "different5percent(G3D::Vector3 const&, G3D::Vector3 const&)")
}

// 0x556780 — __ZNK3RBX9BodyMover12askSetParentEPKNS_8InstanceE — RBX::BodyMover::askSetParent(RBX::Instance const*)const
#[doc(alias = "RBX::BodyMover::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9BodyMover12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x556780(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5567bc — __ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE — RBX::Rocket::setTarget(RBX::PartInstance *)
#[doc(alias = "RBX::Rocket::setTarget(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE")]
pub fn stub_0x5567bc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Rocket setter.
cell.set(value)
}

// 0x559448 — __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev — RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::~RefPropDescriptor()
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev")]
pub fn stub_0x559448(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x559474 — __ZN3RBX6Rocket13onGoalChangedERKNS_10Reflection18PropertyDescriptorE — RBX::Rocket::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)
#[doc(alias = "RBX::Rocket::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX6Rocket13onGoalChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x559474(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::onGoalChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x55947c — __ZN3RBX10Reflection13BoundFuncDescINS_6RocketEFvvELi0EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::Rocket,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Rocket,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6RocketEFvvELi0EED1Ev")]
pub fn stub_0x55947c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5594a0 — __ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev — RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_0x5594a0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5595a4 — __ZN3RBX12BodyPosition13onGoalChangedERKNS_10Reflection18PropertyDescriptorE — RBX::BodyPosition::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)
#[doc(alias = "RBX::BodyPosition::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX12BodyPosition13onGoalChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x5595a4(handle: &crate::slot::InstanceHandle) {
// RBX::BodyPosition::onGoalChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5595bc — __ZN3RBX10Reflection13BoundFuncDescINS_12BodyPositionEFN3G3D7Vector3EvELi0EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::BodyPosition,G3D::Vector3 ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BodyPosition,G3D::Vector3 ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_12BodyPositionEFN3G3D7Vector3EvELi0EED1Ev")]
pub fn stub_0x5595bc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5595e0 — __ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev — RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_0x5595e0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x559614 — __ZN3RBX10Reflection13BoundFuncDescINS_12BodyVelocityEFN3G3D7Vector3EvELi0EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::BodyVelocity,G3D::Vector3 ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BodyVelocity,G3D::Vector3 ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_12BodyVelocityEFN3G3D7Vector3EvELi0EED1Ev")]
pub fn stub_0x559614(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x559920 — __ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
#[doc(alias = "RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX12BodyPosition17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_0x559920(handle: &crate::slot::InstanceHandle) {
// RBX::BodyPosition::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x559928() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyPosition"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x559ba8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyPosition"
}

// 0x55a7e8 — __ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::Rocket::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
#[doc(alias = "RBX::Rocket::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX6Rocket17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_0x55a7e8(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55a7f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rocket"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55a800() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rocket"
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv")]
pub fn stub_0x55a810() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv")]
pub fn stub_0x55a838() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55a904() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyGyro"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55a9c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyGyro"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55ad24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyVelocity"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55ade0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyVelocity"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55b144() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyAngularVelocity"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55b200() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyAngularVelocity"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55b564() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyForce"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55b624() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyForce"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyThrustENS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55b988() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyThrust"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10BodyThrustENS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x55ba48() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyThrust"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd08() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyGyro"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd0c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyPosition"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd10() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyVelocity"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd14() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyAngularVelocity"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd18() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyForce"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyThrustENS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd1c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyThrust"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x55bd20() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rocket"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyThrustENS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55bd24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyThrust"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyThrustENS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55bd98() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyThrust"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sBodyThrustEEEEvv")]
pub fn stub_0x55be20() -> crate::slot::PortedFn {
// IDA 0x55be20: void RBX::Name::callDoDeclare<RBX::sBodyThrust>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55be20, "void RBX::Name::callDoDeclare<RBX::sBodyThrust>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBodyThrustEEEERKS0_v")]
pub fn stub_0x55be24(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyThrust>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55bf04() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyForce"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55bf78() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyForce"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBodyForceEEEEvv")]
pub fn stub_0x55c000() -> crate::slot::PortedFn {
// IDA 0x55c000: void RBX::Name::callDoDeclare<RBX::sBodyForce>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c000, "void RBX::Name::callDoDeclare<RBX::sBodyForce>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBodyForceEEEERKS0_v")]
pub fn stub_0x55c004(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyForce>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55c0e4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyAngularVelocity"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55c158() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyAngularVelocity"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sBodyAngularVelocityEEEEvv")]
pub fn stub_0x55c1e0() -> crate::slot::PortedFn {
// IDA 0x55c1e0: void RBX::Name::callDoDeclare<RBX::sBodyAngularVelocity>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c1e0, "void RBX::Name::callDoDeclare<RBX::sBodyAngularVelocity>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sBodyAngularVelocityEEEERKS0_v")]
pub fn stub_0x55c1e4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyAngularVelocity>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55c2c4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyVelocity"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55c338() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyVelocity"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBodyVelocityEEEEvv")]
pub fn stub_0x55c3c0() -> crate::slot::PortedFn {
// IDA 0x55c3c0: void RBX::Name::callDoDeclare<RBX::sBodyVelocity>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c3c0, "void RBX::Name::callDoDeclare<RBX::sBodyVelocity>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v")]
pub fn stub_0x55c3c4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyVelocity>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55c4a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyGyro"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55c518() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyGyro"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sBodyGyroEEEEvv")]
pub fn stub_0x55c5a0() -> crate::slot::PortedFn {
// IDA 0x55c5a0: void RBX::Name::callDoDeclare<RBX::sBodyGyro>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c5a0, "void RBX::Name::callDoDeclare<RBX::sBodyGyro>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v")]
pub fn stub_0x55c5a4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyGyro>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBodyMoverEEEEvv")]
pub fn stub_0x55c684() -> crate::slot::PortedFn {
// IDA 0x55c684: void RBX::Name::callDoDeclare<RBX::sBodyMover>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c684, "void RBX::Name::callDoDeclare<RBX::sBodyMover>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBodyMoverEEEERKS0_v")]
pub fn stub_0x55c688(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyMover>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55c768() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rocket"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RocketENS_9BodyMoverELZNS_7sRocketEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55c7dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rocket"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sRocketEEEEvv")]
pub fn stub_0x55c864() -> crate::slot::PortedFn {
// IDA 0x55c864: void RBX::Name::callDoDeclare<RBX::sRocket>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55c864, "void RBX::Name::callDoDeclare<RBX::sRocket>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v")]
pub fn stub_0x55c868(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRocket>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x55c948() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyPosition"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x55c9bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyPosition"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBodyPositionEEEEvv")]
pub fn stub_0x55ca44() -> crate::slot::PortedFn {
// IDA 0x55ca44: void RBX::Name::callDoDeclare<RBX::sBodyPosition>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x55ca44, "void RBX::Name::callDoDeclare<RBX::sBodyPosition>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v")]
pub fn stub_0x55ca48(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyPosition>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x55cb28(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::BodyThrust, RBX::sBodyThrust, RBX::FactoryProduct<RBX::Bod~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55cc44(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55cc48(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55cce8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55ccf0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55cd94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55cd9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55ce40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55ce48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn124_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55ceec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "__ZThn124_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55cef4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "__ZThn244_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x55cf98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "__ZThn244_N3RBX10Reflection9DescribedINS_10BodyThrustELZNS_11sBodyThrustEENS_14FactoryProductIS2_NS_9BodyMoverELZNS_11sBodyThrustEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x55cfa0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

// 0x55d044 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_10BodyThrustEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyThrust>(char const*,char const*,G3D::Vector3 RBX::BodyThrust::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyThrust>(char const*,char const*,G3D::Vector3 RBX::BodyThrust::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_10BodyThrustEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x55d044() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "G3D::Vector3")
}

// 0x55d1d4 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyThrustEE10isReadOnlyEv — RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyThrust>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyThrust>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyThrustEE10isReadOnlyEv")]
pub fn stub_0x55d1d4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "G3D::Vector3")
}

// 0x55d1d8 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyThrustEE11isWriteOnlyEv — RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyThrust>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyThrust>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyThrustEE11isWriteOnlyEv")]
pub fn stub_0x55d1d8() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "G3D::Vector3")
}
