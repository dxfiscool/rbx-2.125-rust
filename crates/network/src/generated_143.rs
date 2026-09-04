//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|RBX::Network (case-insensitive) -> 4479 funcs, 2640 already stubbed (1839 remaining before batch); continuing filtered ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x9e8a8c..0x9fa9a4 | existing 2640 -> 2740 filtered total (out of 4479), rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x9e8a8c — __ZN3RBX7Network16ServerReplicator15ServerStatsItemC2ERKN5boost10shared_ptrIKS1_EE
// demangled: RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(boost::shared_ptr<RBX::Network::ServerReplicator const> const&)
// type: RBX::Stats::Item *__fastcall(RBX::Stats::Item *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Network::Replicator::StatsItem *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(rbx_core::SharedPtr<RBX::Network::ServerReplicator const> const&)")]
pub fn stub_9e8a8c() -> crate::replicator::ServerStatsItem {
    // IDA 0x9e8a8c (C2): `ServerStatsItem` from the replicator shared pointer; stateless.
    crate::replicator::create_stats_item()
}

// 0x9e9460 — __ZN3RBX7Network10Replicator9StatsItemD2Ev
// demangled: RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9e9460() {
    // IDA 0x9e9460: `StatsItem` D2; rows stay engine-side.
}

// 0x9e967c — __ZN3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
// demangled: RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e967c(_item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e967c (D1): tail-calls D2; stateless drop covers it.
}

// 0x9e9688 — __ZN3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
// demangled: RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e9688(item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e9688 (D0): D2 then `operator delete`; by-value drop covers both.
    drop(item);
}

// 0x9e9728 — __ZN3RBX7Network16ServerReplicator15ServerStatsItem6updateEv
// demangled: RBX::Network::ServerReplicator::ServerStatsItem::update(void)
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *this)
#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::update(void)")]
pub fn stub_9e9728(replicator_present: bool, base_refresh: &mut dyn FnMut(), extra_refresh: &mut dyn FnMut()) {
    // IDA 0x9e9728: base update first, then the per-replicator rows.
    crate::replicator::server_stats_item_update(replicator_present, base_refresh, extra_refresh);
}

// 0x9e9b30 — __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
// demangled: non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e9b30(_item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e9b30 (ZThn32 D1): adjusts `this`, then D1.
}

// 0x9e9b3c — __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
// demangled: non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e9b3c(item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e9b3c (ZThn32 D0): adjusts `this`, then D0.
    drop(item);
}

// 0x9e9be4 — __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
// demangled: non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e9be4(_item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e9be4 (ZThn36 D1): adjusts `this`, then D1.
}

// 0x9e9bf0 — __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
// demangled: non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()
// type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_9e9bf0(item: crate::replicator::ServerStatsItem) {
    // IDA 0x9e9bf0 (ZThn36 D0): adjusts `this`, then D0.
    drop(item);
}

// 0x9e9c98 — __ZN3RBX7Network10Replicator9StatsItem6updateEv
// demangled: RBX::Network::Replicator::StatsItem::update(void)
// type: void __fastcall(RBX::Network::Replicator::StatsItem *this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::update(void)")]
pub fn stub_9e9c98(refresh: &mut dyn FnMut()) {
    // IDA 0x9e9c98: refresh the base stats rows.
    crate::replicator::base_stats_item_update(refresh);
}

// 0x9ea2b8 — __ZN3RBX7Network10Replicator9StatsItemD1Ev
// demangled: RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea2b8() {
    // IDA 0x9ea2b8: `StatsItem` D1; rows stay engine-side.
}

// 0x9ea2c4 — __ZN3RBX7Network10Replicator9StatsItemD0Ev
// demangled: RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea2c4() {
    // IDA 0x9ea2c4: `StatsItem` D0; rows stay engine-side.
}

// 0x9ea364 — __ZThn32_N3RBX7Network10Replicator9StatsItemD1Ev
// demangled: non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea364() {
    // IDA 0x9ea364 (ZThn32 D1): adjusts `this`, then D1.
}

// 0x9ea370 — __ZThn32_N3RBX7Network10Replicator9StatsItemD0Ev
// demangled: non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea370() {
    // IDA 0x9ea370 (ZThn32 D0): adjusts `this`, then D0.
}

