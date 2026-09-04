//! network generated_03 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|RBX::Replicator (4479 funcs, 150 stubs here, 3269 combined, 1429 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

use std::collections::{HashMap, BTreeMap};

/// `rbx::signals::signal` slot list reduced to linkage bits.
#[derive(Clone, Debug, Default)]
pub struct GenSignalState {
    pub slots: Vec<(u64, bool)>,
    pub next: u64,
}

fn gen_connect(s: &mut GenSignalState) -> u64 {
    let id = s.next;
    s.next = s.next.wrapping_add(1);
    s.slots.push((id, true));
    id
}

fn gen_disconnect(s: &mut GenSignalState, id: u64) {
    s.slots.retain(|(i, _)| *i != id);
}

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944).
#[derive(Clone, Debug, Default)]
pub struct GenEventState {
    pub mode: bool,
    pub conn: bool,
    pub listener: bool,
    pub watched: u32,
    pub count: i32,
}

/// Reflection descriptor row (Bound/Prop/Event desc common shape).
#[derive(Clone, Debug, Default)]
pub struct GenDesc {
    pub name: String,
    pub value: i32,
    pub text: String,
    pub readable: bool,
    pub writable: bool,
    pub scriptable: bool,
    pub broadcast: bool,
}

/// `RBX::Network::Peer` transport view.
#[derive(Clone, Debug, Default)]
pub struct GenPeer {
    pub kbps: i32,
    pub connected: bool,
    pub port: u16,
    pub ip: u32,
}

/// RakNet stats accumulation (`PeerStatsItem::update`, IDA 0xad5790).
#[derive(Clone, Debug, Default)]
pub struct GenStats {
    pub packets: u64,
    pub bytes: u64,
    pub enabled: bool,
    pub checked: bool,
}

/// `TopNErrorsPhysicsSender` tables: part -> error plus descending top-N.
#[derive(Clone, Debug, Default)]
pub struct GenTopN {
    pub map: HashMap<u32, f32>,
    pub top: Vec<u32>,
}

