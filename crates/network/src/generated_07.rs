//! network generated_07 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 120 stubs here, 3759 combined, 1038 remaining).
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


// 0xb2ddc8 — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::PropDescriptor<int (RBX::Network::Replicator::*)(void)const,int>(char const*,char const*,int (RBX::Network::Replicator::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_b2ddc8(name: &str) -> GenDesc {
    // IDA 0xb2ddc8: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb2dfdc — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::~PropDescriptor()")]
pub fn stub_b2dfdc(d: GenDesc) {
    // IDA 0xb2dfdc: prop descriptor dtor.
    let _ = d;
}
// 0xb2e188 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::GetImpl<int (RBX::Network::Replicator::*)(void)const>::isReadOnly(void)const")]
pub fn stub_b2e188(d: &GenDesc) -> bool {
    // IDA 0xb2e188: read-only when no setter was installed.
    !d.writable
}
// 0xb2e18c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::GetImpl<int (RBX::Network::Replicator::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_b2e18c(d: &GenDesc) -> bool {
    // IDA 0xb2e18c: write-only when no getter was installed.
    !d.readable
}
// 0xb2e190 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::GetImpl<int (RBX::Network::Replicator::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_b2e190(d: &GenDesc) -> i32 {
    // IDA 0xb2e190: virtual getter dispatch; returns the scalar.
    d.value
}
// 0xb2e1b4 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::GetImpl<int (RBX::Network::Replicator::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_b2e1b4(d: &mut GenDesc, v: i32) {
    // IDA 0xb2e1b4: virtual setter dispatch; stores the scalar.
    d.value = v;
}
// 0xb2e2d4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvdELi1EEC2EMS3_FvdEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(double),1>::BoundFuncDesc(void (RBX::Network::Replicator::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_b2e2d4(name: &str) -> GenDesc {
    // IDA 0xb2e2d4: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb2e540 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvdELi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(double),1>::~BoundFuncDesc()")]