// 0x9ea414 — __ZThn36_N3RBX7Network10Replicator9StatsItemD1Ev
// demangled: non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea414() {
    // IDA 0x9ea414 (ZThn36 D1): adjusts `this`, then D1.
}

// 0x9ea420 — __ZThn36_N3RBX7Network10Replicator9StatsItemD0Ev
// demangled: non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_9ea420() {
    // IDA 0x9ea420 (ZThn36 D0): adjusts `this`, then D0.
}

// 0x9eaa38 — __ZN3RBX7Network12RakStatsItemD1Ev
// demangled: RBX::Network::RakStatsItem::~RakStatsItem()
// type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
#[doc(alias = "RBX::Network::RakStatsItem::~RakStatsItem()")]
pub fn stub_9eaa38() {
    // IDA 0x9eaa38: `RakStatsItem` D1; rows stay engine-side.
}

// 0x9eaab0 — __ZThn32_N3RBX7Network12RakStatsItemD1Ev
// demangled: non-virtual thunk to RBX::Network::RakStatsItem::~RakStatsItem()
// type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::RakStatsItem::~RakStatsItem()")]
pub fn stub_9eaab0() {
    // IDA 0x9eaab0 (ZThn32 D1): adjusts `this`, then D1.
}

// 0x9eab30 — __ZThn32_N3RBX7Network12RakStatsItemD0Ev
// demangled: non-virtual thunk to RBX::Network::RakStatsItem::~RakStatsItem()
// type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::RakStatsItem::~RakStatsItem()")]
pub fn stub_9eab30() {
    // IDA 0x9eab30 (ZThn32 D0): adjusts `this`, then D0.
}

// 0x9ebd98 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network12RakStatsItemES7_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::RakStatsItem,RBX::Network::RakStatsItem>(boost::shared_ptr<RBX::Network::RakStatsItem> const*,RBX::Network::RakStatsItem *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::RakStatsItem,RBX::Network::RakStatsItem>(rbx_core::SharedPtr<RBX::Network::RakStatsItem> const*,RBX::Network::RakStatsItem *)const")]
pub fn stub_9ebd98() {
    // IDA 0x9ebd98: `enable_shared_from_this::accept_owner`; weak-owner wiring stays engine-side.
}

// 0x9ec058 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network12RakStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void __fastcall(void *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9ec058() {
    // IDA 0x9ec058: `sp_counted_impl_pd<RakStatsItem>` D0; no crate state.
}

// 0x9ec068 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network12RakStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9ec068() {
    // IDA 0x9ec068: `sp_counted_impl_pd<RakStatsItem>::get_deleter`; no crate state.
}

// 0x9ec080 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network12RakStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::RakStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9ec080() {
    // IDA 0x9ec080: `sp_counted_impl_pd<RakStatsItem>::get_untyped_deleter`; no crate state.
}

// 0x9ec084 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ServerReplicator15ServerStatsItemES8_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ServerReplicator::ServerStatsItem,RBX::Network::ServerReplicator::ServerStatsItem>(boost::shared_ptr<RBX::Network::ServerReplicator::ServerStatsItem> const*,RBX::Network::ServerReplicator::ServerStatsItem *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ServerReplicator::ServerStatsItem,RBX::Network::ServerReplicator::ServerStatsItem>(rbx_core::SharedPtr<RBX::Network::ServerReplicator::ServerStatsItem> const*,RBX::Network::ServerReplicator::ServerStatsItem *)const")]
pub fn stub_9ec084() {
    // IDA 0x9ec084: `enable_shared_from_this::accept_owner`; weak-owner wiring stays engine-side.
}

// 0x9ec340 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicator15ServerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9ec340() {
    // IDA 0x9ec340: `sp_counted_impl_pd<ServerStatsItem>` D1; no crate state.
}

// 0x9ec344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicator15ServerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void __fastcall(void *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9ec344() {
    // IDA 0x9ec344: `sp_counted_impl_pd<ServerStatsItem>` D0; no crate state.
}

// 0x9ec350 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicator15ServerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9ec350() {
    // IDA 0x9ec350: `sp_counted_impl_pd<ServerStatsItem>::dispose`; no crate state.
}

// 0x9ec36c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicator15ServerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9ec36c() {
    // IDA 0x9ec36c: `sp_counted_impl_pd<ServerStatsItem>::get_deleter`; no crate state.
}

