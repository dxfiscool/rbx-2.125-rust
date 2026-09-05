// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) gap filler EA-sorted asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x860708..0xacc888 | filtered 8110->8230/10215 (remaining 1985) | datamodel distinct 19745->19865
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: datamodel_shard_276 EA-sorted ascending next uncovered filtered gap, no overlap

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use crate::generated_05::Instance;
use crate::instance::{LuaWebService, PartInstance};
use rbx_core::shared_ptr::{
    ControlBlockPd, CreatableInstanceDeleter, shared_ptr_from_raw,
};
use std::sync::Arc;

/// Rust model of `RBX::TextureTrail` (IDA `0x860708`): the trail adornment
/// leaf; the `PartInstance` ref behind `assignIDREF` is the only member
/// modelled so far.
#[derive(Default)]
pub struct TextureTrail {
    pub part: Mutex<Option<SharedPtr<PartInstance>>>,
}

/// Rust model of `RBX::FloorWire` (IDA `0x86a8ec`): same single-ref shape as
/// `TextureTrail`.
#[derive(Default)]
pub struct FloorWire {
    pub part: Mutex<Option<SharedPtr<PartInstance>>>,
}

/// Setter behind `RefPropDescriptor<TextureTrail, PartInstance>` (IDA
/// `0x860708` `assignIDREF`): stores the retained part.
pub type TrailPartSetter = fn(&TextureTrail, &SharedPtr<PartInstance>);

/// Rust model of `RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,
/// RBX::PartInstance>`: the name/category words, the setter, and the
/// trailing flags/attributes/permissions words. Twin of
/// `generated_14::PlayerModelRefProp` with no getter.
pub struct TextureTrailRefProp {
    pub name: String,
    pub category: String,
    pub setter: TrailPartSetter,
    pub flags: i32,
    pub attributes: u32,
    pub permissions: u32,
}

/// Setter behind `RefPropDescriptor<FloorWire, PartInstance>` (IDA `0x86a8ec`
/// `assignIDREF`).
pub type WirePartSetter = fn(&FloorWire, &SharedPtr<PartInstance>);

/// Rust model of `RBX::Reflection::RefPropDescriptor<RBX::FloorWire,
/// RBX::PartInstance>`: same shape as `TextureTrailRefProp`.
pub struct FloorWireRefProp {
    pub name: String,
    pub category: String,
    pub setter: WirePartSetter,
    pub flags: i32,
    pub attributes: u32,
    pub permissions: u32,
}

/// Rust model of `RBX::MarketplaceService::CurrencyType` (IDA `0x8d2bd0`
/// `Type::getSingleton<CurrencyType>`): the tag word crossing the 4-arg
/// marketplace event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MarketplaceCurrency(pub i32);

/// Slot callback behind the 3-arg marketplace event
/// `(Instance, int, bool)` (IDA `0x8d235a`).
pub type MarketplaceHandler3 = Arc<dyn Fn(SharedPtr<Instance>, i32, bool) + Send + Sync>;

/// Rust model of `rbx::signals::signal<void ()(SharedPtr<Instance>, int,
/// bool)>` (IDA `0x8d235a`): the slot list behind `EventDescImpl<3,
/// MarketplaceService, ...>`; `Mutex` replaces the member-signal word.
#[derive(Default)]
pub struct MarketplaceSignal3 {
    slots: Mutex<Vec<MarketplaceHandler3>>,
}

impl MarketplaceSignal3 {
    pub fn connect(&self, handler: MarketplaceHandler3) {
        self.slots.lock().push(handler);
    }

    pub fn emit(&self, instance: &SharedPtr<Instance>, product: i32, purchased: bool) {
        let live = self.slots.lock().clone();
        for slot in &live {
            slot(SharedPtr::clone(instance), product, purchased);
        }
    }

    pub fn disconnect_all(&self) {
        self.slots.lock().clear();
    }

    pub fn len(&self) -> usize {
        self.slots.lock().len()
    }
}

/// Slot callback behind the 4-arg marketplace event
/// `(Instance, int, bool, CurrencyType)` (IDA `0x8d2a66`).
pub type MarketplaceHandler4 =
    Arc<dyn Fn(SharedPtr<Instance>, i32, bool, MarketplaceCurrency) + Send + Sync>;

/// Rust model of `rbx::signals::signal<void ()(SharedPtr<Instance>, int,
/// bool, CurrencyType)>` (IDA `0x8d2a66`): same slot-list shape as
/// `MarketplaceSignal3`.
#[derive(Default)]
pub struct MarketplaceSignal4 {
    slots: Mutex<Vec<MarketplaceHandler4>>,
}

impl MarketplaceSignal4 {
    pub fn connect(&self, handler: MarketplaceHandler4) {
        self.slots.lock().push(handler);
    }

    pub fn emit(
        &self,
        instance: &SharedPtr<Instance>,
        product: i32,
        purchased: bool,
        currency: MarketplaceCurrency,
    ) {
        let live = self.slots.lock().clone();
        for slot in &live {
            slot(SharedPtr::clone(instance), product, purchased, currency);
        }
    }

    pub fn disconnect_all(&self) {
        self.slots.lock().clear();
    }

    pub fn len(&self) -> usize {
        self.slots.lock().len()
    }
}

/// Rust model of `RBX::Reflection::EventDescImpl<3, MarketplaceService, ...>`
/// (IDA `0x8d66a4` `connectGeneric`, `0x8d6810` `isBroadcast`): the member
/// signal (`+40`) and the broadcast flag (`+44 & 1`) plus the replication
/// half invoked by `sendEvent`/`replicateEvent`.
#[derive(Default)]
pub struct MarketplaceEvent3Desc {
    pub name: String,
    pub broadcast: bool,
    pub signal: MarketplaceSignal3,
    pub remote: MarketplaceSignal3,
}

