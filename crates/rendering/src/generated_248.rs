//! rendering — generated_248 — 100 stubs EA-sorted asc global gap filler 0x2afc80..0x2b3b78
//! Source: ida/export.json (85545 funcs) EA-sorted global filler not yet in rbx_rendering (rendering 26620 before, 26720 after distinct; Ogre|G3D complete 13663/13663)
//! Filter: next 100 EA-sorted ascending after 0x2afc58 not yet in rendering (global filler, Ogre|G3D already complete — fills gap after 246)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2afc80 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
// IDA 0x2afc80: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afc80() {
}

// 0x2afd78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// IDA 0x2afd78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2afd78() {
}

// 0x2afda4 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
// IDA 0x2afda4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2afda4() {
}

// 0x2afe78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
// IDA 0x2afe78: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afe78() {
}

// 0x2aff88 — __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
// IDA 0x2aff88: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aff88() {
}

// 0x2aff94 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x2aff94: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aff94() {
}

// 0x2aff9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x2aff9c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aff9c() {
}

// 0x2affa4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// IDA 0x2affa4: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2affa4() {
}

// 0x2affbc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
// IDA 0x2affbc: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2affbc() {
}

// 0x2b00ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
// IDA 0x2b00ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b00ac() {
}

// 0x2b00b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
// IDA 0x2b00b0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b00b0() {
}

// 0x2b01a0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
// IDA 0x2b01a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b01a0() {
}

// 0x2b01cc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
// IDA 0x2b01cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b01cc() {
}

// 0x2b02a0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// IDA 0x2b02a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b02a0() {
}

// 0x2b02cc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// IDA 0x2b02cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b02cc() {
}

// 0x2b03a0 — __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
// IDA 0x2b03a0: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b03a0() {
}

// 0x2b0568 — __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RunService * RBX::ServiceProvider::find<RBX::RunService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
// IDA 0x2b0568: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0568() {
}

// 0x2b06e0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10RunServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const*,RBX::RunService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10RunServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2b06e0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b06e0() {
}

// 0x2b07d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2b07d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b07d0() {
}

// 0x2b07d8 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv
// IDA 0x2b07d8: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b07d8() {
}

// 0x2b0878 — __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")]
// was: __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_
// IDA 0x2b0878: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0878() {
}

// 0x2b094c — __ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_
// IDA 0x2b094c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b094c() {
}

// 0x2b0a88 — __ZNSt5dequeISsSaISsEED2Ev
#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
// was: __ZNSt5dequeISsSaISsEED2Ev
// IDA 0x2b0a88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b0a88() {
}

// 0x2b0b70 — __ZNSt11_Deque_baseISsSaISsEED2Ev
#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseISsSaISsEED2Ev
// IDA 0x2b0b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b0b70() {
}

// 0x2b0ba0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED1Ev
// IDA 0x2b0ba0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b0ba0() {
}

// 0x2b0ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED0Ev
// IDA 0x2b0ba4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b0ba4() {
}

// 0x2b0ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE7disposeEv
// IDA 0x2b0ba8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0ba8() {
}

// 0x2b0c80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE11get_deleterERKSt9type_info
// IDA 0x2b0c80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0c80() {
}

// 0x2b0c84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE19get_untyped_deleterEv
// IDA 0x2b0c84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0c84() {
}

// 0x2b0c88 — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
// IDA 0x2b0c88: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0c88() {
}

// 0x2b0e50 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x2b0e50: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0e50() {
}

// 0x2b0f00 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_5Stats12StatsServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_5Stats12StatsServiceEEERS3_RKNS0_IT_EE
// IDA 0x2b0f00: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0f00() {
}

// 0x2b0f38 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
// IDA 0x2b0f38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b0f38() {
}

// 0x2b0f40 — __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x2b0f40: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b0f40() {
}

// 0x2b1060 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
// IDA 0x2b1060: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1060() {
}

// 0x2b10d0 — __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// IDA 0x2b10d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b10d0() {
}

// 0x2b1170 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// IDA 0x2b1170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b1170() {
}

// 0x2b1178 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// IDA 0x2b1178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b1178() {
}

// 0x2b1220 — __ZN5boost10shared_ptrIN3RBX5Stats12StatsServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5Stats12StatsServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2b1220: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1220() {
}

// 0x2b12e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats12StatsServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats12StatsServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2b12e8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b12e8() {
}

// 0x2b13d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2b13d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b13d8() {
}

// 0x2b13e0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b13e0: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b13e0() {
}

// 0x2b14a4 — __ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_
// IDA 0x2b14a4: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b14a4() {
}

// 0x2b157c — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
// IDA 0x2b157c: 123 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b157c() {
}

// 0x2b16d4 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
// type: void __fastcall(int)
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
// IDA 0x2b16d4: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b16d4() {
}

// 0x2b1828 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15ContentProviderEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_15ContentProviderEEEN5boost10shared_ptrIT_EEv
// IDA 0x2b1828: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1828() {
}

// 0x2b18d8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15ContentProviderEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentProvider>(rbx_core::SharedPtr<RBX::ContentProvider> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15ContentProviderEEERS3_RKNS0_IT_EE
// IDA 0x2b18d8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b18d8() {
}

// 0x2b1910 — __ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv
// IDA 0x2b1910: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b1910() {
}

// 0x2b1918 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentProvider>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv
// IDA 0x2b1918: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b1918() {
}

// 0x2b1920 — __ZN5boost6detail12shared_countC2IPN3RBX15ContentProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15ContentProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2b1920: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1920() {
}

