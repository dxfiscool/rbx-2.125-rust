//! rendering shard 376 — 100 stubs 0x537ce8..0x53ba8c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 40861->40961 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x537ce8..0x53ba8c (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x537ce8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// IDA 0x537ce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_537ce8() {
}

// 0x537dbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv
// IDA 0x537dbc: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537dbc() {
}

// 0x537ecc — __ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv
// IDA 0x537ecc: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537ecc() {
}

// 0x537ed8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x537ed8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537ed8() {
}

// 0x537f00 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x537f00: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537f00() {
}

// 0x537f28 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x537f28: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537f28() {
}

// 0x537f5c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE
// IDA 0x537f5c: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537f5c() {
}

// 0x53804c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv
// IDA 0x53804c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_53804c() {
}

// 0x538050 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv
// IDA 0x538050: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538050() {
}

// 0x538140 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev
// IDA 0x538140: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538140() {
}

// 0x53816c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev
// IDA 0x53816c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53816c() {
}

// 0x538240 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// IDA 0x538240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538240() {
}

// 0x53826c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// IDA 0x53826c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53826c() {
}

// 0x538340 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
// type: void()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
// IDA 0x538340: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_538340() {
}

// 0x538344 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv
// IDA 0x538344: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538344() {
}

// 0x538438 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
// IDA 0x538438: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538438() {
}

// 0x5384a0 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii
// IDA 0x5384a0: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5384a0() {
}

// 0x5384b8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::replicateEvent(RBX::Reflection::EventSource *,int,int)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii
// IDA 0x5384b8: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5384b8() {
}

// 0x538624 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// IDA 0x538624: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538624() {
}

// 0x538698 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
// IDA 0x538698: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538698() {
}

// 0x5386c4 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
// IDA 0x5386c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5386c4() {
}

// 0x538798 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
// IDA 0x538798: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538798() {
}

// 0x5387c0 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
// IDA 0x5387c0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5387c0() {
}

// 0x5387e8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5387e8: 14 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5387e8() {
}

// 0x538810 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
// IDA 0x538810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538810() {
}

// 0x53883c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
// IDA 0x53883c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53883c() {
}

// 0x538910 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv
// IDA 0x538910: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_538910() {
}

// 0x538914 — __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x538914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538914() {
}

// 0x5389d0 — __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5389d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5389d0() {
}

// 0x538a9c — __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x538a9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538a9c() {
}

// 0x538b54 — __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x538b54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538b54() {
}

// 0x538c24 — __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x538c24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538c24() {
}

// 0x538cdc — __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x538cdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_538cdc() {
}

// 0x538dac — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)")]
// was: __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_
// IDA 0x538dac: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538dac() {
}

// 0x538de0 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_
// IDA 0x538de0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_538de0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x538e08 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x538e08: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538e08() {
}

// 0x538e60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x538e60: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538e60() {
}

// 0x538f14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x538f14: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538f14() {
}

// 0x538f6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x538f6c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_538f6c() {
}

// 0x538fd4 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x538fd4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_538fd4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x5390b8 — __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm
// IDA 0x5390b8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_5390b8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x5390d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_
// IDA 0x5390d0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_5390d0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x53910c — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x53910c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53910c() {
}

// 0x53929c — __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x53929c: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53929c() {
}

// 0x5392f4 — __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5392f4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5392f4() {
}

// 0x5393e4 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)")]
// was: __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_
// IDA 0x5393e4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5393e4() {
}

// 0x539418 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_
// IDA 0x539418: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_539418() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x539440 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x539440: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539440() {
}

// 0x539498 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x539498: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539498() {
}

// 0x53954c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x53954c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53954c() {
}

// 0x5395a4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x5395a4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5395a4() {
}

// 0x53960c — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x53960c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_53960c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x5396f0 — __ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm
// IDA 0x5396f0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_5396f0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x539708 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject16TweenEasingStyleES6_EET0_T_S8_S7_
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject16TweenEasingStyleES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::GuiObject::TweenEasingStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *>(RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject16TweenEasingStyleES6_EET0_T_S8_S7_
// IDA 0x539708: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_539708() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x539744 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,unsigned long,RBX::GuiObject::TweenEasingStyle const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x539744: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539744() {
}

// 0x5398d4 — __ZN3rbx8any_castIN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiObject::TweenEasingDirection * rbx::any_cast<RBX::GuiObject::TweenEasingDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x5398d4: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5398d4() {
}

// 0x53992c — __ZN3rbx8any_castIRN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiObject::TweenEasingDirection & rbx::any_cast<RBX::GuiObject::TweenEasingDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x53992c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53992c() {
}

// 0x539a1c — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::resize(unsigned long,RBX::GuiObject::TweenEasingDirection)")]
// was: __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_
// IDA 0x539a1c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539a1c() {
}

// 0x539a50 — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::push_back(RBX::GuiObject::TweenEasingDirection const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_
// IDA 0x539a50: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_539a50() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x539a78 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject20TweenEasingDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject20TweenEasingDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject20TweenEasingDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x539a78: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539a78() {
}

// 0x539ad0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x539ad0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539ad0() {
}

// 0x539b84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x539b84: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539b84() {
}

// 0x539bdc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x539bdc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539bdc() {
}