pub fn stub_b2e540(d: GenDesc) {
    // IDA 0xb2e540: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb2e63c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_b2e63c(args: &[String]) -> Vec<String> {
    // IDA 0xb2e63c: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xb2e67c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFSsiELi1EEC2EMS3_FSsiEPKcS9_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,std::string ()(int),1>::BoundFuncDesc(std::string (RBX::Network::Replicator::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_b2e67c(name: &str) -> GenDesc {
    // IDA 0xb2e67c: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb2e970 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFSsiELi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,std::string ()(int),1>::~BoundFuncDesc()")]
pub fn stub_b2e970(d: GenDesc) {
    // IDA 0xb2e970: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb2ea6c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFSsiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,std::string ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_b2ea6c(arg: &str) -> String {
    // IDA 0xb2ea6c: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xb2ec78 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b2ec78(d: GenDesc) {
    // IDA 0xb2ec78: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb2ed54 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_b2ed54(args: &[String]) -> Vec<String> {
    // IDA 0xb2ed54: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xb2ed78 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b2ed78(d: GenDesc) {
    // IDA 0xb2ed78: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb2ee54 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_b2ee54(args: &[String]) -> Vec<String> {
    // IDA 0xb2ee54: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xb2f098 — __ZN3RBX10Reflection9EventDescINS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::EventDesc(rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_b2f098(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xb2f098: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xb2f434 — __ZN3RBX10Reflection9EventDescINS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::~EventDesc()")]
pub fn stub_b2f434(d: GenDesc) {
    // IDA 0xb2f434: event descriptor dtor.
    let _ = d;
}
// 0xb2f510 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_b2f510(s: &mut GenSignalState) -> u64 {
    // IDA 0xb2f510: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xb2f994 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_b2f994(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xb2f994: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xb2fc00 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_b2fc00(s: &mut GenSignalState) {
    // IDA 0xb2fc00: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xb2fdd0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,bool)> const&)const")]
pub fn stub_b2fdd0(s: &mut GenSignalState) -> u64 {
    // IDA 0xb2fdd0: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xb32510 — __ZN3RBX7Network10Replicator7PingJobC2ERS1_
// type: RBX::Network::Replicator::PingJob *__fastcall(RBX::Network::Replicator::PingJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::Replicator::PingJob::PingJob(RBX::Network::Replicator&)")]
pub fn stub_b32510() -> Option<u32> {
    // IDA 0xb32510: nullable object query (id when live, None when unset).
    None
}
// 0xb32864 — __ZN3RBX7Network10Replicator7PingJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
pub fn stub_b32864() {
    // IDA 0xb32864: dtor releases the owned control block/slots.
}
// 0xb32930 — __ZN3RBX7Network10Replicator7PingJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
pub fn stub_b32930() {
    // IDA 0xb32930: dtor releases the owned control block/slots.
}
// 0xb32a10 — __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b32a10(queue: usize) -> f64 {
    // IDA 0xb32a10: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xb32a2c — __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b32a2c() -> Option<u32> {
    // IDA 0xb32a2c: nullable object query (id when live, None when unset).
    None
}
// 0xb32a48 — __ZN3RBX7Network10Replicator7PingJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::Replicator::PingJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b32a48() -> Option<u32> {
    // IDA 0xb32a48: nullable object query (id when live, None when unset).
    None
}
// 0xb32b18 — __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_
// type: RBX::Network::Replicator::ProcessPacketsJob *__fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::ProcessPacketsJob(RBX::Network::Replicator&)")]
pub fn stub_b32b18() -> Option<u32> {
    // IDA 0xb32b18: nullable object query (id when live, None when unset).
    None
}
// 0xb32ed4 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
pub fn stub_b32ed4() {
    // IDA 0xb32ed4: dtor releases the owned control block/slots.
}
// 0xb32fa0 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
pub fn stub_b32fa0() {
    // IDA 0xb32fa0: dtor releases the owned control block/slots.
}
// 0xb33080 — __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b33080(queue: usize) -> f64 {
    // IDA 0xb33080: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xb33128 — __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b33128() -> Option<u32> {
    // IDA 0xb33128: nullable object query (id when live, None when unset).
    None
}
// 0xb33300 — __ZN3RBX7Network10Replicator17ProcessPacketsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b33300() -> Option<u32> {
    // IDA 0xb33300: nullable object query (id when live, None when unset).
    None
}
// 0xb33f20 — __ZN3RBX7Network10Replicator12JoinDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
pub fn stub_b33f20() {
    // IDA 0xb33f20: dtor releases the owned control block/slots.
}
// 0xb34b1c — __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE
// type: void()
#[doc(alias = "RBX::Network::PhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
pub fn stub_b34b1c() {
    // IDA 0xb34b1c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb34b20 — __ZN3RBX7Network21DirectPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
pub fn stub_b34b20() {
    // IDA 0xb34b20: dtor releases the owned control block/slots.
}
// 0xb34b44 — __ZN3RBX7Network21DirectPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
pub fn stub_b34b44() {
    // IDA 0xb34b44: dtor releases the owned control block/slots.
}
// 0xb34f70 — __ZN3RBX7Network15ReplicatorStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats *__hidden this)
#[doc(alias = "RBX::Network::ReplicatorStats::~ReplicatorStats()")]
pub fn stub_b34f70() {
    // IDA 0xb34f70: dtor releases the owned control block/slots.
}
// 0xb35228 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *__hidden this)
#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::~PhysicsSenderStats()")]
pub fn stub_b35228() {
    // IDA 0xb35228: dtor releases the owned control block/slots.
}
// 0xb353e4 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_N5boost10shared_ptrINS0_7Network22SharedStringDictionaryEEEESt10_Select1stISC_ESt4lessIS4_ESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>> *)")]
pub fn stub_b353e4(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0xb353e4: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0xb35414 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_N5boost10shared_ptrINS0_7Network31SharedStringProtectedDictionaryEEEESt10_Select1stISC_ESt4lessIS4_ESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringProtectedDictionary>>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringProtectedDictionary>>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringProtectedDictionary>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringProtectedDictionary>>> *)")]
pub fn stub_b35414(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0xb35414: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0xb35444 — __ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_N5boost10shared_ptrINS0_7Network22SharedStringDictionaryEEEESt10_Select1stISC_ESt4lessIS4_ESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::EventDescriptor const* const,rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>>> *)")]
pub fn stub_b35444(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0xb35444: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0xb36628 — __ZN3RBX7Network19PersistentDataStoreC1EPKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEPKNS0_7PlayersEi
// type: int()
#[doc(alias = "RBX::Network::PersistentDataStore::PersistentDataStore(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const*,RBX::Network::Players const*,int)")]
pub fn stub_b36628() {
    // IDA 0xb36628: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36634 — __ZN3RBX7Network19PersistentDataStoreC2EPKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEPKNS0_7PlayersEi
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::PersistentDataStore::PersistentDataStore(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const*,RBX::Network::Players const*,int)")]
pub fn stub_b36634() {
    // IDA 0xb36634: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb367c4 — __ZN3RBX7NetworkL20computeValueMapLimitERKSt4pairISsNS_10Reflection7VariantEEPi
#[doc(alias = "RBX::Network::computeValueMapLimit(std::pair<std::string,RBX::Reflection::Variant> const&,int *)")]
pub fn stub_b367c4() {
    // IDA 0xb367c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb367d8 — __ZN3RBX7Network19PersistentDataStore17serializeValueMapERSsRKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEE
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::Network::PersistentDataStore::serializeValueMap(std::string &,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const&)")]
pub fn stub_b367d8() {
    // IDA 0xb367d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36ae0 — __ZN3RBX7Network19PersistentDataStore15saveLeaderboardERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::saveLeaderboard(std::string &)")]
pub fn stub_b36ae0() {
    // IDA 0xb36ae0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36cd8 — __ZN3RBX7Network19PersistentDataStore9getNumberERKSs
// type: __int64 __fastcall(RBX::Network::PersistentDataStore *this, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::getNumber(std::string const&)")]
pub fn stub_b36cd8() {
    // IDA 0xb36cd8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36dc0 — __ZN3RBX7Network19PersistentDataStore4saveERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::save(std::string &)")]
pub fn stub_b36dc0() {
    // IDA 0xb36dc0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36dd0 — __ZN3RBX7Network19PersistentDataStore18setComplexityLimitEi
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Network::PersistentDataStore::setComplexityLimit(int)")]
pub fn stub_b36dd0() {
    // IDA 0xb36dd0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36dd4 — __ZN3RBX7Network19PersistentDataStore9removeKeyERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::removeKey(std::string const&)")]
pub fn stub_b36dd4() {
    // IDA 0xb36dd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb36ee4 — __ZN3RBX7NetworkL12computeLimitERKNS_10Reflection7VariantE
// type: int __fastcall(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::computeLimit(RBX::Reflection::Variant const&)")]
pub fn stub_b36ee4() {
    // IDA 0xb36ee4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb37448 — __ZN3RBX7Network19PersistentDataStore17enforceComplexityERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::enforceComplexity(std::string const&)")]
pub fn stub_b37448() {
    // IDA 0xb37448: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb374c8 — __ZN3RBX7Network19PersistentDataStore8isNumberERKSs
// type: bool __fastcall(int, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::isNumber(std::string const&)")]
pub fn stub_b374c8() {
    // IDA 0xb374c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb37590 — __ZN3RBX7Network19PersistentDataStore9setNumberERKSsd
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, double)
#[doc(alias = "RBX::Network::PersistentDataStore::setNumber(std::string const&,double)")]
pub fn stub_b37590() {
    // IDA 0xb37590: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb376a4 — __ZN3RBX7Network19PersistentDataStore9getStringERKSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::getString(std::string const&)")]
pub fn stub_b376a4() {
    // IDA 0xb376a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb3778c — __ZN3RBX7Network19PersistentDataStore9setStringERKSsS3_
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::setString(std::string const&,std::string const&)")]
pub fn stub_b3778c() {
    // IDA 0xb3778c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb3786c — __ZN3RBX7Network19PersistentDataStore10getBooleanERKSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::getBoolean(std::string const&)")]
pub fn stub_b3786c() {
    // IDA 0xb3786c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb3793c — __ZN3RBX7Network19PersistentDataStore10setBooleanERKSsb
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, int)
#[doc(alias = "RBX::Network::PersistentDataStore::setBoolean(std::string const&,bool)")]
pub fn stub_b3793c() {
    // IDA 0xb3793c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb379f4 — __ZN3RBX7Network19PersistentDataStore11getInstanceERKSs
// type: void __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::getInstance(std::string const&)")]
pub fn stub_b379f4() -> Option<u32> {
    // IDA 0xb379f4: nullable object query (id when live, None when unset).
    None
}
// 0xb37cd4 — __ZN3RBX7Network19PersistentDataStore11setInstanceERKSsN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::Network::PersistentDataStore *, const std::string *, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PersistentDataStore::setInstance(std::string const&,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b37cd4() -> Option<u32> {
    // IDA 0xb37cd4: nullable object query (id when live, None when unset).
    None
}
// 0xb39228 — __ZN3RBX7Network18PhysicsPacketCacheC1Ev
// type: int __fastcall(RBX::Network::PhysicsPacketCache *this)
#[doc(alias = "RBX::Network::PhysicsPacketCache::PhysicsPacketCache(void)")]
pub fn stub_b39228() -> Option<u32> {
    // IDA 0xb39228: nullable object query (id when live, None when unset).
    None
}
// 0xb39234 — __ZN3RBX7Network18PhysicsPacketCacheC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::PhysicsPacketCache *this)
#[doc(alias = "RBX::Network::PhysicsPacketCache::PhysicsPacketCache(void)")]
pub fn stub_b39234() -> Option<u32> {
    // IDA 0xb39234: nullable object query (id when live, None when unset).
    None
}
// 0xb395fc — __ZN3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b395fc() {
    // IDA 0xb395fc: dtor releases the owned control block/slots.
}
// 0xb3969c — __ZN3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b3969c() {
    // IDA 0xb3969c: dtor releases the owned control block/slots.
}
// 0xb396a8 — __ZThn32_N3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b396a8(fire: &dyn Fn()) {
    // IDA 0xb396a8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb3974c — __ZThn36_N3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b3974c(fire: &dyn Fn()) {
    // IDA 0xb3974c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb397f0 — __ZN3RBX7Network18PhysicsPacketCacheD2Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b397f0() {
    // IDA 0xb397f0: dtor releases the owned control block/slots.
}
// 0xb399ec — __ZThn32_N3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b399ec(fire: &dyn Fn()) {
    // IDA 0xb399ec: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb399f8 — __ZThn36_N3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
pub fn stub_b399f8(fire: &dyn Fn()) {
    // IDA 0xb399f8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb39a04 — __ZN3RBX7Network18PhysicsPacketCache6insertEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsPacketCache::insert(RBX::Assembly const*)")]
pub fn stub_b39a04(s: &mut GenSignalState) -> u64 {
    // IDA 0xb39a04: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xb3a434 — __ZN3RBX7Network18PhysicsPacketCache19insertChildAssemblyEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::insertChildAssembly(RBX::Assembly const*)")]
pub fn stub_b3a434() -> Option<u32> {
    // IDA 0xb3a434: nullable object query (id when live, None when unset).
    None
}
// 0xb3ad00 — __ZN3RBX7Network18PhysicsPacketCache6removeEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::remove(RBX::Assembly const*)")]
pub fn stub_b3ad00(s: &mut GenSignalState, id: u64) {
    // IDA 0xb3ad00: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0xb3af80 — __ZN3RBX7Network18PhysicsPacketCache19removeChildAssemblyEPKNS_8AssemblyE
// type: int __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::removeChildAssembly(RBX::Assembly const*)")]
pub fn stub_b3af80() -> Option<u32> {
    // IDA 0xb3af80: nullable object query (id when live, None when unset).
    None
}
// 0xb3b690 — __ZN3RBX7Network18PhysicsPacketCache17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, int, RBX::ServiceProvider *, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_b3b690(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xb3b690: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xb3bd0c — __ZN3RBX7Network18PhysicsPacketCache7addPartERNS_12PartInstanceE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, RBX::Primitive **)
#[doc(alias = "RBX::Network::PhysicsPacketCache::addPart(RBX::PartInstance &)")]
pub fn stub_b3bd0c() -> Option<u32> {
    // IDA 0xb3bd0c: nullable object query (id when live, None when unset).
    None
}
// 0xb3bd24 — __ZN3RBX7Network18PhysicsPacketCache16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, _DWORD *, int, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::onAddingAssembly(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b3bd24(top: &mut GenTopN, inst: u32) {
    // IDA 0xb3bd24: for_each hook over new assembly parts.
    top.map.insert(inst, 0.0);
    gen_refresh_top(top);
}
// 0xb3c02c — __ZN3RBX7Network18PhysicsPacketCache17onRemovedAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, _DWORD *, int, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache::onRemovedAssembly(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b3c02c() -> Option<u32> {
    // IDA 0xb3c02c: nullable object query (id when live, None when unset).
    None
}
// 0xb3c334 — __ZN3RBX7Network19InstancePacketCacheC1Ev
// type: int __fastcall(RBX::Network::InstancePacketCache *this)
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c334() -> Option<u32> {
    // IDA 0xb3c334: nullable object query (id when live, None when unset).
    None
}
// 0xb3c340 — __ZN3RBX7Network19InstancePacketCacheC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::InstancePacketCache *this)
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c340() -> Option<u32> {
    // IDA 0xb3c340: nullable object query (id when live, None when unset).
    None
}
// 0xb3c6d8 — __ZN3RBX7Network19InstancePacketCacheD0Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c6d8() {
    // IDA 0xb3c6d8: dtor releases the owned control block/slots.
}
// 0xb3c778 — __ZN3RBX7Network19InstancePacketCacheD1Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c778() {
    // IDA 0xb3c778: dtor releases the owned control block/slots.
}
// 0xb3c784 — __ZThn32_N3RBX7Network19InstancePacketCacheD0Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c784(fire: &dyn Fn()) {
    // IDA 0xb3c784: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb3c828 — __ZThn36_N3RBX7Network19InstancePacketCacheD0Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c828(fire: &dyn Fn()) {
    // IDA 0xb3c828: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb3c8cc — __ZN3RBX7Network19InstancePacketCacheD2Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c8cc() {
    // IDA 0xb3c8cc: dtor releases the owned control block/slots.
}
// 0xb3caa4 — __ZThn32_N3RBX7Network19InstancePacketCacheD1Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3caa4(fire: &dyn Fn()) {
    // IDA 0xb3caa4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb3cab0 — __ZThn36_N3RBX7Network19InstancePacketCacheD1Ev
// type: void __fastcall(RBX::Network::InstancePacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3cab0(fire: &dyn Fn()) {
    // IDA 0xb3cab0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb3cabc — __ZN3RBX7Network19InstancePacketCache17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD *__fastcall(_DWORD *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Network::InstancePacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_b3cabc(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xb3cabc: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xb3cb4c — __ZN3RBX7Network19InstancePacketCache17onAncestorChangedEN5boost10shared_ptrINS_8InstanceEEES5_
// type: RBX::Network::InstancePacketCache *__fastcall(RBX::Network::InstancePacketCache *result, const RBX::Instance **, _DWORD *)
#[doc(alias = "RBX::Network::InstancePacketCache::onAncestorChanged(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b3cb4c() -> Option<u32> {
    // IDA 0xb3cb4c: nullable object query (id when live, None when unset).
    None
}
// 0xb3cb60 — __ZN3RBX7Network19InstancePacketCache6removeEPKNS_8InstanceE
// type: int __fastcall(RBX::Network::InstancePacketCache *this, unsigned int)
#[doc(alias = "RBX::Network::InstancePacketCache::remove(RBX::Instance const*)")]
pub fn stub_b3cb60(s: &mut GenSignalState, id: u64) {
    // IDA 0xb3cb60: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0xb3cbf8 — __ZN3RBX7Network19InstancePacketCache6insertEPKNS_8InstanceE
// type: void __fastcall(RBX::Network::InstancePacketCache *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::InstancePacketCache::insert(RBX::Instance const*)")]
pub fn stub_b3cbf8(s: &mut GenSignalState) -> u64 {
    // IDA 0xb3cbf8: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xb3e6e4 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network18PhysicsPacketCacheERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(_DWORD *, void *, void *, char *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")]
pub fn stub_b3e6e4(parts: &[u32], top: &mut GenTopN) {
    // IDA 0xb3e6e4: for_each over the physics set with the addNugget binder.
    for &p in parts { top.map.insert(p, 0.0); }
    gen_refresh_top(top);
}
// 0xb3e754 — __ZN5boost4bindIvN3RBX7Network19InstancePacketCache15CachedBitStreamEPKNS1_10Reflection18PropertyDescriptorENS_10shared_ptrIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)")]
pub fn stub_b3e754() -> Option<u32> {
    // IDA 0xb3e754: nullable object query (id when live, None when unset).
    None
}
// 0xb3edb8 — __ZN3RBX7Network19InstancePacketCache15CachedBitStream17onPropertyChangedEPKNS_10Reflection18PropertyDescriptorE
// type: _BYTE *__fastcall(_BYTE *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::InstancePacketCache::CachedBitStream::onPropertyChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b3edb8(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xb3edb8: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xb3f318 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS_10shared_ptrINS5_7Network19InstancePacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>,RBX::Instance const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>> *)")]
pub fn stub_b3f318(map: &mut HashMap<u32, f32>, part: u32) -> bool {
    // IDA 0xb3f318: erases the node chain for one key.
    map.remove(&part).is_some()
}
// 0xb3f8e8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_7Network19InstancePacketCacheES6_S6_EENSA_5list3INSA_5valueIPSF_EENS2_3argILi1EEENSL_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::InstancePacketCache*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_b3f8e8(s: &mut GenSignalState, id: u64) {
    // IDA 0xb3f8e8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb3f944 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_7Network19InstancePacketCacheES6_S6_EENSA_5list3INSA_5valueIPSF_EENS2_3argILi1EEENSL_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::InstancePacketCache*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_b3f944(s: &mut GenSignalState, id: u64) {
    // IDA 0xb3f944: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb3fa50 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_7Network19InstancePacketCacheES7_S7_EENSB_5list3INSB_5valueIPSG_EENS3_3argILi1EEENSM_ILi2EEEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::InstancePacketCache*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b3fa50(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb3fa50: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb3fa6c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_7Network19InstancePacketCacheES7_S7_EENSB_5list3INSB_5valueIPSG_EENS3_3argILi1EEENSM_ILi2EEEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::InstancePacketCache*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b3fa6c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb3fa6c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb3fa88 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network19InstancePacketCacheEEENS_3argILi1EEENS8_ILi2EEEEclINS_4_mfi3mf2IvS5_NS_10shared_ptrINS3_8InstanceEEESH_EENS0_5list2IRSH_SK_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(pthread_mutex_t **, int, int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Network::InstancePacketCache *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_b3fa88(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb3fa88: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb3fee0 — __ZNK5boost4_mfi3mf2IvN3RBX7Network19InstancePacketCacheENS_10shared_ptrINS2_8InstanceEEES7_EclEPS4_S7_S7_
// type: void __fastcall(char **, int, int *, int *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::Network::InstancePacketCache,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::InstancePacketCache*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b3fee0() -> Option<u32> {
    // IDA 0xb3fee0: nullable object query (id when live, None when unset).
    None
}
// 0xb40350 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b40350(s: &mut GenSignalState, id: u64) {
    // IDA 0xb40350: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb4035c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b4035c(s: &mut GenSignalState, id: u64) {
    // IDA 0xb4035c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb40414 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40414(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb40414: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb40430 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40430(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb40430: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb4044c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b4044c() {
    // IDA 0xb4044c: drops the bound functor held by the callable.
}
// 0xb405c8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: int()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405c8() {
    // IDA 0xb405c8: drops the bound functor held by the callable.
}
// 0xb405d4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405d4() {
    // IDA 0xb405d4: drops the bound functor held by the callable.
}
// 0xb4068c — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network19InstancePacketCache15CachedBitStreamEEEEENS_3argILi1EEEEC2ES9_SB_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)")]
pub fn stub_b4068c(slot: &mut GenFunctor) {
    // IDA 0xb4068c: packs the bound argument list.
    slot.has = true;
}
// 0xb408dc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS_10shared_ptrINS5_7Network19InstancePacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS9_RKT_
// type: void __fastcall(_DWORD *, _DWORD *, unsigned int *, int, void *, char, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>,RBX::Instance const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>> const&)")]
pub fn stub_b408dc(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb408dc: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb40aac — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8InstanceENS_10shared_ptrINS5_7Network19InstancePacketCache15CachedBitStreamEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>> const&)")]
pub fn stub_b40aac(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb40aac: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb40b90 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS_10shared_ptrINS5_7Network19InstancePacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>,RBX::Instance const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b40b90(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb40b90: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb40d38 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS_10shared_ptrINS5_7Network19InstancePacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>>,RBX::Instance const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")]
pub fn stub_b40d38(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb40d38: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb40de8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network19InstancePacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)")]
pub fn stub_b40de8(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb40de8: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb40ff0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff0() {
    // IDA 0xb40ff0: counted-impl dtor frees the control block.
}
// 0xb40ff4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff4() {
    // IDA 0xb40ff4: counted-impl dtor frees the control block.
}
// 0xb41000 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::dispose(void)")]
pub fn stub_b41000() -> Option<u32> {
    // IDA 0xb41000: nullable object query (id when live, None when unset).
    None
}
// 0xb4110c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_deleter(std::type_info const&)")]
pub fn stub_b4110c() -> bool {
    // IDA 0xb4110c: deleter query misses for this control block.
    false
}
// 0xb41110 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_untyped_deleter(void)")]
pub fn stub_b41110() -> bool {
    // IDA 0xb41110: deleter query misses for this control block.
    false
}
