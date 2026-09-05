// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace (filtered 10215 total, 0 remaining globally — all covered, 0 remaining in datamodel strict) — using datamodel gap filler EA-sorted asc distinct not yet in datamodel crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x56cbd4..0x573768 | EA-sorted asc distinct not yet in datamodel/src (global filtered exhausted, datamodel gap filler)
// Shard: generated_datamodel_shard_291 EA-sorted ascending next 120 datamodel gap filler after shard_290 (0x4f8874..0x56cbb0)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::generated_datamodel_shard_290::{
    HandlesEvent1Desc, HandlesEvent2Desc, HandlesHandler1, HandlesNormalId, HandlesSignal1,
};

/// Rust model of `RBX::Reflection::GenericSlotWrapper` restricted to the
/// 1-arg `Handles` slot (IDA `0x56d07c` `execute1<NormalId>`): the native
/// handler stands in for the Lua frame until the script bridge exists.
pub struct HandlesSlotWrapper1 {
    pub handler: HandlesHandler1,
}

impl HandlesSlotWrapper1 {
    /// IDA `0x56d07c`: packs the 1-`Variant` vector with the `NormalId`
    /// singleton, dispatches the wrapped slot (`*a1 + 8`), destroys the vector.
    pub fn execute1(&self, normal: HandlesNormalId) {
        (self.handler)(normal);
    }
}

/// Rust model of `boost::_bi::bind_t<void, mf1<GenericSlotWrapper, NormalId>,
/// list2<value<SharedPtr<GenericSlotWrapper>>, arg<1>>>` (IDA `0x56cf60`): the
/// retained wrapper; the arg placeholder carries no data.
#[derive(Clone)]
pub struct HandlesBind1 {
    pub wrapper: SharedPtr<HandlesSlotWrapper1>,
}
/// Rust model of `boost::function1<void, RBX::NormalId>` holding the
/// `execute1` bind (IDA `0x56d2d0` et al.): the vtable word collapses into
/// nullability of the retained bind.
#[derive(Clone, Default)]
pub struct HandlesFunction1 {
    pub target: Option<HandlesBind1>,
}

/// `functor_manager_operation_type` dispatch behind `manage`/`manager` (IDA
/// `0x56d4b0`, `0x56d798`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlesBind1Op {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    Check = 3,
    GetType = 4,
}

/// `typeinfo` name compared by the `manager` check-type path (IDA `0x56d4c6`
/// `typeinfo for'bind_t<...>`).
pub const HANDLES_BIND1_TYPE_NAME: &str = "N5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEE";

/// Shared `manager` switch (IDA `0x56d798`): 0 clones (`operator new(0x10)` +
/// memberwise + `shared_count` copy, 0x56d816-0x56d848), 1 moves, 2 destroys
/// (release + `operator delete`, 0x56d858-0x56d876), 3 checks the `typeinfo`
/// name (single monomorph, always matches), default reports the name. Move is
/// clone-shaped under `Arc`.
fn handles_manage1(
    slot: &mut HandlesFunction1,
    other: &HandlesFunction1,
    op: HandlesBind1Op,
) {
    match op {
        HandlesBind1Op::Clone | HandlesBind1Op::Move => *slot = other.clone(),
        HandlesBind1Op::Destroy => *slot = HandlesFunction1::default(),
        HandlesBind1Op::Check | HandlesBind1Op::GetType => {}
    }
}
/// Rust model of `rbx::callable<signal<void(NormalId)>::slot,
/// function<void(NormalId)>, 1, void(NormalId)>` (IDA `0x56d9e4` ctor,
/// `0x56dd20` call, `0x56ddf4`/`0x56df04` dtors): the intrusive slot link
/// behind `signal::connect` (IDA `0x56d8f0`); retain/release become
/// `clone`/`drop`, the vtables collapse.
#[derive(Clone, Default)]
pub struct HandlesCallable1 {
    pub func: HandlesFunction1,
}