/// Rust model of `RBX::Reflection::EventDescImpl<4, MarketplaceService, ...>`
/// (IDA `0x8d29f0` `fireEvent`): same shape with the currency arg.
#[derive(Default)]
pub struct MarketplaceEvent4Desc {
    pub name: String,
    pub broadcast: bool,
    pub signal: MarketplaceSignal4,
    pub remote: MarketplaceSignal4,
}

/// Rust model of `RBX::Reflection::GenericSlotWrapper` restricted to the
/// 3-arg marketplace slot (IDA `0x8d6ae4` `execute3`): the native handler
/// stands in for the Lua frame until the script bridge exists.
pub struct MarketplaceSlotWrapper {
    pub handler: MarketplaceHandler3,
}

impl MarketplaceSlotWrapper {
    /// IDA `0x8d6ae4`: packs the 3-`Variant` vector and dispatches the
    /// wrapped slot (`vfptr + 8`); the vector teardown collapses into the
    /// return.
    pub fn execute3(&self, instance: &SharedPtr<Instance>, product: i32, purchased: bool) {
        (self.handler)(SharedPtr::clone(instance), product, purchased);
    }
}

/// Rust model of `boost::_bi::bind_t<void, mf3<GenericSlotWrapper, ...>,
/// list4<value<SharedPtr<GenericSlotWrapper>>, arg<1>, arg<2>, arg<3>>>`
/// (IDA `0x8d69c8`): the retained wrapper; the arg placeholders carry no
/// data.
#[derive(Clone)]
pub struct MarketplaceBind3 {
    pub wrapper: SharedPtr<MarketplaceSlotWrapper>,
}

/// Rust model of `boost::function3<void, SharedPtr<Instance>, int, bool>`
/// holding that bind (IDA `0x8d6e68`): nullability of the retained bind is
/// the vtable word.
#[derive(Clone, Default)]
pub struct MarketplaceFunction3 {
    pub target: Option<MarketplaceBind3>,
}

/// `functor_manager_operation_type` dispatch behind `manage`/`manager`
/// (IDA `0x8d6f60`, `0x8d7270`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketplaceBindOp {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    Check = 3,
    GetType = 4,
}

/// `typeinfo` name compared by the `manager` check-type path (IDA `0x8d736a`
/// `strcmp`).
pub const MARKETPLACE_BIND3_TYPE_NAME: &str = "N5boost3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKiRKbEENS0_5list4INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEE";

/// Shared `manager` switch (IDA `0x8d7270`): 0 clones, 1 moves, 2 destroys,
/// 3 checks the `typeinfo` name (single monomorph, always matches),
/// default reports the name. Move is clone-shaped under `Arc`.
fn marketplace_manage(
    slot: &mut MarketplaceFunction3,
    other: &MarketplaceFunction3,
    op: MarketplaceBindOp,
) {
    match op {
        MarketplaceBindOp::Clone | MarketplaceBindOp::Move => *slot = other.clone(),
        MarketplaceBindOp::Destroy => *slot = MarketplaceFunction3::default(),
        MarketplaceBindOp::Check | MarketplaceBindOp::GetType => {}
    }
}


// 0x860708 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12TextureTrailENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x860708(
    prop: &TextureTrailRefProp,
    trail: &TextureTrail,
    part: &SharedPtr<PartInstance>,
) {
    // IDA 0x860708: retain the handle's `shared_ptr` word
    // (`shared_count` copy, 0x860736), adjust the control block to the
    // contained object (`pi - 36`, 0x86076e), call the virtual setter at
    // `*(desc + 44) + 12` (0x860782), then release the temp (0x86078e).
    // Clone plus the setter dispatch plus `Drop` is the same sequence
    // (cf. `generated_14::stub_ac0518`).
    let part = SharedPtr::clone(part);
    (prop.setter)(trail, &part);
}

// 0x8607e8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12TextureTrailENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use rbx_reflection::generated_shard_fi::stub_0x8607e8 as stub_0x8607e8;

// 0x86a8ec — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x86a8ec(
    prop: &FloorWireRefProp,
    wire: &FloorWire,
    part: &SharedPtr<PartInstance>,
) {
    // IDA 0x86a8ec: same retain/adjust/setter/release shape as 0x860708
    // (decompile 0x86a91a-0x86a972), instantiated for `FloorWire`.
    let part = SharedPtr::clone(part);
    (prop.setter)(wire, &part);
}

// 0x86a9cc — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use rbx_reflection::generated_shard_fj::stub_0x86a9cc as stub_0x86a9cc;

// 0x8ccf1c — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8ccf1c as stub_0x8ccf1c;

// 0x8cd028 — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8cd028 as stub_0x8cd028;

// 0x8cd124 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8cd124 as stub_0x8cd124;

// 0x8cd224 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::~BoundFuncDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8cd224 as stub_0x8cd224;

// 0x8cd33c — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::~RemoteEventDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8cd33c as stub_0x8cd33c;

// 0x8cd8d4 — __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE21fireAndReplicateEventEPS2_S6_ibS7_
// type: void __fastcall(int, int, const shared_count *, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::fireAndReplicateEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::fireAndReplicateEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_0x8cd8d4(
    desc: &MarketplaceEvent4Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
    currency: MarketplaceCurrency,
) {
    // IDA 0x8cd8d4 `RemoteEventDescImpl<4, ...>::fireAndReplicateEvent`:
    // retains the args, `EventDescImpl<4>::fireEvent` (0x8cd95a), releases,
    // then `replicateEvent` (0x8cd9a0) with its own retain/release pair.
    stub_0x8d29f0(desc, instance, product, purchased, currency);
    stub_0x8d2ad0(desc, instance, product, purchased, currency);
}

