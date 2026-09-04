// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xaa1e2c..0xad6038 | total filtered 10215, remaining 3308 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::{SharedPtr, WeakPtr};
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};
use parking_lot::Mutex;
use rbx_core::signal::Signal;
use crate::data_model::DataModel;
use crate::generated_05::{EventDescPayload, FunctorOp, GenericSlotWrapper, Instance, SignatureItem, Variant, instance_is_a};
use crate::generated_13::{FriendStatus, HttpRequestResult, ModelLoadBind, Player, PlayerAppearanceBind, PlayerDataModelBind, PlayerInstMethod, PlayerMouse, stub_a8d6b4, stub_a90080, stub_a91498};
use crate::instance::{Backpack, ModelInstance};

/// Rust model of an `rbx::signals::signal<void ()(SharedPtr<Instance>,
/// FriendStatus)>::slot` link walked by `next` (IDA `0xaa1e70`): the
/// intrusive successor becomes `next`; retain/release become `clone`/`drop`.
/// Twin of `Chat4SlotNode` (IDA `0xa4c674`) with a 2-arg callback.
pub struct FriendSlotNode {
    pub next: Option<SharedPtr<FriendSlotNode>>,
}
/// Process-wide mutex behind the friend-signal `mutex`/`safe_static_init`
/// pair (IDA `0xaa2074`, `0xaa223c`); per-instantiation twin of
/// `CHAT4_SLOT_STATIC_MUTEX`.
static FRIEND_SLOT_STATIC_MUTEX: Mutex<()> = Mutex::new(());
/// Rust model of `boost::_bi::bind_t<void, mf1<void, Player,
/// SharedPtr<Instance>>, list2<value<Player*>, arg<1>>>` (IDA `0xaa2d08`):
/// the unretained player word plus the member pointer; the instance arg
/// stays late-bound.
#[derive(Clone, Copy)]
pub struct PlayerInstBind {
    pub player: *const Player,
    pub method: PlayerInstMethod,
}
/// Rust model of the `callable_slot` node holding that bind (IDA `0xaa2d08`
/// D1, `0xaa2d64` D0, `0xaa2e6c` call): the intrusive `+8` successor becomes
/// `next`; the link release clears it.
pub struct PlayerInstSlotNode {
    pub next: Option<SharedPtr<PlayerInstSlotNode>>,
    pub bind: PlayerInstBind,
}
/// Rust model of `boost::function2<void, RequestResult,
/// SharedPtr<vector<SharedPtr<Instance>>>>` holding the `CharacterLoadHelper`
/// bind (IDA `0xaa38cc`): nullability of the retained bind is the vtable
/// word. Twin of `Chat4WrapperFunction`.
#[derive(Clone, Default)]
pub struct ModelLoadFunction {
    pub target: Option<ModelLoadBind>,
}

/// Rust model of `boost::function2<void, string*, exception*>` holding the
/// `makeAccoutrementRequests` bind (IDA `0xaa6274`): nullability of the
/// retained bind is the vtable word. Twin of `ModelLoadFunction`.
#[derive(Clone, Default)]
pub struct PlayerDataFunction {
    pub target: Option<PlayerDataModelBind>,
}
/// Mangled type name `strcmp`ed by the `functor_manager::manager`
/// check-type path (disasm `0xaa6a42`-`0xaa6a4e`, `__ZTS` symbol).
pub const PLAYER_DATA_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS0_5list4INS_3argILi1EEENSF_ILi2EEENS0_5valueIS9_EENSI_ISB_EEEEEE";
/// Rust model of `boost::_bi::bind_t<void, mf2 execute2 on
/// GenericSlotWrapper>` (IDA `0xab62b8`): retained wrapper (the
/// `shared_count` copy at bind time) plus late-bound instance/status args.
#[derive(Clone)]
pub struct FriendWrapperBind {
    pub target: SharedPtr<GenericSlotWrapper>,
}
/// Rust model of `boost::function2<void, SharedPtr<Instance>, FriendStatus>`
/// holding the `execute2` bind (IDA `0xab6e04`): nullability of the retained
/// wrapper is the vtable word. Twin of `Chat4WrapperFunction`.
#[derive(Clone, Default)]
pub struct FriendWrapperFunction {
    pub target: Option<SharedPtr<GenericSlotWrapper>>,
}
/// Mangled type name `strcmp`ed by the `functor_manager::manager`
/// check-type path (disasm `0xab76d2`-`0xab76de`, `__ZTS` symbol).
pub const FRIEND_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKNS4_13FriendService12FriendStatusEEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSL_ILi2EEEEEEE";

// 0xaa1e2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PlayerMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_aa1e2c(block: *mut ControlBlockPd<PlayerMouse, CreatableInstanceDeleter>) {
    // IDA 0xaa1e2c: D0 — `operator delete(a1)` (decompile 0xaa1e30); the box
    // reclaim runs the field drops and frees together. Twin of 0xaa1e28 (D1).
    // SAFETY: `block` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(block));
    }
}

// 0xaa1e38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PlayerMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_aa1e38(block: *mut ControlBlockPd<PlayerMouse, CreatableInstanceDeleter>) {
    // IDA 0xaa1e38: `Instance::predelete(px)` (decompile 0xaa1e40), null-px
    // early-out (decompile 0xaa1e46), then the virtual delete through `*px +
    // 8` (decompile 0xaa1e50). `dispose_with` with the no-op predelete takes
    // the payload — the delete. Same shape as 0xf19c.
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0xaa1e54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PlayerMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_aa1e54(block: *const ControlBlockPd<PlayerMouse, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xaa1e54: `strcmp` against
    // `"N3RBX9CreatableINS_8InstanceEE7DeleterE"` (decompile 0xaa1e66),
    // mismatch returns 0; a hit returns `this + 16`. Same shape as 0xf1bc.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_deleter(type_name) }
}

