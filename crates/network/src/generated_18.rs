//! network generated_18 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for network|replicator|raknet (5119 funcs, 120 stubs here, EA-sorted ascending earliest gap shard R).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::sync::{Mutex, OnceLock};

use rbx_core::SharedPtr;

// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub fn stub_17e68(reachable: bool, connection_opened: bool) -> bool {
    // IDA 0x17e68: -[Appirater connectedToNetwork] — zero-address SCNetworkReachability flags (0x17ea8..0x17ece) plus an apple.com probe connection (0x17ede..0x17f3a); reachable (flags&6==2 || flags&1, 0x17f4a) reports the connection (0x17f52), no-flags logs + 0 (0x17f60..0x17f64). System reachability folds; the decision stays 1:1.
        reachable && connection_opened}

// 0x35c6c — -[Reachability networkStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability networkStatusForFlags:]")]
pub fn stub_35c6c(flags: u32, log_flags: &mut dyn FnMut(u32)) -> i32 {
    // IDA 0x35c6c: PrintReachabilityFlags(0x35c7e); 0 when !(flags&2) (0x35c8a);
    // else direct = flags&0x28 (0x35c94..0x35c9a), status = !(flags&4) (0x35ca0)
    // forced to 1 when direct && !(flags&0x10) (0x35ca6..0x35ca8), 2 when
    // flags&0x40000 (0x35cae..0x35cb0), else status (0x35cb2).
    log_flags(flags);
    reachability_status_for_flags(flags)
}

/// Shared core of `-[Reachability networkStatusForFlags:]` (IDA 0x35c6c)
/// and `-[BSReachability networkStatusForFlags:]` (IDA 0xeee6f0): the flag
/// folds are identical, only the logging differs.
fn reachability_status_for_flags(flags: u32) -> i32 {
    if flags & 2 == 0 {
        return 0;
    }
    let direct = flags & 0x28 != 0;
    let mut status = i32::from(flags & 4 == 0);
    if direct && flags & 0x10 == 0 {
        status = 1;
    }
    if flags & 0x40000 != 0 {
        return 2;
    }
    status
}

/// `RBX::DataModel` network-metric slot (IDA 0x427db8 stores the
/// `IMetric *` at `this + 3000`); the DataModel layout stays engine-side.
#[derive(Debug, Default)]
pub struct DataModelMetricSlot {
    pub metric: usize,
}

// 0x427db8 — __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE
// type: int __fastcall(int this, IMetric *)
#[doc(alias = "__ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE")]
pub fn stub_427db8<'a>(slot: &'a mut DataModelMetricSlot, metric: usize) -> &'a mut DataModelMetricSlot {
    // IDA 0x427db8: *(this+3000) = metric, returns this (0x427dbc).
    slot.metric = metric;
    slot
}

/// Lazily-initialized reflection `ClassDescriptor` mirror: the guard +
/// base-descriptor init + `__cxa_atexit` stay engine-side (IDA 0x4da954..0x4da9ea).
#[derive(Debug)]
pub struct NetworkClassDescriptor {
    pub name: &'static str,
    pub base: &'static str,
}

fn network_descriptor(cell: &'static OnceLock<NetworkClassDescriptor>, name: &'static str) -> &'static NetworkClassDescriptor {
    cell.get_or_init(|| NetworkClassDescriptor { name, base: "Instance" })
}

// 0x4da8f8 — __ZN3RBX10Reflection9DescribedINS_7Network10ChatFilterELZNS2_11sChatFilterEENS_17NonFactoryProductINS_8InstanceELZNS2_11sChatFilterEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network10ChatFilterELZNS2_11sChatFilterEENS_17NonFactoryProductINS_8InstanceELZNS2_11sChatFilterEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4da8f8() -> &'static NetworkClassDescriptor {
    // IDA 0x4da8f8: guard-checked once init of the ChatFilter descriptor over
    // the Instance base (0x4da960..0x4da9c0), returns the static (0x4da9ea).
    static CELL: OnceLock<NetworkClassDescriptor> = OnceLock::new();
    network_descriptor(&CELL, "ChatFilter")
}

// 0x4daa18 — __ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4daa18() -> &'static NetworkClassDescriptor {
    // IDA 0x4daa18: same once-init shape as 0x4da8f8 for sClusterPacketCache.
    static CELL: OnceLock<NetworkClassDescriptor> = OnceLock::new();
    network_descriptor(&CELL, "ClusterPacketCache")
}

// 0x4dab38 — __ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4dab38() -> &'static NetworkClassDescriptor {
    // IDA 0x4dab38: same once-init shape as 0x4da8f8 for sInstancePacketCache.
    static CELL: OnceLock<NetworkClassDescriptor> = OnceLock::new();
    network_descriptor(&CELL, "InstancePacketCache")
}

// 0x4dac58 — __ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4dac58() -> &'static NetworkClassDescriptor {
    // IDA 0x4dac58: same once-init shape as 0x4da8f8 for sPhysicsPacketCache.
    static CELL: OnceLock<NetworkClassDescriptor> = OnceLock::new();
    network_descriptor(&CELL, "PhysicsPacketCache")
}

/// `RBX::SystemAddress` network-owner word: the 8-byte owner stored at
/// primitive + 92 (IDA 0x5d9166, 0x5d918c).
pub type NetworkOwnerWord = u64;

// 0x5d910c — __ZNK3RBX12PartInstance15getNetworkOwnerEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZNK3RBX12PartInstance15getNetworkOwnerEv")]
pub fn stub_5d910c(primitive_present: bool, owner: NetworkOwnerWord) -> NetworkOwnerWord {
    // IDA 0x5d910c: ReleaseAssert(getConstPartPrimitive(), PartInstance.cpp:1603,
    // 0x5d9120..0x5d9162) then copies the owner QWORD to the out-param (0x5d9166..0x5d9172).
    assert!(primitive_present, "getConstPartPrimitive() PartInstance.cpp:1603");
    owner
}

/// Outcome of `RBX::PartInstance::setNetworkOwner` (IDA 0x5d9174): the
/// early-out when the value is unchanged, else the side effects the caller
/// must apply engine-side (history clear, flag store, property raise).
#[derive(Debug, PartialEq, Eq)]
pub enum SetNetworkOwnerEffect {
    Unchanged,
    Changed { clear_history: bool, someone_else: bool },
}

// 0x5d9174 — __ZN3RBX12PartInstance15setNetworkOwnerENS_13SystemAddressE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX12PartInstance15setNetworkOwnerENS_13SystemAddressE")]
pub fn stub_5d9174(
    current: NetworkOwnerWord,
    new_owner: NetworkOwnerWord,
    new_is_unassigned: bool,
    local_simulator: NetworkOwnerWord,
    someone_else: bool,
) -> SetNetworkOwnerEffect {
    // IDA 0x5d9174: returns when new == current (0x5d9190..0x5d9240); asserts
    // new != Unassigned() (PartInstance.cpp:1611, 0x5d91ac..0x5d91fc); clears
    // interpolation history when new == local simulator (0x5d91fc..0x5d9210);
    // stores the someone-else flag (0x5d921c), the owner word (0x5d9230..0x5d9234)
    // and raises prop_NetworkOwner (0x5d923a).
    if new_owner == current {
        return SetNetworkOwnerEffect::Unchanged;
    }
    assert!(!new_is_unassigned, "value != Network::NetworkOwner::Unassigned() PartInstance.cpp:1611");
    SetNetworkOwnerEffect::Changed {
        clear_history: new_owner == local_simulator,
        someone_else,
    }
}