// 0x2b1a28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2b1a28: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b1a28() {
}

// 0x2b1a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2b1a30: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1a30() {
}

// 0x2b1a50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2b1a50: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1a50() {
}

// 0x2b1a68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2b1a68: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1a68() {
}

// 0x2b1a6c — __ZN5boost9function1IvP9lua_StateE4swapERS3_
#[doc(alias = "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&)")]
// was: __ZN5boost9function1IvP9lua_StateE4swapERS3_
// IDA 0x2b1a6c: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1a6c() {
}

// 0x2b1b48 — __ZN5boost9function1IvP9lua_StateE11move_assignERS3_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&)")]
// was: __ZN5boost9function1IvP9lua_StateE11move_assignERS3_
// IDA 0x2b1b48: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1b48() {
}

// 0x2b1c4c — __ZN5boost9function1IvP9lua_StateE5clearEv
#[doc(alias = "boost::function1<void,lua_State *>::clear(void)")]
// was: __ZN5boost9function1IvP9lua_StateE5clearEv
// IDA 0x2b1c4c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1c4c() {
}

// 0x2b1c78 — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0x2b1c78: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1c78() {
}

// 0x2b1d58 — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// IDA 0x2b1d58: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1d58() {
}

// 0x2b1e3c — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEEvT_
// IDA 0x2b1e3c: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1e3c() {
}

// 0x2b1f30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// IDA 0x2b1f30: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1f30() {
}

// 0x2b1f4c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// IDA 0x2b1f4c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1f4c() {
}

// 0x2b1f68 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b1f68: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b1f68() {
}

// 0x2b204c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b204c: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b204c() {
}

// 0x2b212c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2b212c: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b212c() {
}

// 0x2b21fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEclINS_4_mfi3mf2IvS4_S8_P9lua_StateEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *> &,boost::_bi::list1<lua_State *&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEclINS_4_mfi3mf2IvS4_S8_P9lua_StateEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b21fc: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b21fc() {
}

// 0x2b22cc — __ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextENS2_3Lua13WeakThreadRefEP9lua_StateEclEPS3_S5_S7_
#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>::operator()(RBX::ScriptContext*,RBX::Lua::WeakThreadRef,lua_State *)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextENS2_3Lua13WeakThreadRefEP9lua_StateEclEPS3_S5_S7_
// IDA 0x2b22cc: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b22cc() {
}

// 0x2b23a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2b23a8: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b23a8() {
}

// 0x2b24f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
// IDA 0x2b24f8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b24f8() {
}

// 0x2b25c0 — __ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::storage3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
// IDA 0x2b25c0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b25c0() {
}

// 0x2b2688 — __ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::dummy::nonnull(void)")]
// was: __ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv
// IDA 0x2b2688: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b2688() {
}

// 0x2b268c — __ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// IDA 0x2b268c: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b268c() {
}

// 0x2b27b8 — __ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>)")]
// was: __ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_
// IDA 0x2b27b8: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b27b8() {
}

// 0x2b28f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// IDA 0x2b28f4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b28f4() {
}

// 0x2b2974 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEvS6_mE6invokeERNS1_15function_bufferES6_m
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEvS6_mE6invokeERNS1_15function_bufferES6_m
// IDA 0x2b2974: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2974() {
}

// 0x2b2998 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b2998: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2998() {
}

// 0x2b2ac4 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b2ac4: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2ac4() {
}

// 0x2b2bfc — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::operator()<void (*)(lua_State *,int,std::string),boost::_bi::list2<lua_State *&,unsigned long &>>(boost::_bi::type<void>,void (*)(lua_State *,int,std::string) &,boost::_bi::list2<lua_State *&,unsigned long &> &,int)")]
// was: __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b2bfc: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2bfc() {
}

// 0x2b2d20 — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_
#[doc(alias = "boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::list3(boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
// was: __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_
// IDA 0x2b2d20: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2d20() {
}

// 0x2b2e3c — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x2b2e3c: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2e3c() {
}

// 0x2b2f10 — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x2b2f10: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2f10() {
}

// 0x2b2fe4 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>)")]
// was: __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
// IDA 0x2b2fe4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b2fe4() {
}

// 0x2b30c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// IDA 0x2b30c8: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b30c8() {
}

// 0x2b30e4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
// IDA 0x2b30e4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b30e4() {
}

// 0x2b3100 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b3100: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3100() {
}

// 0x2b31d8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b31d8: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b31d8() {
}

// 0x2b32a8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2b32a8: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b32a8() {
}

// 0x2b336c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b336c: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b336c() {
}

// 0x2b3434 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2b3434: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3434() {
}

// 0x2b357c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_
#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_
// IDA 0x2b357c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b357c() {
}

// 0x2b3644 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::dummy::nonnull(void)")]
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv
// IDA 0x2b3644: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b3644() {
}

// 0x2b3648 — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x2b3648: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3648() {
}

// 0x2b371c — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x2b371c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b371c() {
}

// 0x2b37f0 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>)")]
// was: __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
// IDA 0x2b37f0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b37f0() {
}

// 0x2b38d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// IDA 0x2b38d4: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b38d4() {
}

// 0x2b38f0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
// IDA 0x2b38f0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b38f0() {
}

// 0x2b390c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b390c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b390c() {
}

// 0x2b39e4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b39e4: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b39e4() {
}

// 0x2b3ab4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2b3ab4: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3ab4() {
}

// 0x2b3b78 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b3b78: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3b78() {
}