// 0xaa1e6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PlayerMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_aa1e6c(block: *const ControlBlockPd<PlayerMouse, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xaa1e6c: unconditional `this + 16` (decompile 0xaa1e6e). Same
    // shape as 0xf1d4.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_untyped_deleter() }
}

// 0xaa1e70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot> &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot> &)
pub fn stub_aa1e70(slot: &SharedPtr<FriendSlotNode>) -> Option<SharedPtr<FriendSlotNode>> {
    // IDA 0xaa1e70: intrusive retain of the incoming slot (`OSAtomicAdd32(1)`
    // + max-count assert, decompile 0xaa1ed4-0xaa1f38), `signal::mutex()` lock
    // (decompile 0xaa1f3c-0xaa1f48), then the successor walk. The
    // retain/release ride on the clones; the walk is the `next` clone under
    // the static guard.
    let _guard = FRIEND_SLOT_STATIC_MUTEX.lock();
    slot.next.clone()
}

// 0xaa2074 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::mutex(void)
pub fn stub_aa2074() -> &'static Mutex<()> {
    // IDA 0xaa2074: `call_once(once_init_mutex, safe_static_init_mutex)`
    // (decompile 0xaa20a8) then the guard-checked
    // `safe_static_do_get_mutex::value` init (decompile 0xaa20ec-0xaa211c),
    // returning the value (decompile 0xaa214a). A `static` with `const` init
    // is the same once-init; the pthread object lives inside `Mutex`. Twin
    // of 0xa4d820.
    &FRIEND_SLOT_STATIC_MUTEX
}

// 0xaa2188 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotEEaSERKSD_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot> const&)
pub fn stub_aa2188(dst: &mut Option<SharedPtr<FriendSlotNode>>, src: &Option<SharedPtr<FriendSlotNode>>) {
    // IDA 0xaa2188: retain-new (`OSAtomicAdd32(1)` + max-count assert,
    // decompile 0xaa219e-0xaa21f6), store (decompile 0xaa21fe), release-old
    // with virtual delete + free at zero (decompile 0xaa220e-0xaa222e).
    // Clone-assign plus `Drop` is the same sequence.
    *dst = src.clone();
}

// 0xaa223c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::safe_static_init_mutex(void)
pub fn stub_aa223c() -> &'static Mutex<()> {
    // IDA 0xaa223c: guard-checked once-init of
    // `safe_static_do_get_mutex::value` (decompile 0xaa2294-0xaa2298),
    // `operator new(0x2c)` + `mutex::mutex` (decompile 0xaa22ac-0xaa22b2). A
    // `static` with `const` init is the same once-init. Twin of 0xa4d820.
    &FRIEND_SLOT_STATIC_MUTEX
}

// 0xaa2cc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_aa2cc0(_block: *mut ControlBlockPd<Backpack, CreatableInstanceDeleter>) {
    // IDA 0xaa2cc0: D1 — empty; the vtable reset is compiler-managed and
    // storage is released by the D0/owner path. Same shape as 0xf198.
}

// 0xaa2cc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_aa2cc4(block: *mut ControlBlockPd<Backpack, CreatableInstanceDeleter>) {
    // IDA 0xaa2cc4: D0 — the D1 body plus `operator delete`; the box reclaim
    // runs the field drops and frees together. Twin of 0xaa1e2c.
    // SAFETY: `block` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(block));
    }
}

// 0xaa2cd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_aa2cd0(block: *mut ControlBlockPd<Backpack, CreatableInstanceDeleter>) {
    // IDA 0xaa2cd0: `predelete` + null-px early-out + deleter delete — same
    // shape as 0xaa1e38. Twin of 0xf19c.
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0xaa2cec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_aa2cec(block: *const ControlBlockPd<Backpack, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xaa2cec: deleter-name `strcmp`, `this + 0x10` on hit — same shape
    // as 0xaa1e54. Twin of 0xf1bc.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_deleter(type_name) }
}

// 0xaa2d04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_aa2d04(block: *const ControlBlockPd<Backpack, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xaa2d04: unconditional `this + 0x10` — same shape as 0xaa1e6c.
    // Twin of 0xf1d4.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_untyped_deleter() }
}

// 0xaa2d08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_aa2d08(slot: *mut PlayerInstSlotNode) {
    // IDA 0xaa2d08: `callable_slot` D1 — vtable resets (compiler-managed;
    // decompile 0xaa2d1e-0xaa2d26) plus the intrusive link release at `+8`
    // (decompile 0xaa2d2a-0xaa2d58); the bind word is untouched and storage
    // is kept.
    // SAFETY: `slot` must point to a valid `PlayerInstSlotNode`.
    unsafe {
        (*slot).next = None;
    }
}

// 0xaa2d64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_aa2d64(slot: *mut PlayerInstSlotNode) {
    // IDA 0xaa2d64: `callable_slot` D0 — vtable resets plus the link release
    // (decompile 0xaa2d94-0xaa2e04) plus `intrusive_ptr_target::operator
    // delete` (decompile 0xaa2e10); the box reclaim runs the field drops and
    // frees together.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xaa2e6c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_aa2e6c(slot: &PlayerInstSlotNode, inst: &SharedPtr<Instance>) {
    // IDA 0xaa2e6c: retained `shared_ptr` copy of the arg (spinlock-guarded
    // `shared_count` bump, decompile 0xaa2ea0-0xaa2f10) then
    // `mf1::operator()` on the bind words (decompile 0xaa2f20). Clone plus
    // dispatch plus `Drop` is the same sequence.
    let inst = inst.clone();
    stub_aa31f4(&slot.bind, &inst);
}

