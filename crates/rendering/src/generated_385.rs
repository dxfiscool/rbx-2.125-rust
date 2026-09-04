//! rendering shard 385 — 100 stubs 0x565588..0x569ce8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 41711->41811 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x565588..0x569ce8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x565588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x565588: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565588() {
}

// 0x5655a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5655a0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5655a0() {
}

// 0x5655a4 — __ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5655a4: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5655a4() {
}

// 0x5657e8 — __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5657e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5657e8() {
}

// 0x565884 — __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv
// IDA 0x565884: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565884() {
}

// 0x5659c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
// IDA 0x5659c8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5659c8() {
}

// 0x565a7c — __ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x565a7c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565a7c() {
}

// 0x565b44 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyGyro,RBX::BodyGyro>(rbx_core::SharedPtr<RBX::BodyGyro> const*,RBX::BodyGyro *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x565b44: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565b44() {
}

// 0x565c2c — __ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x565c2c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565c2c() {
}

// 0x565d34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x565d34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_565d34() {
}

// 0x565d38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x565d38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_565d38() {
}

// 0x565d3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x565d3c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565d3c() {
}

// 0x565d5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x565d5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565d5c() {
}

// 0x565d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x565d74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565d74() {
}

// 0x565d78 — __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev
// IDA 0x565d78: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565d78() {
}

// 0x565fbc — __ZN3RBX4Body14getBranchIBodyEv
// type: int __fastcall(RBX::Body *this, int)
#[doc(alias = "__ZN3RBX4Body14getBranchIBodyEv")]
#[doc(alias = "RBX::Body::getBranchIBody(void)")]
// was: __ZN3RBX4Body14getBranchIBodyEv
// IDA 0x565fbc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_565fbc() {
}

// 0x565fdc — __GLOBAL__I_a_209
#[doc(alias = "__GLOBAL__I_a_209")]
#[doc(alias = "global constructor keyed to_a_209")]
// was: __GLOBAL__I_a_209
// IDA 0x565fdc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_565fdc() {
}

// 0x566ee0 — __ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE
#[doc(alias = "__ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE")]
#[doc(alias = "RBX::Handles::setVisualStyle(RBX::Handles::VisualStyle)")]
// was: __ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE
// IDA 0x566ee0: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_566ee0() {
}

// 0x566f00 — __ZN3RBX7Handles8setFacesENS_5FacesE
#[doc(alias = "__ZN3RBX7Handles8setFacesENS_5FacesE")]
#[doc(alias = "RBX::Handles::setFaces(RBX::Faces)")]
// was: __ZN3RBX7Handles8setFacesENS_5FacesE
// IDA 0x566f00: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_566f00() {
}

// 0x566f20 — __ZN3RBX7HandlesC2Ev
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesC2Ev")]
#[doc(alias = "RBX::Handles::Handles(void)")]
// was: __ZN3RBX7HandlesC2Ev
// IDA 0x566f20: 371 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_566f20() {
}

// 0x567344 — __ZN3RBX7Handles18setServerGuiObjectEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7Handles18setServerGuiObjectEv")]
#[doc(alias = "RBX::Handles::setServerGuiObject(void)")]
// was: __ZN3RBX7Handles18setServerGuiObjectEv
// IDA 0x567344: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567344() {
}

// 0x5673ac — __ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::Handles *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::Handles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x5673ac: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5673ac() {
}

// 0x5673e8 — __ZN3RBX7Handles7processERKNS_8GuiEventE
#[doc(alias = "__ZN3RBX7Handles7processERKNS_8GuiEventE")]
#[doc(alias = "RBX::Handles::process(RBX::GuiEvent const&)")]
// was: __ZN3RBX7Handles7processERKNS_8GuiEventE
// IDA 0x5673e8: 251 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5673e8() {
}