// 0x9ec384 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicator15ServerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator::ServerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9ec384() {
    // IDA 0x9ec384: `sp_counted_impl_pd<ServerStatsItem>::get_untyped_deleter`; no crate state.
}

// 0x9ec38c — __ZN3RBX10Reflection9EventDescINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::EventDesc(rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::EventDesc(rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_9ec38c() {
    // IDA 0x9ec38c: `EventDesc<ServerReplicator, ...>` C2; descriptor state stays engine-side.
}

// 0x9ec76c — __ZN3RBX10Reflection9EventDescINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::~EventDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::~EventDesc()")]
pub fn stub_9ec76c() {
    // IDA 0x9ec76c: `EventDesc<ServerReplicator, ...>` D0; descriptor state stays engine-side.
}

// 0x9ec848 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<3,RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_9ec848(list: &mut crate::signal::SlotList) -> crate::signal::SlotId {
    // IDA 0x9ec848: `EventDescImpl::connectGeneric` — wrap and insert the generic slot.
    list.insert()
}

// 0x9ecccc — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// demangled: RBX::Reflection::EventDescImpl<3,RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_9ecccc(list: &crate::signal::SlotList, fire: impl FnMut()) {
    // IDA 0x9ecccc: `EventDescImpl::fireEvent` — emit to each connected slot.
    crate::signal::emit_each(list, fire);
}

// 0x9ecef4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_9ecef4(list: &mut crate::signal::SlotList) {
    // IDA 0x9ecef4: `EventDescBase::disconnectAll` — tears down the event signal (unlink all).
    list.disconnect_all();
}

// 0x9ed0b8 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// demangled: RBX::Reflection::EventDescBase<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(int,bool,int)> const&)const
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(int,bool,int)> const&)const")]
pub fn stub_9ed0b8<T>(
    list: &mut crate::signal::SlotList,
    slot: &mut Option<Box<T>>,
    functor: T,
    source_present: bool,
) -> Option<crate::signal::SlotId> {
    // IDA 0x9ed0b8: `EventDescBase::connect` — wrap, assign, insert; null when sourceless.
    crate::signal::connect(list, slot, functor, source_present)
}

// 0x9ef7d8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvvELi0EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(void),0>::~BoundFuncDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_9ef7d8() {
    // IDA 0x9ef7d8: `BoundFuncDesc<ServerReplicator, void()>` D0; descriptor state stays engine-side.
}

// 0x9ef8b4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_9ef8b4(run: impl FnOnce()) {
    // IDA 0x9ef8b4: member-pointer dispatch into the bound `void()` (0x9ef8b6..0x9ef8d6).
    run();
}

// 0x9ef8dc — __ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvbELi1EEC2EMS3_FvbEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::ServerReplicator::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::ServerReplicator::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_9ef8dc() {
    // IDA 0x9ef8dc: `BoundFuncDesc<ServerReplicator, void(bool)>` C2; descriptor state stays engine-side.
}

// 0x9efaac — __ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvbELi1EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::~BoundFuncDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_9efaac() {
    // IDA 0x9efaac: `BoundFuncDesc<ServerReplicator, void(bool)>` D0; descriptor state stays engine-side.
}

// 0x9efba8 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_9efba8(arg: bool, run: impl FnOnce(bool)) {
    // IDA 0x9efba8: member-pointer dispatch with `getArg<bool>` (0x9efbb0..0x9efbe0); the argument arrives engine-side.
    run(arg);
}

// 0x9efbe8 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EEC2ERNS0_15ClassDescriptorEPKcSD_SD_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)
// type: char **__fastcall(char **, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string),2>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_9efbe8() {
    // IDA 0x9efbe8: `CallbackDescImpl<...>` C2; descriptor state stays engine-side.
}

// 0x9eff64 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEED0Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::~BoundCallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::~BoundCallbackDesc()")]
pub fn stub_9eff64() {
    // IDA 0x9eff64: `BoundCallbackDesc<...>` D0; descriptor state stays engine-side.
}

// 0x9f00c0 — __ZNK3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EE18setGenericCallbackEPNS0_13DescribedBaseENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSD_EEEEEEE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const
// type: void __fastcall(int, int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string),2>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
pub fn stub_9f00c0() {
    // IDA 0x9f00c0: `CallbackDescImpl::setGenericCallback`; the stored callback stays engine-side.
}