// 0x5d9244 — __ZNK3RBX12PartInstance20getNetworkIsSleepingEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZNK3RBX12PartInstance20getNetworkIsSleepingEv")]
pub fn stub_5d9244(sleeping: bool) -> bool {
    // IDA 0x5d9244: byte at *(this+43)+100 (0x5d924c).
    sleeping
}

// 0x5d9250 — __ZN3RBX12PartInstance20setNetworkIsSleepingEb
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this, bool)
#[doc(alias = "__ZN3RBX12PartInstance20setNetworkIsSleepingEb")]
pub fn stub_5d9250(slot: &mut bool, value: bool) -> bool {
    // IDA 0x5d9250: forwards to RBX::Primitive::setNetworkIsSleeping(*(this+43)).
    *slot = value;
    *slot
}

// 0x5da97c — __ZN3RBX12PartInstance21resetNetworkOwnerTimeEd
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this, double)
#[doc(alias = "__ZN3RBX12PartInstance21resetNetworkOwnerTimeEd")]
pub fn stub_5da97c(now: f64, delay: f64) -> f64 {
    // IDA 0x5da97c: *(this+39) = Time::now() + delay (0x5da98a..0x5da99a).
    now + delay
}

// 0x5da9a4 — __ZNK3RBX12PartInstance18networkOwnerTimeUpEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZNK3RBX12PartInstance18networkOwnerTimeUpEv")]
pub fn stub_5da9a4(now: f64, timeout_at: f64) -> bool {
    // IDA 0x5da9a4: Time::now() > *(this+39) (0x5da9ae..0x5da9ca).
    now > timeout_at
}

// 0x5ddecc — __ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZNK3RBX12PartInstance32computeNetworkOwnerIsSomeoneElseEv")]
pub fn stub_5ddecc(mechanism_flag: Option<bool>) -> bool {
    // IDA 0x5ddecc: 0 when no mechanism (0x5ddeda..0x5dde0); else the byte at
    // *(mechanismPrimitive+244)+224 (0x5ddef2).
    mechanism_flag.unwrap_or(false)
}

// 0x5de570 — __ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZN3RBX12PartInstance26onNetworkIsSleepingChangedEv")]
pub fn stub_5de570(sync_rendered_frame: &mut dyn FnMut(), raise_changed: &mut dyn FnMut()) {
    // IDA 0x5de570: InterpolatedCFrame::setRenderedFrame(this+46, coordinate
    // frame) (0x5de57a..0x5de584) then raises prop_NetworkIsSleeping (0x5de585).
    sync_rendered_frame();
    raise_changed();
}

// 0x5de59c — __ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv
// type: _DWORD __fastcall(RBX::PartInstance *__hidden this)
#[doc(alias = "__ZThn96_N3RBX12PartInstance26onNetworkIsSleepingChangedEv")]
pub fn stub_5de59c(
    this: usize,
    sync_rendered_frame: &mut dyn FnMut(),
    raise_changed: &mut dyn FnMut(),
) {
    // IDA 0x5de59c: non-virtual thunk, this-24 then onNetworkIsSleepingChanged.
    let _adjusted = this - 24;
    stub_5de570(sync_rendered_frame, raise_changed);
}

// 0x6d0358 — __ZNK3RBX9Workspace26getNetworkStreamingEnabledEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
#[doc(alias = "__ZNK3RBX9Workspace26getNetworkStreamingEnabledEv")]
pub fn stub_6d0358(enabled: bool) -> bool {
    // IDA 0x6d0358: byte at this+584 (0x6d035c).
    enabled
}

// 0x6d0360 — __ZN3RBX9Workspace26setNetworkStreamingEnabledEb
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, bool)
#[doc(alias = "__ZN3RBX9Workspace26setNetworkStreamingEnabledEb")]
pub fn stub_6d0360(slot: &mut bool, value: bool) {
    // IDA 0x6d0360: *(this+584) = value (0x6d0360), returns this (0x6d0364).
    *slot = value;
}

// 0xaacf2c — __ZThn36_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aacf2c(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xaacf2c: thunk this-36 (0xaacf56), ~Instance (0xaacf7e), operator delete (0xaacf84).
    let adjusted = this - 36;
    destroy_instance(adjusted);
    free(adjusted);
}

// 0xaacfd0 — __ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aacfd0(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xaacfd0: RBX::Instance::~Instance(a1) (0xaacfd4).
    destroy_instance(this);
}

// 0xaacfdc — __ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aacfdc(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xaacfdc: ~Instance (0xaad02c) then operator delete (0xaad032).
    destroy_instance(this);
    free(this);
}

// 0xaad07c — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aad07c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xaad07c: thunk this-32 into the Player Described D1 (0xaad082).
    stub_aacfd0(this - 32, destroy_instance);
}

// 0xaad088 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aad088(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xaad088: thunk this-32 (0xaad0b2), ~Instance (0xaad0da), delete (0xaad0e0).
    stub_aacfdc(this - 32, destroy_instance, free);
}

// 0xaad12c — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aad12c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xaad12c: thunk this-36 into the Player Described D1 (0xaad132).
    stub_aacfd0(this - 36, destroy_instance);
}

// 0xaad138 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aad138(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xaad138: thunk this-36 (0xaad162), ~Instance (0xaad18a), delete (0xaad190).
    stub_aacfdc(this - 36, destroy_instance, free);
}

/// Bound `Player` fetch-completion functor (IDA 0xacba9c): the
/// `boost::function<void(RequestResult, SharedPtr<vector<SharedPtr<Instance>>>)>`
/// built from a bind over `weak_ptr<Player>` + `(string, bool, double)`.
/// Retains mirror `boost::detail::function`'s clone/copy/assign_to steps.
#[derive(Debug, Clone)]
pub struct PlayerFetchBind {
    pub player: std::sync::Weak<()>,
    pub path: String,
    pub flag: bool,
    pub delay: f64,
}

// 0xacba9c — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSE_5list6INSE_5valueISJ_EENS_3argILi1EEENSP_ILi2EEENSN_ISsEENSN_IbEENSN_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSE_5list6INSE_5valueISJ_EENS_3argILi1EEENSP_ILi2EEENSN_ISsEENSN_IbEENSN_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
pub fn stub_acba9c(player: &SharedPtr<()>, path: &str, flag: bool, delay: f64) -> PlayerFetchBind {
    // IDA 0xacba9c: retains the weak player (0xacbac0..0xacbb40), copies the
    // string/bool/double (0xacbb54..0xacbb6e), assign_to the bind
    // (0xacbbf4), then releases the temporaries (0xacbc06..0xacbd48).
    // was: boost::function<...> ctor from boost::_bi::bind_t.
    PlayerFetchBind {
        player: SharedPtr::downgrade(player),
        path: path.to_owned(),
        flag,
        delay,
    }
}

/// Declared `RBX::Name` mirror: `call_once` declare then return (IDA
/// 0xad1544..0xad1922); the `Name::declare` store stays engine-side.
fn declared_name(cell: &'static OnceLock<&'static str>, name: &'static str) -> &'static str {
    *cell.get_or_init(|| name)
}

// 0xad1544 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network7sMarkerEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network7sMarkerEEE12getClassNameEv")]
pub fn stub_ad1544() -> &'static str {
    // IDA 0xad1544: call_once declare(sMarker) (0xad1578) then return the
    // declared name (0xad1614).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "Marker")
}

// 0xad16f0 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network7sMarkerEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network7sMarkerEEE12getClassNameEv")]
pub fn stub_ad16f0() -> &'static str {
    // IDA 0xad16f0: Thn32 of 0xad1544, identical declare + return (0xad1724..0xad17c0).
    stub_ad1544()
}