fn gen_refresh_top(t: &mut GenTopN) {
    let mut ids: Vec<u32> = t.map.keys().copied().collect();
    ids.sort_by(|a, b| {
        t.map
            .get(b)
            .partial_cmp(&t.map.get(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    t.top = ids;
}

/// `InterpolatingPhysicsReceiver` lerp queue (IDA 0xada700).
#[derive(Clone, Debug, Default)]
pub struct GenInterp {
    pub alpha: f32,
    pub active: bool,
    pub queue: Vec<u32>,
}

/// `RBX::Network::Replicator` connection view.
#[derive(Clone, Debug, Default)]
pub struct GenReplicator {
    pub open: bool,
    pub process: bool,
    pub port: u16,
    pub ip: u32,
    pub markers: u64,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct GenFunctor {
    pub has: bool,
}

/// `boost::multi_index` nugget index: hash by part + order by stamp.
#[derive(Clone, Debug, Default)]
pub struct GenIndex {
    pub by_id: HashMap<u32, u64>,
    pub by_time: BTreeMap<u64, u32>,
}

/// TaskScheduler job view (`sleepTime`, IDA 0xad74f8).
#[derive(Clone, Debug, Default)]
pub struct GenJob {
    pub owner: u32,
    pub running: bool,
}

/// `RBX::Network::Marker` fire state (IDA 0xad12d0).
#[derive(Clone, Debug, Default)]
pub struct GenMarker {
    pub returned: bool,
    pub fired: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct GenOwner {
    pub ip: u32,
    pub port: u16,
    pub server: bool,
}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}


// 0xac4cb8 — __ZN3RBX10Reflection11Call2HelperINS_7Network6PlayerEMS3_FvSsSsESsSsvE4callEPS3_S5_RNS0_7VariantERKSsSB_
// type: void __fastcall(int, void (__fastcall *)(int, int *, int *), int, int, std::string *, const std::string *)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,std::string),std::string,std::string,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
pub fn stub_ac4cb8(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xac4cb8: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xac4e7c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFSsSsELi1EEC2EMS3_FSsSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,std::string ()(std::string),1>::BoundFuncDesc(std::string (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac4e7c(name: &str) -> GenDesc {
    // IDA 0xac4e7c: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac5124 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFSsSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,std::string ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_ac5124(d: GenDesc) {
    // IDA 0xac5124: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac525c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFSsSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,std::string ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac525c(arg: &str) -> String {
    // IDA 0xac525c: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xac539c — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FSsSsESsSsE4callEPS3_S5_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, _DWORD *, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,std::string (RBX::Network::Player::*)(std::string),std::string,std::string>::call(RBX::Network::Player*,std::string (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_ac539c(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xac539c: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xac5604 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_ac5604(d: GenDesc) {
    // IDA 0xac5604: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac56e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac56e0(args: &[String]) -> Vec<String> {
    // IDA 0xac56e0: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xac5924 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::~RemoteEventDesc()")]
pub fn stub_ac5924(d: GenDesc) {
    // IDA 0xac5924: event descriptor dtor.
    let _ = d;
}
// 0xac5a00 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ac5a00(s: &mut GenSignalState) -> u64 {
    // IDA 0xac5a00: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xac5e98 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::isScriptable(void)const")]
pub fn stub_ac5e98(d: &GenDesc) -> bool {
    // IDA 0xac5e98: scriptable flag from the descriptor.
    d.scriptable
}
// 0xac5ea0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::isBroadcast(void)const")]
pub fn stub_ac5ea0(d: &GenDesc) -> bool {
    // IDA 0xac5ea0: broadcast flag from the descriptor.
    d.broadcast
}
// 0xac5ea8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ac5ea8(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xac5ea8: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xac6070 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ac6070(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xac6070: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xac6088 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ac6088(s: &mut GenSignalState) {
    // IDA 0xac6088: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xac7cd4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac7cd4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xac7cd4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xac8070 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ac8070(d: GenDesc) {
    // IDA 0xac8070: event descriptor dtor.
    let _ = d;
}
// 0xac80b8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>,rbx::remote_signal<void ()(bool,int)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ac80b8(d: GenDesc) {
    // IDA 0xac80b8: event descriptor dtor.
    let _ = d;
}
// 0xac8194 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac8194(name: &str) -> GenDesc {
    // IDA 0xac8194: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac8424 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_ac8424(d: GenDesc) {
    // IDA 0xac8424: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac84c4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac84c4(args: &[String]) -> Vec<String> {
    // IDA 0xac84c4: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xac86f8 — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FvN5boost10shared_ptrINS_8InstanceEEEES7_vE4callEPS3_S9_RNS0_7VariantERKS7_
// type: void __fastcall(int, char *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_ac86f8(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xac86f8: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xac8970 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbvEbLi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
pub fn stub_ac8970(d: GenDesc) {
    // IDA 0xac8970: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac8a4c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbvEbLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// type: void __fastcall(int, int, int, int *, int *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_ac8a4c(arg: &str) -> String {
    // IDA 0xac8a4c: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xac8cf0 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::PropDescriptor<bool (RBX::Network::Player::*)(void)const,int>(char const*,char const*,bool (RBX::Network::Player::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_ac8cf0(name: &str) -> GenDesc {
    // IDA 0xac8cf0: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac8f04 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::~PropDescriptor()")]
pub fn stub_ac8f04(d: GenDesc) {
    // IDA 0xac8f04: prop descriptor dtor.
    let _ = d;
}
// 0xac8f2c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetImpl<bool (RBX::Network::Player::*)(void)const>::isReadOnly(void)const")]
pub fn stub_ac8f2c(d: &GenDesc) -> bool {
    // IDA 0xac8f2c: read-only when no setter was installed.
    !d.writable
}
// 0xac8f30 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetImpl<bool (RBX::Network::Player::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_ac8f30(d: &GenDesc) -> bool {
    // IDA 0xac8f30: write-only when no getter was installed.
    !d.readable
}
// 0xac8f34 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetImpl<bool (RBX::Network::Player::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_ac8f34(d: &GenDesc) -> bool {
    // IDA 0xac8f34: virtual getter dispatch (this-36 adjust); returns the flag.
    d.value != 0
}
// 0xac8f58 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetImpl<bool (RBX::Network::Player::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_ac8f58(d: &mut GenDesc, v: bool) {
    // IDA 0xac8f58: virtual setter dispatch; stores the flag.
    d.value = i32::from(v);
}
// 0xac9078 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::PropDescriptor<int (RBX::Network::Player::*)(void)const,int>(char const*,char const*,int (RBX::Network::Player::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_ac9078(name: &str) -> GenDesc {
    // IDA 0xac9078: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac928c — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::~PropDescriptor()")]
pub fn stub_ac928c(d: GenDesc) {
    // IDA 0xac928c: prop descriptor dtor.
    let _ = d;
}
// 0xac92b4 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetImpl<int (RBX::Network::Player::*)(void)const>::isReadOnly(void)const")]
pub fn stub_ac92b4(d: &GenDesc) -> bool {
    // IDA 0xac92b4: read-only when no setter was installed.
    !d.writable
}
// 0xac92b8 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetImpl<int (RBX::Network::Player::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_ac92b8(d: &GenDesc) -> bool {
    // IDA 0xac92b8: write-only when no getter was installed.
    !d.readable
}
// 0xac92bc — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetImpl<int (RBX::Network::Player::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_ac92bc(d: &GenDesc) -> i32 {
    // IDA 0xac92bc: virtual getter dispatch; returns the scalar.
    d.value
}
// 0xac92e0 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetImpl<int (RBX::Network::Player::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_ac92e0(d: &mut GenDesc, v: i32) {
    // IDA 0xac92e0: virtual setter dispatch; stores the scalar.
    d.value = v;
}
// 0xac9400 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbiEbLi1EEC2EMS3_FviN5boost8functionIFvbEEENS7_IFvSsEEEEPKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac9400(name: &str) -> GenDesc {
    // IDA 0xac9400: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac966c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbiEbLi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::~BoundYieldFuncDesc()")]
pub fn stub_ac966c(d: GenDesc) {
    // IDA 0xac966c: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac9768 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// type: void __fastcall(int, int, int, int *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_ac9768(arg: &str) -> String {
    // IDA 0xac9768: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xac9a24 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFSsvESsLi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::~BoundYieldFuncDesc()")]
pub fn stub_ac9a24(d: GenDesc) {
    // IDA 0xac9a24: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xac9b00 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFSsvESsLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// type: void __fastcall(int, int, int, int *, int *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_ac9b00(arg: &str) -> String {
    // IDA 0xac9b00: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xac9da4 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, __int64, int, int, char, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::PropDescriptor<int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int)>(char const*,char const*,int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_ac9da4(name: &str) -> GenDesc {
    // IDA 0xac9da4: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xac9fcc — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE10GetSetImplIMS3_KFivEMS3_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetSetImpl<int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int)>::isReadOnly(void)const")]
pub fn stub_ac9fcc(d: &GenDesc) -> bool {
    // IDA 0xac9fcc: read-only when no setter was installed.
    !d.writable
}
// 0xac9fd0 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE10GetSetImplIMS3_KFivEMS3_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetSetImpl<int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int)>::isWriteOnly(void)const")]
pub fn stub_ac9fd0(d: &GenDesc) -> bool {
    // IDA 0xac9fd0: write-only when no getter was installed.
    !d.readable
}
// 0xac9fd4 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE10GetSetImplIMS3_KFivEMS3_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetSetImpl<int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_ac9fd4(d: &GenDesc) -> i32 {
    // IDA 0xac9fd4: virtual getter dispatch; returns the scalar.
    d.value
}
// 0xac9ff8 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiE10GetSetImplIMS3_KFivEMS3_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::GetSetImpl<int (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_ac9ff8(d: &mut GenDesc, v: i32) {
    // IDA 0xac9ff8: virtual setter dispatch; stores the scalar.
    d.value = v;
}
// 0xaca020 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, __int64, int, int, char, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::PropDescriptor<bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool)>(char const*,char const*,bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_aca020(name: &str) -> GenDesc {
    // IDA 0xaca020: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaca248 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE10GetSetImplIMS3_KFbvEMS3_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetSetImpl<bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool)>::isReadOnly(void)const")]
pub fn stub_aca248(d: &GenDesc) -> bool {
    // IDA 0xaca248: read-only when no setter was installed.
    !d.writable
}
// 0xaca24c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE10GetSetImplIMS3_KFbvEMS3_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetSetImpl<bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_aca24c(d: &GenDesc) -> bool {
    // IDA 0xaca24c: write-only when no getter was installed.
    !d.readable
}
// 0xaca250 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetSetImpl<bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aca250(d: &GenDesc) -> bool {
    // IDA 0xaca250: virtual getter dispatch (this-36 adjust); returns the flag.
    d.value != 0
}
// 0xaca274 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::GetSetImpl<bool (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_aca274(d: &mut GenDesc, v: bool) {
    // IDA 0xaca274: virtual setter dispatch; stores the flag.
    d.value = i32::from(v);
}
// 0xaca29c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_aca29c(d: GenDesc) {
    // IDA 0xaca29c: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xaca378 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_aca378(args: &[String]) -> Vec<String> {
    // IDA 0xaca378: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xaca39c — __ZN5boost4bindIvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS1_INS2_9DataModelEEESsS5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_T2_ENS8_9list_av_3IT3_T4_T5_E4typeEEESF_SH_SI_SJ_