// 0x9f0544 — __ZNK3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE13clearCallbackEPNS0_13DescribedBaseE
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::clearCallback(RBX::Reflection::DescribedBase *)const
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
pub fn stub_9f0544() {
    // IDA 0x9f0544: `CallbackDesc::clearCallback`; the stored callback stays engine-side.
}

// 0x9f0638 — __ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESsSD_NS_3argILi1EEENSG_ILi2EEEEENS_3_bi6bind_tIT_PFSL_T0_T1_T2_ENSJ_9list_av_3IT3_T4_T5_E4typeEEESQ_SS_ST_SU_
// demangled: boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list_av_3<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)
// type: void __fastcall(int, int, int *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list_av_3<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_9f0638() {
    // IDA 0x9f0638: `boost::bind` over the FilterResult functor; the bind object stays engine-side.
}

// 0x9f0aa0 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EE11callGenericENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSB_EEEEEEES7_Ss
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string)
// type: pthread_mutex_t *__fastcall(int *, int *, const std::string *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string),2>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
pub fn stub_9f0aa0(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f0aa0: `CallbackDescImpl::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f1414 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
// demangled: boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)
// type: pthread_mutex_t *__fastcall(int *, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::callGeneric<RBX::Network::FilterResult>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f1414(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f1414: `CallbackDesc::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f19f4 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
// demangled: boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)
// type: int __fastcall(int **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::convertResult<RBX::Network::FilterResult>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f19f4(value: bool) -> bool {
    // IDA 0x9f19f4: `CallbackDesc::convertResult` — the result codec stays engine-side.
    value
}

// 0x9f220c — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_SsENS9_5list3INS9_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
// demangled: void boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)
// type: void __fastcall(pthread_mutex_t *, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_9f220c<T>(slot: &mut Option<Box<T>>, functor: T) {
    // IDA 0x9f220c: `function2::assign_to` — installs the bind into the `function` object (void return).
    crate::functor::assign_to(slot, functor);
}

// 0x9f267c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9f267c(op: crate::functor::FunctorOp) -> crate::functor::ManageOutcome {
    // IDA 0x9f267c: same small-object `manage` shape as 0xa368fc for the `FilterResult(*)(Tuple-fn,Instance,std::string)` bind.
    crate::functor::manage_small(op, "bind_t<FilterResult(*)(Tuple-fn,Instance,std::string)>")
}

// 0x9f26a0 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEES7_SJ_SsE6invokeERNS1_15function_bufferESJ_Ss
// demangled: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,std::string)
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
pub fn stub_9f26a0<R, A, B>(slot: &mut dyn FnMut(A, B) -> R, a: A, b: B) -> R {
    // IDA 0x9f26a0: `function_obj_invoker2` — bound filter fn plus `(Instance,std::string)`.
    crate::functor::invoke_ret2(slot, a, b)
}

// 0x9f26bc — __ZNK5boost6detail8function13basic_vtable2IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEESsE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_SsENSB_5list3INSB_5valueISL_EENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int *, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_9f26bc<T>(slot: &mut Option<Box<T>>, functor: T) -> bool {
    // IDA 0x9f26bc: `basic_vtable2::assign_to` — refcount bumps + word copy (engine-side), `return 1`.
    crate::functor::assign_to(slot, functor)
}

// 0x9f295c — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEEEclINS5_7Network12FilterResultEPFSL_SD_NS3_INS5_8InstanceEEESsENS0_5list2IRSN_RSsEEEET_NS0_4typeISU_EERT0_RT1_l
// demangled: RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,std::string &>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string) &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,std::string &> &,long)
// type: struct _Unwind_Exception *__fastcall(int *, int (__fastcall **)(int *, __int32 *, int *), pthread_mutex_t **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,std::string &>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string) &,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,std::string &> &,long)")]
pub fn stub_9f295c<R, A, B>(slot: &mut dyn FnMut(A, B) -> R, a: A, b: B) -> R {
    // IDA 0x9f295c: `list3::operator()` returning `FilterResult` — bound filter fn plus `(Instance,std::string)`.
    crate::functor::invoke_ret2(slot, a, b)
}