// 0x56cbd4 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
pub fn stub_0x56cbd4(desc: *mut HandlesEvent2Desc) {
    // IDA 0x56cbd4 `EventDesc<Handles, void(NormalId, float)>::D0`: vtable
    // reset (`off_122F5A8`, 0x56cc12) + `_M_clear(a1 + 8)` (0x56cc38) plus
    // `operator delete` (0x56cc3e); the D1 twin at 0x56cbb0 keeps storage.
    // Reclaiming the box runs the field drops (the clear).
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x56cc88 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED0Ev")]
pub fn stub_0x56cc88(desc: *mut HandlesEvent1Desc) {
    // IDA 0x56cc88 `RemoteEventDesc<Handles, void(NormalId)>::D0`: same
    // vtable-reset + `_M_clear` + `operator delete` shape as 0x56cbd4
    // (0x56ccc6-0x56ccf2).
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x56cd3c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x56cd3c(desc: &HandlesEvent1Desc, wrapper: &SharedPtr<HandlesSlotWrapper1>) {
    // IDA 0x56cd3c `EventDescImpl<1, Handles, void(NormalId)>::
    // connectGeneric`: retains the wrapper `shared_ptr` (`shared_count` copy,
    // 0x56cd6c), `bind`s `GenericSlotWrapper::execute1` with `arg<1>`
    // (0x56cdb4), wraps it in a `function1` (0x56cdc0), adjusts to the member
    // signal (`+ *(a4 + 40) - 36`, 0x56cdd8) and `connect`s (0x56cdea). The
    // wrapper's handler is already the bound 1-arg closure; connecting it to
    // the member signal is the same subscription.
    desc.signal.connect(SharedPtr::clone(&wrapper.handler));
}

// 0x56cea0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
pub fn stub_0x56cea0(desc: &HandlesEvent1Desc) -> bool {
    // IDA 0x56cea0 `RemoteEventDesc<Handles, void(NormalId)>::isScriptable`:
    // `*(a1 + 48) & 1` (0x56cea6).
    desc.scriptable
}

// 0x56cea8 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
pub fn stub_0x56cea8(desc: &HandlesEvent1Desc) -> bool {
    // IDA 0x56cea8 `RemoteEventDesc<Handles, void(NormalId)>::isBroadcast`:
    // `*(a1 + 44) & 1` (0x56ceae); 2-arg twin is 0x56b7c0.
    desc.broadcast
}

// 0x56ceb0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_0x56ceb0(desc: &HandlesEvent1Desc, normal: HandlesNormalId) {
    // IDA 0x56ceb0 `EventDescImpl<1, Handles, void(NormalId)>::fireEvent`:
    // asserts `args.size() == 1` (Event.h:320, 0x56ceca-0x56cf02), adjusts the
    // source (`a2 ? a2 - 36 : 0`, 0x56cf14-0x56cf1a), `any_cast`s the
    // `NormalId` arg (0x56cf2a), then `signal_with_args<1>::operator()` on
    // the member signal (`*(a1 + 40) + v14`, 0x56cf20). The typed signature
    // guarantees the arity and the cast.
    desc.signal.emit(normal);
}
// 0x56cf3c — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x56cf3c(desc: &HandlesEvent1Desc, normal: HandlesNormalId) {
    // IDA 0x56cf3c `RemoteEventDesc<Handles, void(NormalId)>::sendEvent`:
    // tail-calls the remote half's virtual at `*a2 + 12` with the `Variant`
    // vector; 2-arg twin is 0x56b864.
    desc.remote.emit(normal);
}
// 0x56cf4c — __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x56cf4c(desc: &HandlesEvent1Desc) {
    // IDA 0x56cf4c `EventDescBase<Handles, void(NormalId)>::disconnectAll`:
    // adjusts the source (`a2 ? a2 - 36 : 0`, 0x56cf4c-0x56cf52) and
    // `signal::disconnectAll`s the member signal (`*(a1 + 40) + v10`); 2-arg
    // twin is 0x56b874.
    desc.signal.disconnect_all();
}
// 0x56cf60 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_0x56cf60(wrapper: &SharedPtr<HandlesSlotWrapper1>) -> HandlesBind1 {
    // IDA 0x56cf60 `bind<void, GenericSlotWrapper, NormalId const&,
    // SharedPtr<GenericSlotWrapper>, arg<1>>`: retains the wrapper
    // `shared_ptr` (`shared_count` copy, 0x56cf8c-0x56cf90), builds the
    // `list2` (0x56cfca), and stores the bind words plus the count
    // (0x56cfd2-0x56cffa). The retained wrapper is the whole payload; the arg
    // placeholder carries no data.
    HandlesBind1 { wrapper: SharedPtr::clone(wrapper) }
}
// 0x56d07c — __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_8NormalIdEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::NormalId>(RBX::NormalId const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_8NormalIdEEEvRKT_")]
pub fn stub_0x56d07c(wrapper: &HandlesSlotWrapper1, normal: HandlesNormalId) {
    // IDA 0x56d07c `GenericSlotWrapper::execute1<NormalId>`: packs the
    // 1-`Variant` vector with the `NormalId` singleton
    // (`getSingleton<NormalId>(2)`, 0x56d112), dispatches the wrapped slot
    // (`*a1 + 8`, 0x56d12e), destroys the vector (0x56d138); 2-arg twin is
    // 0x56b9a4.
    wrapper.execute1(normal);
}
// 0x56d1c0 — __ZN5boost9function1IvN3RBX8NormalIdEE5clearEv
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::NormalId>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE5clearEv")]
pub fn stub_0x56d1c0(func: &mut HandlesFunction1) {
    // IDA 0x56d1c0 `function1<void, NormalId>::clear`: runs the vtable
    // destroy path when the word is set (`(result & 1) == 0` heap tag,
    // 0x56d1ca-0x56d1e4), then `*a1 = 0`. Clearing the retained bind is the
    // same release.
    *func = HandlesFunction1::default();
}

// 0x56d1ec — __ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x56d1ec(dst: &mut HandlesFunction1, src: &HandlesBind1) {
    // IDA 0x56d1ec `function<void(NormalId)>::function<bind_t<...>>`: spills
    // the bind words plus the `shared_count` (0x56d210-0x56d224), routes
    // through `function1<bind_t>` (0x56d266), releases the temp (0x56d26c).
    // Clone-assign is the same retain/install/release.
    dst.target = Some(src.clone());
}

// 0x56d2d0 — __ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x56d2d0(dst: &mut HandlesFunction1, src: &HandlesBind1) {
    // IDA 0x56d2d0 `function1<void, NormalId>::function1<bind_t<...>>`:
    // `*a1 = 0` (0x56d2f2), then `assign_to<bind_t>` (0x56d34e), releases the
    // temp (0x56d354). Same clone-assign shape as 0x56d1ec.
    stub_0x56d3b8(dst, src);
}

// 0x56d3b8 — __ZN5boost9function1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_")]
// was: void boost::function1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_0x56d3b8(dst: &mut HandlesFunction1, src: &HandlesBind1) {
    // IDA 0x56d3b8 `function1::assign_to<bind_t<...>>`: copies the bind words
    // plus the `shared_count` (0x56d3dc-0x56d3f0), installs the stored vtable
    // through `basic_vtable1::assign_to` (0x56d440), then releases the temp
    // (0x56d446). Clone-assign is the same retain/install/release.
    dst.target = Some(src.clone());
}

// 0x56d4b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x56d4b0(
    slot: &mut HandlesFunction1,
    other: &HandlesFunction1,
    op: HandlesBind1Op,
) {
    // IDA 0x56d4b0 `functor_manager<...>::manage`: non-`GetType` ops go to
    // `manager()` (0x56d4b2-0x56d4b4); `GetType` (4) writes the `typeinfo`
    // (0x56d4c6-0x56d4ca). Both delegate to the shared switch; `GetType`
    // only reports the name.
    let _ = HANDLES_BIND1_TYPE_NAME;
    handles_manage1(slot, other, op);
}

// 0x56d4cc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::NormalId>::invoke(boost::detail::function::function_buffer &,RBX::NormalId)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::NormalId>::invoke(boost::detail::function::function_buffer &,RBX::NormalId)
pub fn stub_0x56d4cc(bind: &HandlesBind1, normal: HandlesNormalId) {
    // IDA 0x56d4cc `void_function_obj_invoker1<...>::invoke`: tail-calls
    // `bind_t::operator()<NormalId>` (0x56d4de), which unpacks to the `mf1`
    // call on the retained wrapper — the `execute1` path.
    stub_0x56d07c(&bind.wrapper, normal);
}