// 0xaa2f88 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_aa2f88(slot: &PlayerInstSlotNode, inst: &SharedPtr<Instance>) {
    // IDA 0xaa2f88: non-virtual thunk — adjusts the `callable` subobject back
    // to the slot base, then tail-calls `callable::call`. The adjustment is a
    // vtable-layout detail that collapses away here. Twin of 0xa4d130.
    stub_aa2e6c(slot, inst);
}

// 0xaa31f4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network6PlayerENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Player*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_aa31f4(bind: &PlayerInstBind, inst: &SharedPtr<Instance>) {
    // IDA 0xaa31f4: member-pointer adjust (`a2 + (tag >> 1)`, decompile
    // 0xaa3242-0xaa3252), retained `shared_ptr` copy of the arg (decompile
    // 0xaa3254-0xaa32a8), the member call (decompile 0xaa32b2), then the
    // mirrored release. Clone plus the member call plus `Drop` is the same
    // sequence; the member identity rides on the bind.
    // SAFETY: `bind.player` must point to a valid `Player`.
    let inst = inst.clone();
    let player = unsafe { &*bind.player };
    (bind.method)(player, &inst);
}

// 0xaa38cc — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS4_INS1_13ModelInstanceEEES3_SA_ENSD_5list3INSD_5valueISG_EENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_aa38cc(dst: &mut ModelLoadFunction, src: &ModelLoadBind) {
    // IDA 0xaa38cc: `function2::assign_to<bind_t>` spills the bind functor
    // and heap-installs it through `basic_vtable2::assign_to` (IDA 0xaa3d80);
    // the retained model clone is that same copy. Twin of 0xa4bd28.
    dst.target = Some(src.clone());
}

// 0xaa3d3c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13ModelInstanceEEENS6_14AsyncHttpQueue13RequestResultENS5_ISt6vectorINS5_INS6_8InstanceEEESaISD_EEEEENS3_5list3INS3_5valueIS8_EENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_aa3d3c(src: &ModelLoadBind, dst: &mut ModelLoadBind, op: FunctorOp) -> bool {
    // IDA 0xaa3d3c: `functor_manager::manage` dispatches on `op`;
    // discriminants mirror the `0x705780` family (0 clone, 1 move, 2
    // destroy, 3 check-type, 4 get-type). Twin of 0xa4c1a0.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = src.clone();
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = src.clone();
            true
        }
        FunctorOp::GetType => true,
    }
}

// 0xaa3d60 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13ModelInstanceEEENS6_14AsyncHttpQueue13RequestResultENS5_ISt6vectorINS5_INS6_8InstanceEEESaISD_EEEEENS3_5list3INS3_5valueIS8_EENS_3argILi1EEENSM_ILi2EEEEEEEvSA_SG_E6invokeERNS1_15function_bufferESA_SG_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>,void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_aa3d60(bind: &ModelLoadBind, result: HttpRequestResult, instances: &SharedPtr<Vec<SharedPtr<Instance>>>) {
    // IDA 0xaa3d60: unwrap the buffer to the `bind_t` and tail-call the
    // `list3::operator()` (IDA 0xaa4020) with the late-bound result and
    // vector. The clones plus the `CharacterLoadHelper` dispatch (IDA
    // 0xa91498) are the same sequence.
    let instances = instances.clone();
    stub_aa4020(bind, result, &instances);
}

// 0xaa3d80 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS6_INS3_13ModelInstanceEEES5_SC_ENSF_5list3INSF_5valueISI_EENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_aa3d80(dst: &mut ModelLoadFunction, src: &ModelLoadBind) -> bool {
    // IDA 0xaa3d80: `basic_vtable2::assign_to<bind_t>` — heap-clone the bind
    // words plus a `shared_count` retain; always succeeds for the small
    // functor. The install is `function2::assign_to` (IDA 0xaa38cc).
    stub_aa38cc(dst, src);
    true
}

// 0xaa4020 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX13ModelInstanceEEEEENS_3argILi1EEENS8_ILi2EEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENS3_ISt6vectorINS3_INS4_8InstanceEEESaISH_EEEEENS0_5list2IRSE_RSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&> &,int)
pub fn stub_aa4020(bind: &ModelLoadBind, result: HttpRequestResult, instances: &SharedPtr<Vec<SharedPtr<Instance>>>) {
    // IDA 0xaa4020: `list3::operator()` — the retained model (the `value`
    // word) plus the late-bound result/vector forwarded to
    // `CharacterLoadHelper` (IDA 0xa91498). Clones plus the call plus `Drop`
    // mirror the arg-forwarding releases.
    let model = bind.model.clone();
    let instances = instances.clone();
    stub_a91498(&model, result, &instances);
}

// 0xaa4478 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13ModelInstanceEEENS6_14AsyncHttpQueue13RequestResultENS5_ISt6vectorINS5_INS6_8InstanceEEESaISD_EEEEENS3_5list3INS3_5valueIS8_EENS_3argILi1EEENSM_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::ModelInstance>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_aa4478(src: &ModelLoadBind, dst: &mut ModelLoadBind, op: FunctorOp) -> bool {
    // IDA 0xaa4478: `functor_manager_common::manage_small` — same op dispatch
    // as `manage` (IDA 0xaa3d3c) for the small (inline) functor;
    // discriminants mirror the `0x705780` family.
    stub_aa3d3c(src, dst, op)
}