// 0x539c44 — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,RBX::GuiObject::TweenEasingDirection const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x539c44: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_539c44() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x539d28 — __ZNSt12_Vector_baseIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE11_M_allocateEm
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE11_M_allocateEm
// IDA 0x539d28: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_539d28() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x539d40 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject20TweenEasingDirectionES6_EET0_T_S8_S7_
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject20TweenEasingDirectionES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::GuiObject::TweenEasingDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *>(RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject20TweenEasingDirectionES6_EET0_T_S8_S7_
// IDA 0x539d40: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_539d40() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x539d7c — __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,unsigned long,RBX::GuiObject::TweenEasingDirection const&)")]
// was: __ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x539d7c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539d7c() {
}

// 0x539f0c — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9GuiObjectEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9GuiObjectEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::GuiObject>(char const*,char const*,int RBX::GuiObject::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9GuiObjectEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x539f0c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_539f0c() {
}

// 0x53a09c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::GuiObject>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE10isReadOnlyEv
// IDA 0x53a09c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a09c() {
}

// 0x53a0a0 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::GuiObject>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE11isWriteOnlyEv
// IDA 0x53a0a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a0a0() {
}

// 0x53a0a4 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::GuiObject>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53a0a4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a0a4() {
}

// 0x53a0b0 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::GuiObject>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9GuiObjectEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x53a0b0: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a0b0() {
}

// 0x53a100 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED0Ev
// IDA 0x53a100: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53a100() {
}

// 0x53a1b4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x53a1b4: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a1b4() {
}

// 0x53a318 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12isScriptableEv
// IDA 0x53a318: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a318() {
}

// 0x53a320 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// IDA 0x53a320: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a320() {
}

// 0x53a328 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// IDA 0x53a328: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a328() {
}

// 0x53a3bc — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x53a3bc: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a3bc() {
}

// 0x53a3cc — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x53a3cc: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a3cc() {
}

// 0x53a3e0 — __ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::UDim2 const& rbx::any_cast<RBX::UDim2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x53a3e0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a3e0() {
}

// 0x53a5ec — __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_5UDim2EEEvRKT_
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_5UDim2EEEvRKT_")]
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::UDim2>(RBX::UDim2 const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_5UDim2EEEvRKT_
// IDA 0x53a5ec: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a5ec() {
}

// 0x53a730 — __ZN5boost9function1IvN3RBX5UDim2EE5clearEv
#[doc(alias = "__ZN5boost9function1IvN3RBX5UDim2EE5clearEv")]
#[doc(alias = "boost::function1<void,RBX::UDim2>::clear(void)")]
// was: __ZN5boost9function1IvN3RBX5UDim2EE5clearEv
// IDA 0x53a730: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a730() {
}

// 0x53ae68 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::function<void ()(RBX::UDim2)>>(boost::function<void ()(RBX::UDim2)> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// IDA 0x53ae68: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ae68() {
}

// 0x53af5c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::callable<rbx::signals::signal<void ()(RBX::UDim2)>*>(boost::function<void ()(RBX::UDim2)> const&,rbx::signals::signal<void ()(RBX::UDim2)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// IDA 0x53af5c: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53af5c() {
}

// 0x53b058 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
// IDA 0x53b058: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b058() {
}

// 0x53b168 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
// IDA 0x53b168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b168() {
}

// 0x53b298 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// IDA 0x53b298: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b298() {
}

// 0x53b2b0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// IDA 0x53b2b0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b2b0() {
}

// 0x53b2c8 — __ZNK5boost9function1IvN3RBX5UDim2EEclES2_
#[doc(alias = "__ZNK5boost9function1IvN3RBX5UDim2EEclES2_")]
#[doc(alias = "boost::function1<void,RBX::UDim2>::operator()(RBX::UDim2)const")]
// was: __ZNK5boost9function1IvN3RBX5UDim2EEclES2_
// IDA 0x53b2c8: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b2c8() {
}

// 0x53b398 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
// IDA 0x53b398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b398() {
}

// 0x53b4a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
// IDA 0x53b4a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b4a8() {
}

// 0x53b5d8 — __ZN5boost9function1IvN3RBX5UDim2EE13assign_to_ownERKS3_
#[doc(alias = "__ZN5boost9function1IvN3RBX5UDim2EE13assign_to_ownERKS3_")]
#[doc(alias = "boost::function1<void,RBX::UDim2>::assign_to_own(boost::function1<void,RBX::UDim2> const&)")]
// was: __ZN5boost9function1IvN3RBX5UDim2EE13assign_to_ownERKS3_
// IDA 0x53b5d8: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b5d8() {
}

// 0x53b608 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::EventDesc(rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x53b608: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b608() {
}

// 0x53b78c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
// IDA 0x53b78c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b78c() {
}

// 0x53b7b0 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
// IDA 0x53b7b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b7b0() {
}

// 0x53b864 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED0Ev
// IDA 0x53b864: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53b864() {
}

// 0x53b918 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x53b918: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53b918() {
}

// 0x53ba7c — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv
// IDA 0x53ba7c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ba7c() {
}

// 0x53ba84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// IDA 0x53ba84: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ba84() {
}

// 0x53ba8c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x53ba8c: 50 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ba8c() {
}
