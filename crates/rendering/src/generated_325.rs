//! rendering shard 325 — 100 stubs 0x496188..0x499a84 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35420->35520 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35420 before -> 35520 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x496188 (lowest remaining 0x496188..0x499a84, next lowest 0x499aa8)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x496188 — __ZNK3RBX10DialogRoot16getDialogPurposeEv
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::getDialogPurpose(void)const")]
// was: __ZNK3RBX10DialogRoot16getDialogPurposeEv
// IDA 0x496188: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496188() {
}

// 0x49618c — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED1Ev
// IDA 0x49618c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49618c() {
}

// 0x4961b0 — __ZNK3RBX10DialogRoot13getDialogToneEv
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::getDialogTone(void)const")]
// was: __ZNK3RBX10DialogRoot13getDialogToneEv
// IDA 0x4961b0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4961b0() {
}

// 0x4961b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEED1Ev
// IDA 0x4961b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4961b4() {
}

// 0x4961d8 — __ZNK3RBX10DialogRoot23getConversationDistanceEv
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::getConversationDistance(void)const")]
// was: __ZNK3RBX10DialogRoot23getConversationDistanceEv
// IDA 0x4961d8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4961d8() {
}

// 0x4961dc — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfED1Ev
// IDA 0x4961dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4961dc() {
}

// 0x496200 — __ZNK3RBX10DialogRoot8getInUseEv
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::getInUse(void)const")]
// was: __ZNK3RBX10DialogRoot8getInUseEv
// IDA 0x496200: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496200() {
}

// 0x496208 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbED1Ev
// IDA 0x496208: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496208() {
}

// 0x49622c — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED1Ev
// IDA 0x49622c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49622c() {
}

// 0x496344 — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev
// IDA 0x496344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496344() {
}

// 0x496368 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::addPair(RBX::DialogRoot::DialogPurpose,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc
// IDA 0x496368: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496368() {
}

// 0x4966c8 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::addPair(RBX::DialogRoot::DialogTone,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc
// IDA 0x4966c8: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4966c8() {
}

// 0x496a28 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_
// IDA 0x496a28: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496a28() {
}

// 0x496bd8 — __ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)")]
// was: __ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x496bd8: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496bd8() {
}

// 0x496d48 — __ZN3RBX10DialogRootD1Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// was: __ZN3RBX10DialogRootD1Ev
// IDA 0x496d48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_496d48() {
}

// 0x496d4c — __ZN3RBX10DialogRootD0Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// was: __ZN3RBX10DialogRootD0Ev
// IDA 0x496d4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496d4c() {
}

// 0x496dec — __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv
// IDA 0x496dec: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496dec() {
}

// 0x496dfc — __ZThn32_N3RBX10DialogRootD1Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// was: __ZThn32_N3RBX10DialogRootD1Ev
// IDA 0x496dfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496dfc() {
}

// 0x496e04 — __ZThn32_N3RBX10DialogRootD0Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// was: __ZThn32_N3RBX10DialogRootD0Ev
// IDA 0x496e04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496e04() {
}

// 0x496ea8 — __ZThn32_NK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv
// IDA 0x496ea8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_496ea8() {
}

// 0x496eb8 — __ZThn36_N3RBX10DialogRootD1Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// was: __ZThn36_N3RBX10DialogRootD1Ev
// IDA 0x496eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496eb8() {
}

// 0x496ec0 — __ZThn36_N3RBX10DialogRootD0Ev
// type: void __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
// was: __ZThn36_N3RBX10DialogRootD0Ev
// IDA 0x496ec0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496ec0() {
}

// 0x496f64 — __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD1Ev
// IDA 0x496f64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_496f64() {
}

// 0x496f68 — __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD2Ev
// IDA 0x496f68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_496f68() {
}

// 0x497004 — __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator12getClassNameEv
// IDA 0x497004: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497004() {
}

// 0x49708c — __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator6createEv
// IDA 0x49708c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49708c() {
}

// 0x4971d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10DialogRootEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::Creatable<RBX::Instance>::create<RBX::DialogRoot>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10DialogRootEEEN5boost10shared_ptrIT_EEv
// IDA 0x4971d0: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4971d0() {
}

// 0x497280 — __ZN5boost10shared_ptrIN3RBX10DialogRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot>::shared_ptr<RBX::DialogRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10DialogRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x497280: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497280() {
}

// 0x497430 — __ZN5boost6detail12shared_countC2IPN3RBX10DialogRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10DialogRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x497430: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497430() {
}

// 0x497538 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x497538: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_497538() {
}

// 0x49753c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x49753c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49753c() {
}

// 0x497540 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x497540: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497540() {
}