// 0xaa60bc — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ERKSF_
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage4(boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>> const&)")]
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage4(boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>> const&)
pub fn stub_aa60bc<'a>(out: &'a mut PlayerDataModelBind, src: &PlayerDataModelBind) -> &'a mut PlayerDataModelBind {
    // IDA 0xaa60bc: `storage4` copy-ctor — word copies plus the two
    // spinlock-guarded `weak_count` bumps for the player and data-model
    // weaks (decompile 0xaa6112-0xaa61b8). The retained clones are those
    // same copies.
    *out = src.clone();
    out
}

// 0xaa6274 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS6_5list4INS_3argILi1EEENSI_ILi2EEENS6_5valueISC_EENSL_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>)")]
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)
pub fn stub_aa6274(dst: &mut PlayerDataFunction, src: &PlayerDataModelBind) {
    // IDA 0xaa6274: `function2::assign_to<bind_t>` spills the bind functor
    // and heap-installs it through the basic vtable; the retained weak-pair
    // clone is that same copy. Twin of 0xaa38cc.
    dst.target = Some(src.clone());
}

// 0xaa662c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_aa662c(src: &PlayerDataModelBind, dst: &mut PlayerDataModelBind, op: FunctorOp) -> bool {
    // IDA 0xaa662c: `functor_manager::manage` for the player/data-model
    // bind — same op dispatch as 0xaa3d3c; discriminants mirror the
    // `0x705780` family. (Decompile yields no pseudo for this ea; the
    // template is identical to the 0xaa6970 `manager` below, whose disasm
    // 0xaa69d4-0xaa6a62 shows the same clone/move/destroy/check-type
    // shape as 0xa4c4e0.)
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = src.clone();
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = src.clone();
            true
        }
        FunctorOp::GetType => true,
    }
}

// 0xaa6650 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)
pub fn stub_aa6650(bind: &PlayerDataModelBind, url: &str) {
    // IDA 0xaa6650: unwrap the buffer to the `bind_t` and tail-call the
    // `list4::operator()` (IDA 0xaa6670) with the late-bound string and
    // exception args. (Decompile yields no pseudo for this ea; same shape
    // as 0xaa3d60.) The exception arg collapses — its text only surfaces
    // in the unmodeled catch-`printf` path of 0xa90080.
    stub_aa6670(bind, url);
}

// 0xaa6670 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEclIPFvPSsPSt9exceptionSA_SD_ENS0_5list2IRSH_RSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::operator()<void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// was: void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_aa6670(bind: &PlayerDataModelBind, url: &str) {
    // IDA 0xaa6670: `list4::operator()` — the two retained weaks (the
    // `value` words, same spinlock-guarded retains as decompile
    // 0xaa6112-0xaa61b8) plus the late-bound string/exception forwarded to
    // `makeAccoutrementRequests` (IDA 0xa90080). Clones plus the call plus
    // arg-forwarding releases.
    stub_a90080(url, &bind.player, &bind.data_model);
}

// 0xaa6970 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_aa6970(src: &PlayerDataModelBind, dst: &mut PlayerDataFunction, op: FunctorOp, type_name: &str) -> bool {
    // IDA 0xaa6970: `functor_manager::manager` switch on `op` (disasm
    // 0xaa69d4): 0 heap-clone the bind (`operator new(0x14)` plus the
    // `storage4` copy-ctor, disasm 0xaa69ee-0xaa6a12), 1 move the words and
    // clear the source, 2 run the `storage4` D2 plus `operator delete`,
    // 3 conditional copy on `strcmp` against the bind `__ZTS` name (disasm
    // 0xaa6a42-0xaa6a62). The heap words collapse into the retained clone;
    // `CheckType` reports the match. Twin of 0xa4c4e0.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.target = Some(src.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.target = None;
            false
        }
        FunctorOp::CheckType => type_name == PLAYER_DATA_BIND_TYPE_NAME,
        FunctorOp::GetType => true,
    }
}

// 0xaa6ab4 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEED2Ev
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::~storage4()")]
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::~storage4()
pub fn stub_aa6ab4(bind: *mut PlayerDataModelBind) {
    // IDA 0xaa6ab4: `storage4` D2 — spinlock-guarded `weak_count` release of
    // the data-model weak (decompile 0xaa6ae6-0xaa6b64) then the player weak
    // (decompile 0xaa6b68-0xaa6bc2), each with the zero-count virtual delete;
    // storage is kept (subobject). `drop_in_place` runs the same field
    // drops without freeing.
    // SAFETY: `bind` must point to a valid `PlayerDataModelBind` that is not used again.
    unsafe {
        core::ptr::drop_in_place(bind);
    }
}

// 0xaa6c74 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
// was: boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)
pub fn stub_aa6c74<'a>(out: &'a mut PlayerDataModelBind, player: &WeakPtr<Player>, data_model: &WeakPtr<DataModel>) -> &'a mut PlayerDataModelBind {
    // IDA 0xaa6c74: `list4` C2 — retain the player and data-model weaks
    // (spinlock-guarded `weak_count` bumps, same shape as decompile
    // 0xaa6112-0xaa61b8) and store the `arg<1>/arg<2>` tag words. Collapses
    // to the retained clones.
    // SAFETY: `out` must point to valid uninitialized `PlayerDataModelBind` storage.
    unsafe {
        core::ptr::write(
            out,
            PlayerDataModelBind { player: player.clone(), data_model: data_model.clone() },
        );
    }
    out
}