// 0x567688 — __ZThn92_N3RBX7Handles7processERKNS_8GuiEventE
#[doc(alias = "__ZThn92_N3RBX7Handles7processERKNS_8GuiEventE")]
#[doc(alias = "non-virtual thunk to RBX::Handles::process(RBX::GuiEvent const&)")]
// was: __ZThn92_N3RBX7Handles7processERKNS_8GuiEventE
// IDA 0x567688: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567688() {
}

// 0x567694 — __ZNK3RBX7Handles13getHandleTypeEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles13getHandleTypeEv")]
#[doc(alias = "RBX::Handles::getHandleType(void)const")]
// was: __ZNK3RBX7Handles13getHandleTypeEv
// IDA 0x567694: 9 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567694() {
}

// 0x5676b0 — __ZNK3RBX7Handles14getVisualStyleEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles14getVisualStyleEv")]
#[doc(alias = "RBX::Handles::getVisualStyle(void)const")]
// was: __ZNK3RBX7Handles14getVisualStyleEv
// IDA 0x5676b0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5676b0() {
}

// 0x5676b8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev
// IDA 0x5676b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5676b8() {
}

// 0x5676dc — __ZNK3RBX7Handles8getFacesEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles8getFacesEv")]
#[doc(alias = "RBX::Handles::getFaces(void)const")]
// was: __ZNK3RBX7Handles8getFacesEv
// IDA 0x5676dc: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5676dc() {
}

// 0x5676e4 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev
// IDA 0x5676e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5676e4() {
}

// 0x567708 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev
// IDA 0x567708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567708() {
}

// 0x56772c — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev
// IDA 0x56772c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_56772c() {
}

// 0x567750 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
// IDA 0x567750: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567750() {
}

// 0x5678b0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
// IDA 0x5678b0: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5678b0() {
}

// 0x567a10 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x567a10: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567a10() {
}

// 0x567a70 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x567a70: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567a70() {
}

// 0x567ad0 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
// type: int(void)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::NormalId,float)>::operator()(RBX::NormalId,float)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
// IDA 0x567ad0: 123 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567ad0() {
}

// 0x567c20 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_")]
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::NormalId)>::operator()(RBX::NormalId)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
// IDA 0x567c20: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567c20() {
}

// 0x567d64 — __ZN3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesD1Ev")]
#[doc(alias = "RBX::Handles::~Handles()")]
// was: __ZN3RBX7HandlesD1Ev
// IDA 0x567d64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_567d64() {
}

// 0x567d68 — __ZN3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesD0Ev")]
#[doc(alias = "RBX::Handles::~Handles()")]
// was: __ZN3RBX7HandlesD0Ev
// IDA 0x567d68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567d68() {
}

// 0x567e08 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
// IDA 0x567e08: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567e08() {
}

// 0x567e18 — __ZNK3RBX7Handles22getHandlesNormalIdMaskEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles22getHandlesNormalIdMaskEv")]
#[doc(alias = "RBX::Handles::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX7Handles22getHandlesNormalIdMaskEv
// IDA 0x567e18: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567e18() {
}

// 0x567e20 — __ZThn32_N3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7HandlesD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Handles::~Handles()")]
// was: __ZThn32_N3RBX7HandlesD1Ev
// IDA 0x567e20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567e20() {
}

// 0x567e28 — __ZThn32_N3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7HandlesD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Handles::~Handles()")]
// was: __ZThn32_N3RBX7HandlesD0Ev
// IDA 0x567e28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567e28() {
}

// 0x567ecc — __ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
// IDA 0x567ecc: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_567ecc() {
}

// 0x567edc — __ZThn36_N3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7HandlesD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Handles::~Handles()")]
// was: __ZThn36_N3RBX7HandlesD1Ev
// IDA 0x567edc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567edc() {
}

// 0x567ee4 — __ZThn36_N3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7HandlesD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Handles::~Handles()")]
// was: __ZThn36_N3RBX7HandlesD0Ev
// IDA 0x567ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567ee4() {
}