// 0x56d4e0 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x56d4e0(dst: &mut HandlesFunction1, src: &HandlesBind1) -> bool {
    // IDA 0x56d4e0 `basic_vtable1::assign_to<bind_t<...>>` (words form):
    // spills the bind words plus the `shared_count`, installs through the
    // nested `assign_to`, returns 1. Clone-assign plus success is the same
    // outcome.
    stub_0x56d3b8(dst, src);
    true
}

// 0x56d5c8 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x56d5c8(dst: &mut HandlesFunction1, src: &HandlesBind1) -> bool {
    // IDA 0x56d5c8 `basic_vtable1::assign_to<bind_t<...>>` (count form):
    // retains via `shared_count` copy (0x56d5e8-0x56d616), `assign_functor`
    // (0x56d640), releases (0x56d646), returns 1 (0x56d66e).
    stub_0x56d6ac(dst, src);
    true
}

// 0x56d6ac — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x56d6ac(dst: &mut HandlesFunction1, src: &HandlesBind1) {
    // IDA 0x56d6ac `basic_vtable1::assign_functor<bind_t<...>>`:
    // `operator new(0x10)`, memberwise copy of the bind words (0x56d6d4-0x56d72e)
    // plus the `shared_count` copy, installs the heap functor (`*a3 = v6`,
    // 0x56d736). Clone-assign is the same install.
    dst.target = Some(src.clone());
}

// 0x56d780 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)
pub fn stub_0x56d780(bind: &HandlesBind1, normal: HandlesNormalId) {
    // IDA 0x56d780 `bind_t::operator()<NormalId>`: loads the mf1 fn ptr +
    // wrapper from the bind (0x56d780), resolves the member target with the
    // virtual-thunk check (`(v2 & 1)`, 0x56d78a-0x56d792), then calls it —
    // `execute1(wrapper, normal)` (cf. 2-arg 0x708a94).
    stub_0x56d07c(&bind.wrapper, normal);
}

// 0x56d798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x56d798(
    src: &HandlesBind1,
    dst: &mut HandlesFunction1,
    op: HandlesBind1Op,
) {
    // IDA 0x56d798 `functor_manager::manager` with `mpl::bool_<false>`
    // (heap-only): `case 0` clones via `operator new(0x10)` + memberwise +
    // `shared_count` copy (0x56d816-0x56d848), 1 moves, 2 destroys (release +
    // `operator delete`, 0x56d858-0x56d876); the check-type arm compares
    // `HANDLES_BIND1_TYPE_NAME`. All collapse into the shared switch.
    let _ = HANDLES_BIND1_TYPE_NAME;
    match op {
        HandlesBind1Op::Clone => *dst = HandlesFunction1 { target: Some(src.clone()) },
        HandlesBind1Op::Move => *dst = HandlesFunction1 { target: Some(src.clone()) },
        HandlesBind1Op::Destroy => *dst = HandlesFunction1::default(),
        HandlesBind1Op::Check | HandlesBind1Op::GetType => {}
    }
}

// 0x56d8f0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::function<void ()(RBX::NormalId)>>(boost::function<void ()(RBX::NormalId)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
pub fn stub_0x56d8f0(sig: &HandlesSignal1, func: &HandlesFunction1) {
    // IDA 0x56d8f0 `signal<void(NormalId)>::connect<function<void(NormalId)>>`:
    // `operator new(32)` callable slot (0x56d92a), `callable` ctor (0x56d952),
    // vtable installs (0x56d96c-0x56d972), `insert` (0x56d97a) plus the weak
    // ref (0x56d986-0x56d98c). The slot owns the closure's strong ref, so
    // connecting the retained handler is the same subscription.
    if let Some(bind) = func.target.as_ref() {
        sig.connect(SharedPtr::clone(&bind.wrapper.handler));
    }
}

// 0x56d9e4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::callable<rbx::signals::signal<void ()(RBX::NormalId)>*>(boost::function<void ()(RBX::NormalId)> const&,rbx::signals::signal<void ()(RBX::NormalId)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
pub fn stub_0x56d9e4(slot: &mut HandlesCallable1, func: &HandlesFunction1) {
    // IDA 0x56d9e4 `callable<slot, function, 1>::callable`: zeroes the link
    // words (0x56da16-0x56da3e), installs the vtables (`off_1263C58` /
    // `off_1263C74`, 0x56da2c-0x56da32), `assign_to_own`s the function
    // (0x56da64). Clone-assign is the same install.
    slot.func = func.clone();
}

// 0x56dae0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
pub fn stub_0x56dae0(slot: &mut HandlesCallable1) {
    // IDA 0x56dae0 `callable_slot<function>::~callable_slot` D1: vtable
    // installs (0x56db18-0x56db24), `function1::clear` (0x56db4c), base
    // vtables (0x56db62-0x56db68), intrusive release (0x56db6c-0x56db76);
    // storage kept. Clearing the function is the same release.
    slot.func = HandlesFunction1::default();
}