// 0xaa6f70 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)
pub fn stub_aa6f70<'a>(out: &'a mut PlayerDataModelBind, player: &WeakPtr<Player>, data_model: &WeakPtr<DataModel>) -> &'a mut PlayerDataModelBind {
    // IDA 0xaa6f70: `storage4` 4-arg C2 — same weak retains and word stores
    // as the `list4` C2 (IDA 0xaa6c74), one storage layer down. Same
    // collapse.
    // SAFETY: `out` must point to valid uninitialized `PlayerDataModelBind` storage.
    stub_aa6c74(out, player, data_model)
}

// 0xaa71e4 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEclIPFvS7_NS3_INS4_8InstanceEEEbENS0_5list1IRNS_10shared_ptrISE_EEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::_bi::list1<boost::shared_ptr<RBX::Instance> &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> &> &,int)
pub fn stub_aa71e4(bind: &PlayerAppearanceBind, appearance: &SharedPtr<Instance>) {
    // IDA 0xaa71e4: `list3::operator()` — the retained player weak (the
    // `value` word) and the bool value plus the late-bound appearance
    // `shared_ptr` (converted to `weak` for the call, decompile
    // 0xaa3254-0xaa32a8 retains) forwarded to `setAppearanceParent` (IDA
    // 0xa8d6b4). The downgrade plus the call plus `Drop` mirror the
    // arg-forwarding releases.
    stub_a8d6b4(&bind.player, &SharedPtr::downgrade(appearance), bind.flag);
}

// 0xaa7a00 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_11StarterGearEEEPKT_v
#[doc(alias = "RBX::StarterGear const* RBX::Instance::findConstFirstChildOfType<RBX::StarterGear>(void)const")]
// was: RBX::StarterGear const* RBX::Instance::findConstFirstChildOfType<RBX::StarterGear>(void)const
pub fn stub_aa7a00(parent: *const Instance) -> Option<SharedPtr<Instance>> {
    // IDA 0xaa7a00: null child-list returns null (decompile 0xaa7a4e-0xaa7a5c);
    // otherwise scans the child vector checking `isA StarterGear`
    // (`sStarterGear`, decompile 0xaa7a6c-0xaa7b32) — first hit wins, miss
    // returns null. Same shape as 0x3f1cac.
    // SAFETY: `parent` must be null or point to a valid `Instance`.
    if parent.is_null() {
        return None;
    }
    unsafe {
        let children: &[SharedPtr<Instance>] = &(*parent).children;
        for child in children.iter() {
            if instance_is_a(SharedPtr::as_ptr(child), "StarterGear") {
                return Some(child.clone());
            }
        }
        None
    }
}

// 0xab524c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_ab524c(this: *mut EventDescPayload, name: &str, permissions: u32, attributes: u32) {
    // IDA 0xab524c: `Player::classDescriptor` once-init, base
    // `EventDescriptor` init, then two signature items —
    // `shared_ptr<Instance>` and `FriendStatus` (each a `Type::getSingleton`
    // + `Item::Item` + list `hook`, same shape as decompile 0xab676e-0xab6798
    // in `execute2`). The member signal offset collapses into the
    // payload-side list, as in the `0x707b28` twin.
    // SAFETY: `this` must point to valid uninitialized `EventDescPayload` storage.
    unsafe {
        core::ptr::write(
            this,
            EventDescPayload {
                name: name.to_string(),
                permissions,
                attributes,
                items: vec![
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                    SignatureItem { type_name: "FriendStatus" },
                ],
                connections: Mutex::new(Vec::new()),
                single: Signal::new(),
                triple: Signal::new(),
                triple_isi: Signal::new(),
                pair_if: Signal::new(),
            },
        );
    }
}

// 0xab55e8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_ab55e8(this: *mut EventDescPayload) {
    // IDA 0xab55e8: D0 — vtable reset plus signature-list `_M_clear` plus
    // `operator delete`; the box reclaim runs the field drops and frees
    // together. Same shape as 0xa4a0a0.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xab56c4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_ab56c4(desc: *const EventDescPayload, slot: &SharedPtr<GenericSlotWrapper>) {
    // IDA 0xab56c4: retain the wrapper, `bind(execute2-mf2, wrapper, _1,
    // _2)`, wrap in `function2`, `EventDescBase::connect`, then the mirrored
    // releases. Collapses to a retained clone + push onto the payload-side
    // list. Twin of 0xa4a17c with (Instance, FriendStatus) args.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().push(slot.clone());
    }
}

// 0xab5b48 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_ab5b48(desc: *const EventDescPayload, args: &[Variant]) {
    // IDA 0xab5b48: `ReleaseAssert(args.size() == 2)`, two `any_cast`s —
    // `shared_ptr<Instance>` and `FriendStatus` (`bad_placement_any_cast`
    // on mismatch) — then `signal_with_args<2>::operator()` fans out to
    // each connected slot's `callable::call`. Twin of 0xa4a600 with 2 args.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    assert!(args.len() == 2, "0xab5b48: args.size() == 2");
    let (inst, status) = match (&args[0], &args[1]) {
        (Variant::Instance(a), Variant::Friend(b)) => (a, FriendStatus(*b)),
        _ => panic!("0xab5b48: any_cast<(Instance, FriendStatus)> failed"),
    };
    unsafe {
        let slots = (*desc).connections.lock().clone();
        for slot in slots.iter() {
            if let Some(cb) = slot.on_friend {
                cb(inst, status.0);
            }
        }
    }
}