// 0x9f2e64 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: void __fastcall(_DWORD *, _WORD *, unsigned int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9f2e64(op: crate::functor::FunctorOp) -> crate::functor::ManageOutcome {
    // IDA 0x9f2e64: small-object `manage_small` for the `FilterResult(*)(Tuple-fn,Instance,std::string)` bind.
    crate::functor::manage_small(op, "bind_t<FilterResult(*)(Tuple-fn,Instance,std::string)>")
}

// 0x9f2f3c — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEED1Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::~CallbackDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::~CallbackDesc()")]
pub fn stub_9f2f3c() {
    // IDA 0x9f2f3c: `CallbackDesc<...>` D1; descriptor state stays engine-side.
}

// 0x9f307c — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEED0Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::~CallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::~CallbackDesc()")]
pub fn stub_9f307c() {
    // IDA 0x9f307c: `CallbackDesc<...>` D0; descriptor state stays engine-side.
}

// 0x9f32b4 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE6SetterINS2_16ServerReplicatorEED1Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::~Setter()
// type: void()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
pub fn stub_9f32b4() {
    // IDA 0x9f32b4: `BoundCallbackDesc::Setter<ServerReplicator>` D1; no crate state.
}

// 0x9f32b8 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE6SetterINS2_16ServerReplicatorEED0Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::~Setter()
// type: void __fastcall(void *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
pub fn stub_9f32b8() {
    // IDA 0x9f32b8: `BoundCallbackDesc::Setter<ServerReplicator>` D0; no crate state.
}

// 0x9f32c4 — __ZNK3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE6SetterINS2_16ServerReplicatorEE11setCallbackEPNS0_13DescribedBaseERKNS4_8functionIS8_EE
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)> const&)const
// type: int __fastcall(_DWORD *, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)> const&)const")]
pub fn stub_9f32c4() {
    // IDA 0x9f32c4: `BoundCallbackDesc::Setter::setCallback`; the stored callback stays engine-side.
}

// 0x9f3300 — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEEaSERKS8_
// demangled: boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)> const&)
// type: int *__fastcall(int *, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::operator=(boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)> const&)")]
pub fn stub_9f3300<T>(slot: &mut Option<Box<T>>, functor: T) {
    // IDA 0x9f3300: `function<FilterResult>::operator=`; stores the functor.
    crate::functor::assign_to(slot, functor);
}

// 0x9f35f8 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EED1Ev
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::~CallbackDescImpl()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string),2>::~CallbackDescImpl()")]
pub fn stub_9f35f8() {
    // IDA 0x9f35f8: `CallbackDescImpl<...>` D1; descriptor state stays engine-side.
}

// 0x9f3738 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EED0Ev
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::~CallbackDescImpl()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string),2>::~CallbackDescImpl()")]
pub fn stub_9f3738() {
    // IDA 0x9f3738: `CallbackDescImpl<...>` D0; descriptor state stays engine-side.
}

// 0x9f3894 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EEC2ERNS0_15ClassDescriptorEPKcSE_SE_SE_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)
// type: char **__fastcall(char **, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_9f3894() {
    // IDA 0x9f3894: `CallbackDescImpl<..., Variant, 3>` C2; descriptor state stays engine-side.
}

// 0x9f3d1c — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEED0Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~BoundCallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~BoundCallbackDesc()")]
pub fn stub_9f3d1c() {
    // IDA 0x9f3d1c: `BoundCallbackDesc<..., Variant>` D0; descriptor state stays engine-side.
}

// 0x9f3e78 — __ZNK3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EE18setGenericCallbackEPNS0_13DescribedBaseENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSE_EEEEEEE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const
// type: void __fastcall(int, int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
pub fn stub_9f3e78() {
    // IDA 0x9f3e78: `CallbackDescImpl::setGenericCallback`; the stored callback stays engine-side.
}

// 0x9f42fc — __ZNK3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE13clearCallbackEPNS0_13DescribedBaseE
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::clearCallback(RBX::Reflection::DescribedBase *)const
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
pub fn stub_9f42fc() {
    // IDA 0x9f42fc: `CallbackDesc::clearCallback`; the stored callback stays engine-side.
}

// 0x9f43f0 — __ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESsNS6_7VariantESD_NS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEENS_3_bi6bind_tIT_PFSN_T0_T1_T2_T3_ENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEST_SV_SW_SX_SY_
// demangled: boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list_av_4<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
// type: void __fastcall(int, int, int *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list_av_4<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<RBX::Network::FilterResult,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_9f43f0() {
    // IDA 0x9f43f0: `boost::bind` over the 3-arg FilterResult functor; the bind object stays engine-side.
}