// 0xad189c — __ZN3RBX4Name13callDoDeclareILZNS_7Network7sMarkerEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network7sMarkerEEEEvv")]
pub fn stub_ad189c() -> &'static str {
    // IDA 0xad189c: guard-checked declare of sMarker (0xad18f4..0xad1922).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "Marker")
}

// 0xad196c — __ZN3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad196c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad196c: RBX::Instance::~Instance(a1) (0xad1970).
    destroy_instance(this);
}

// 0xad1978 — __ZN3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad1978(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad1978: ~Instance (0xad19c8) then operator delete (0xad19ce).
    destroy_instance(this);
    free(this);
}

// 0xad1a18 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad1a18(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad1a18: thunk this-32 into the Marker Described D1 (0xad1a1e).
    stub_ad196c(this - 32, destroy_instance);
}

// 0xad1a24 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad1a24(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad1a24: thunk this-32 (0xad1a4e), ~Instance (0xad1a76), delete (0xad1a7c).
    stub_ad1978(this - 32, destroy_instance, free);
}

// 0xad1ac8 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad1ac8(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad1ac8: thunk this-36 into the Marker Described D1 (0xad1ace).
    stub_ad196c(this - 36, destroy_instance);
}

// 0xad1ad4 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6MarkerELZNS2_7sMarkerEENS_17NonFactoryProductINS_8InstanceELZNS2_7sMarkerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad1ad4(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad1ad4: thunk this-36 (0xad1afe), ~Instance (0xad1b26), delete (0xad1b2c).
    stub_ad1978(this - 36, destroy_instance, free);
}

// 0xad6768 — __ZN3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad6768(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad6768: RBX::Instance::~Instance(a1) (0xad676c).
    destroy_instance(this);
}

// 0xad6774 — __ZN3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad6774(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad6774: ~Instance (0xad67c4) then operator delete (0xad67ca).
    destroy_instance(this);
    free(this);
}

// 0xad6814 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad6814(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad6814: thunk this-32 into the Peer Described D1 (0xad681a).
    stub_ad6768(this - 32, destroy_instance);
}

// 0xad6820 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad6820(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad6820: thunk this-32 (0xad684a), ~Instance (0xad6872), delete (0xad6878).
    stub_ad6774(this - 32, destroy_instance, free);
}

// 0xad68c4 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_ad68c4(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xad68c4: thunk this-36 into the Peer Described D1 (0xad68ca).
    stub_ad6768(this - 36, destroy_instance);
}

// 0xad68d0 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network4PeerELZNS2_5sPeerEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_ad68d0(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xad68d0: thunk this-36 (0xad68fa), ~Instance (0xad6922), delete (0xad6928).
    stub_ad6774(this - 36, destroy_instance, free);
}

// 0xb0dfb8 — __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorD1Ev
// type: int __fastcall(pthread_mutex_t *, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_b0dfb8(destroy_creator: &mut dyn FnMut()) {
    // IDA 0xb0dfb8: tail-jumps to Creator D2 (0xb0dfc0).
    destroy_creator();
}

// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
pub fn stub_b2332c() -> &'static str {
    // IDA 0xb2332c: guard-checked declare of sClusterPacketCache (0xb23384..0xb233b4).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "ClusterPacketCache")
}

// 0xb23b3c — __ZN3RBX4Name13callDoDeclareILZNS_7Network20sInstancePacketCacheEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network20sInstancePacketCacheEEEEvv")]
pub fn stub_b23b3c() -> &'static str {
    // IDA 0xb23b3c: guard-checked declare of sInstancePacketCache (0xb23b94..0xb23bc4).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "InstancePacketCache")
}

// 0xb3ee50 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sPhysicsPacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sPhysicsPacketCacheEEE12getClassNameEv")]
pub fn stub_b3ee50() -> &'static str {
    // IDA 0xb3ee50: call_once declare(sPhysicsPacketCache) then return the name.
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "PhysicsPacketCache")
}

// 0xb3ef50 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sPhysicsPacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sPhysicsPacketCacheEEE12getClassNameEv")]
pub fn stub_b3ef50() -> &'static str {
    // IDA 0xb3ef50: Thn32 of 0xb3ee50, identical declare + return.
    stub_b3ee50()
}

// 0xb3f050 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sInstancePacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sInstancePacketCacheEEE12getClassNameEv")]
pub fn stub_b3f050() -> &'static str {
    // IDA 0xb3f050: call_once declare(sInstancePacketCache) then return the name.
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "InstancePacketCache")
}

// 0xb3f14c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sInstancePacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sInstancePacketCacheEEE12getClassNameEv")]
pub fn stub_b3f14c() -> &'static str {
    // IDA 0xb3f14c: Thn32 of 0xb3f050, identical declare + return.
    stub_b3f050()
}

// 0xb3f248 — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sPhysicsPacketCacheEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sPhysicsPacketCacheEEEEvv")]
pub fn stub_b3f248() -> &'static str {
    // IDA 0xb3f248: guard-checked declare of sPhysicsPacketCache (0xb3f2a0..0xb3f2ce).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "PhysicsPacketCache")
}

// 0xb411b4 — __ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b411b4(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb411b4: RBX::Instance::~Instance(a1) (0xb411b8).
    destroy_instance(this);
}

// 0xb411c0 — __ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b411c0(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb411c0: ~Instance (0xb41210) then operator delete (0xb41216).
    destroy_instance(this);
    free(this);
}

// 0xb41260 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b41260(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb41260: thunk this-32 into the InstancePacketCache D1 (0xb41266).
    stub_b411b4(this - 32, destroy_instance);
}

// 0xb4126c — __ZThn32_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b4126c(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb4126c: thunk this-32 (0xb41296), ~Instance (0xb412be), delete (0xb412c4).
    stub_b411c0(this - 32, destroy_instance, free);
}

// 0xb41310 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b41310(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb41310: thunk this-36 into the InstancePacketCache D1 (0xb41316).
    stub_b411b4(this - 36, destroy_instance);
}

// 0xb4131c — __ZThn36_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b4131c(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb4131c: thunk this-36 (0xb41346), ~Instance (0xb4136e), delete (0xb41374).
    stub_b411c0(this - 36, destroy_instance, free);
}

// 0xb4336c — __ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b4336c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb4336c: RBX::Instance::~Instance(a1) (0xb43370).
    destroy_instance(this);
}

// 0xb43378 — __ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b43378(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb43378: ~Instance (0xb433c8) then operator delete (0xb433ce).
    destroy_instance(this);
    free(this);
}

// 0xb43418 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b43418(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb43418: thunk this-32 into the PhysicsPacketCache D1 (0xb4341e).
    stub_b4336c(this - 32, destroy_instance);
}

// 0xb43424 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b43424(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb43424: thunk this-32 (0xb4344e), ~Instance (0xb43476), delete (0xb4347c).
    stub_b43378(this - 32, destroy_instance, free);
}

// 0xb434c8 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b434c8(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb434c8: thunk this-36 into the PhysicsPacketCache D1 (0xb434ce).
    stub_b4336c(this - 36, destroy_instance);
}

// 0xb434d4 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b434d4(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb434d4: thunk this-36 (0xb434fe), ~Instance (0xb43526), delete (0xb4352c).
    stub_b43378(this - 36, destroy_instance, free);
}