// 0xab5f20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_ab5f20(desc: *const EventDescPayload) {
    // IDA 0xab5f20: `source ? source - 36 : 0` selects the member signal,
    // then `signal::disconnectAll`; the addressing collapses into the
    // payload-side list, so this clears the connections. Twin of 0xa4ad50.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().clear();
    }
}

// 0xab5f38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::disconnectAll(void)
pub fn stub_ab5f38(sig: *mut Signal<(SharedPtr<Instance>, FriendStatus)>) {
    // IDA 0xab5f38: loop over the slot list: lock the signal mutex, unlink
    // slots, splice the remainder back, release the unlinked head, repeat
    // until empty. `Signal::disconnect_all` holds the same lock and drops
    // the same slot list. Twin of 0xa4ad68.
    // SAFETY: `sig` must point to a valid `Signal`.
    unsafe {
        (*sig).disconnect_all();
    }
}

// 0xab60e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const
pub fn stub_ab60e4(desc: *const EventDescPayload, func: &FriendWrapperFunction) -> Option<SharedPtr<GenericSlotWrapper>> {
    // IDA 0xab60e4: null function returns a null connection; else `malloc`
    // the `callable` slot, copy the functor into it, `signal::insert` into
    // the member signal, publish the connection. Collapses to retaining the
    // bound wrapper and pushing it onto the payload-side list; the returned
    // clone is the connection keep-alive. Twin of 0xa4af20.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    let target = func.target.clone()?;
    unsafe {
        (*desc).connections.lock().push(target.clone());
    }
    Some(target)
}

// 0xab62b8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKNS1_13FriendService12FriendStatusENS4_IS3_EENS_3argILi1EEENSE_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISJ_T0_T1_T2_EENSH_9list_av_3IT3_T4_T5_E4typeEEEMSM_FSJ_SN_SO_ESR_SS_ST_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_ab62b8<'a>(out: &'a mut FriendWrapperBind, target: &SharedPtr<GenericSlotWrapper>) -> &'a mut FriendWrapperBind {
    // IDA 0xab62b8: retain the wrapper (the `shared_count` copy), build the
    // `list3<value<wrapper>, _1, _2>`, store the `mf2` + list words, then
    // release the temporary. Collapses to the retained clone. Twin of
    // 0xa4b0f4 with (Instance, FriendStatus) args.
    *out = FriendWrapperBind { target: target.clone() };
    out
}

// 0xab6724 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>(rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>(boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&)
pub fn stub_ab6724(wrapper: &SharedPtr<GenericSlotWrapper>, inst: &SharedPtr<Instance>, status: FriendStatus) {
    // IDA 0xab6724: build the 2-`Variant` vector (`Type::getSingleton<void>`
    // items, decompile 0xab676e-0xab6798) — retained instance plus the
    // status tag — call the wrapper's script callback, then run the
    // per-item disposers. The callback is the native `on_friend` stand-in;
    // the vector temps are `Drop`-managed here. Twin of 0xa4b560.
    if let Some(cb) = wrapper.on_friend {
        cb(inst, status.0);
    }
}

// 0xab6e04 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEENS2_13FriendService12FriendStatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_EENS9_5list3INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_ab6e04(dst: &mut FriendWrapperFunction, src: &FriendWrapperBind) {
    // IDA 0xab6e04: `function2::assign_to<bind_t>` spills the bind functor
    // and heap-installs it through `basic_vtable2::assign_to` (IDA 0xab72c8);
    // the retained wrapper clone is that same copy. Twin of 0xa4bd28.
    dst.target = Some(src.target.clone());
}

// 0xab727c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_ab727c(src: &FriendWrapperBind, dst: &mut FriendWrapperBind, op: FunctorOp) -> bool {
    // IDA 0xab727c: `functor_manager::manage` dispatches on `op`;
    // discriminants mirror the `0x705780` family. Twin of 0xaa3d3c.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = src.clone();
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = src.clone();
            true
        }
        FunctorOp::GetType => true,
    }
}

// 0xab72a0 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEvSC_SG_E6invokeERNS1_15function_bufferESC_SG_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
pub fn stub_ab72a0(bind: &FriendWrapperBind, inst: &SharedPtr<Instance>, status: FriendStatus) {
    // IDA 0xab72a0: unwrap the buffer to the `bind_t` and tail-call the
    // `mf2::operator()` path (`execute2`, IDA 0xab6724) with the late-bound
    // instance and status. The clones plus the dispatch are the same
    // sequence. Twin of 0xaa3d60.
    let inst = inst.clone();
    stub_ab6724(&bind.target, &inst, status);
}

// 0xab72c8 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKS8_EENSB_5list3INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_ab72c8(dst: &mut FriendWrapperFunction, src: &FriendWrapperBind) -> bool {
    // IDA 0xab72c8: `basic_vtable2::assign_to<bind_t>` — heap-clone the bind
    // words plus a `shared_count` retain; always succeeds for the small
    // functor. The install is `function2::assign_to` (IDA 0xab6e04). Twin of
    // 0xaa3d80.
    stub_ab6e04(dst, src);
    true
}