// 0x56dbf0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
pub fn stub_0x56dbf0(slot: *mut HandlesCallable1) {
    // IDA 0x56dbf0 `callable_slot<function>::~callable_slot` D0: the D1 body
    // (0x56dc28-0x56dc84) plus `operator delete` (0x56dc90). Reclaiming the
    // box runs the field drops (the clear).
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0x56dd20 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
pub fn stub_0x56dd20(slot: &HandlesCallable1, normal: HandlesNormalId) {
    // IDA 0x56dd20 `callable<slot, function, 1>::call`: tail-calls
    // `function1::operator()` — the `execute1` dispatch below.
    stub_0x56dd30(&slot.func, normal);
}

// 0x56dd28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
pub fn stub_0x56dd28(slot: &HandlesCallable1, normal: HandlesNormalId) {
    // IDA 0x56dd28 non-virtual thunk to `callable::call`: same tail-call to
    // `function1::operator()` (cf. 2-arg thunk at 0x56c67c).
    stub_0x56dd30(&slot.func, normal);
}

// 0x56dd30 — __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::NormalId>::operator()(RBX::NormalId)const")]
#[doc(alias = "__ZNK5boost9function1IvN3RBX8NormalIdEEclES2_")]
pub fn stub_0x56dd30(func: &HandlesFunction1, normal: HandlesNormalId) {
    // IDA 0x56dd30 `function1<void, NormalId>::operator()`: empty function
    // throws `bad_function_call` (0x56dd7e-0x56dde6); else the vtable invoke
    // runs (`(*a1 & ~1) + 4`, 0x56dd90) — the `execute1` path through the
    // retained bind.
    let bind = func.target.as_ref().expect("0x56dd30: bad_function_call");
    stub_0x56d07c(&bind.wrapper, normal);
}

// 0x56ddf4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")]
pub fn stub_0x56ddf4(slot: &mut HandlesCallable1) {
    // IDA 0x56ddf4 `callable<slot, function, 1>::~callable` D1: same
    // vtable-install + `clear` + release shape as the slot D1 at 0x56dae0
    // (0x56de2c-0x56de8a); storage kept.
    slot.func = HandlesFunction1::default();
}

// 0x56df04 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")]
pub fn stub_0x56df04(slot: *mut HandlesCallable1) {
    // IDA 0x56df04 `callable<slot, function, 1>::~callable` D0: the D1 body
    // (0x56df3c-0x56df98) plus `operator delete` (0x56dfa4).
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0x56e034 — __ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::NormalId>::assign_to_own(boost::function1<void,RBX::NormalId> const&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_")]
pub fn stub_0x56e034(dst: &mut HandlesFunction1, src: &HandlesFunction1) {
    // IDA 0x56e034 `function1::assign_to_own`: empty source stores nothing;
    // heap-tagged sources memberwise-copy the buffer words (0x56e03c-0x56e04c),
    // else the vtable copy runs (0x56e062). Clone-assign is the same copy.
    *dst = src.clone();
}

// 0x56e064 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x56e064(
    this: *mut HandlesEvent1Desc,
    name: &str,
    permissions: u32,
    attributes: u32,
) {
    // IDA 0x56e064 `EventDesc<Handles, void(NormalId)>::EventDesc` C2: runs
    // the `Described<Handles>` + `EventDescriptor` bases (0x56e09c + 0x56e0ba),
    // stores the member signal pointer at `+40` (0x56e0de), installs the
    // `EventDesc` vtable (`off_1263C88`, 0x56e0e2), declares the `Name` and
    // appends the single signature item — `NormalId`
    // (`getSingleton<NormalId>(2)`, 0x56e10e) — via `_M_create_node` + `hook`
    // (0x56e12c); 2-arg twin is 0x56c9c0.
    // SAFETY: `this` must point to valid uninitialized `HandlesEvent1Desc` storage.
    let _ = permissions;
    let _ = attributes;
    unsafe {
        core::ptr::write(
            this,
            HandlesEvent1Desc { name: name.to_string(), ..Default::default() },
        );
    }
}

// 0x56e1e8 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
pub fn stub_0x56e1e8(desc: *mut HandlesEvent1Desc) {
    // IDA 0x56e1e8 `EventDesc<Handles, void(NormalId)>::D1`: vtable reset
    // (`off_122F5A8`, 0x56e200) + `_M_clear(a1 + 8)` (0x56e204); storage kept
    // (cf. D0 at 0x56e20c). Clearing the name drops the item list.
    // SAFETY: `desc` must point to a valid `HandlesEvent1Desc`.
    unsafe {
        (*desc).name.clear();
    }
}

// 0x56e20c — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
pub fn stub_0x56e20c(desc: *mut HandlesEvent1Desc) {
    // IDA 0x56e20c `EventDesc<Handles, void(NormalId)>::D0`: the D1 body
    // (`*a1` vtable reset + `_M_clear`, 0x56e24a-0x56e270) plus
    // `operator delete` (0x56e276).
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x56e2c0 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::PropDescriptor<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>(char const*,char const*,RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x56e2c0() -> ! {
    todo!("0x56e2c0 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::PropDescriptor<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>(char const*,char const*,RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x56e3d4 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED0Ev")]
pub fn stub_0x56e3d4() -> ! {
    todo!("0x56e3d4 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")
}

// 0x56e400 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x56e400() -> ! {
    todo!("0x56e400 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isReadOnly(void)const")
}

// 0x56e404 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x56e404() -> ! {
    todo!("0x56e404 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isWriteOnly(void)const")
}

// 0x56e408 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56e408() -> ! {
    todo!("0x56e408 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56e428 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::setValue(RBX::Reflection::DescribedBase *,RBX::Faces const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x56e428() -> ! {
    todo!("0x56e428 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::setValue(RBX::Reflection::DescribedBase *,RBX::Faces const&)const")
}

// 0x56e44c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::EnumPropDescriptor<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>(char const*,char const*,RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x56e44c() -> ! {
    todo!("0x56e44c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::EnumPropDescriptor<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>(char const*,char const*,RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x56e600 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED0Ev")]
pub fn stub_0x56e600() -> ! {
    todo!("0x56e600 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")
}

// 0x56e62c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10isReadOnlyEv")]
pub fn stub_0x56e62c() -> ! {
    todo!("0x56e62c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isReadOnly(void)const")
}

// 0x56e63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11isWriteOnlyEv")]
pub fn stub_0x56e63c() -> ! {
    todo!("0x56e63c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isWriteOnly(void)const")
}

// 0x56e64c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x56e64c() -> ! {
    todo!("0x56e64c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x56e674 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x56e674() -> ! {
    todo!("0x56e674 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x56e698 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x56e698() -> ! {
    todo!("0x56e698 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x56e7e4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x56e7e4() -> ! {
    todo!("0x56e7e4 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x56e808 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14hasStringValueEv")]
pub fn stub_0x56e808() -> ! {
    todo!("0x56e808 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::hasStringValue(void)const")
}

// 0x56e80c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56e80c() -> ! {
    todo!("0x56e80c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56e830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x56e830() -> ! {
    todo!("0x56e830 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x56e870 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x56e870() -> ! {
    todo!("0x56e870 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x56e890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x56e890() -> ! {
    todo!("0x56e890 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x56ead0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56ead0() -> ! {
    todo!("0x56ead0 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eaec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x56eaec() -> ! {
    todo!("0x56eaec RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x56eb20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56eb20() -> ! {
    todo!("0x56eb20 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eb28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x56eb28() -> ! {
    todo!("0x56eb28 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x56eb74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x56eb74() -> ! {
    todo!("0x56eb74 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eb94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x56eb94() -> ! {
    todo!("0x56eb94 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x56ebc8 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToIndex(RBX::Handles::VisualStyle)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToIndexES3_")]
pub fn stub_0x56ebc8() -> ! {
    todo!("0x56ebc8 RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToIndex(RBX::Handles::VisualStyle)const")
}

// 0x56ec38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x56ec38() -> ! {
    todo!("0x56ec38 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x56ec78 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x56ec78() -> ! {
    todo!("0x56ec78 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isReadOnly(void)const")
}

// 0x56ec7c — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x56ec7c() -> ! {
    todo!("0x56ec7c RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isWriteOnly(void)const")
}

// 0x56ec80 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56ec80() -> ! {
    todo!("0x56ec80 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eca0 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::Handles::VisualStyle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x56eca0() -> ! {
    todo!("0x56eca0 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::Handles::VisualStyle const&)const")
}

// 0x56ecc4 — __ZN3RBX7HandlesD2Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "RBX::Handles::~Handles()")]
#[doc(alias = "__ZN3RBX7HandlesD2Ev")]
pub fn stub_0x56ecc4() -> ! {
    todo!("0x56ecc4 RBX::Handles::~Handles()")
}

// 0x56eef8 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev")]
pub fn stub_0x56eef8() -> ! {
    todo!("0x56eef8 rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")
}

// 0x56f044 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev")]
pub fn stub_0x56f044() -> ! {
    todo!("0x56f044 rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")
}

// 0x56f190 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev")]
pub fn stub_0x56f190() -> ! {
    todo!("0x56f190 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")
}

// 0x56f2c0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev")]
pub fn stub_0x56f2c0() -> ! {
    todo!("0x56f2c0 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")
}

// 0x56f9fc — __ZN3RBX11HandlesBaseC2EPKc
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, const char *)
#[doc(alias = "RBX::HandlesBase::HandlesBase(char const*)")]
#[doc(alias = "__ZN3RBX11HandlesBaseC2EPKc")]
pub fn stub_0x56f9fc() -> ! {
    todo!("0x56f9fc RBX::HandlesBase::HandlesBase(char const*)")
}

// 0x56fc18 — __ZN3RBX11HandlesBase16findTargetHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::HandlesBase::findTargetHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)")]
#[doc(alias = "__ZN3RBX11HandlesBase16findTargetHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE")]
pub fn stub_0x56fc18() -> ! {
    todo!("0x56fc18 RBX::HandlesBase::findTargetHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)")
}

// 0x56fd94 — __ZN3RBX11HandlesBase21getDistanceFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERf
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::HandlesBase::getDistanceFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,float &)")]
#[doc(alias = "__ZN3RBX11HandlesBase21getDistanceFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERf")]
pub fn stub_0x56fd94() -> ! {
    todo!("0x56fd94 RBX::HandlesBase::getDistanceFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,float &)")
}

// 0x56ff64 — __ZN3RBX11HandlesBase20getFacePosFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERNS5_7Vector2ESA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::HandlesBase::getFacePosFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,G3D::Vector2 &,G3D::Vector2 &)")]
#[doc(alias = "__ZN3RBX11HandlesBase20getFacePosFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERNS5_7Vector2ESA_")]
pub fn stub_0x56ff64() -> ! {
    todo!("0x56ff64 RBX::HandlesBase::getFacePosFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,G3D::Vector2 &,G3D::Vector2 &)")
}

// 0x5702b0 — __ZN3RBX11HandlesBase24getAngleRadiusFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERfS9_S9_S9_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::HandlesBase::getAngleRadiusFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,float &,float &,float &,float &)")]
#[doc(alias = "__ZN3RBX11HandlesBase24getAngleRadiusFromHandleERKNS_7UIEventENS_8NormalIdERKN3G3D7Vector3ERfS9_S9_S9_")]
pub fn stub_0x5702b0() -> ! {
    todo!("0x5702b0 RBX::HandlesBase::getAngleRadiusFromHandle(RBX::UIEvent const&,RBX::NormalId,G3D::Vector3 const&,float &,float &,float &,float &)")
}

// 0x5703e0 — __ZNK3RBX11HandlesBase26canProcessMeAndDescendantsEv
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "RBX::HandlesBase::canProcessMeAndDescendants(void)const")]
#[doc(alias = "__ZNK3RBX11HandlesBase26canProcessMeAndDescendantsEv")]
pub fn stub_0x5703e0() -> ! {
    todo!("0x5703e0 RBX::HandlesBase::canProcessMeAndDescendants(void)const")
}

// 0x5704c4 — __ZN3RBX11HandlesBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::HandlesBase::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11HandlesBase8render2dEPNS_5AdornE")]
pub fn stub_0x5704c4() -> ! {
    todo!("0x5704c4 RBX::HandlesBase::render2d(RBX::Adorn *)")
}

// 0x570614 — __ZThn96_N3RBX11HandlesBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX11HandlesBase8render2dEPNS_5AdornE")]
pub fn stub_0x570614() -> ! {
    todo!("0x570614 non-virtual thunk toRBX::HandlesBase::render2d(RBX::Adorn *)")
}

// 0x57061c — __ZN3RBX11HandlesBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::HandlesBase::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11HandlesBase13render3dAdornEPNS_5AdornE")]
pub fn stub_0x57061c() -> ! {
    todo!("0x57061c RBX::HandlesBase::render3dAdorn(RBX::Adorn *)")
}

// 0x57079c — __ZThn96_N3RBX11HandlesBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX11HandlesBase13render3dAdornEPNS_5AdornE")]
pub fn stub_0x57079c() -> ! {
    todo!("0x57079c non-virtual thunk toRBX::HandlesBase::render3dAdorn(RBX::Adorn *)")
}

// 0x5707a4 — __ZN3RBX11HandlesBase18setServerGuiObjectEv
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "RBX::HandlesBase::setServerGuiObject(void)")]
#[doc(alias = "__ZN3RBX11HandlesBase18setServerGuiObjectEv")]
pub fn stub_0x5707a4() -> ! {
    todo!("0x5707a4 RBX::HandlesBase::setServerGuiObject(void)")
}

// 0x5707ac — __ZN3RBX11HandlesBase17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::HandlesBase::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX11HandlesBase17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x5707ac() -> ! {
    todo!("0x5707ac RBX::HandlesBase::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x57117c — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC1Ev")]
pub fn stub_0x57117c() -> ! {
    todo!("0x57117c RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")
}

// 0x571180 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC2Ev")]
pub fn stub_0x571180() -> ! {
    todo!("0x571180 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")
}

// 0x5713d0 — __ZNK3RBX12BackpackItem12getTextureIdEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "RBX::BackpackItem::getTextureId(void)const")]
#[doc(alias = "__ZNK3RBX12BackpackItem12getTextureIdEv")]
pub fn stub_0x5713d0() -> ! {
    todo!("0x5713d0 RBX::BackpackItem::getTextureId(void)const")
}

// 0x5713e8 — __ZN3RBX12BackpackItem12setTextureIdERKNS_9TextureIdE
// type: int __fastcall(RBX::BackpackItem *this, const RBX::TextureId *)
#[doc(alias = "RBX::BackpackItem::setTextureId(RBX::TextureId const&)")]
#[doc(alias = "__ZN3RBX12BackpackItem12setTextureIdERKNS_9TextureIdE")]
pub fn stub_0x5713e8() -> ! {
    todo!("0x5713e8 RBX::BackpackItem::setTextureId(RBX::TextureId const&)")
}

// 0x571428 — __ZN3RBX9HopperBin10setBinTypeENS0_7BinTypeE
#[doc(alias = "RBX::HopperBin::setBinType(RBX::HopperBin::BinType)")]
#[doc(alias = "__ZN3RBX9HopperBin10setBinTypeENS0_7BinTypeE")]
pub fn stub_0x571428() -> ! {
    todo!("0x571428 RBX::HopperBin::setBinType(RBX::HopperBin::BinType)")
}

// 0x5715a8 — __ZN3RBX9HopperBin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::HopperBin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX9HopperBin11dataChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x5715a8() -> ! {
    todo!("0x5715a8 RBX::HopperBin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x5715ac — __ZN3RBX9HopperBin7disableEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::disable(void)")]
#[doc(alias = "__ZN3RBX9HopperBin7disableEv")]
pub fn stub_0x5715ac() -> ! {
    todo!("0x5715ac RBX::HopperBin::disable(void)")
}

// 0x5715f8 — __ZN3RBX9HopperBin16setLegacyCommandERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
#[doc(alias = "RBX::HopperBin::setLegacyCommand(std::string const&)")]
#[doc(alias = "__ZN3RBX9HopperBin16setLegacyCommandERKSs")]
pub fn stub_0x5715f8() -> ! {
    todo!("0x5715f8 RBX::HopperBin::setLegacyCommand(std::string const&)")
}

// 0x571654 — __ZN3RBX9HopperBin20setLegacyTextureNameERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
#[doc(alias = "RBX::HopperBin::setLegacyTextureName(std::string const&)")]
#[doc(alias = "__ZN3RBX9HopperBin20setLegacyTextureNameERKSs")]
pub fn stub_0x571654() -> ! {
    todo!("0x571654 RBX::HopperBin::setLegacyTextureName(std::string const&)")
}

// 0x57195c — __ZN3RBX11StarterGearC1Ev
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "RBX::StarterGear::StarterGear(void)")]
#[doc(alias = "__ZN3RBX11StarterGearC1Ev")]
pub fn stub_0x57195c() -> ! {
    todo!("0x57195c RBX::StarterGear::StarterGear(void)")
}