// 0x9f4858 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EE11callGenericENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSC_EEEEEEES7_SsS8_
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)
// type: pthread_mutex_t *__fastcall(int *, pthread_mutex_t *, const std::string *, unsigned int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)")]
pub fn stub_9f4858(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f4858: `CallbackDescImpl::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f5220 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESE_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSJ_EEEEEEESK_
// demangled: boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)
// type: pthread_mutex_t *__fastcall(int *, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::callGeneric<RBX::Network::FilterResult>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f5220(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f5220: `CallbackDesc::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f5800 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESH_E4typeENS5_ISE_EE
// demangled: boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)
// type: int __fastcall(int **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::convertResult<RBX::Network::FilterResult>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f5800(value: bool) -> bool {
    // IDA 0x9f5800: `CallbackDesc::convertResult` — the result codec stays engine-side.
    value
}

// 0x9f6200 — __ZN5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS7_5TupleEEENS4_IKSE_EEEEEEES6_SsS8_ENSB_5list4INSB_5valueISK_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_
// demangled: void boost::function3<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)
// type: void __fastcall(pthread_mutex_t *, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_9f6200<T>(slot: &mut Option<Box<T>>, functor: T) {
    // IDA 0x9f6200: `function3::assign_to` — installs the bind into the `function` object (void return).
    crate::functor::assign_to(slot, functor);
}

// 0x9f6670 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsNSA_7VariantEENS3_5list4INS3_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9f6670(op: crate::functor::FunctorOp) -> crate::functor::ManageOutcome {
    // IDA 0x9f6670: same small-object `manage` shape as 0xa368fc for the `FilterResult(*)(Tuple-fn,Instance,std::string,Variant)` bind.
    crate::functor::manage_small(op, "bind_t<FilterResult(*)(Tuple-fn,Instance,std::string,Variant)>")
}

// 0x9f6694 — __ZN5boost6detail8function21function_obj_invoker3INS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsNSA_7VariantEENS3_5list4INS3_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEES7_SJ_SsSK_E6invokeERNS1_15function_bufferESJ_SsSK_
// demangled: boost::detail::function::function_obj_invoker3<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)
// type: int __fastcall(int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::function_obj_invoker3<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)")]
pub fn stub_9f6694<R, A, B, C>(slot: &mut dyn FnMut(A, B, C) -> R, a: A, b: B, c: C) -> R {
    // IDA 0x9f6694: `function_obj_invoker3` — bound filter fn plus `(Instance,std::string,Variant)`.
    crate::functor::invoke_ret3(slot, a, b, c)
}

// 0x9f66b0 — __ZNK5boost6detail8function13basic_vtable3IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEESsNS3_10Reflection7VariantEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS9_5TupleEEENS6_IKSG_EEEEEEES8_SsSA_ENSD_5list4INSD_5valueISM_EENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable3<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int *, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool boost::detail::function::basic_vtable3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_9f66b0<T>(slot: &mut Option<Box<T>>, functor: T) -> bool {
    // IDA 0x9f66b0: `basic_vtable3::assign_to` — installs the bind, `return 1`.
    crate::functor::assign_to(slot, functor)
}

// 0x9f6950 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEENSF_ILi3EEEEclINS5_7Network12FilterResultEPFSM_SD_NS3_INS5_8InstanceEEESsNS6_7VariantEENS0_5list3IRSO_RSsRSP_EEEET_NS0_4typeISX_EERT0_RT1_l
// demangled: RBX::Network::FilterResult boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant) &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&> &,long)
// type: pthread_mutex_t *__fastcall(int *, pthread_mutex_t **, pthread_mutex_t **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::FilterResult boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant) &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&> &,long)")]
pub fn stub_9f6950<R, A, B, C>(slot: &mut dyn FnMut(A, B, C) -> R, a: A, b: B, c: C) -> R {
    // IDA 0x9f6950: `list4::operator()` returning `FilterResult` — bound filter fn plus `(Instance,std::string,Variant)`.
    crate::functor::invoke_ret3(slot, a, b, c)
}