// 0xab75b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_ab75b0() -> ! {
    todo!("0xab75b0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xab7744 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE6insertEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)
pub fn stub_ab7744() -> ! {
    todo!("0xab7744 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)")
}

// 0xab79f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotEEaSEPSC_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot*)
pub fn stub_ab79f8() -> ! {
    todo!("0xab79f8 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot*)")
}

// 0xab7aac — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE13callable_slotINS2_8functionIS9_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()
pub fn stub_ab7aac() -> ! {
    todo!("0xab7aac rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()")
}

// 0xab7ab8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE13callable_slotINS2_8functionIS9_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()
pub fn stub_ab7ab8() -> ! {
    todo!("0xab7ab8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>>::~callable_slot()")
}

// 0xab7b6c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::disconnect(void)
pub fn stub_ab7b6c() -> ! {
    todo!("0xab7b6c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::disconnect(void)")
}

// 0xab7ce0 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::connected(void)const
pub fn stub_ab7ce0() -> ! {
    todo!("0xab7ce0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::connected(void)const")
}

// 0xab7cec — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotENS3_8functionISA_EELi2ESA_E4callES7_S9_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
pub fn stub_ab7cec() -> ! {
    todo!("0xab7cec rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")
}

// 0xab7e04 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotENS3_8functionISA_EELi2ESA_E4callES7_S9_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
pub fn stub_ab7e04() -> ! {
    todo!("0xab7e04 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")
}

// 0xab806c — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEENS2_13FriendService12FriendStatusEEclES4_S6_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)const")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::operator()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)const
pub fn stub_ab806c() -> ! {
    todo!("0xab806c boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)const")
}

// 0xab83c4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE6removeEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)
pub fn stub_ab83c4() -> ! {
    todo!("0xab83c4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot *)")
}

// 0xab84b0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::safe_static_init_mutex(void)
pub fn stub_ab84b0() -> ! {
    todo!("0xab84b0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::safe_static_init_mutex(void)")
}

// 0xab8594 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotENS3_8functionISA_EELi2ESA_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()
pub fn stub_ab8594() -> ! {
    todo!("0xab8594 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")
}

// 0xab872c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotENS3_8functionISA_EELi2ESA_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()
pub fn stub_ab872c() -> ! {
    todo!("0xab872c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")
}

// 0xab8738 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_13FriendService12FriendStatusEEE4slotENS3_8functionISA_EELi2ESA_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()
pub fn stub_ab8738() -> ! {
    todo!("0xab8738 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::~callable()")
}

// 0xab87ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()
pub fn stub_ab87ec() -> ! {
    todo!("0xab87ec rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()")
}

// 0xab8848 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()
pub fn stub_ab8848() -> ! {
    todo!("0xab8848 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::slot::~slot()")
}

// 0xab94c0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_ab94c0() -> ! {
    todo!("0xab94c0 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab9770 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_ab9770() -> ! {
    todo!("0xab9770 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab984c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_ab984c() -> ! {
    todo!("0xab984c RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab9cd0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_ab9cd0() -> ! {
    todo!("0xab9cd0 RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaba024 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_aba024() -> ! {
    todo!("0xaba024 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xaba03c — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_aba03c() -> ! {
    todo!("0xaba03c RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")
}

// 0xaba210 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_aba210() -> ! {
    todo!("0xaba210 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xaba5ac — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_aba5ac() -> ! {
    todo!("0xaba5ac RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xaba688 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_aba688() -> ! {
    todo!("0xaba688 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xabab0c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_abab0c() -> ! {
    todo!("0xabab0c RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xabaf7c — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_abaf7c() -> ! {
    todo!("0xabaf7c RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xabaf94 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_abaf94() -> ! {
    todo!("0xabaf94 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")
}

// 0xabe964 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FS5_S9_EPKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(RBX::FriendService::FriendStatus (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_abe964() -> ! {
    todo!("0xabe964 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xabebf4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_abebf4() -> ! {
    todo!("0xabebf4 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xabec94 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_abec94() -> ! {
    todo!("0xabec94 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xabeecc — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEES9_S5_E4callEPS3_SB_RNS0_7VariantERKS9_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::call(RBX::Network::Player*,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Player,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::call(RBX::Network::Player*,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_abeecc() -> ! {
    todo!("0xabeecc RBX::Reflection::Call1Helper<RBX::Network::Player,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::call(RBX::Network::Player*,RBX::FriendService::FriendStatus (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xac0518 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network6PlayerENS_13ModelInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
pub fn stub_ac0518() -> ! {
    todo!("0xac0518 RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xac0790 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_7Network6PlayerENS_13ModelInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
pub fn stub_ac0790() -> ! {
    todo!("0xac0790 non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xac1a6c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS3_FvSsS7_EPKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_ac1a6c() -> ! {
    todo!("0xac1a6c RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xac1dac — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_ac1dac() -> ! {
    todo!("0xac1dac RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0xac1e4c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_ac1e4c() -> ! {
    todo!("0xac1e4c RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xac2134 — __ZN3RBX10Reflection11Call2HelperINS_7Network6PlayerEMS3_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS7_vE4callEPS3_S9_RNS0_7VariantERKSsRKS7_
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),std::string,boost::shared_ptr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_ac2134() -> ! {
    todo!("0xac2134 RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xac245c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS3_FS7_SsEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_ac245c() -> ! {
    todo!("0xac245c RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xac2704 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_ac2704() -> ! {
    todo!("0xac2704 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")
}

// 0xac283c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_ac283c() -> ! {
    todo!("0xac283c RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xac297c — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FN5boost10shared_ptrINS_8InstanceEEESsESsS7_E4callEPS3_S9_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Player,boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Player*,boost::shared_ptr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)
pub fn stub_ac297c() -> ! {
    todo!("0xac297c RBX::Reflection::Call1Helper<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0xac5604 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
pub fn stub_ac5604() -> ! {
    todo!("0xac5604 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")
}

// 0xac56e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_ac56e0() -> ! {
    todo!("0xac56e0 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xac8194 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS3_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_ac8194() -> ! {
    todo!("0xac8194 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xac8424 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_ac8424() -> ! {
    todo!("0xac8424 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xac84c4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_ac84c4() -> ! {
    todo!("0xac84c4 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xac86f8 — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FvN5boost10shared_ptrINS_8InstanceEEEES7_vE4callEPS3_S9_RNS0_7VariantERKS7_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_ac86f8() -> ! {
    todo!("0xac86f8 RBX::Reflection::Call1Helper<RBX::Network::Player,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xaca39c — __ZN5boost4bindIvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS1_INS2_9DataModelEEESsS5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_T2_ENS8_9list_av_3IT3_T4_T5_E4typeEEESF_SH_SI_SJ_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list_av_3<std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>::type> boost::bind<void,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>(void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")]
// was: boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_3<std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>,std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>(void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
pub fn stub_aca39c() -> ! {
    todo!("0xaca39c boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list_av_3<std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>::type> boost::bind<void,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>,std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>(void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")
}

// 0xaca760 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEED1Ev
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::~list3()")]
// was: boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::~list3()
pub fn stub_aca760() -> ! {
    todo!("0xaca760 boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::~list3()")
}

// 0xacaa30 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS8_INS2_8InstanceEEESaISB_EEEESsbdS5_NS_3argILi1EEENSF_ILi2EEESsbdEENS_3_bi6bind_tIT_PFSK_T0_T1_T2_T3_T4_T5_ENSI_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESS_SU_SV_SW_SX_SY_SZ_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)
pub fn stub_acaa30() -> ! {
    todo!("0xacaa30 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double>(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,boost::arg<2>,std::string,bool,double)")
}

// 0xacb914 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS9_INS3_8InstanceEEESaISC_EEEESsbdENS0_5list6INS0_5valueIS6_EENS_3argILi1EEENSL_ILi2EEENSJ_ISsEENSJ_IbEENSJ_IdEEEEEC2ESH_RKSR_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)
pub fn stub_acb914() -> ! {
    todo!("0xacb914 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>> const&)")
}

// 0xacbfd8 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSD_5list6INSD_5valueISI_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)")]
// was: void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)
pub fn stub_acbfd8() -> ! {
    todo!("0xacbfd8 void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>)")
}

// 0xacc888 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_acc888() -> ! {
    todo!("0xacc888 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xacc8ac — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEvSB_SI_E6invokeERNS1_15function_bufferESB_SI_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>,void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_acc8ac() -> ! {
    todo!("0xacc8ac boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")
}

// 0xacc8c8 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEENS2_IdEEEclIPFvS7_NS4_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSJ_INS4_8InstanceEEESaISM_EEEESsbdENS0_5list2IRSI_RSP_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")]
// was: void boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&> &,int)
pub fn stub_acc8c8() -> ! {
    todo!("0xacc8c8 void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")
}

// 0xaccd24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS6_8InstanceEEESaISF_EEEESsbdENS3_5list6INS3_5valueIS9_EENS_3argILi1EEENSO_ILi2EEENSM_ISsEENSM_IbEENSM_IdEEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_accd24() -> ! {
    todo!("0xaccd24 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xaccffc — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ES3_S9_SC_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
// was: boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)
pub fn stub_accffc() -> ! {
    todo!("0xaccffc boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")
}

// 0xacd390 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ES3_S9_SC_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)
pub fn stub_acd390() -> ! {
    todo!("0xacd390 boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")
}

// 0xacd910 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEC2ERKSD_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>> const&)")]
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage3(boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>> const&)
pub fn stub_acd910() -> ! {
    todo!("0xacd910 boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::storage3(boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>> const&)")
}

// 0xacdbac — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvSsNS_8weak_ptrINS1_7Network6PlayerEEENS8_IS2_EEENS6_5list3INS6_5valueISsEENSG_ISB_EENSG_ISC_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)
pub fn stub_acdbac() -> ! {
    todo!("0xacdbac void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>)")
}

// 0xacdd2c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_acdd2c() -> ! {
    todo!("0xacdd2c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xacdd50 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEvPSA_E6invokeERNS1_15function_bufferESL_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)
pub fn stub_acdd50() -> ! {
    todo!("0xacdd50 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")
}

// 0xacdd6c — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEclIPFvSsS8_SB_ENS0_5list1IRPSA_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::operator()<void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list1<RBX::DataModel*&> &,int)
pub fn stub_acdd6c() -> ! {
    todo!("0xacdd6c void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::operator()<void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")
}

// 0xace0fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_ace0fc() -> ! {
    todo!("0xace0fc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xace240 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_ace240() -> ! {
    todo!("0xace240 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xace390 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_ace390() -> ! {
    todo!("0xace390 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0xace664 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_ace664() -> ! {
    todo!("0xace664 RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xad3838 — __ZNK3RBX7Network4Peer11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Peer::askAddChild(RBX::Instance const*)const")]
// was: RBX::Network::Peer::askAddChild(RBX::Instance const*)const
pub fn stub_ad3838() -> ! {
    todo!("0xad3838 RBX::Network::Peer::askAddChild(RBX::Instance const*)const")
}

// 0xad6034 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_ad6034() -> ! {
    todo!("0xad6034 boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xad6038 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_ad6038() -> ! {
    todo!("0xad6038 boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}