// 0x8cda20 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_ib
// type: void __fastcall(int, int, const shared_count *, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::fireAndReplicateEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool)")]
// was: RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::fireAndReplicateEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool)
pub fn stub_0x8cda20(
    desc: &MarketplaceEvent3Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8cda20 `RemoteEventDescImpl<3, ...>::fireAndReplicateEvent`:
    // `EventDescImpl<3>::fireEvent` (0x8cdaa2) then `replicateEvent`
    // (0x8cdae4), same retain/release bracketing as the 4-arg twin.
    stub_0x8d22e8(desc, instance, product, purchased);
    stub_0x8d23c4(desc, instance, product, purchased);
}

// 0x8d05d8 — __ZN3RBX15ServiceProvider6createINS_13LuaWebServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(RBX::Instance const*)")]
pub fn stub_0x8d05d8(instance: *const Instance) -> Option<SharedPtr<LuaWebService>> {
    // IDA 0x8d05d8: `findServiceProvider(a1, a2)` null yields `0`
    // (0x8d05e4); else the no-arg service `create` (0x8d05ec). The provider
    // search collapses to instance reachability and creation is
    // default-construct + adopt (same shape as `instance::stub_0x28e0c8`).
    if instance.is_null() {
        return None;
    }
    Some(SharedPtr::new(LuaWebService::default()))
}

// 0x8d0b38 — __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x8d0b38(service: Box<LuaWebService>) -> SharedPtr<LuaWebService> {
    // IDA 0x8d0b38 `shared_ptr<LuaWebService>::shared_ptr<LuaWebService,
    // Creatable<Instance>::Deleter>`: stores px (0x8d0b58), builds the
    // `shared_count` (0x8d0b60), then `_internal_accept_owner<LuaWebService>`
    // at `px + 40` (0x8d0b9e). Box-into-`Arc` is the same single-owner
    // adoption; `LuaWebService` carries no weak-owner word yet, so the
    // accept-owner step is unmodeled.
    shared_ptr_from_raw(service)
}

// 0x8d0ce8 — __ZN5boost6detail12shared_countC2IPN3RBX13LuaWebServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x8d0ce8(
    service: Box<LuaWebService>,
) -> ControlBlockPd<LuaWebService, CreatableInstanceDeleter> {
    // IDA 0x8d0ce8 `shared_count<LuaWebService *, Creatable<Instance>::
    // Deleter>`: nulls the word (0x8d0d14), `operator new(0x14)`
    // (0x8d0d3c), both counts to 1, vtable + px stored (0x8d0d4a-0x8d0d5c).
    // A fresh unit-count block with the tag deleter is the same state.
    ControlBlockPd::new(service, CreatableInstanceDeleter)
}

// 0x8d0df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x8d0df0() {
    // IDA 0x8d0df0 `sp_counted_impl_pd<LuaWebService *,
    // Creatable<Instance>::Deleter>::~sp_counted_impl_pd()`: empty body.
    // Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8d0df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x8d0df8(block: *mut ControlBlockPd<LuaWebService, CreatableInstanceDeleter>) {
    // IDA 0x8d0df8 `dispose`: `Instance::predelete(px)` (0x8d0e00),
    // null-px early-out (0x8d0e06), then the virtual delete through
    // `*px + 8` (0x8d0e14). `dispose_with` with the no-op predelete takes
    // the payload — the delete (same shape as
    // `generated_14::stub_aa1e38`).
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0x8d0e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x8d0e18(
    block: *const ControlBlockPd<LuaWebService, CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<CreatableInstanceDeleter> {
    // IDA 0x8d0e18 `get_deleter`: returns `this + 16` only when the queried
    // `type_info` name matches
    // `"N3RBX9CreatableINS_8InstanceEE7DeleterE"` (0x8d0e2a), else 0.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_deleter(type_name) }
}

// 0x8d0e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x8d0e30(
    block: *const ControlBlockPd<LuaWebService, CreatableInstanceDeleter>,
) -> CreatableInstanceDeleter {
    // IDA 0x8d0e30 `get_untyped_deleter`: unconditionally `this + 16`
    // (0x8d0e32) — the stored deleter.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_untyped_deleter() }
}

// 0x8d22e8 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_ib
// type: void __fastcall(int, int, const shared_count *, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool)const
pub fn stub_0x8d22e8(
    desc: &MarketplaceEvent3Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d22e8 `EventDescImpl<3, ...>::fireEvent`: retains the arg
    // `shared_ptr` (0x8d231c), `signal_with_args<3>::operator()` on the
    // member signal (`a2 + *(a1 + 40)`, 0x8d235a), then releases (0x8d2368).
    // Retain + emit + `Drop` is the same sequence.
    desc.signal.emit(instance, product, purchased);
}

// 0x8d23c4 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_ib
// type: int __fastcall(int, int, int, int, char)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,int,bool)")]
// was: RBX::Reflection::RemoteEventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,int,bool)
pub fn stub_0x8d23c4(
    desc: &MarketplaceEvent3Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d23c4 `RemoteEventDescImpl<3, ...>::replicateEvent`: packs the
    // 3-`Variant` vector with the `Instance`/`int`/`bool` type singletons
    // (0x8d2440-0x8d24ae), fires the replication half (`*a2 + 12`,
    // 0x8d24c2), then destroys the vector (0x8d24cc). Emitting the typed
    // remote signal is the same delivery.
    desc.remote.emit(instance, product, purchased);
}