// 0xb4f298 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sClusterPacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sClusterPacketCacheEEE12getClassNameEv")]
pub fn stub_b4f298() -> &'static str {
    // IDA 0xb4f298: call_once declare(sClusterPacketCache) then return the name.
    static CELL: OnceLock<&'static str> = OnceLock::new();
    declared_name(&CELL, "ClusterPacketCache")
}

// 0xb4f448 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sClusterPacketCacheEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network19sClusterPacketCacheEEE12getClassNameEv")]
pub fn stub_b4f448() -> &'static str {
    // IDA 0xb4f448: Thn32 of 0xb4f298, identical declare + return.
    stub_b4f298()
}

// 0xb502d0 — __ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b502d0(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb502d0: RBX::Instance::~Instance(a1) (0xb502d4).
    destroy_instance(this);
}

// 0xb502dc — __ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b502dc(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb502dc: ~Instance (0xb5032c) then operator delete (0xb50332).
    destroy_instance(this);
    free(this);
}

// 0xb5037c — __ZThn32_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b5037c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb5037c: thunk this-32 into the ClusterPacketCache D1 (0xb50382).
    stub_b502d0(this - 32, destroy_instance);
}

// 0xb50388 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b50388(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb50388: thunk this-32 (0xb503b2), ~Instance (0xb503da), delete (0xb503e0).
    stub_b502dc(this - 32, destroy_instance, free);
}

// 0xb5042c — __ZThn36_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b5042c(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0xb5042c: thunk this-36 into the ClusterPacketCache D1 (0xb50432).
    stub_b502d0(this - 36, destroy_instance);
}

// 0xb50438 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b50438(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0xb50438: thunk this-36 (0xb50462), ~Instance (0xb5048a), delete (0xb50490).
    stub_b502dc(this - 36, destroy_instance, free);
}

// 0xecd5d0 — _TFSetUpCrashSafeNetworking
#[doc(alias = "_TFSetUpCrashSafeNetworking")]
pub fn stub_ecd5d0(token: usize, dispatch_async: &mut dyn FnMut(usize)) {
    // IDA 0xecd5d0: retains the token (0xecd5da), builds the block for
    // __TFSetUpCrashSafeNetworking_block_invoke (0xecd608..0xecd614) and
    // dispatch_async's it on the global queue (0xecd61a..0xecd620).
    dispatch_async(token);
}

// 0xecd628 — ___TFSetUpCrashSafeNetworking_block_invoke
#[doc(alias = "___TFSetUpCrashSafeNetworking_block_invoke")]
pub fn stub_ecd628(token_sha1: &str, write_pack: &mut dyn FnMut(&str), create_socket: &mut dyn FnMut(&str)) {
    // IDA 0xecd628: sha1(token) (0xecd64a), msgpack it (0xecd670), write
    // <cache>/app_token_sha1.mpack (0xecd67c..0xecd6d0), TFCreateCrashSocket
    // (0xecd6d6). Cache-path joining stays engine-side.
    write_pack(token_sha1);
    create_socket(token_sha1);
}

/// `+[TFEventManager _ensureNetworkStartDataIsSetup]` (IDA 0xed3f80) packs a
/// 4-entry msgpack map: `bundle_info`, `device_info`, `sdk_version`.
pub const TF_START_DATA_MAP_ENTRIES: u32 = 4;
/// `__libTestFlight_v2.0.2-beta_c7ba611b7c05e2477a92133076e63042076ffb5d__` (IDA 0xed419c).
pub const TF_SDK_VERSION: &str = "__libTestFlight_v2.0.2-beta_c7ba611b7c05e2477a92133076e63042076ffb5d__";

// 0xed3f80 — +[TFEventManager _ensureNetworkStartDataIsSetup]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[TFEventManager _ensureNetworkStartDataIsSetup]")]
pub fn stub_ed3f80(already_setup: bool, append_pair: &mut dyn FnMut(&str)) -> bool {
    // IDA 0xed3f80: early-out when the setup flag is set (0xed3f98); else
    // mkdirs the event dir (0xed3fb8..0xed3fe2), packs the 4-map header
    // (0xed4030) plus bundle_info (0xed4078..0xed40e0), device_info
    // (0xed40f4..0xed415c) and sdk_version (0xed4170..0xed41b4) pairs, writes
    // the file (0xed41d4) and sets the flag (0xed41e4).
    if already_setup {
        return true;
    }
    append_pair("bundle_info");
    append_pair("device_info");
    append_pair("sdk_version");
    true
}

// 0xed41f4 — +[TFEventManager bustCachedNetworkStartData]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[TFEventManager bustCachedNetworkStartData]")]
pub fn stub_ed41f4(setup: &mut bool, end_event_queues: &mut dyn FnMut()) {
    // IDA 0xed41f4: clears the setup flag (0xed4220) and ends every queued
    // event queue (0xed422c).
    *setup = false;
    end_event_queues();
}

// 0xed53e8 — _tf_event_pack_network_start_data_safe
#[doc(alias = "_tf_event_pack_network_start_data_safe")]
pub fn stub_ed53e8(cache_dir: &str, copy_file: &mut dyn FnMut(&str) -> i32) -> i32 {
    // IDA 0xed53e8: joins <cache_dir>/TFEventManager/start network
    // data.msgpack (0xed5412..0xed545c), opens + tf_copy_file + close
    // (0xed5468..0xed5474).
    let path = format!("{cache_dir}/TFEventManager/start network data.msgpack");
    copy_file(&path)
}

/// `TFNetworkManager` resolved state (IDA 0xed5764..0xed6664). ObjC objects
/// (queue, file manager, reachability) stay engine-side; the counters, flags
/// and task id that drive every branch live here.
#[derive(Debug, Default)]
pub struct TfNetworkManagerState {
    pub pause_counter: i32,
    pub paused: bool,
    pub paused_for_reachability: bool,
    pub background_task: u32,
    pub max_concurrent_operations: usize,
    pub queue_suspended: bool,
    pub operation_count: usize,
    pub reach_status: i32,
}

/// `UIBackgroundTaskInvalid` (IDA 0xed5890, 0xed6396, 0xed661a, 0xed64b0).
pub const UI_BACKGROUND_TASK_INVALID: u32 = 0;

fn tf_manager_cell() -> &'static Mutex<TfNetworkManagerState> {
    static CELL: OnceLock<Mutex<TfNetworkManagerState>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(TfNetworkManagerState {
        max_concurrent_operations: 1,
        background_task: UI_BACKGROUND_TASK_INVALID,
        ..TfNetworkManagerState::default()
    }))
}

// 0xed56fc — +[TFNetworkManager sharedManager]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[TFNetworkManager sharedManager]")]
pub fn stub_ed56fc() -> &'static Mutex<TfNetworkManagerState> {
    // IDA 0xed56fc: dispatch_once (0xed5710..0xed5726), returns the shared
    // manager (0xed5718).
    tf_manager_cell()
}

// 0xed572c — ___33+[TFNetworkManager sharedManager]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___33+[TFNetworkManager sharedManager]_block_invoke")]
pub fn stub_ed572c() -> &'static Mutex<TfNetworkManagerState> {
    // IDA 0xed572c: +new into the shared slot (0xed5748..0xed575c).
    tf_manager_cell()
}