// 0x571960 — __ZN3RBX11StarterGearC2Ev
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "RBX::StarterGear::StarterGear(void)")]
#[doc(alias = "__ZN3RBX11StarterGearC2Ev")]
pub fn stub_0x571960() -> ! {
    todo!("0x571960 RBX::StarterGear::StarterGear(void)")
}

// 0x571b94 — __ZN3RBX12BackpackItem7setNameERKSs
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, const std::string *)
#[doc(alias = "RBX::BackpackItem::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX12BackpackItem7setNameERKSs")]
pub fn stub_0x571b94() -> ! {
    todo!("0x571b94 RBX::BackpackItem::setName(std::string const&)")
}

// 0x571bb4 — __ZNK3RBX12BackpackItem8getBinIdEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "RBX::BackpackItem::getBinId(void)const")]
#[doc(alias = "__ZNK3RBX12BackpackItem8getBinIdEv")]
pub fn stub_0x571bb4() -> ! {
    todo!("0x571bb4 RBX::BackpackItem::getBinId(void)const")
}

// 0x571c18 — __ZN3RBX12BackpackItem10inBackpackEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "RBX::BackpackItem::inBackpack(void)")]
#[doc(alias = "__ZN3RBX12BackpackItem10inBackpackEv")]
pub fn stub_0x571c18() -> ! {
    todo!("0x571c18 RBX::BackpackItem::inBackpack(void)")
}