// 0x8d29f0 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_ibS7_
// type: void __fastcall(int, int, const shared_count *, int, const void *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const
pub fn stub_0x8d29f0(
    desc: &MarketplaceEvent4Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
    currency: MarketplaceCurrency,
) {
    // IDA 0x8d29f0 `EventDescImpl<4, ...>::fireEvent`: same retain/emit/
    // release shape as the 3-arg twin (0x8d2a24-0x8d2a74), carrying the
    // currency word through `signal_with_args<4>` (0x8d2a66).
    desc.signal.emit(instance, product, purchased, currency);
}

// 0x8d2ad0 — __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE14replicateEventEPNS0_11EventSourceES6_ibS7_
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: RBX::Reflection::RemoteEventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_0x8d2ad0(
    desc: &MarketplaceEvent4Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
    currency: MarketplaceCurrency,
) {
    // IDA 0x8d2ad0 `RemoteEventDescImpl<4, ...>::replicateEvent`: packs the
    // 4-`Variant` vector (`Instance`/`int`/`bool`/`CurrencyType` singletons,
    // 0x8d2b50-0x8d2bde), fires the replication half (0x8d2bf2), destroys
    // the vector (0x8d2bfc).
    desc.remote.emit(instance, product, purchased, currency);
}

// 0x8d65f0 — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::~RemoteEventDesc()
pub use rbx_reflection::generated_shard_fl::stub_0x8d65f0 as stub_0x8d65f0;

// 0x8d66a4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x8d66a4(desc: &MarketplaceEvent3Desc, wrapper: &SharedPtr<MarketplaceSlotWrapper>) {
    // IDA 0x8d66a4 `EventDescImpl<3, ...>::connectGeneric`: retains the
    // wrapper (0x8d66d4), `bind` over `GenericSlotWrapper::execute3` with
    // `arg<1..3>` (0x8d671c), wraps it in a `function3` (0x8d6728), adjusts
    // to the member signal (`+ *(a4+40) - 36`, 0x8d6740) and `connect`s
    // (0x8d6752), then clears the temp (0x8d6764) and releases (0x8d677e).
    // The wrapper's handler is already the bound 3-arg closure; connecting
    // it to the member signal is the same subscription.
    desc.signal.connect(SharedPtr::clone(&wrapper.handler));
}

// 0x8d6808 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::isScriptable(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::isScriptable(void)const
pub use rbx_reflection::generated_shard_fl::stub_0x8d6808 as stub_0x8d6808;

// 0x8d6810 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::isBroadcast(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::isBroadcast(void)const
pub fn stub_0x8d6810(desc: &MarketplaceEvent3Desc) -> bool {
    // IDA 0x8d6810 `RemoteEventDesc<...>::isBroadcast`: `*(a1 + 44) & 1`
    // (0x8d6816).
    desc.broadcast
}

// 0x8d6818 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x8d6818(
    desc: &MarketplaceEvent3Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d6818 `EventDescImpl<3, ...>::fireEvent` (args-vector form):
    // asserts `args.size() == 3` (`FLog::Asserts` + `ReleaseAssert`,
    // 0x8d6854-0x8d68c8), adjusts the service (`a2 - 36`, 0x8d68ce),
    // `any_cast`s the three args out of the `Variant` vector
    // (0x8d68ea-0x8d692c), then `signal_with_args<3>::operator()`
    // (0x8d6938). The typed signature guarantees the arity and the casts;
    // the dispatch is the 0x8d22e8 path.
    stub_0x8d22e8(desc, instance, product, purchased);
}

// 0x8d69a4 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x8d69a4(
    desc: &MarketplaceEvent3Desc,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d69a4 `RemoteEventDesc<...>::sendEvent`: tail-calls the remote
    // half's virtual at `*a2 + 12` (0x8d69a4 body). Emitting the remote
    // signal is that delivery.
    desc.remote.emit(instance, product, purchased);
}

// 0x8d69b4 — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x8d69b4(desc: &MarketplaceEvent3Desc) {
    // IDA 0x8d69b4 `EventDescBase<...>::disconnectAll`: adjusts the service
    // to the member signal (`a2 - 36`, 0x8d69ba) and
    // `signal::disconnectAll`s it (`*(a1 + 40) + v2`, 0x8d69b4 body).
    desc.signal.disconnect_all();
}

// 0x8d69c8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKiRKbNS4_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0x8d69c8(wrapper: &SharedPtr<MarketplaceSlotWrapper>) -> MarketplaceBind3 {
    // IDA 0x8d69c8 `bind<void, GenericSlotWrapper, Instance const&, int
    // const&, bool const&, SharedPtr<GenericSlotWrapper>, arg<1..3>>`:
    // retains the wrapper `shared_ptr` (0x8d69f8), builds the `list4`
    // (0x8d6a32), and stores the bind words plus the count (0x8d6a3a-0x8d6a62).
    // The retained wrapper is the whole payload; the arg placeholders
    // carry no data.
    MarketplaceBind3 { wrapper: SharedPtr::clone(wrapper) }
}

// 0x8d6ae4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEEibEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,int,bool>(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute3<boost::shared_ptr<RBX::Instance>,int,bool>(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&)
pub fn stub_0x8d6ae4(
    wrapper: &MarketplaceSlotWrapper,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d6ae4 `GenericSlotWrapper::execute3<Instance, int, bool>`:
    // packs the 3-`Variant` vector with the `Instance`/`int`/`bool`
    // singletons (0x8d6b5a-0x8d6bc8), dispatches the wrapped slot
    // (`*a1 + 8`, 0x8d6bd8), destroys the vector (0x8d6be2).
    wrapper.execute3(instance, product, purchased);
}