// 0xed5764 — -[TFNetworkManager init]
// type: TFNetworkManager *__cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager init]")]
pub fn stub_ed5764(initial_reach_status: i32) -> TfNetworkManagerState {
    // IDA 0xed5764: super init (0xed578e); serial queue named
    // "TestFlight Network Connection Operation Queue" with maxConcurrent 1 +
    // operationCount observer (0xed57bc..0xed582c); fm=new (0xed5848);
    // task=INVALID (0xed5890); app-active/background observers (0xed5896..0xed5940);
    // reachabilityWithHostName www.testflightapp.com (0xed5966); initial
    // _handleReachabilityStatus: (0xed5994..0xed59a8); update block (0xed59d6..0xed59ee).
    let mut state = TfNetworkManagerState {
        max_concurrent_operations: 1,
        background_task: UI_BACKGROUND_TASK_INVALID,
        reach_status: initial_reach_status,
        ..TfNetworkManagerState::default()
    };
    if let Some(pause) = stub_ed623c(&mut state.paused_for_reachability, initial_reach_status) {
        stub_ed5bd4(&mut state, pause);
    }
    state
}

// 0xed5a04 — ___24-[TFNetworkManager init]_block_invoke
#[doc(alias = "___24-[TFNetworkManager init]_block_invoke")]
pub fn stub_ed5a04(state: &mut TfNetworkManagerState, status: i32) -> Option<bool> {
    // IDA 0xed5a04: forwards to -[TFNetworkManager _handleReachabilityStatus:] (0xed5a1c).
    stub_ed623c(&mut state.paused_for_reachability, status)
}

// 0xed5a20 — -[TFNetworkManager dealloc]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager dealloc]")]
pub fn stub_ed5a20(state: &mut TfNetworkManagerState, remove_observers: &mut dyn FnMut()) {
    // IDA 0xed5a20: clears the reachability block (0xed5a44), ends the
    // background task (0xed5a56), removes the notification + operationCount
    // observers (0xed5a72..0xed5abe), then super dealloc (0xed5ad6..0xed5ae0).
    stub_ed65f0(state);
    remove_observers();
    *state = TfNetworkManagerState::default();
}

// 0xed5ae8 — -[TFNetworkManager sendFile:toURL:]
// type: void __cdecl(TFNetworkManager *self, SEL, id, id)
#[doc(alias = "-[TFNetworkManager sendFile:toURL:]")]
pub fn stub_ed5ae8(create_operation: &mut dyn FnMut(), add_operation: &mut dyn FnMut()) {
    // IDA 0xed5ae8: _createFileOperationForFile:url: (0xed5b0a) then
    // addNetworkOperation: (0xed5b26).
    create_operation();
    add_operation();
}

// 0xed5b38 — -[TFNetworkManager sendFile:toURL:withPriority:]
// type: void __cdecl(TFNetworkManager *self, SEL, id, id, int)
#[doc(alias = "-[TFNetworkManager sendFile:toURL:withPriority:]")]
pub fn stub_ed5b38(
    create_operation: &mut dyn FnMut(),
    set_priority: &mut dyn FnMut(i32),
    add_operation: &mut dyn FnMut(),
    priority: i32,
) {
    // IDA 0xed5b38: create (0xed5b5a), setQueuePriority: (0xed5b76), add (0xed5b8a).
    create_operation();
    set_priority(priority);
    add_operation();
}

// 0xed5b9c — -[TFNetworkManager addNetworkOperation:]
// type: void __cdecl(TFNetworkManager *self, SEL, id)
#[doc(alias = "-[TFNetworkManager addNetworkOperation:]")]
pub fn stub_ed5b9c(enqueue: &mut dyn FnMut(), start_if_appropriate: &mut dyn FnMut()) {
    // IDA 0xed5b9c: addOperation: (0xed5bbc) then
    // _startBackgroundTaskIfAppropriate (0xed5bce).
    enqueue();
    start_if_appropriate();
}

/// `-[TFNetworkManager _setPaused:]` transition (IDA 0xed5bd4).
#[derive(Debug, PartialEq, Eq)]
pub enum TfPauseTransition {
    NoChange,
    Paused,
    Resumed,
}

// 0xed5bd4 — -[TFNetworkManager _setPaused:]
// type: void __cdecl(TFNetworkManager *self, SEL, char)
#[doc(alias = "-[TFNetworkManager _setPaused:]")]
pub fn stub_ed5bd4(state: &mut TfNetworkManagerState, pause: bool) -> TfPauseTransition {
    // IDA 0xed5bd4: counter += pause ? +1 : -1 (0xed5be0..0xed5bfa); pausing
    // acts at counter == 1 (0xed5bfe..0xed5c1a), resuming at counter == 0
    // (0xed5c20..0xed5c3c); then -endBackgroundTask /
    // -_startBackgroundTaskIfAppropriate (0xed5c42) and setSuspended:
    // (0xed5c62).
    if pause {
        state.pause_counter += 1;
        if state.pause_counter != 1 {
            return TfPauseTransition::NoChange;
        }
        state.paused = true;
        state.queue_suspended = true;
        TfPauseTransition::Paused
    } else {
        state.pause_counter -= 1;
        if state.pause_counter != 0 {
            return TfPauseTransition::NoChange;
        }
        state.paused = false;
        state.queue_suspended = false;
        TfPauseTransition::Resumed
    }
}

/// `-[TFNetworkManager _createFileOperationForFile:url:]` request
/// descriptor (IDA 0xed5c68): POST, `NSNetworkServiceType` 3, file body
/// stream, `Content-Length` header. The ObjC request stays engine-side.
#[derive(Debug, PartialEq, Eq)]
pub struct TfFileUpload {
    pub content_length: u64,
}

// 0xed5c68 — -[TFNetworkManager _createFileOperationForFile:url:]
// type: id __cdecl(TFNetworkManager *self, SEL, id, id)
#[doc(alias = "-[TFNetworkManager _createFileOperationForFile:url:]")]
pub fn stub_ed5c68(content_length: u64) -> TfFileUpload {
    // IDA 0xed5c68: requestWithURL (0xed5cbc), setNetworkServiceType:3
    // (0xed5ce8..0xed5cf4), POST (0xed5d10), HTTPBodyStream from file
    // (0xed5d2e..0xed5d50), Content-Length from file size (0xed5d72..0xed5dee),
    // TFURLConnectionOperation with the 0xed5eac completion block
    // (0xed5e12..0xed5ea8).
    TfFileUpload { content_length }
}

/// `__52-[TFNetworkManager _createFileOperationForFile:url:]_block_invoke`
/// outcome (IDA 0xed5eac).
#[derive(Debug, PartialEq, Eq)]
pub enum TfFileCompletion {
    RemoveFile,
    PauseAndResend { priority: i32 },
    Ignored,
}

/// Resend priority used by the 0xed5eac block (IDA 0xed5f5a: `-4`).
pub const TF_RESEND_PRIORITY: i32 = -4;

// 0xed5eac — ___52-[TFNetworkManager _createFileOperationForFile:url:]_block_invoke
// type: int __fastcall(int, int, id, int, id)
#[doc(alias = "___52-[TFNetworkManager _createFileOperationForFile:url:]_block_invoke")]
pub fn stub_ed5eac(is_success: bool, should_retry: bool) -> TfFileCompletion {
    // IDA 0xed5eac: success → removeItemAtPath (0xed5f0e); else when
    // _shouldPauseAndRetry… → _pauseQueueForABit + sendFile:toURL:withPriority:
    // -4 (0 xed5f2a..0xed5f5a).
    if is_success {
        TfFileCompletion::RemoveFile
    } else if should_retry {
        TfFileCompletion::PauseAndResend { priority: TF_RESEND_PRIORITY }
    } else {
        TfFileCompletion::Ignored
    }
}