// 0x9f6ed0 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsNSA_7VariantEENS3_5list4INS3_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE12manage_smallERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: void __fastcall(_DWORD *, _WORD *, unsigned int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9f6ed0(op: crate::functor::FunctorOp) -> crate::functor::ManageOutcome {
    // IDA 0x9f6ed0: small-object `manage_small` for the 3-arg `FilterResult(*)(Tuple-fn,Instance,std::string,Variant)` bind.
    crate::functor::manage_small(op, "bind_t<FilterResult(*)(Tuple-fn,Instance,std::string,Variant)>")
}

// 0x9f6fa8 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEED1Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~CallbackDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~CallbackDesc()")]
pub fn stub_9f6fa8() {
    // IDA 0x9f6fa8: `CallbackDesc<..., Variant>` D1; descriptor state stays engine-side.
}

// 0x9f70e8 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEED0Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~CallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~CallbackDesc()")]
pub fn stub_9f70e8() {
    // IDA 0x9f70e8: `CallbackDesc<..., Variant>` D0; descriptor state stays engine-side.
}

// 0x9f7244 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE6SetterINS2_16ServerReplicatorEED1Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::~Setter()
// type: void()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
pub fn stub_9f7244() {
    // IDA 0x9f7244: `BoundCallbackDesc::Setter<ServerReplicator>` D1; no crate state.
}

// 0x9f7248 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE6SetterINS2_16ServerReplicatorEED0Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::~Setter()
// type: void __fastcall(void *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::~Setter()")]
pub fn stub_9f7248() {
    // IDA 0x9f7248: `BoundCallbackDesc::Setter<ServerReplicator>` D0; no crate state.
}

// 0x9f7254 — __ZNK3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE6SetterINS2_16ServerReplicatorEE11setCallbackEPNS0_13DescribedBaseERKNS4_8functionIS9_EE
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)> const&)const
// type: int __fastcall(_DWORD *, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::Setter<RBX::Network::ServerReplicator>::setCallback(RBX::Reflection::DescribedBase *,boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)> const&)const")]
pub fn stub_9f7254() {
    // IDA 0x9f7254: `BoundCallbackDesc::Setter::setCallback`; the stored callback stays engine-side.
}

// 0x9f7290 — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEEaSERKSA_
// demangled: boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)> const&)
// type: int *__fastcall(int *, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::operator=(boost::function<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)> const&)")]
pub fn stub_9f7290<T>(slot: &mut Option<Box<T>>, functor: T) {
    // IDA 0x9f7290: `function<FilterResult>::operator=`; stores the functor.
    crate::functor::assign_to(slot, functor);
}

// 0x9f7588 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EED1Ev
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::~CallbackDescImpl()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::~CallbackDescImpl()")]
pub fn stub_9f7588() {
    // IDA 0x9f7588: `CallbackDescImpl<...>` D1; descriptor state stays engine-side.
}

// 0x9f76c8 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EED0Ev
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::~CallbackDescImpl()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::~CallbackDescImpl()")]
pub fn stub_9f76c8() {
    // IDA 0x9f76c8: `CallbackDescImpl<...>` D0; descriptor state stays engine-side.
}

// 0x9f7824 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2ERNS0_15ClassDescriptorEPKcSD_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>),1>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)
// type: char **__fastcall(char **, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>),1>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_9f7824() {
    // IDA 0x9f7824: `CallbackDescImpl<Instance>` C2; descriptor state stays engine-side.
}

// 0x9f7ab4 — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEED0Ev
// demangled: RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::~BoundCallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::~BoundCallbackDesc()")]
pub fn stub_9f7ab4() {
    // IDA 0x9f7ab4: `BoundCallbackDesc<Instance>` D0; descriptor state stays engine-side.
}

// 0x9f7c10 — __ZNK3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEELi1EE18setGenericCallbackEPNS0_13DescribedBaseENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSD_EEEEEEE
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>),1>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const
// type: void __fastcall(int, int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>),1>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
pub fn stub_9f7c10() {
    // IDA 0x9f7c10: `CallbackDescImpl::setGenericCallback`; the stored callback stays engine-side.
}

// 0x9f8094 — __ZNK3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEE13clearCallbackEPNS0_13DescribedBaseE
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::clearCallback(RBX::Reflection::DescribedBase *)const
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
pub fn stub_9f8094() {
    // IDA 0x9f8094: `CallbackDesc::clearCallback`; the stored callback stays engine-side.
}