// 0x571c5c — __ZNK3RBX12BackpackItem7getSizeENS_6CanvasE
#[doc(alias = "RBX::BackpackItem::getSize(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX12BackpackItem7getSizeENS_6CanvasE")]
pub fn stub_0x571c5c() -> ! {
    todo!("0x571c5c RBX::BackpackItem::getSize(RBX::Canvas)const")
}

// 0x571c90 — __ZN3RBX12BackpackItem8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::BackpackItem::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX12BackpackItem8render2dEPNS_5AdornE")]
pub fn stub_0x571c90() -> ! {
    todo!("0x571c90 RBX::BackpackItem::render2d(RBX::Adorn *)")
}

// 0x5721a8 — __ZN3RBX9HopperBinC2Ev
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::HopperBin(void)")]
#[doc(alias = "__ZN3RBX9HopperBinC2Ev")]
pub fn stub_0x5721a8() -> ! {
    todo!("0x5721a8 RBX::HopperBin::HopperBin(void)")
}

// 0x572710 — __ZN3RBX9HopperBin30selectedConnectionShimFunctionEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::selectedConnectionShimFunction(void)")]
#[doc(alias = "__ZN3RBX9HopperBin30selectedConnectionShimFunctionEv")]
pub fn stub_0x572710() -> ! {
    todo!("0x572710 RBX::HopperBin::selectedConnectionShimFunction(void)")
}