// 0xed5fb0 — -[TFNetworkManager _isResponseASuccess:]
// type: char __cdecl(TFNetworkManager *self, SEL, id)
#[doc(alias = "-[TFNetworkManager _isResponseASuccess:]")]
pub fn stub_ed5fb0(status: Option<u16>) -> bool {
    // IDA 0xed5fb0: non-HTTP → 0 (0xed5fee); else 200 (0xC8) <= code < 300
    // (0x12C) (0xed6002..0xed6012). `None` = not an NSHTTPURLResponse.
    matches!(status, Some(code) if (200..300).contains(&code))
}

/// Retry mask for `-[TFNetworkManager
/// _shouldPauseAndRetryFailedFileSend:response:error:]` (IDA 0xed60fc..0xed610e,
/// u32 at 0x44401 = loc_443FE+3): bit `code + 1019` set retries NSError
/// -1001/-1007/-1009/-1010/-1011/-1014/-1015/-1017.
pub const TF_RETRY_ERROR_MASK: u32 = 0xF7941734;

// 0xed6020 — -[TFNetworkManager _shouldPauseAndRetryFailedFileSend:response:error:]
// type: char __cdecl(TFNetworkManager *self, SEL, id, id, id)
#[doc(alias = "-[TFNetworkManager _shouldPauseAndRetryFailedFileSend:response:error:]")]
pub fn stub_ed6020(is_http_response: bool, status: Option<u16>, nsurl_error: Option<i32>) -> bool {
    // IDA 0xed6020: NSURLErrorDomain error → bit = code+1019 (0xed6094),
    // retry iff bit <= 0x12 and the mask bit is set (0xed6098..0xed610e);
    // else HTTP 500 (0x1F4) <= code < 600 (0x258) (0xed60d2..0xed60f8).
    // `nsurl_error` is Some only when the error domain is NSURLErrorDomain.
    if let Some(code) = nsurl_error {
        let bit = code.wrapping_add(1019) as u32;
        return bit <= 0x12 && (TF_RETRY_ERROR_MASK >> bit) & 1 == 1;
    }
    is_http_response && matches!(status, Some(code) if (500..600).contains(&code))
}

// 0xed6124 — -[TFNetworkManager _pauseQueueForABit]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _pauseQueueForABit]")]
pub fn stub_ed6124(state: &mut TfNetworkManagerState) -> u64 {
    // IDA 0xed6124: _setPaused:YES (0xed6140) then dispatch_after 30s an
    // unpause block (0xed616a..0xed617c, see 0xed61b8).
    stub_ed5bd4(state, true);
    TF_PAUSE_RETRY_DELAY_NS
}

/// dispatch_after delay used by `-[TFNetworkManager _pauseQueueForABit]`
/// (IDA 0xed616a: 30_000_000_000 ns).
pub const TF_PAUSE_RETRY_DELAY_NS: u64 = 30_000_000_000;

// 0xed61b8 — ___38-[TFNetworkManager _pauseQueueForABit]_block_invoke
#[doc(alias = "___38-[TFNetworkManager _pauseQueueForABit]_block_invoke")]
pub fn stub_ed61b8(state: &mut TfNetworkManagerState) -> TfPauseTransition {
    // IDA 0xed61b8: _setPaused:NO (0xed61d0).
    stub_ed5bd4(state, false)
}

// 0xed61ec — -[TFNetworkManager observeValueForKeyPath:ofObject:change:context:]
// type: void __cdecl(TFNetworkManager *self, SEL, id, id, id, void *)
#[doc(alias = "-[TFNetworkManager observeValueForKeyPath:ofObject:change:context:]")]
pub fn stub_ed61ec(
    state: &TfNetworkManagerState,
    is_own_queue: bool,
    end_if_appropriate: &mut dyn FnMut(),
) {
    // IDA 0xed61ec: when the observed object is our queue and its
    // operationCount hit 0 → _endBackgroundTaskIfAppropriateAsyncSafe
    // (0xed621a..0xed622e).
    if is_own_queue && state.operation_count == 0 {
        end_if_appropriate();
    }
}

// 0xed623c — -[TFNetworkManager _handleReachabilityStatus:]
// type: void __cdecl(TFNetworkManager *self, SEL, int)
#[doc(alias = "-[TFNetworkManager _handleReachabilityStatus:]")]
pub fn stub_ed623c(paused_for_reachability: &mut bool, status: i32) -> Option<bool> {
    // IDA 0xed623c: reachable + pausedForReachability → clear + _setPaused:NO
    // (0xed6250..0xed626a); unreachable + !pausedForReachability → set +
    // _setPaused:YES (0xed626c..0xed6286). Nonzero status counts as reachable
    // (0xed6250). Returns the _setPaused: argument when it acts.
    let reachable = status != 0;
    if reachable {
        if *paused_for_reachability {
            *paused_for_reachability = false;
            Some(false)
        } else {
            None
        }
    } else if !*paused_for_reachability {
        *paused_for_reachability = true;
        Some(true)
    } else {
        None
    }
}

// 0xed628c — -[TFNetworkManager _startBackgroundTaskIfAppropriateAsyncSafe]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _startBackgroundTaskIfAppropriateAsyncSafe]")]
pub fn stub_ed628c(state: &TfNetworkManagerState, start_if_appropriate: &mut dyn FnMut()) {
    // IDA 0xed628c: dispatches the 0xed62f0 block on the tf queue
    // (0xed654c..0xed655c shape) which calls _startBackgroundTaskIfAppropriate.
    stub_ed6320(state, start_if_appropriate);
}

// 0xed62f0 — ___62-[TFNetworkManager _startBackgroundTaskIfAppropriateAsyncSafe]_block_invoke
#[doc(alias = "___62-[TFNetworkManager _startBackgroundTaskIfAppropriateAsyncSafe]_block_invoke")]
pub fn stub_ed62f0(state: &TfNetworkManagerState, start_if_appropriate: &mut dyn FnMut()) {
    // IDA 0xed62f0: _startBackgroundTaskIfAppropriate (0xed6306).
    stub_ed6320(state, start_if_appropriate);
}

// 0xed6320 — -[TFNetworkManager _startBackgroundTaskIfAppropriate]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _startBackgroundTaskIfAppropriate]")]
pub fn stub_ed6320(state: &TfNetworkManagerState, start_task: &mut dyn FnMut()) {
    // IDA 0xed6320: operationCount != 0 (0xed6340) && !paused (0xed6352) →
    // _startBackgroundTask (0xed6368).
    if state.operation_count != 0 && !state.paused {
        start_task();
    }
}

// 0xed6370 — -[TFNetworkManager _startBackgroundTask]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _startBackgroundTask]")]
pub fn stub_ed6370(state: &mut TfNetworkManagerState, begin_task: &mut dyn FnMut() -> u32) {
    // IDA 0xed6370: no-op unless task == INVALID (0xed6396); else
    // beginBackgroundTaskWithExpirationHandler: (0xed63b0..0xed640c, handler
    // 0xed6428) and stores the id (0xed6410..0xed6414).
    if state.background_task == UI_BACKGROUND_TASK_INVALID {
        state.background_task = begin_task();
    }
}

// 0xed6428 — ___40-[TFNetworkManager _startBackgroundTask]_block_invoke
#[doc(alias = "___40-[TFNetworkManager _startBackgroundTask]_block_invoke")]
pub fn stub_ed6428(task: u32, end_task: &mut dyn FnMut(u32), after_end: &mut dyn FnMut(u32)) {
    // IDA 0xed6428: expiration handler — endBackgroundTask: the stored id
    // (0xed6448) then dispatches block_2 (0xed646c..0xed647c, see 0xed6498).
    end_task(task);
    after_end(task);
}