// 0x8d6e68 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEEibE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKiRKbEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: void boost::function3<void,boost::shared_ptr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)
pub fn stub_0x8d6e68(dst: &mut MarketplaceFunction3, src: &MarketplaceBind3) {
    // IDA 0x8d6e68 `function3::assign_to<bind_t<...>>`: copies the bind
    // words plus the `shared_count` (0x8d6e8c-0x8d6ea0), installs the
    // stored vtable through `basic_vtable3::assign_to` (0x8d6ef0), then
    // releases the temp (0x8d6efe). Clone-assign is the same
    // retain/install/release.
    dst.target = Some(src.clone());
}

// 0x8d6f60 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x8d6f60(
    slot: &mut MarketplaceFunction3,
    other: &MarketplaceFunction3,
    op: MarketplaceBindOp,
) {
    // IDA 0x8d6f60 `functor_manager<...>::manage`: non-`GetType` ops go to
    // `manager()` (0x8d6f64); `GetType` (4) writes the `typeinfo`
    // (0x8d6f76-0x8d6f7a). Both delegate to the shared switch; `GetType`
    // only reports the name.
    let _ = MARKETPLACE_BIND3_TYPE_NAME;
    marketplace_manage(slot, other, op);
}

// 0x8d6f7c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEvSC_ibE6invokeERNS1_15function_bufferESC_ib
// type: int __fastcall(int *, int, int, char)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,int,bool)")]
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,boost::shared_ptr<RBX::Instance>,int,bool>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,int,bool)
pub fn stub_0x8d6f7c(
    bind: &MarketplaceBind3,
    instance: &SharedPtr<Instance>,
    product: i32,
    purchased: bool,
) {
    // IDA 0x8d6f7c `void_function_obj_invoker3<...>::invoke`: builds the
    // `list3` of arg refs (0x8d6f90-0x8d6f98) and runs
    // `list4::operator()` (0x8d6fa4), which unpacks to the `mf3` call on
    // the retained wrapper — the `execute3` path.
    stub_0x8d6ae4(&bind.wrapper, instance, product, purchased);
}