// 0x497560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x497560: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497560() {
}

// 0x497578 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x497578: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497578() {
}

// 0x49757c — __ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv
// IDA 0x49757c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49757c() {
}

// 0x497580 — __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v
// IDA 0x497580: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497580() {
}

// 0x497660 — __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorC2Ev
// IDA 0x497660: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497660() {
}

// 0x4978a4 — __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E17static_getCreatorEv
// IDA 0x4978a4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4978a4() {
}

// 0x497918 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_
// IDA 0x497918: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497918() {
}

// 0x497a38 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_
// IDA 0x497a38: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497a38() {
}

// 0x497ba4 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev
// IDA 0x497ba4: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497ba4() {
}

// 0x497d00 — __ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x497d00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_497d00() {
}

// 0x497d04 — __ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x497d04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_497d04() {
}

// 0x497da4 — __ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x497da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_497da4() {
}

// 0x497dac — __ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x497dac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_497dac() {
}

// 0x497e50 — __ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x497e50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_497e50() {
}

// 0x497e58 — __ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x497e58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_497e58() {
}

// 0x497efc — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_
// IDA 0x497efc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497efc() {
}

// 0x497f30 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_
// IDA 0x497f30: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_497f30() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x497f58 — __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x497f58: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497f58() {
}

// 0x497fb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x497fb0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497fb0() {
}

// 0x498064 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x498064: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498064() {
}

// 0x4980bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x4980bc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4980bc() {
}

// 0x498124 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x498124: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_498124() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x498208 — __ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm
// IDA 0x498208: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_498208() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x498220 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_
// IDA 0x498220: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_498220() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49825c — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49825c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49825c() {
}

// 0x4983ec — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_
// IDA 0x4983ec: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4983ec() {
}

// 0x498420 — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_
// IDA 0x498420: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_498420() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x498448 — __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x498448: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498448() {
}

// 0x4984a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x4984a0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4984a0() {
}

// 0x498554 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x498554: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498554() {
}

// 0x4985ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x4985ac: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4985ac() {
}

// 0x498614 — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x498614: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_498614() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x4986f8 — __ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm
// IDA 0x4986f8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_4986f8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x498710 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_
// IDA 0x498710: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_498710() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49874c — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)")]
// was: __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x49874c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49874c() {
}

// 0x4988dc — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev
// IDA 0x4988dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4988dc() {
}

// 0x498990 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x498990: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498990() {
}

// 0x498af4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv
// IDA 0x498af4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498af4() {
}

// 0x498afc — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv
// IDA 0x498afc: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498afc() {
}

// 0x498b04 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x498b04: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498b04() {
}

// 0x498cb4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// IDA 0x498cb4: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498cb4() {
}

// 0x498cc4 — __ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x498cc4: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498cc4() {
}

// 0x498cd8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x498cd8: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498cd8() {
}

// 0x498ec8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// IDA 0x498ec8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_498ec8() {
}

// 0x498eec — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// IDA 0x498eec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_498eec() {
}

// 0x498fa0 — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x498fa0: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_498fa0() {
}

// 0x499170 — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// IDA 0x499170: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499170() {
}

// 0x4991bc — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED0Ev
// IDA 0x4991bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4991bc() {
}

// 0x4992e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x4992e4: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4992e4() {
}

// 0x499408 — __ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_
// IDA 0x499408: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499408() {
}

// 0x499540 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::PropDescriptor<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>(char const*,char const*,bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x499540: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499540() {
}

// 0x499654 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbED0Ev
// IDA 0x499654: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_499654() {
}

// 0x499680 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x499680: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499680() {
}

// 0x499684 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x499684: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499684() {
}

// 0x499688 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x499688: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499688() {
}

// 0x4996ac — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x4996ac: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4996ac() {
}

// 0x4996d0 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::PropDescriptor<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>(char const*,char const*,float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x4996d0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4996d0() {
}

// 0x4997e4 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfED0Ev
// IDA 0x4997e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4997e4() {
}

// 0x499810 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x499810: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499810() {
}

// 0x499814 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x499814: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499814() {
}

// 0x499818 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x499818: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499818() {
}

// 0x499838 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x499838: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499838() {
}

// 0x49985c — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::EnumPropDescriptor<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>(char const*,char const*,RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x49985c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49985c() {
}

// 0x499a10 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEED0Ev
// IDA 0x499a10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_499a10() {
}

// 0x499a3c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10isReadOnlyEv
// IDA 0x499a3c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499a3c() {
}

// 0x499a4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11isWriteOnlyEv
// IDA 0x499a4c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499a4c() {
}

// 0x499a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x499a5c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499a5c() {
}

// 0x499a84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x499a84: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_499a84() {
}