// 0xed6498 — ___40-[TFNetworkManager _startBackgroundTask]_block_invoke_2
#[doc(alias = "___40-[TFNetworkManager _startBackgroundTask]_block_invoke_2")]
pub fn stub_ed6498(state_task: &mut u32, finished: u32) {
    // IDA 0xed6498: clears self id iff it still equals the finished id
    // (0xed64b0..0xed64c2), else leaves it.
    if *state_task == finished {
        *state_task = UI_BACKGROUND_TASK_INVALID;
    }
}

// 0xed651c — -[TFNetworkManager _endBackgroundTaskIfAppropriateAsyncSafe]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _endBackgroundTaskIfAppropriateAsyncSafe]")]
pub fn stub_ed651c(state: &TfNetworkManagerState, end_if_appropriate: &mut dyn FnMut()) {
    // IDA 0xed651c: dispatches the 0xed6580 block on the tf queue
    // (0xed654c..0xed655c shape) which calls _endBackgroundTaskIfAppropriate.
    stub_ed65b0(state, end_if_appropriate);
}

// 0xed6580 — ___60-[TFNetworkManager _endBackgroundTaskIfAppropriateAsyncSafe]_block_invoke
#[doc(alias = "___60-[TFNetworkManager _endBackgroundTaskIfAppropriateAsyncSafe]_block_invoke")]
pub fn stub_ed6580(state: &TfNetworkManagerState, end_if_appropriate: &mut dyn FnMut()) {
    // IDA 0xed6580: _endBackgroundTaskIfAppropriate (0xed6596).
    stub_ed65b0(state, end_if_appropriate);
}

// 0xed65b0 — -[TFNetworkManager _endBackgroundTaskIfAppropriate]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _endBackgroundTaskIfAppropriate]")]
pub fn stub_ed65b0(state: &TfNetworkManagerState, end_task: &mut dyn FnMut()) {
    // IDA 0xed65b0: operationCount == 0 (0xed65d0) → _endBackgroundTask (0xed65e8).
    if state.operation_count == 0 {
        end_task();
    }
}

// 0xed65f0 — -[TFNetworkManager _endBackgroundTask]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager _endBackgroundTask]")]
pub fn stub_ed65f0(state: &mut TfNetworkManagerState) {
    // IDA 0xed65f0: no-op when task == INVALID (0xed661a); else
    // endBackgroundTask: (0xed6634..0xed6650) and reset to INVALID (0xed665a).
    if state.background_task != UI_BACKGROUND_TASK_INVALID {
        state.background_task = UI_BACKGROUND_TASK_INVALID;
    }
}

// 0xed6664 — -[TFNetworkManager .cxx_destruct]
// type: void __cdecl(TFNetworkManager *self, SEL)
#[doc(alias = "-[TFNetworkManager .cxx_destruct]")]
pub fn stub_ed6664(state: &mut TfNetworkManagerState) {
    // IDA 0xed6664: releases _reach/_networkQueue/_fm (0xed667a..0xed66a2).
    *state = TfNetworkManagerState::default();
}

/// `TFReachability` resolved state (IDA 0xed69a4): the
/// `SCNetworkReachability` refs stay engine-side.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TfReachability {
    pub has_ref: bool,
    pub local_wifi: bool,
}

// 0xed69a4 — -[TFReachability initWithNetworkReachabilityRef:localWifiRef:]
// type: TFReachability *__cdecl(TFReachability *self, SEL, __SCNetworkReachability *, char)
#[doc(alias = "-[TFReachability initWithNetworkReachabilityRef:localWifiRef:]")]
pub fn stub_ed69a4(reach_ref_present: bool, local_wifi: bool) -> Option<TfReachability> {
    // IDA 0xed69a4: nil when the reachability ref is nil (0xed69b6); else
    // super init (0xed69d6), CFRetain the ref (0xed6a04), store the wifi flag
    // (0xed6a08).
    if !reach_ref_present {
        return None;
    }
    Some(TfReachability { has_ref: true, local_wifi })
}

// 0xed6c70 — +[TFURLConnectionOperation _runNetworkThread:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[TFURLConnectionOperation _runNetworkThread:]")]
pub fn stub_ed6c70(attach_port: &mut dyn FnMut(), run_once: &mut dyn FnMut()) -> ! {
    // IDA 0xed6c70: retains the arg (0xed6c7a), installs an NSMachPort on the
    // current run loop (0xed6c96..0xed6cea), then loops forever over
    // pool/run/drain (0xed6d30..0xed6d48). Never returns.
    attach_port();
    loop {
        run_once();
    }
}

// 0xed6e48 — +[TFURLConnectionOperation _networkThread]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[TFURLConnectionOperation _networkThread]")]
pub fn stub_ed6e48() -> usize {
    // IDA 0xed6e48: returns the shared network thread (dword_13AA594,
    // 0xed6e5c); the NSThread stays engine-side.
    static THREAD: OnceLock<usize> = OnceLock::new();
    *THREAD.get_or_init(|| 0)
}

// 0xedad4c — +[BugSenseController setErrorNetworkOperationsCompletionBlock:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[BugSenseController setErrorNetworkOperationsCompletionBlock:]")]
pub fn stub_edad4c(block: usize) {
    // IDA 0xedad4c: copies the block into dword_13AA9DC (0xedad6c).
    static COMPLETION: Mutex<Option<usize>> = Mutex::new(None);
    *COMPLETION.lock().unwrap() = Some(block);
}

// 0xee1060 — +[BSAFHTTPRequestOperation networkRequestThread]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[BSAFHTTPRequestOperation networkRequestThread]")]
pub fn stub_ee1060(spawn: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0xee1060: dispatch_once (0xee10a8) building the thread via the
    // 0xee10c4 block, returns the shared thread.
    static THREAD: OnceLock<usize> = OnceLock::new();
    *THREAD.get_or_init(|| spawn())
}

// 0xee10c4 — ___48+[BSAFHTTPRequestOperation networkRequestThread]_block_invoke
#[doc(alias = "___48+[BSAFHTTPRequestOperation networkRequestThread]_block_invoke")]
pub fn stub_ee10c4(spawn_thread: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0xee10c4: NSThread alloc + initWithTarget networkRequestThreadEntryPoint: (0xee10e4..0xee111e).
    spawn_thread()
}

// 0xee114c — +[BSAFHTTPRequestOperation networkRequestThreadEntryPoint:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[BSAFHTTPRequestOperation networkRequestThreadEntryPoint:]")]
pub fn stub_ee114c(pool_drain: &mut dyn FnMut()) -> ! {
    // IDA 0xee114c: loops forever over pool/currentRunLoop/run/drain
    // (0xee11a4..). Never returns.
    loop {
        pool_drain();
    }
}

/// `BSAFNetworkActivityIndicatorManager` resolved state (IDA 0xee2a1c..0xee2cc4).
#[derive(Debug, Default)]
pub struct BsafActivityState {
    pub count: i32,
}

fn bsaf_manager_cell() -> &'static Mutex<BsafActivityState> {
    static CELL: OnceLock<Mutex<BsafActivityState>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(BsafActivityState::default()))
}

// 0xee2a1c — +[BSAFNetworkActivityIndicatorManager sharedManager]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[BSAFNetworkActivityIndicatorManager sharedManager]")]
pub fn stub_ee2a1c() -> &'static Mutex<BsafActivityState> {
    // IDA 0xee2a1c: dispatch_once (0xee2a30..0xee2a42), returns the shared
    // manager (0xee2a34).
    bsaf_manager_cell()
}