// 0x8d6fa8 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEEibE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x8d6fa8() -> ! {
    todo!("0x8d6fa8 bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x8d7090 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEEibE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x8d7090() -> ! {
    todo!("0x8d7090 bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x8d7174 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEEibE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,int,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x8d7174() -> ! {
    todo!("0x8d7174 void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,int,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x8d7248 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKiRKbEENS0_5list3IRSI_RiRbEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,int &,bool &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,int &,bool &> &,int)
pub fn stub_0x8d7248() -> ! {
    todo!("0x8d7248 void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &> &,int)")
}

// 0x8d7270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x8d7270() -> ! {
    todo!("0x8d7270 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x8d8294 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8d8294() -> ! {
    todo!("0x8d8294 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8d84f0 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8d84f0 as stub_0x8d84f0;

// 0x8d8514 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8d8514 as stub_0x8d8514;

// 0x8d85c8 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EEC2EMS2_FvS6_ibS7_EPKcSD_SD_SD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),char const*,char const*,char const*,char const*,bool,char const*,RBX::MarketplaceService::CurrencyType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::BoundFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),char const*,char const*,char const*,char const*,bool,char const*,RBX::MarketplaceService::CurrencyType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8d85c8() -> ! {
    todo!("0x8d85c8 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),char const*,char const*,char const*,char const*,bool,char const*,RBX::MarketplaceService::CurrencyType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8d88a4 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_
// type: int __fastcall(int, int, int *, int, int *, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0x8d88a4() -> ! {
    todo!("0x8d88a4 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8d8928 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::~BoundFuncDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8d8928 as stub_0x8d8928;

// 0x8d8a54 — __ZNK3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0x8d8a54() -> ! {
    todo!("0x8d8a54 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8d8b88 — __ZN3RBX10Reflection11Call4HelperINS_18MarketplaceServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEES6_ibS7_vE4callEPS2_S9_RNS0_7VariantERKS6_RKiRKbRKS7_
// type: void __fastcall(int, char *, int, int, const shared_count *, boost::detail::sp_counted_base *, unsigned __int8 *, _DWORD *, int, int)
#[doc(alias = "RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")]
// was: RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)
pub fn stub_0x8d8b88() -> ! {
    todo!("0x8d8b88 RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")
}

// 0x8d8e70 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8d8e70() -> ! {
    todo!("0x8d8e70 RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8d903c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// type: int __fastcall(int, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0x8d903c() -> ! {
    todo!("0x8d903c RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8d9088 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8d9088 as stub_0x8d9088;

// 0x8d919c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_0x8d919c() -> ! {
    todo!("0x8d919c RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x8dd22c — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8dd22c as stub_0x8dd22c;

// 0x8dd2e0 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x8dd2e0() -> ! {
    todo!("0x8dd2e0 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8dd444 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const
pub use rbx_reflection::generated_shard_fm::stub_0x8dd444 as stub_0x8dd444;

// 0x8dd44c — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const
pub fn stub_0x8dd44c() -> ! {
    todo!("0x8dd44c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")
}

// 0x8dd454 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x8dd454() -> ! {
    todo!("0x8dd454 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd600 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x8dd600() -> ! {
    todo!("0x8dd600 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd610 — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x8dd610() -> ! {
    todo!("0x8dd610 RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8dd624 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKiRKbRKNS1_18MarketplaceService12CurrencyTypeENS4_IS3_EENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISP_T0_T1_T2_T3_T4_EENSN_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSS_FSP_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_0x8dd624() -> ! {
    todo!("0x8dd624 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x8dd740 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4IN5boost10shared_ptrINS_8InstanceEEEibNS_18MarketplaceService12CurrencyTypeEEEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute4<boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)
pub fn stub_0x8dd740() -> ! {
    todo!("0x8dd740 void RBX::Reflection::GenericSlotWrapper::execute4<rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")
}

// 0x8ddc84 — __ZN5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS2_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKiRKbRKS6_EENS9_5list5INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: void boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_0x8ddc84() -> ! {
    todo!("0x8ddc84 void boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0x8ddd7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x8ddd7c() -> ! {
    todo!("0x8ddd7c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8ddd98 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEvSC_ibSK_E6invokeERNS1_15function_bufferESC_ibSK_
// type: int __fastcall(int *, int, int, char, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_0x8ddd98() -> ! {
    todo!("0x8ddd98 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")
}

// 0x8dddcc — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x8dddcc() -> ! {
    todo!("0x8dddcc bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0x8ddeb4 — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x8ddeb4() -> ! {
    todo!("0x8ddeb4 bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x8ddf98 — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x8ddf98() -> ! {
    todo!("0x8ddf98 void boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x8de06c — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKNS3_INS4_8InstanceEEERKiRKbRKNS4_18MarketplaceService12CurrencyTypeEEENS0_5list4IRSJ_RiRbRSR_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<boost::shared_ptr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<boost::shared_ptr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)
pub fn stub_0x8de06c() -> ! {
    todo!("0x8de06c void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)")
}

// 0x8de09c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x8de09c() -> ! {
    todo!("0x8de09c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x8df0c4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8df0c4() -> ! {
    todo!("0x8df0c4 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8df390 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8df390 as stub_0x8df390;

// 0x8df3b4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8df3b4 as stub_0x8df3b4;

// 0x8df468 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EEC2EMS2_FvS6_ibEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8df468() -> ! {
    todo!("0x8df468 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8df684 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
// type: int __fastcall(int, int, int *, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0x8df684() -> ! {
    todo!("0x8df684 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8df6ec — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8df6ec as stub_0x8df6ec;

// 0x8df80c — __ZNK3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0x8df80c() -> ! {
    todo!("0x8df80c RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8df928 — __ZN3RBX10Reflection11Call3HelperINS_18MarketplaceServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEibES6_ibvE4callEPS2_S8_RNS0_7VariantERKS6_RKiRKb
// type: void __fastcall(int, char *, int, int, const shared_count *, _DWORD *, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx_core::SharedPtr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&)")]
// was: RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),boost::shared_ptr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&)
pub fn stub_0x8df928() -> ! {
    todo!("0x8df928 RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx_core::SharedPtr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&)")
}

// 0x8e6498 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8e6498 as stub_0x8e6498;

// 0x8e6f60 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x8e6f60() -> ! {
    todo!("0x8e6f60 RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8e70e4 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()
pub use rbx_reflection::generated_shard_fm::stub_0x8e70e4 as stub_0x8e70e4;

// 0x8e7198 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x8e7198() -> ! {
    todo!("0x8e7198 RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8e72ec — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x8e72ec() -> ! {
    todo!("0x8e72ec RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8e744c — __ZNK3RBX10Reflection13EventDescBaseINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x8e744c() -> ! {
    todo!("0x8e744c RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8e81e0 — __ZN3RBX13ScriptService12waitForChildEN5boost8weak_ptrINS_8InstanceEEESsNS1_8functionIFvNS1_10shared_ptrIS3_EEEEENS5_IFvSsEEE
// type: void __fastcall(int, _DWORD *, const std::string *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptService::waitForChild(rbx_core::Weak<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: RBX::ScriptService::waitForChild(boost::weak_ptr<RBX::Instance>,std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_0x8e81e0() -> ! {
    todo!("0x8e81e0 RBX::ScriptService::waitForChild(rbx_core::Weak<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")
}

// 0x8e83c4 — __ZN3RBX13ScriptService12onChildAddedEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, RBX::Instance **)
#[doc(alias = "RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ScriptService::onChildAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_0x8e83c4() -> ! {
    todo!("0x8e83c4 RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8690 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)
pub fn stub_0x8e8690() -> ! {
    todo!("0x8e8690 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")
}

// 0x8e8d24 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x8e8d24() -> ! {
    todo!("0x8e8d24 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8d50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x8e8d50() -> ! {
    todo!("0x8e8d50 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8e24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_0x8e8e24() -> ! {
    todo!("0x8e8e24 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8e40 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_0x8e8e40() -> ! {
    todo!("0x8e8e40 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8e5c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_0x8e8e5c() -> ! {
    todo!("0x8e8e5c void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0xa04c10 — __ZN3RBX7Network7Players14reportAbuseLuaEN5boost10shared_ptrINS_8InstanceEEESsSs
// type: void __fastcall(RBX::Network::Players *, _DWORD *, int, int)
#[doc(alias = "RBX::Network::Players::reportAbuseLua(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string)")]
// was: RBX::Network::Players::reportAbuseLua(boost::shared_ptr<RBX::Instance>,std::string,std::string)
pub fn stub_0xa04c10() -> ! {
    todo!("0xa04c10 RBX::Network::Players::reportAbuseLua(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string)")
}

// 0xa23aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa23aa8() -> ! {
    todo!("0xa23aa8 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa23ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xa23ab0() -> ! {
    todo!("0xa23ab0 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xa23ad0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa23ad0() -> ! {
    todo!("0xa23ad0 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa23ae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa23ae8() -> ! {
    todo!("0xa23ae8 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa327a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xa327a0() -> ! {
    todo!("0xa327a0 boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xa327b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xa327b0() -> ! {
    todo!("0xa327b0 boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xa327c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xa327c8() -> ! {
    todo!("0xa327c8 boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xa851a8 — __ZN3RBX7Network6Player29loadCharacterAppearanceScriptEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Player::loadCharacterAppearanceScript(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::loadCharacterAppearanceScript(boost::shared_ptr<RBX::Instance>)
pub use crate::generated_13::stub_a851a8 as stub_0xa851a8;

// 0xa8e5f4 — __ZL29setAppearanceParentNullScriptN5boost10shared_ptrIN3RBX8InstanceEEE
// type: void __fastcall(RBX::Instance **)
#[doc(alias = "setAppearanceParentNullScript(rbx_core::SharedPtr<RBX::Instance>)")]
// was: setAppearanceParentNullScript(boost::shared_ptr<RBX::Instance>)
pub use crate::generated_13::stub_a8e5f4 as stub_0xa8e5f4;

// 0xac1a6c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS3_FvSsS7_EPKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::generated_14::stub_ac1a6c as stub_0xac1a6c;

// 0xac1dac — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub use crate::generated_14::stub_ac1dac as stub_0xac1dac;

// 0xac1e4c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::generated_14::stub_ac1e4c as stub_0xac1e4c;

// 0xac2134 — __ZN3RBX10Reflection11Call2HelperINS_7Network6PlayerEMS3_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS7_vE4callEPS3_S9_RNS0_7VariantERKSsRKS7_
// type: void __fastcall(int, char *, int, int, std::string *, int *)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),std::string,boost::shared_ptr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,boost::shared_ptr<RBX::Instance> const&)
pub use crate::generated_14::stub_ac2134 as stub_0xac2134;

// 0xac245c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS3_FS7_SsEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::generated_14::stub_ac245c as stub_0xac245c;

// 0xac2704 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
pub use crate::generated_14::stub_ac2704 as stub_0xac2704;

// 0xac283c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::generated_14::stub_ac283c as stub_0xac283c;

// 0xac297c — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FN5boost10shared_ptrINS_8InstanceEEESsESsS7_E4callEPS3_S9_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, _DWORD *, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Player,boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Player*,boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)
pub use crate::generated_14::stub_ac297c as stub_0xac297c;

// 0xac5604 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
pub use crate::generated_14::stub_ac5604 as stub_0xac5604;

// 0xac56e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::generated_14::stub_ac56e0 as stub_0xac56e0;

// 0xac8194 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::generated_14::stub_ac8194 as stub_0xac8194;

// 0xac8424 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub use crate::generated_14::stub_ac8424 as stub_0xac8424;

// 0xac84c4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::generated_14::stub_ac84c4 as stub_0xac84c4;

// 0xac86f8 — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FvN5boost10shared_ptrINS_8InstanceEEEES7_vE4callEPS3_S9_RNS0_7VariantERKS7_
// type: void __fastcall(int, char *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub use crate::generated_14::stub_ac86f8 as stub_0xac86f8;

// 0xaca39c — __ZN5boost4bindIvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS1_INS2_9DataModelEEESsS5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_T2_ENS8_9list_av_3IT3_T4_T5_E4typeEEESF_SH_SI_SJ_
// type: void __fastcall(_DWORD *, int, std::string *, pthread_mutex_t *, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list_av_3<std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>>::type> boost::bind<void,std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>,std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>>(void (*)(std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>)")]
// was: boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_3<std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>,std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>(void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
pub use crate::generated_14::stub_aca39c as stub_0xaca39c;

// 0xaca760 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::~list3()")]
// was: boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::~list3()
pub use crate::generated_14::stub_aca760 as stub_0xaca760;

// 0xacaa30 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS8_INS2_8InstanceEEESaISB_EEEESsbdS5_NS_3argILi1EEENSF_ILi2EEESsbdEENS_3_bi6bind_tIT_PFSK_T0_T1_T2_T3_T4_T5_ENSI_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESS_SU_SV_SW_SX_SY_SZ_
// type: void __fastcall(int, int, int *, const std::string *, int, double)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double,rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)
pub use crate::generated_14::stub_acaa30 as stub_0xacaa30;

// 0xacb914 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS9_INS3_8InstanceEEESaISC_EEEESsbdENS0_5list6INS0_5valueIS6_EENS_3argILi1EEENSL_ILi2EEENSJ_ISsEENSJ_IbEENSJ_IdEEEEEC2ESH_RKSR_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)
pub use crate::generated_14::stub_acb914 as stub_0xacb914;

// 0xacbfd8 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSD_5list6INSD_5valueISI_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)")]
// was: void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)
pub use crate::generated_14::stub_acbfd8 as stub_0xacbfd8;

// 0xacc888 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub use crate::generated_14::stub_acc888 as stub_0xacc888;


#[cfg(test)]
mod marketplace_batch1_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    fn trail_prop() -> TextureTrailRefProp {
        TextureTrailRefProp {
            name: "Trail".to_string(),
            category: "Appearance".to_string(),
            setter: |trail, part| *trail.part.lock() = Some(SharedPtr::clone(part)),
            flags: 0,
            attributes: 0,
            permissions: 0,
        }
    }

    fn wire_prop() -> FloorWireRefProp {
        FloorWireRefProp {
            name: "Wire".to_string(),
            category: "Appearance".to_string(),
            setter: |wire, part| *wire.part.lock() = Some(SharedPtr::clone(part)),
            flags: 0,
            attributes: 0,
            permissions: 0,
        }
    }

    #[test]
    fn assign_idref_stores_part() {
        let part = SharedPtr::new(PartInstance::default());
        let trail = TextureTrail::default();
        stub_0x860708(&trail_prop(), &trail, &part);
        assert!(trail.part.lock().as_ref().is_some_and(|p| SharedPtr::ptr_eq(p, &part)));
        let wire = FloorWire::default();
        stub_0x86a8ec(&wire_prop(), &wire, &part);
        assert!(wire.part.lock().as_ref().is_some_and(|p| SharedPtr::ptr_eq(p, &part)));
    }

    #[test]
    fn create_needs_live_instance() {
        assert!(stub_0x8d05d8(std::ptr::null()).is_none());
        let root = Instance::default();
        let svc = stub_0x8d05d8(&root as *const Instance);
        assert!(svc.is_some());
    }

    #[test]
    fn control_block_round_trip() {
        let svc = stub_0x8d0b38(Box::new(LuaWebService::default()));
        assert_eq!(SharedPtr::strong_count(&svc), 1);
        drop(svc);
        let mut block = stub_0x8d0ce8(Box::new(LuaWebService::default()));
        assert_eq!(block.use_count(), 1);
        assert!(block.get().is_some());
        assert!(stub_0x8d0e18(&block as *const _, "bogus-type").is_none());
        assert!(stub_0x8d0e18(
            &block as *const _,
            rbx_core::shared_ptr::CREATABLE_INSTANCE_DELETER_TYPE_NAME
        )
        .is_some());
        let _ = stub_0x8d0e30(&block as *const _);
        stub_0x8d0df8(&mut block as *mut _);
        assert!(block.get().is_none());
        stub_0x8d0df0();
    }

    #[test]
    fn connect_fire_disconnect() {
        let desc = MarketplaceEvent3Desc { name: "Prompt".to_string(), ..Default::default() };
        assert!(!stub_0x8d6810(&desc));
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        let wrapper = SharedPtr::new(MarketplaceSlotWrapper {
            handler: Arc::new(move |_, product: i32, purchased: bool| {
                probe.store(product + i32::from(purchased), Ordering::Relaxed);
            }),
        });
        stub_0x8d66a4(&desc, &wrapper);
        assert_eq!(desc.signal.len(), 1);
        let inst = SharedPtr::new(Instance::default());
        stub_0x8d22e8(&desc, &inst, 40, true);
        assert_eq!(seen.load(Ordering::Relaxed), 41);
        stub_0x8d6818(&desc, &inst, 7, false);
        assert_eq!(seen.load(Ordering::Relaxed), 7);
        stub_0x8d69b4(&desc);
        assert_eq!(desc.signal.len(), 0);
        stub_0x8d22e8(&desc, &inst, 1, true);
        assert_eq!(seen.load(Ordering::Relaxed), 7);
    }

    #[test]
    fn fire_and_replicate_reaches_both_halves() {
        let desc = MarketplaceEvent4Desc { name: "Purchase".to_string(), ..Default::default() };
        let local = Arc::new(AtomicI32::new(0));
        let remote = Arc::new(AtomicI32::new(0));
        let lp = Arc::clone(&local);
        desc.signal.connect(Arc::new(move |_, p: i32, _: bool, c: MarketplaceCurrency| {
            lp.store(p + c.0, Ordering::Relaxed);
        }));
        let rp = Arc::clone(&remote);
        desc.remote.connect(Arc::new(move |_, p: i32, _: bool, c: MarketplaceCurrency| {
            rp.store(1000 + p + c.0, Ordering::Relaxed);
        }));
        let inst = SharedPtr::new(Instance::default());
        stub_0x8cd8d4(&desc, &inst, 9, true, MarketplaceCurrency(2));
        assert_eq!(local.load(Ordering::Relaxed), 11);
        assert_eq!(remote.load(Ordering::Relaxed), 1011);
        stub_0x8d2ad0(&desc, &inst, 1, false, MarketplaceCurrency(0));
        assert_eq!(remote.load(Ordering::Relaxed), 1001);
        let desc3 = MarketplaceEvent3Desc::default();
        let rp3 = Arc::clone(&remote);
        desc3.remote.connect(Arc::new(move |_, p: i32, _: bool| {
            rp3.store(p, Ordering::Relaxed);
        }));
        stub_0x8cda20(&desc3, &inst, 5, false);
        assert_eq!(remote.load(Ordering::Relaxed), 5);
        stub_0x8d69a4(&desc3, &inst, 6, true);
        assert_eq!(remote.load(Ordering::Relaxed), 6);
        stub_0x8d23c4(&desc3, &inst, 8, false);
        assert_eq!(remote.load(Ordering::Relaxed), 8);
    }

    #[test]
    fn bind_execute_invoker_and_manager() {
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        let wrapper = SharedPtr::new(MarketplaceSlotWrapper {
            handler: Arc::new(move |_, p: i32, _: bool| {
                probe.store(p, Ordering::Relaxed);
            }),
        });
        let bind = stub_0x8d69c8(&wrapper);
        let inst = SharedPtr::new(Instance::default());
        stub_0x8d6ae4(&wrapper, &inst, 3, true);
        assert_eq!(seen.load(Ordering::Relaxed), 3);
        stub_0x8d6f7c(&bind, &inst, 4, false);
        assert_eq!(seen.load(Ordering::Relaxed), 4);
        let mut slot = MarketplaceFunction3::default();
        stub_0x8d6e68(&mut slot, &bind);
        assert!(slot.target.is_some());
        let mut other = MarketplaceFunction3::default();
        stub_0x8d6f60(&mut other, &slot, MarketplaceBindOp::Clone);
        assert!(other.target.is_some());
        stub_0x8d6f60(&mut other, &slot, MarketplaceBindOp::Check);
        assert!(other.target.is_some());
        stub_0x8d6f60(&mut other, &slot, MarketplaceBindOp::GetType);
        assert_eq!(
            MARKETPLACE_BIND3_TYPE_NAME,
            "N5boost3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKiRKbEENS0_5list4INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEE"
        );
        stub_0x8d6f60(&mut other, &slot, MarketplaceBindOp::Destroy);
        assert!(other.target.is_none());
    }
}