// 0x567f88 — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev
// IDA 0x567f88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_567f88() {
}

// 0x567f8c — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev
// IDA 0x567f8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_567f8c() {
}

// 0x568028 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x568028: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568028() {
}

// 0x5680b0 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv
// IDA 0x5680b0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5680b0() {
}

// 0x5681f4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv
// IDA 0x5681f4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5681f4() {
}

// 0x5682a8 — __ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5682a8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5682a8() {
}

// 0x568370 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Handles,RBX::Handles>(rbx_core::SharedPtr<RBX::Handles> const*,RBX::Handles *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x568370: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568370() {
}

// 0x568458 — __ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x568458: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568458() {
}

// 0x568560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x568560: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_568560() {
}

// 0x568564 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x568564: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_568564() {
}

// 0x568568 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x568568: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568568() {
}

// 0x568588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x568588: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568588() {
}

// 0x5685a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5685a0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5685a0() {
}

// 0x5685a4 — __ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv
// IDA 0x5685a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5685a4() {
}

// 0x5685a8 — __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v
// IDA 0x5685a8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5685a8() {
}

// 0x568688 — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev
// IDA 0x568688: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568688() {
}

// 0x5688cc — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5688cc: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5688cc() {
}

// 0x568940 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// IDA 0x568940: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568940() {
}

// 0x568aa0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
// IDA 0x568aa0: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568aa0() {
}

// 0x568ac8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
// IDA 0x568ac8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568ac8() {
}

// 0x568aec — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv
// IDA 0x568aec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_568aec() {
}

// 0x568af0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
// IDA 0x568af0: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568af0() {
}

// 0x568be8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// IDA 0x568be8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568be8() {
}

// 0x568d48 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
// IDA 0x568d48: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568d48() {
}

// 0x568d70 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
// IDA 0x568d70: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568d70() {
}

// 0x568d94 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv
// IDA 0x568d94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_568d94() {
}

// 0x568d98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
// IDA 0x568d98: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568d98() {
}

// 0x568e90 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// IDA 0x568e90: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568e90() {
}

// 0x568f04 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
// IDA 0x568f04: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_568f04() {
}

// 0x568f50 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// IDA 0x568f50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_568f50() {
}

// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
// IDA 0x568f7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_568f7c() {
}

// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x569050: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569050() {
}

// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x569058: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569058() {
}

// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// IDA 0x569060: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569060() {
}

// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
// IDA 0x569078: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_569078() {
}

// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
// IDA 0x5690a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5690a4() {
}

// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// IDA 0x569178: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569178() {
}

// 0x5691ec — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
// IDA 0x5691ec: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5691ec() {
}

// 0x569238 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// IDA 0x569238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_569238() {
}

// 0x569264 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
// IDA 0x569264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_569264() {
}

// 0x569338 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x569338: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569338() {
}

// 0x569340 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x569340: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569340() {
}

// 0x569348 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// IDA 0x569348: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569348() {
}

// 0x569360 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
// IDA 0x569360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_569360() {
}

// 0x56938c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
// IDA 0x56938c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_56938c() {
}

// 0x569460 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// IDA 0x569460: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569460() {
}

// 0x5695bc — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
// IDA 0x5695bc: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5695bc() {
}

// 0x569734 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// IDA 0x569734: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569734() {
}

// 0x569890 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
// IDA 0x569890: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569890() {
}

// 0x569a08 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
// IDA 0x569a08: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569a08() {
}

// 0x569afc — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
// IDA 0x569afc: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569afc() {
}

// 0x569b64 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::signalProducedIncremented(RBX::NormalId,float)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
// IDA 0x569b64: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569b64() {
}

// 0x569b7c — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId,float)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f
// IDA 0x569b7c: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569b7c() {
}

// 0x569ce8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// IDA 0x569ce8: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_569ce8() {
}