// 0xee2a48 — ___52+[BSAFNetworkActivityIndicatorManager sharedManager]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___52+[BSAFNetworkActivityIndicatorManager sharedManager]_block_invoke")]
pub fn stub_ee2a48() -> &'static Mutex<BsafActivityState> {
    // IDA 0xee2a48: alloc+init into the shared slot (0xee2a64..0xee2a82).
    bsaf_manager_cell()
}

// 0xee2a88 — -[BSAFNetworkActivityIndicatorManager setActivityCount:]
// type: void __cdecl(BSAFNetworkActivityIndicatorManager *self, SEL, int)
#[doc(alias = "-[BSAFNetworkActivityIndicatorManager setActivityCount:]")]
pub fn stub_ee2a88(count: &mut i32, value: i32) -> bool {
    // IDA 0xee2a88: KVO will/didChange (0xee2aaa/0xee2ad2), clamps value <= 0
    // to 0 (0xee2ac4..0xee2ace), stores it, then sets the app activity
    // indicator visible iff activityCount > 0 (0xee2af2..0xee2b1c). Returns
    // the visibility decision.
    let clamped = value.max(0);
    *count = clamped;
    clamped > 0
}

// 0xee2b24 — -[BSAFNetworkActivityIndicatorManager startAnimating]
// type: void __cdecl(BSAFNetworkActivityIndicatorManager *self, SEL)
#[doc(alias = "-[BSAFNetworkActivityIndicatorManager startAnimating]")]
pub fn stub_ee2b24(count: &mut i32) -> bool {
    // IDA 0xee2b24: sync enter (0xee2b46), setActivityCount:(count+1)
    // (0xee2b88..0xee2ba2), sync exit (0xee2ba8).
    stub_ee2a88(count, *count + 1)
}

// 0xee2bf4 — -[BSAFNetworkActivityIndicatorManager stopAnimating]
// type: void __cdecl(BSAFNetworkActivityIndicatorManager *self, SEL)
#[doc(alias = "-[BSAFNetworkActivityIndicatorManager stopAnimating]")]
pub fn stub_ee2bf4(count: &mut i32) -> bool {
    // IDA 0xee2bf4: sync enter (0xee2c16), setActivityCount:(count-1)
    // (0xee2c58..0xee2c72), sync exit (0xee2c78).
    stub_ee2a88(count, *count - 1)
}

// 0xee2cc4 — -[BSAFNetworkActivityIndicatorManager activityCount]
// type: int __cdecl(BSAFNetworkActivityIndicatorManager *self, SEL)
#[doc(alias = "-[BSAFNetworkActivityIndicatorManager activityCount]")]
pub fn stub_ee2cc4(count: i32) -> i32 {
    // IDA 0xee2cc4: returns _activityCount (0xee2cd2).
    count
}

// 0xeee6f0 — -[BSReachability networkStatusForFlags:]
// type: int __cdecl(BSReachability *self, SEL, unsigned int)
#[doc(alias = "-[BSReachability networkStatusForFlags:]")]
pub fn stub_eee6f0(flags: u32) -> i32 {
    // IDA 0xeee6f0: same flag folds as -[Reachability networkStatusForFlags:]
    // (0x35c6c), without the PrintReachabilityFlags call: 0 when !(flags&2)
    // (0xeee6f8), direct/status folds (0xeee702..0xeee716), 2 when
    // flags&0x40000 (0xeee71c..0xeee71e).
    reachability_status_for_flags(flags)
}

// 0xf00008 — -[FlurryReachability flurryNetworkStatusForFlags:]
// type: int __cdecl(FlurryReachability *self, SEL, unsigned int)
#[doc(alias = "-[FlurryReachability flurryNetworkStatusForFlags:]")]
pub fn stub_f00008(is_local_wifi_key: bool, flags: u32, log_level: i32, log_uncaught: &mut dyn FnMut()) -> i32 {
    // IDA 0xf00008: 0 when !(flags&2) (0xf0003c); LocalWiFiConnection key →
    // (flags>>17)&1 (0xf00036..0xf00038); else 2 unless flags&0x40000 clear
    // (0xf0003e..0xf00044), then masked (flags&0xFFFCFFFD) == 5 → 0, == 0 →
    // 1, else 1 unless neither bit 0 nor bit 2 is set → 0 with an
    // "Uncaught reachability" log at level 3 (0xf00052..0xf000ac).
    if flags & 2 == 0 {
        return 0;
    }
    if is_local_wifi_key {
        return ((flags >> 17) & 1) as i32;
    }
    if flags & 0x40000 != 0 {
        return 2;
    }
    let masked = flags & 0xFFFC_FFFD;
    if masked == 5 {
        return 0;
    }
    if masked == 0 {
        return 1;
    }
    if flags & 1 == 0 && flags & 4 == 0 {
        if log_level == 3 {
            log_uncaught();
        }
        return 0;
    }
    1
}

/// `FlurryNetworkUtil` resolved state (IDA 0xf0b8bc..0xf0b9c0).
#[derive(Debug, Default)]
pub struct FlurryNetworkUtil {
    pub last_error: Option<FlurryNetworkError>,
}

/// Last network operation error recorded by `FlurryNetworkUtil`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlurryNetworkError {
    pub domain: &'static str,
    pub code: i32,
}

fn flurry_util_cell() -> &'static Mutex<FlurryNetworkUtil> {
    static CELL: OnceLock<Mutex<FlurryNetworkUtil>> = OnceLock::new();
    CELL.get_or_init(Mutex::default)
}

// 0xf0b8bc — +[FlurryNetworkUtil instance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[FlurryNetworkUtil instance]")]
pub fn stub_f0b8bc() -> &'static Mutex<FlurryNetworkUtil> {
    // IDA 0xf0b8bc: synchronized alloc+new when nil (0xf0b8dc..0xf0b942),
    // returns the shared instance (0xf0b968).
    flurry_util_cell()
}

// 0xf0b994 — +[FlurryNetworkUtil setLastNetworkOperationError:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[FlurryNetworkUtil setLastNetworkOperationError:]")]
pub fn stub_f0b994(error: FlurryNetworkError) {
    // IDA 0xf0b994: instance (0xf0b9a6) then setLastNetworkOperationError: (0xf0b9b8).
    flurry_util_cell().lock().unwrap().last_error = Some(error);
}

/// Domain used by `+[FlurryNetworkUtil setNetworkLikelyNotReachable]`
/// (IDA 0xf0b9fe: `kCFErrorDomainCFNetwork`).
pub const FLURRY_NOT_REACHABLE_DOMAIN: &str = "kCFErrorDomainCFNetwork";
/// Code used by `+[FlurryNetworkUtil setNetworkLikelyNotReachable]`
/// (IDA 0xf0ba0e: `1`).
pub const FLURRY_NOT_REACHABLE_CODE: i32 = 1;

// 0xf0b9c0 — +[FlurryNetworkUtil setNetworkLikelyNotReachable]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[FlurryNetworkUtil setNetworkLikelyNotReachable]")]
pub fn stub_f0b9c0() {
    // IDA 0xf0b9c0: instance (0xf0b9da), NSError(kCFErrorDomainCFNetwork, 1)
    // (0xf0b9f0..0xf0ba20), setLastNetworkOperationError: (0xf0ba26..0xf0ba34).
    stub_f0b994(FlurryNetworkError {
        domain: FLURRY_NOT_REACHABLE_DOMAIN,
        code: FLURRY_NOT_REACHABLE_CODE,
    });
}