// 0x572714 — __ZN3RBX9HopperBin14onSelectScriptEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::onSelectScript(void)")]
#[doc(alias = "__ZN3RBX9HopperBin14onSelectScriptEv")]
pub fn stub_0x572714() -> ! {
    todo!("0x572714 RBX::HopperBin::onSelectScript(void)")
}

// 0x5728bc — __ZN3RBX9HopperBin17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::HopperBin::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX9HopperBin17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x5728bc() -> ! {
    todo!("0x5728bc RBX::HopperBin::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x572b14 — __ZN3RBX9HopperBin15onSelectCommandEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::onSelectCommand(void)")]
#[doc(alias = "__ZN3RBX9HopperBin15onSelectCommandEv")]
pub fn stub_0x572b14() -> ! {
    todo!("0x572b14 RBX::HopperBin::onSelectCommand(void)")
}

// 0x572e98 — __ZN3RBX9HopperBin14onLocalClickedEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::onLocalClicked(void)")]
#[doc(alias = "__ZN3RBX9HopperBin14onLocalClickedEv")]
pub fn stub_0x572e98() -> ! {
    todo!("0x572e98 RBX::HopperBin::onLocalClicked(void)")
}

// 0x572ef8 — __ZN3RBX9HopperBin19onLocalOtherClickedEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::onLocalOtherClicked(void)")]
#[doc(alias = "__ZN3RBX9HopperBin19onLocalOtherClickedEv")]
pub fn stub_0x572ef8() -> ! {
    todo!("0x572ef8 RBX::HopperBin::onLocalOtherClicked(void)")
}

// 0x572efc — __ZN3RBX6HopperC2Ev
// type: _DWORD __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "RBX::Hopper::Hopper(void)")]
#[doc(alias = "__ZN3RBX6HopperC2Ev")]
pub fn stub_0x572efc() -> ! {
    todo!("0x572efc RBX::Hopper::Hopper(void)")
}

// 0x572f78 — __ZN3RBX6Hopper8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Hopper *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Hopper::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX6Hopper8render2dEPNS_5AdornE")]
pub fn stub_0x572f78() -> ! {
    todo!("0x572f78 RBX::Hopper::render2d(RBX::Adorn *)")
}

// 0x573090 — __ZN3RBX18StarterPackServiceC1Ev
// type: _DWORD __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
#[doc(alias = "__ZN3RBX18StarterPackServiceC1Ev")]
pub fn stub_0x573090() -> ! {
    todo!("0x573090 RBX::StarterPackService::StarterPackService(void)")
}

// 0x573094 — __ZN3RBX18StarterPackServiceC2Ev
// type: _DWORD __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
#[doc(alias = "__ZN3RBX18StarterPackServiceC2Ev")]
pub fn stub_0x573094() -> ! {
    todo!("0x573094 RBX::StarterPackService::StarterPackService(void)")
}

// 0x5732ac — __ZN3RBX18StarterPackService8render2dEPNS_5AdornE
#[doc(alias = "RBX::StarterPackService::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX18StarterPackService8render2dEPNS_5AdornE")]
pub fn stub_0x5732ac() -> ! {
    todo!("0x5732ac RBX::StarterPackService::render2d(RBX::Adorn *)")
}

// 0x5732b0 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addPair(RBX::HopperBin::BinType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE7addPairES3_PKc")]
pub fn stub_0x5732b0() -> ! {
    todo!("0x5732b0 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addPair(RBX::HopperBin::BinType,char const*)")
}

// 0x573610 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE9addLegacyEiPKcS3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addLegacy(int,char const*,RBX::HopperBin::BinType)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE9addLegacyEiPKcS3_")]
pub fn stub_0x573610() -> ! {
    todo!("0x573610 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addLegacy(int,char const*,RBX::HopperBin::BinType)")
}

// 0x573664 — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED1Ev")]
pub fn stub_0x573664() -> ! {
    todo!("0x573664 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")
}

// 0x573688 — __ZNK3RBX9HopperBin10getBinTypeEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "RBX::HopperBin::getBinType(void)const")]
#[doc(alias = "__ZNK3RBX9HopperBin10getBinTypeEv")]
pub fn stub_0x573688() -> ! {
    todo!("0x573688 RBX::HopperBin::getBinType(void)const")
}

// 0x573690 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED1Ev")]
pub fn stub_0x573690() -> ! {
    todo!("0x573690 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")
}

// 0x5736d8 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_0x5736d8() -> ! {
    todo!("0x5736d8 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x5736fc — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_0x5736fc() -> ! {
    todo!("0x5736fc RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x573720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED1Ev")]
pub fn stub_0x573720() -> ! {
    todo!("0x573720 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")
}

// 0x573744 — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED1Ev")]
pub fn stub_0x573744() -> ! {
    todo!("0x573744 RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")
}

// 0x573768 — __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE
// type: int(void)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE")]
pub fn stub_0x573768() -> ! {
    todo!("0x573768 RBX::Reflection::RemoteEventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")
}