// type: void __fastcall(_DWORD *, int, std::string *, pthread_mutex_t *, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list_av_3<std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>::type> boost::bind<void,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>(void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_aca39c() {
    // IDA 0xaca39c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xaca760 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::~list3()")]
pub fn stub_aca760() {
    // IDA 0xaca760: dtor releases the owned control block/slots.
}
// 0xacaa30 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS8_INS2_8InstanceEEESaISB_EEEESsbdS5_NS_3argILi1EEENSF_ILi2EEESsbdEENS_3_bi6bind_tIT_PFSK_T0_T1_T2_T3_T4_T5_ENSI_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESS_SU_SV_SW_SX_SY_SZ_
// type: void __fastcall(int, int, int *, const std::string *, int, double)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)")]
pub fn stub_acaa30() {
    // IDA 0xacaa30: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xacadf0 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEENS2_IdEEEC2ES8_SA_SB_SC_SD_SE_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *, unsigned __int8, int, int)
#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>)")]
pub fn stub_acadf0() -> Option<u32> {
    // IDA 0xacadf0: nullable object query (id when live, None when unset).
    None
}
// 0xacb054 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEENS2_IdEEEC2ES8_SA_SB_SC_SD_SE_
// type: int __fastcall(int, int *, const std::string *, unsigned __int8, int, int)
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>)")]
pub fn stub_acb054() -> Option<u32> {
    // IDA 0xacb054: nullable object query (id when live, None when unset).
    None
}
// 0xacb2b8 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEEEC2ES8_SA_SB_SC_SD_
// type: int __fastcall(int, int *, const std::string *, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
pub fn stub_acb2b8(slot: &mut GenFunctor) {
    // IDA 0xacb2b8: packs the bound argument list.
    slot.has = true;
}
// 0xacb514 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEEEC2ES8_SA_SB_SC_
// type: int __fastcall(int, int *, const std::string *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
pub fn stub_acb514(slot: &mut GenFunctor) {
    // IDA 0xacb514: packs the bound argument list.
    slot.has = true;
}
// 0xacb754 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_acb754(slot: &mut GenFunctor) {
    // IDA 0xacb754: packs the bound argument list.
    slot.has = true;
}
// 0xacb914 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS9_INS3_8InstanceEEESaISC_EEEESsbdENS0_5list6INS0_5valueIS6_EENS_3argILi1EEENSL_ILi2EEENSJ_ISsEENSJ_IbEENSJ_IdEEEEEC2ESH_RKSR_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)")]
pub fn stub_acb914() {
    // IDA 0xacb914: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xacbfd8 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSD_5list6INSD_5valueISI_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)")]
pub fn stub_acbfd8(slot: &mut GenFunctor) -> bool {
    // IDA 0xacbfd8: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xacc888 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_acc888(slot: &mut GenFunctor, op: u32) {
    // IDA 0xacc888: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xacc8ac — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEvSB_SI_E6invokeERNS1_15function_bufferESB_SI_
// type: int __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_acc8ac(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xacc8ac: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xacc8c8 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEENS2_IdEEEclIPFvS7_NS4_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSJ_INS4_8InstanceEEESaISM_EEEESsbdENS0_5list2IRSI_RSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, pthread_mutex_t **, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")]
pub fn stub_acc8c8(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xacc8c8: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaccd24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_accd24(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaccd24: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaccffc — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ES3_S9_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const std::string *, int *, int *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
pub fn stub_accffc(slot: &mut GenFunctor) {
    // IDA 0xaccffc: packs the bound argument list.
    slot.has = true;
}
// 0xacd390 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ES3_S9_SC_
// type: int __fastcall(int, const std::string *, int *, _DWORD *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
pub fn stub_acd390(slot: &mut GenFunctor) {
    // IDA 0xacd390: packs the bound argument list.
    slot.has = true;
}
// 0xacd74c — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEEEC2ES3_S9_
// type: std::string *__fastcall(std::string *, const std::string *, _DWORD *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
pub fn stub_acd74c(slot: &mut GenFunctor) {
    // IDA 0xacd74c: packs the bound argument list.
    slot.has = true;
}
// 0xacd910 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ERKSD_
// type: std::string *__fastcall(std::string *, const std::string *, int, int, char, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>> const&)")]
pub fn stub_acd910(slot: &mut GenFunctor) {
    // IDA 0xacd910: packs the bound argument list.
    slot.has = true;
}
// 0xacdbac — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvSsNS_8weak_ptrINS1_7Network6PlayerEEENS8_IS2_EEENS6_5list3INS6_5valueISsEENSG_ISB_EENSG_ISC_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>)")]
pub fn stub_acdbac(slot: &mut GenFunctor) -> bool {
    // IDA 0xacdbac: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xacdd2c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_acdd2c(slot: &mut GenFunctor, op: u32) {
    // IDA 0xacdd2c: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xacdd50 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEvPSA_E6invokeERNS1_15function_bufferESL_
// type: int __fastcall(int *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_acdd50(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xacdd50: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xacdd6c — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEclIPFvSsS8_SB_ENS0_5list1IRPSA_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(struct _Unwind_Exception **, void (__fastcall **)(int *, struct _Unwind_Exception **, struct _Unwind_Exception **))
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::operator()<void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_acdd6c(fire: &dyn Fn(Option<u32>), player: Option<u32>) {
    // IDA 0xacdd6c: bind/call thunk forwards the locked weak player (mf1).
    fire(player);
}
// 0xace0fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, struct _Unwind_Exception *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_ace0fc(slot: &mut GenFunctor, op: u32) {
    // IDA 0xace0fc: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xace240 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_ace240(d: GenDesc) {
    // IDA 0xace240: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xace390 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_ace390(d: GenDesc) {
    // IDA 0xace390: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xace570 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsSsELi2EED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
pub fn stub_ace570(d: GenDesc) {
    // IDA 0xace570: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xace664 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_ace664(d: GenDesc) {
    // IDA 0xace664: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xad0f88 — __ZN3RBX7Network6MarkerC1Ev
// type: int __fastcall(RBX::Network::Marker *this)
#[doc(alias = "RBX::Network::Marker::Marker(void)")]
pub fn stub_ad0f88() -> GenMarker {
    // IDA 0xad0f88: marker with an empty returned-signal.
    GenMarker::default()
}
// 0xad0f94 — __ZN3RBX7Network6MarkerC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::Marker *this)
#[doc(alias = "RBX::Network::Marker::Marker(void)")]
pub fn stub_ad0f94() -> GenMarker {
    // IDA 0xad0f94: marker with an empty returned-signal.
    GenMarker::default()
}
// 0xad12d0 — __ZN3RBX7Network6Marker12fireReturnedEv
// type: int __fastcall(RBX::Network::Marker *this)
#[doc(alias = "RBX::Network::Marker::fireReturned(void)")]
pub fn stub_ad12d0(m: &mut GenMarker) {
    // IDA 0xad12d0: emits the returned signal (signal_with_args<0> at +100).
    m.returned = true;
    m.fired = m.fired.wrapping_add(1);
}
// 0xad12dc — __ZN3RBX10Reflection9EventDescINS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::~EventDesc()")]
pub fn stub_ad12dc(d: GenDesc) {
    // IDA 0xad12dc: event descriptor dtor.
    let _ = d;
}
// 0xad1324 — __ZN3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *this, int, int, int)
#[doc(alias = "RBX::Network::Marker::~Marker()")]
pub fn stub_ad1324(m: GenMarker) {
    // IDA 0xad1324: marker dtor.
    let _ = m;
}
// 0xad14a4 — __ZN3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
#[doc(alias = "RBX::Network::Marker::~Marker()")]
pub fn stub_ad14a4(m: GenMarker) {
    // IDA 0xad14a4: marker dtor.
    let _ = m;
}
// 0xad1640 — __ZThn32_N3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
pub fn stub_ad1640(fire: &dyn Fn()) {
    // IDA 0xad1640: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad164c — __ZThn32_N3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
pub fn stub_ad164c(fire: &dyn Fn()) {
    // IDA 0xad164c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad17ec — __ZThn36_N3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
pub fn stub_ad17ec(fire: &dyn Fn()) {
    // IDA 0xad17ec: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad17f8 — __ZThn36_N3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
pub fn stub_ad17f8(fire: &dyn Fn()) {
    // IDA 0xad17f8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad1b78 — __ZN3RBX10Reflection9EventDescINS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::~EventDesc()")]
pub fn stub_ad1b78(d: GenDesc) {
    // IDA 0xad1b78: event descriptor dtor.
    let _ = d;
}
// 0xad1c54 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, void *, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ad1c54(s: &mut GenSignalState) -> u64 {
    // IDA 0xad1c54: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xad2464 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: int __fastcall(int, int, __int64)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ad2464(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xad2464: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xad24d8 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ad24d8(s: &mut GenSignalState) {
    // IDA 0xad24d8: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xad24f0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6MarkerEFvvEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Marker,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Network::Marker::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(void)> const&)const")]
pub fn stub_ad24f0(s: &mut GenSignalState) -> u64 {
    // IDA 0xad24f0: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xad2e80 — __ZN3RBX7Network4Peer20setOutgoingKBPSLimitEi
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, int)
#[doc(alias = "RBX::Network::Peer::setOutgoingKBPSLimit(int)")]
pub fn stub_ad2e80(p: &mut GenPeer, kbps: i32) -> i32 {
    // BUG: original at 0xad2e80 ignores the limit argument and returns the current value; this stores positive limits.
    if kbps > 0 { p.kbps = kbps; }
    p.kbps
}
// 0xad2ec0 — __ZN3RBX7Network4PeerC2Ev
// type: RBX::Network::Peer *__fastcall(RBX::Network::Peer *this)
#[doc(alias = "RBX::Network::Peer::Peer(void)")]
pub fn stub_ad2ec0() -> GenPeer {
    // IDA 0xad2ec0: peer with default limits, disconnected.
    GenPeer::default()
}
// 0xad31f0 — __ZN3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "RBX::Network::Peer::~Peer()")]
pub fn stub_ad31f0(p: GenPeer) {
    // IDA 0xad31f0: peer dtor closes the RakPeer.
    let _ = p;
}
// 0xad3290 — __ZN3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "RBX::Network::Peer::~Peer()")]
pub fn stub_ad3290(p: GenPeer) {
    // IDA 0xad3290: peer dtor closes the RakPeer.
    let _ = p;
}
// 0xad329c — __ZThn32_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad329c(fire: &dyn Fn()) {
    // IDA 0xad329c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad3340 — __ZThn36_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad3340(fire: &dyn Fn()) {
    // IDA 0xad3340: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad33e4 — __ZThn92_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad33e4(fire: &dyn Fn()) {
    // IDA 0xad33e4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad3488 — __ZN3RBX7Network4PeerD2Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "RBX::Network::Peer::~Peer()")]
pub fn stub_ad3488(p: GenPeer) {
    // IDA 0xad3488: peer dtor closes the RakPeer.
    let _ = p;
}
// 0xad365c — __ZThn32_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad365c(fire: &dyn Fn()) {
    // IDA 0xad365c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad3668 — __ZThn36_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad3668(fire: &dyn Fn()) {
    // IDA 0xad3668: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad3674 — __ZThn92_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
pub fn stub_ad3674(fire: &dyn Fn()) {
    // IDA 0xad3674: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad3680 — __ZN3RBX7Network4Peer15onCreateRakPeerEv
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this)
#[doc(alias = "RBX::Network::Peer::onCreateRakPeer(void)")]
pub fn stub_ad3680(p: &mut GenPeer) {
    // IDA 0xad3680: allocates the RakPeer and marks the transport live.
    p.connected = true;
}
// 0xad3838 — __ZNK3RBX7Network4Peer11askAddChildEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Peer *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Peer::askAddChild(RBX::Instance const*)const")]
pub fn stub_ad3838(p: &GenPeer) -> bool {
    // IDA 0xad3838: peers accept any instance child.
    let _ = p;
    true
}
// 0xad3984 — __ZN3RBX7Network4Peer17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(struct _Unwind_Exception *this, RBX::ServiceProvider *, pthread_mutex_t *, int)
#[doc(alias = "RBX::Network::Peer::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_ad3984(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xad3984: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xad51f0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network4PeerEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Peer,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_ad51f0(d: GenDesc) {
    // IDA 0xad51f0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xad525c — __ZN5boost10shared_ptrIN3RBX7Network16PacketReceiveJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PacketReceiveJob>::reset(void)")]
pub fn stub_ad525c(slot: &mut Option<u32>) {
    // IDA 0xad525c: releases the owned ref (intrusive release engine-side).
    *slot = None;
}
// 0xad532c — __ZN3RBX7Network13PeerStatsItemC2EPNS0_4PeerE
// type: RBX::Network::PeerStatsItem *__fastcall(RBX::Network::PeerStatsItem *this, RBX::Network::Peer *)
#[doc(alias = "RBX::Network::PeerStatsItem::PeerStatsItem(RBX::Network::Peer *)")]
pub fn stub_ad532c(peer: &GenPeer) -> GenStats {
    // IDA 0xad532c: stats item bound to the peer.
    let _ = peer;
    GenStats::default()
}
// 0xad560c — __ZN3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "RBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad560c(s: GenStats) {
    // IDA 0xad560c: stats item dtor.
    let _ = s;
}
// 0xad5680 — __ZN3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "RBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad5680(s: GenStats) {
    // IDA 0xad5680: stats item dtor.
    let _ = s;
}
// 0xad5790 — __ZN3RBX7Network13PeerStatsItem6updateEv
// type: void __fastcall(RBX::Network::PeerStatsItem *this)
#[doc(alias = "RBX::Network::PeerStatsItem::update(void)")]
pub fn stub_ad5790(s: &mut GenStats, bytes: u64) {
    // IDA 0xad5790: samples RakNet stats (buffer health, loss) into the items.
    s.packets = s.packets.wrapping_add(1);
    s.bytes = s.bytes.wrapping_add(bytes);
}
// 0xad5a58 — __ZThn32_N3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad5a58(fire: &dyn Fn()) {
    // IDA 0xad5a58: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad5ad8 — __ZThn32_N3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad5ad8(fire: &dyn Fn()) {
    // IDA 0xad5ad8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad5be8 — __ZThn36_N3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad5be8(fire: &dyn Fn()) {
    // IDA 0xad5be8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad5c68 — __ZThn36_N3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
pub fn stub_ad5c68(fire: &dyn Fn()) {
    // IDA 0xad5c68: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xad5d78 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network13PeerStatsItemES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::PeerStatsItem,RBX::Network::PeerStatsItem>(rbx_core::SharedPtr<RBX::Network::PeerStatsItem> const*,RBX::Network::PeerStatsItem *)const")]
pub fn stub_ad5d78(has_weak: bool) -> bool {
    // IDA 0xad5d78: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xad6034 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_ad6034() {
    // IDA 0xad6034: counted-impl dtor frees the control block.
}
// 0xad6038 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_ad6038() {
    // IDA 0xad6038: counted-impl dtor frees the control block.
}
// 0xad6044 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_ad6044() -> Option<u32> {
    // IDA 0xad6044: nullable object query (id when live, None when unset).
    None
}
// 0xad6060 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_ad6060() -> bool {
    // IDA 0xad6060: deleter query misses for this control block.
    false
}
// 0xad6078 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_ad6078() -> bool {
    // IDA 0xad6078: deleter query misses for this control block.
    false
}
// 0xad607c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network16PacketReceiveJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(rbx_core::SharedPtr<RBX::Network::PacketReceiveJob> *,RBX::Network::PacketReceiveJob *,boost::detail::shared_count &)")]
pub fn stub_ad607c(slot: &mut Option<u32>, v: u32) {
    // IDA 0xad607c: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xad622c — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16PacketReceiveJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(rbx_core::SharedPtr<RBX::Network::PacketReceiveJob> const*,RBX::Network::PacketReceiveJob *)const")]
pub fn stub_ad622c(has_weak: bool) -> bool {
    // IDA 0xad622c: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xad64d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::~sp_counted_impl_p()")]
pub fn stub_ad64d8() {
    // IDA 0xad64d8: counted-impl dtor frees the control block.
}
// 0xad64dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::~sp_counted_impl_p()")]
pub fn stub_ad64dc() {
    // IDA 0xad64dc: counted-impl dtor frees the control block.
}
// 0xad64e8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::dispose(void)")]
pub fn stub_ad64e8() -> Option<u32> {
    // IDA 0xad64e8: nullable object query (id when live, None when unset).
    None
}
// 0xad64fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::get_deleter(std::type_info const&)")]
pub fn stub_ad64fc() -> bool {
    // IDA 0xad64fc: deleter query misses for this control block.
    false
}
// 0xad6500 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::get_untyped_deleter(void)")]
pub fn stub_ad6500() -> bool {
    // IDA 0xad6500: deleter query misses for this control block.
    false
}
// 0xad6504 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network17ConcurrentRakPeerES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer,RBX::Network::ConcurrentRakPeer>(rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer> *,RBX::Network::ConcurrentRakPeer *,boost::detail::shared_count &)")]
pub fn stub_ad6504(slot: &mut Option<u32>, v: u32) {
    // IDA 0xad6504: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xad66ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::~sp_counted_impl_p()")]
pub fn stub_ad66ac() {
    // IDA 0xad66ac: counted-impl dtor frees the control block.
}
// 0xad66b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::~sp_counted_impl_p()")]
pub fn stub_ad66b0() {
    // IDA 0xad66b0: counted-impl dtor frees the control block.
}
// 0xad66bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::dispose(void)")]
pub fn stub_ad66bc() -> Option<u32> {
    // IDA 0xad66bc: nullable object query (id when live, None when unset).
    None
}
// 0xad6760 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::get_deleter(std::type_info const&)")]
pub fn stub_ad6760() -> bool {
    // IDA 0xad6760: deleter query misses for this control block.
    false
}
// 0xad6764 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::get_untyped_deleter(void)")]
pub fn stub_ad6764() -> bool {
    // IDA 0xad6764: deleter query misses for this control block.
    false
}
// 0xad6974 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network4PeerEFviELi1EEC2EMS3_FviEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Peer,void ()(int),1>::BoundFuncDesc(void (RBX::Network::Peer::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ad6974(name: &str) -> GenDesc {
    // IDA 0xad6974: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xad6be0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network4PeerEFviELi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Peer,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_ad6be0(d: GenDesc) {
    // IDA 0xad6be0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xad6cdc — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network4PeerEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Peer,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ad6cdc(args: &[String]) -> Vec<String> {
    // IDA 0xad6cdc: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xad6d1c — __ZN3RBX7Network16PacketReceiveJobC2EN5boost10shared_ptrINS0_17ConcurrentRakPeerEEEPNS_9DataModelE
// type: int __fastcall(int, int, RBX::Instance *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, RBX::TaskScheduler::Job *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::PacketReceiveJob::PacketReceiveJob(rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>,RBX::DataModel *)")]
pub fn stub_ad6d1c(peer: u32) -> GenJob {
    // IDA 0xad6d1c: receive job bound to the concurrent RakPeer.
    GenJob { owner: peer, running: true }
}
// 0xad744c — __ZN3RBX7Network16PacketReceiveJobD1Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *__hidden this)
#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
pub fn stub_ad744c(j: GenJob) {
    // IDA 0xad744c: job dtor.
    let _ = j;
}
// 0xad7458 — __ZN3RBX7Network16PacketReceiveJobD0Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *__hidden this)
#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
pub fn stub_ad7458(j: GenJob) {
    // IDA 0xad7458: job dtor.
    let _ = j;
}
// 0xad74f8 — __ZN3RBX7Network16PacketReceiveJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PacketReceiveJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PacketReceiveJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ad74f8(queue: usize) -> f64 {
    // IDA 0xad74f8: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xad7514 — __ZN3RBX7Network16PacketReceiveJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::PacketReceiveJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ad7514() -> Option<u32> {
    // IDA 0xad7514: nullable object query (id when live, None when unset).
    None
}
// 0xad7534 — __ZN3RBX7Network16PacketReceiveJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::PacketReceiveJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::PacketReceiveJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ad7534() -> Option<u32> {
    // IDA 0xad7534: nullable object query (id when live, None when unset).
    None
}
// 0xad79a4 — __ZN3RBX7Network16PacketReceiveJobD2Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *this, int, int)
#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
pub fn stub_ad79a4(j: GenJob) {
    // IDA 0xad79a4: job dtor.
    let _ = j;
}
// 0xad7bb0 — __ZN3RBX7Network15ProfiledRakPeerD1Ev
// type: void __fastcall(RBX::Network::ProfiledRakPeer *__hidden this)
#[doc(alias = "RBX::Network::ProfiledRakPeer::~ProfiledRakPeer()")]
pub fn stub_ad7bb0() {
    // IDA 0xad7bb0: dtor releases the owned control block/slots.
}
// 0xad7bbc — __ZN3RBX7Network15ProfiledRakPeerD0Ev
// type: void __fastcall(RBX::Network::ProfiledRakPeer *__hidden this)
#[doc(alias = "RBX::Network::ProfiledRakPeer::~ProfiledRakPeer()")]
pub fn stub_ad7bbc() {
    // IDA 0xad7bbc: dtor releases the owned control block/slots.
}
// 0xad7c5c — __ZN3RBX7Network15ProfiledRakPeer14RunUpdateCycleEyy
// type: int __fastcall(RBX::Network::ProfiledRakPeer *this, unsigned __int64, unsigned __int64)
#[doc(alias = "RBX::Network::ProfiledRakPeer::RunUpdateCycle(unsigned long long,unsigned long long)")]
pub fn stub_ad7c5c(start: u64, end: u64) -> u64 {
    // IDA 0xad7c5c: runs one RakPeer update slice; returns elapsed.
    end.saturating_sub(start)
}