// 0x9f8188 — __ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESD_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSK_T0_T1_ENSI_9list_av_2IT2_T3_E4typeEEESO_SQ_SR_
// demangled: boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_2<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>)
// type: void __fastcall(_DWORD *, int, pthread_mutex_t **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_2<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>::type> boost::bind<RBX::Network::FilterResult,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>(RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::arg<1>)")]
pub fn stub_9f8188() {
    // IDA 0x9f8188: `boost::bind` over the 1-arg FilterResult functor; the bind object stays engine-side.
}

// 0x9f87e8 — __ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEELi1EE11callGenericENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSB_EEEEEEES7_
// demangled: RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>),1>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>)
// type: pthread_mutex_t *__fastcall(int *, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>),1>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9f87e8(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f87e8: `CallbackDescImpl::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f905c — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
// demangled: boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)
// type: pthread_mutex_t *__fastcall(int *, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f905c(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9f905c: `CallbackDesc::callGeneric` — invoke the stored callback.
    call()
}

// 0x9f963c — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
// demangled: boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)
// type: int __fastcall(int **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
pub fn stub_9f963c(value: bool) -> bool {
    // IDA 0x9f963c: `CallbackDesc::convertResult` — the result codec stays engine-side.
    value
}

// 0x9f9bec — __ZN5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_ENS9_5list2INS9_5valueISJ_EENS_3argILi1EEEEEEEEEvT_
// demangled: void boost::function1<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>)
// type: void __fastcall(pthread_mutex_t *, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>)")]
pub fn stub_9f9bec<T>(slot: &mut Option<Box<T>>, functor: T) {
    // IDA 0x9f9bec: `function1::assign_to` — installs the bind into the `function` object (void return).
    crate::functor::assign_to(slot, functor);
}

// 0x9fa05c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEEENS3_5list2INS3_5valueISH_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9fa05c(op: crate::functor::FunctorOp) -> crate::functor::ManageOutcome {
    // IDA 0x9fa05c: same small-object `manage` shape as 0xa368fc for the `FilterResult(*)(Tuple-fn,Instance)` bind.
    crate::functor::manage_small(op, "bind_t<FilterResult(*)(Tuple-fn,Instance)>")
}

// 0x9fa080 — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEEENS3_5list2INS3_5valueISH_EENS_3argILi1EEEEEEES7_SJ_E6invokeERNS1_15function_bufferESJ_
// demangled: boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9fa080<R, A>(slot: &mut dyn FnMut(A) -> R, a: A) -> R {
    // IDA 0x9fa080: `function_obj_invoker1` — bound filter fn plus `Instance`.
    crate::functor::invoke_ret1(slot, a)
}

// 0x9fa098 — __ZNK5boost6detail8function13basic_vtable1IN3RBX7Network12FilterResultENS_10shared_ptrINS3_8InstanceEEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS6_INS_8functionIFNS6_INS3_10Reflection5TupleEEENS6_IKSF_EEEEEEES8_ENSB_5list2INSB_5valueISL_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int *, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool boost::detail::function::basic_vtable1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_9fa098<T>(slot: &mut Option<Box<T>>, functor: T) -> bool {
    // IDA 0x9fa098: `basic_vtable1::assign_to` — installs the bind, `return 1`.
    crate::functor::assign_to(slot, functor)
}

// 0x9fa338 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEEEclINS5_7Network12FilterResultEPFSK_SD_NS3_INS5_8InstanceEEEENS0_5list1IRSM_EEEET_NS0_4typeISS_EERT0_RT1_l
// demangled: RBX::Network::FilterResult boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,long)
// type: int __fastcall(int *, int (__fastcall **)(int *, int *), int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::FilterResult boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,long)")]
pub fn stub_9fa338() -> ! {
    todo!("0x9fa338 RBX::Network::FilterResult boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,long)")
}

// 0x9fa78c — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEEENS3_5list2INS3_5valueISH_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: void __fastcall(_DWORD *, _WORD *, unsigned int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_9fa78c() -> ! {
    todo!("0x9fa78c boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x9fa864 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEED1Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")]
pub fn stub_9fa864() -> ! {
    todo!("0x9fa864 RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()")
}

// 0x9fa9a4 — __ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEED0Ev
// demangled: RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::~CallbackDesc()")]
pub fn stub_9fa9a4() -> ! {
    todo!("0x9fa9a4 RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::~CallbackDesc()")
}