#[cfg(test)]
mod handles_1arg_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn dtors_drop_boxes() {
        stub_0x56cbd4(Box::into_raw(Box::new(HandlesEvent2Desc::default())));
        stub_0x56cc88(Box::into_raw(Box::new(HandlesEvent1Desc::default())));
    }

    #[test]
    fn flags_fire_send_disconnect() {
        let desc = HandlesEvent1Desc { name: "Face".to_string(), ..Default::default() };
        assert!(!stub_0x56cea8(&desc));
        assert!(!stub_0x56cea0(&desc));
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        desc.signal.connect(Arc::new(move |normal: u32| {
            probe.store(normal as i32, Ordering::Relaxed);
        }));
        assert_eq!(desc.signal.len(), 1);
        stub_0x56ceb0(&desc, 3);
        assert_eq!(seen.load(Ordering::Relaxed), 3);
        let remote = Arc::new(AtomicI32::new(0));
        let rp = Arc::clone(&remote);
        desc.remote.connect(Arc::new(move |normal: u32| {
            rp.store(10 + normal as i32, Ordering::Relaxed);
        }));
        stub_0x56cf3c(&desc, 5);
        assert_eq!(remote.load(Ordering::Relaxed), 15);
        stub_0x56cf4c(&desc);
        assert_eq!(desc.signal.len(), 0);
        stub_0x56ceb0(&desc, 9);
        assert_eq!(seen.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn connect_bind_execute() {
        let desc = HandlesEvent1Desc { name: "Face".to_string(), ..Default::default() };
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        let wrapper = SharedPtr::new(HandlesSlotWrapper1 {
            handler: Arc::new(move |normal: u32| {
                probe.store(normal as i32, Ordering::Relaxed);
            }),
        });
        stub_0x56cd3c(&desc, &wrapper);
        assert_eq!(desc.signal.len(), 1);
        let bind = stub_0x56cf60(&wrapper);
        stub_0x56d07c(&bind.wrapper, 7);
        assert_eq!(seen.load(Ordering::Relaxed), 7);
    }
}

#[cfg(test)]
mod handles_bind1_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    fn probe_wrapper(seen: &Arc<AtomicI32>) -> SharedPtr<HandlesSlotWrapper1> {
        let probe = Arc::clone(seen);
        SharedPtr::new(HandlesSlotWrapper1 {
            handler: Arc::new(move |normal: u32| {
                probe.store(normal as i32, Ordering::Relaxed);
            }),
        })
    }

    #[test]
    fn function_assign_invoke_clear() {
        let seen = Arc::new(AtomicI32::new(0));
        let wrapper = probe_wrapper(&seen);
        let bind = stub_0x56cf60(&wrapper);
        let mut func = HandlesFunction1::default();
        stub_0x56d1ec(&mut func, &bind);
        assert!(func.target.is_some());
        stub_0x56d4cc(func.target.as_ref().unwrap(), 4);
        assert_eq!(seen.load(Ordering::Relaxed), 4);
        stub_0x56d780(&bind, 6);
        assert_eq!(seen.load(Ordering::Relaxed), 6);
        stub_0x56d1c0(&mut func);
        assert!(func.target.is_none());
    }

    #[test]
    fn vtable_manage_round_trip() {
        let seen = Arc::new(AtomicI32::new(0));
        let wrapper = probe_wrapper(&seen);
        let bind = stub_0x56cf60(&wrapper);
        let mut func = HandlesFunction1::default();
        stub_0x56d2d0(&mut func, &bind);
        assert!(func.target.is_some());
        assert!(stub_0x56d4e0(&mut func, &bind));
        assert!(stub_0x56d5c8(&mut func, &bind));
        let mut other = HandlesFunction1::default();
        stub_0x56d4b0(&mut other, &func, HandlesBind1Op::Clone);
        assert!(other.target.is_some());
        stub_0x56d798(&bind, &mut other, HandlesBind1Op::Move);
        assert!(other.target.is_some());
        stub_0x56d798(&bind, &mut other, HandlesBind1Op::Check);
        assert!(other.target.is_some());
        stub_0x56d798(&bind, &mut other, HandlesBind1Op::GetType);
        assert_eq!(
            HANDLES_BIND1_TYPE_NAME,
            "N5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEE"
        );
        stub_0x56d798(&bind, &mut other, HandlesBind1Op::Destroy);
        assert!(other.target.is_none());
    }
}

#[cfg(test)]
mod handles_signal1_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    fn probe_func(seen: &Arc<AtomicI32>) -> HandlesFunction1 {
        let probe = Arc::clone(seen);
        let wrapper = SharedPtr::new(HandlesSlotWrapper1 {
            handler: Arc::new(move |normal: u32| {
                probe.store(normal as i32, Ordering::Relaxed);
            }),
        });
        HandlesFunction1 { target: Some(HandlesBind1 { wrapper }) }
    }

    #[test]
    fn connect_call_operator_assign() {
        let seen = Arc::new(AtomicI32::new(0));
        let func = probe_func(&seen);
        let sig = HandlesSignal1::default();
        stub_0x56d8f0(&sig, &func);
        assert_eq!(sig.len(), 1);
        let mut slot = HandlesCallable1::default();
        stub_0x56d9e4(&mut slot, &func);
        stub_0x56dd20(&slot, 11);
        assert_eq!(seen.load(Ordering::Relaxed), 11);
        stub_0x56dd28(&slot, 13);
        assert_eq!(seen.load(Ordering::Relaxed), 13);
        stub_0x56dd30(&func, 17);
        assert_eq!(seen.load(Ordering::Relaxed), 17);
        let mut owned = HandlesFunction1::default();
        stub_0x56e034(&mut owned, &func);
        assert!(owned.target.is_some());
        stub_0x56dae0(&mut slot);
        assert!(slot.func.target.is_none());
        stub_0x56d9e4(&mut slot, &func);
        stub_0x56ddf4(&mut slot);
        assert!(slot.func.target.is_none());
    }

    #[test]
    fn slot_callable_dtors_drop() {
        stub_0x56dbf0(Box::into_raw(Box::new(HandlesCallable1::default())));
        stub_0x56df04(Box::into_raw(Box::new(HandlesCallable1::default())));
    }

    #[test]
    fn event1_ctor_dtors() {
        let mut storage = HandlesEvent1Desc::default();
        stub_0x56e064(&mut storage as *mut HandlesEvent1Desc, "Face", 0, 0);
        assert_eq!(storage.name, "Face");
        stub_0x56e1e8(&mut storage as *mut HandlesEvent1Desc);
        assert!(storage.name.is_empty());
        stub_0x56e20c(Box::into_raw(Box::new(HandlesEvent1Desc::default())));
    }
}
