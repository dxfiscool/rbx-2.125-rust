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
pub fn stub_565588() -> ! {
    todo!("0x565588 boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5655a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyPositionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5655a0() -> ! {
    todo!("0x5655a0 boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5655a4 — __ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5655a4() -> ! {
    todo!("0x5655a4 __ZN3RBX14FactoryProductINS_12BodyPositionENS_9BodyMoverELZNS_13sBodyPositionEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5657e8 — __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5657e8() -> ! {
    todo!("0x5657e8 __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorD2Ev")
}

// 0x565884 — __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv
pub fn stub_565884() -> ! {
    todo!("0x565884 __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7Creator6createEv")
}

// 0x5659c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BodyGyroEEEN5boost10shared_ptrIT_EEv
pub fn stub_5659c8() -> ! {
    todo!("0x5659c8 boost::shared_ptr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")
}

// 0x565a7c — __ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8BodyGyroEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_565a7c() -> ! {
    todo!("0x565a7c boost::shared_ptr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x565b44 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyGyro,RBX::BodyGyro>(rbx_core::SharedPtr<RBX::BodyGyro> const*,RBX::BodyGyro *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BodyGyroES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_565b44() -> ! {
    todo!("0x565b44 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyGyro,RBX::BodyGyro>(boost::shared_ptr<RBX::BodyGyro> const*,RBX::BodyGyro *)const")
}

// 0x565c2c — __ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8BodyGyroENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_565c2c() -> ! {
    todo!("0x565c2c boost::detail::shared_count::shared_count<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x565d34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_565d34() -> ! {
    todo!("0x565d34 boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x565d38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_565d38() -> ! {
    todo!("0x565d38 boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x565d3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_565d3c() -> ! {
    todo!("0x565d3c boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x565d5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_565d5c() -> ! {
    todo!("0x565d5c boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x565d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8BodyGyroENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_565d74() -> ! {
    todo!("0x565d74 boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x565d78 — __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev
pub fn stub_565d78() -> ! {
    todo!("0x565d78 __ZN3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE7CreatorC2Ev")
}

// 0x565fbc — __ZN3RBX4Body14getBranchIBodyEv
// type: int __fastcall(RBX::Body *this, int)
#[doc(alias = "__ZN3RBX4Body14getBranchIBodyEv")]
#[doc(alias = "RBX::Body::getBranchIBody(void)")]
// was: __ZN3RBX4Body14getBranchIBodyEv
pub fn stub_565fbc() -> ! {
    todo!("0x565fbc RBX::Body::getBranchIBody(void)")
}

// 0x565fdc — __GLOBAL__I_a_209
#[doc(alias = "__GLOBAL__I_a_209")]
#[doc(alias = "global constructor keyed to_a_209")]
// was: __GLOBAL__I_a_209
pub fn stub_565fdc() -> ! {
    todo!("0x565fdc `global constructor keyed to'_a_209")
}

// 0x566ee0 — __ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE
#[doc(alias = "__ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE")]
#[doc(alias = "RBX::Handles::setVisualStyle(RBX::Handles::VisualStyle)")]
// was: __ZN3RBX7Handles14setVisualStyleENS0_11VisualStyleE
pub fn stub_566ee0() -> ! {
    todo!("0x566ee0 RBX::Handles::setVisualStyle(RBX::Handles::VisualStyle)")
}

// 0x566f00 — __ZN3RBX7Handles8setFacesENS_5FacesE
#[doc(alias = "__ZN3RBX7Handles8setFacesENS_5FacesE")]
#[doc(alias = "RBX::Handles::setFaces(RBX::Faces)")]
// was: __ZN3RBX7Handles8setFacesENS_5FacesE
pub fn stub_566f00() -> ! {
    todo!("0x566f00 RBX::Handles::setFaces(RBX::Faces)")
}

// 0x566f20 — __ZN3RBX7HandlesC2Ev
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesC2Ev")]
#[doc(alias = "RBX::Handles::Handles(void)")]
// was: __ZN3RBX7HandlesC2Ev
pub fn stub_566f20() -> ! {
    todo!("0x566f20 RBX::Handles::Handles(void)")
}

// 0x567344 — __ZN3RBX7Handles18setServerGuiObjectEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7Handles18setServerGuiObjectEv")]
#[doc(alias = "RBX::Handles::setServerGuiObject(void)")]
// was: __ZN3RBX7Handles18setServerGuiObjectEv
pub fn stub_567344() -> ! {
    todo!("0x567344 RBX::Handles::setServerGuiObject(void)")
}

// 0x5673ac — __ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::Handles *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::Handles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX7Handles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_5673ac() -> ! {
    todo!("0x5673ac RBX::Handles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x5673e8 — __ZN3RBX7Handles7processERKNS_8GuiEventE
#[doc(alias = "__ZN3RBX7Handles7processERKNS_8GuiEventE")]
#[doc(alias = "RBX::Handles::process(RBX::GuiEvent const&)")]
// was: __ZN3RBX7Handles7processERKNS_8GuiEventE
pub fn stub_5673e8() -> ! {
    todo!("0x5673e8 RBX::Handles::process(RBX::GuiEvent const&)")
}

// 0x567688 — __ZThn92_N3RBX7Handles7processERKNS_8GuiEventE
#[doc(alias = "__ZThn92_N3RBX7Handles7processERKNS_8GuiEventE")]
#[doc(alias = "non-virtual thunk toRBX::Handles::process(RBX::GuiEvent const&)")]
// was: __ZThn92_N3RBX7Handles7processERKNS_8GuiEventE
pub fn stub_567688() -> ! {
    todo!("0x567688 `non-virtual thunk to'RBX::Handles::process(RBX::GuiEvent const&)")
}

// 0x567694 — __ZNK3RBX7Handles13getHandleTypeEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles13getHandleTypeEv")]
#[doc(alias = "RBX::Handles::getHandleType(void)const")]
// was: __ZNK3RBX7Handles13getHandleTypeEv
pub fn stub_567694() -> ! {
    todo!("0x567694 RBX::Handles::getHandleType(void)const")
}

// 0x5676b0 — __ZNK3RBX7Handles14getVisualStyleEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles14getVisualStyleEv")]
#[doc(alias = "RBX::Handles::getVisualStyle(void)const")]
// was: __ZNK3RBX7Handles14getVisualStyleEv
pub fn stub_5676b0() -> ! {
    todo!("0x5676b0 RBX::Handles::getVisualStyle(void)const")
}

// 0x5676b8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED1Ev
pub fn stub_5676b8() -> ! {
    todo!("0x5676b8 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")
}

// 0x5676dc — __ZNK3RBX7Handles8getFacesEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles8getFacesEv")]
#[doc(alias = "RBX::Handles::getFaces(void)const")]
// was: __ZNK3RBX7Handles8getFacesEv
pub fn stub_5676dc() -> ! {
    todo!("0x5676dc RBX::Handles::getFaces(void)const")
}

// 0x5676e4 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED1Ev
pub fn stub_5676e4() -> ! {
    todo!("0x5676e4 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")
}

// 0x567708 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev
pub fn stub_567708() -> ! {
    todo!("0x567708 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")
}

// 0x56772c — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev
pub fn stub_56772c() -> ! {
    todo!("0x56772c RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")
}

// 0x567750 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
pub fn stub_567750() -> ! {
    todo!("0x567750 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")
}

// 0x5678b0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
pub fn stub_5678b0() -> ! {
    todo!("0x5678b0 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")
}

// 0x567a10 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_567a10() -> ! {
    todo!("0x567a10 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x567a70 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_567a70() -> ! {
    todo!("0x567a70 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x567ad0 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
// type: int(void)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::NormalId,float)>::operator()(RBX::NormalId,float)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
pub fn stub_567ad0() -> ! {
    todo!("0x567ad0 rbx::signals::signal_with_args<2,void ()(RBX::NormalId,float)>::operator()(RBX::NormalId,float)")
}

// 0x567c20 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_")]
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::NormalId)>::operator()(RBX::NormalId)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
pub fn stub_567c20() -> ! {
    todo!("0x567c20 rbx::signals::signal_with_args<1,void ()(RBX::NormalId)>::operator()(RBX::NormalId)")
}

// 0x567d64 — __ZN3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesD1Ev")]
#[doc(alias = "RBX::Handles::~Handles()")]
// was: __ZN3RBX7HandlesD1Ev
pub fn stub_567d64() -> ! {
    todo!("0x567d64 RBX::Handles::~Handles()")
}

// 0x567d68 — __ZN3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesD0Ev")]
#[doc(alias = "RBX::Handles::~Handles()")]
// was: __ZN3RBX7HandlesD0Ev
pub fn stub_567d68() -> ! {
    todo!("0x567d68 RBX::Handles::~Handles()")
}

// 0x567e08 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
pub fn stub_567e08() -> ! {
    todo!("0x567e08 __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")
}

// 0x567e18 — __ZNK3RBX7Handles22getHandlesNormalIdMaskEv
// type: _DWORD __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZNK3RBX7Handles22getHandlesNormalIdMaskEv")]
#[doc(alias = "RBX::Handles::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX7Handles22getHandlesNormalIdMaskEv
pub fn stub_567e18() -> ! {
    todo!("0x567e18 RBX::Handles::getHandlesNormalIdMask(void)const")
}

// 0x567e20 — __ZThn32_N3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7HandlesD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Handles::~Handles()")]
// was: __ZThn32_N3RBX7HandlesD1Ev
pub fn stub_567e20() -> ! {
    todo!("0x567e20 `non-virtual thunk to'RBX::Handles::~Handles()")
}

// 0x567e28 — __ZThn32_N3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7HandlesD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Handles::~Handles()")]
// was: __ZThn32_N3RBX7HandlesD0Ev
pub fn stub_567e28() -> ! {
    todo!("0x567e28 `non-virtual thunk to'RBX::Handles::~Handles()")
}

// 0x567ecc — __ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv
pub fn stub_567ecc() -> ! {
    todo!("0x567ecc __ZThn32_NK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE12getClassNameEv")
}

// 0x567edc — __ZThn36_N3RBX7HandlesD1Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7HandlesD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Handles::~Handles()")]
// was: __ZThn36_N3RBX7HandlesD1Ev
pub fn stub_567edc() -> ! {
    todo!("0x567edc `non-virtual thunk to'RBX::Handles::~Handles()")
}

// 0x567ee4 — __ZThn36_N3RBX7HandlesD0Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7HandlesD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Handles::~Handles()")]
// was: __ZThn36_N3RBX7HandlesD0Ev
pub fn stub_567ee4() -> ! {
    todo!("0x567ee4 `non-virtual thunk to'RBX::Handles::~Handles()")
}

// 0x567f88 — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev
pub fn stub_567f88() -> ! {
    todo!("0x567f88 __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD1Ev")
}

// 0x567f8c — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev
pub fn stub_567f8c() -> ! {
    todo!("0x567f8c __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorD2Ev")
}

// 0x568028 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_568028() -> ! {
    todo!("0x568028 __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5680b0 — __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv
pub fn stub_5680b0() -> ! {
    todo!("0x5680b0 __ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv")
}

// 0x5681f4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7HandlesEEEN5boost10shared_ptrIT_EEv
pub fn stub_5681f4() -> ! {
    todo!("0x5681f4 boost::shared_ptr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)")
}

// 0x5682a8 — __ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7HandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5682a8() -> ! {
    todo!("0x5682a8 boost::shared_ptr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x568370 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Handles,RBX::Handles>(rbx_core::SharedPtr<RBX::Handles> const*,RBX::Handles *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7HandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_568370() -> ! {
    todo!("0x568370 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Handles,RBX::Handles>(boost::shared_ptr<RBX::Handles> const*,RBX::Handles *)const")
}

// 0x568458 — __ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7HandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_568458() -> ! {
    todo!("0x568458 boost::detail::shared_count::shared_count<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x568560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_568560() -> ! {
    todo!("0x568560 boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x568564 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_568564() -> ! {
    todo!("0x568564 boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x568568 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_568568() -> ! {
    todo!("0x568568 boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x568588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_568588() -> ! {
    todo!("0x568588 boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5685a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7HandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5685a0() -> ! {
    todo!("0x5685a0 boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5685a4 — __ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv
pub fn stub_5685a4() -> ! {
    todo!("0x5685a4 __ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv")
}

// 0x5685a8 — __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v
pub fn stub_5685a8() -> ! {
    todo!("0x5685a8 __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v")
}

// 0x568688 — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev
pub fn stub_568688() -> ! {
    todo!("0x568688 __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5688cc — __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv
pub fn stub_5688cc() -> ! {
    todo!("0x5688cc __ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv")
}

// 0x568940 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_568940() -> ! {
    todo!("0x568940 rbx::signals::signal<void ()(RBX::NormalId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")
}

// 0x568aa0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
pub fn stub_568aa0() -> ! {
    todo!("0x568aa0 rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")
}

// 0x568ac8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
pub fn stub_568ac8() -> ! {
    todo!("0x568ac8 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")
}

// 0x568aec — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE22safe_static_init_mutexEv
pub fn stub_568aec() -> ! {
    todo!("0x568aec rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_init_mutex(void)")
}

// 0x568af0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
pub fn stub_568af0() -> ! {
    todo!("0x568af0 rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_do_get_mutex(void)")
}

// 0x568be8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_568be8() -> ! {
    todo!("0x568be8 rbx::signals::signal<void ()(RBX::NormalId,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")
}

// 0x568d48 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
pub fn stub_568d48() -> ! {
    todo!("0x568d48 rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")
}

// 0x568d70 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
pub fn stub_568d70() -> ! {
    todo!("0x568d70 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")
}

// 0x568d94 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE22safe_static_init_mutexEv
pub fn stub_568d94() -> ! {
    todo!("0x568d94 rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_init_mutex(void)")
}

// 0x568d98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
pub fn stub_568d98() -> ! {
    todo!("0x568d98 rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_do_get_mutex(void)")
}

// 0x568e90 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
pub fn stub_568e90() -> ! {
    todo!("0x568e90 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")
}

// 0x568f04 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
pub fn stub_568f04() -> ! {
    todo!("0x568f04 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")
}

// 0x568f50 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
pub fn stub_568f50() -> ! {
    todo!("0x568f50 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")
}

// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
pub fn stub_568f7c() -> ! {
    todo!("0x568f7c rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")
}

// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_569050() -> ! {
    todo!("0x569050 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_569058() -> ! {
    todo!("0x569058 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
pub fn stub_569060() -> ! {
    todo!("0x569060 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")
}

// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
pub fn stub_569078() -> ! {
    todo!("0x569078 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
pub fn stub_5690a4() -> ! {
    todo!("0x5690a4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
pub fn stub_569178() -> ! {
    todo!("0x569178 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")
}

// 0x5691ec — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
pub fn stub_5691ec() -> ! {
    todo!("0x5691ec RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")
}

// 0x569238 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
pub fn stub_569238() -> ! {
    todo!("0x569238 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")
}

// 0x569264 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
pub fn stub_569264() -> ! {
    todo!("0x569264 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")
}

// 0x569338 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_569338() -> ! {
    todo!("0x569338 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569340 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_569340() -> ! {
    todo!("0x569340 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569348 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
pub fn stub_569348() -> ! {
    todo!("0x569348 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")
}

// 0x569360 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
pub fn stub_569360() -> ! {
    todo!("0x569360 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")
}

// 0x56938c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
pub fn stub_56938c() -> ! {
    todo!("0x56938c rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")
}

// 0x569460 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
pub fn stub_569460() -> ! {
    todo!("0x569460 rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")
}

// 0x5695bc — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
pub fn stub_5695bc() -> ! {
    todo!("0x5695bc rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")
}

// 0x569734 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
pub fn stub_569734() -> ! {
    todo!("0x569734 rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")
}

// 0x569890 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
pub fn stub_569890() -> ! {
    todo!("0x569890 rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")
}

// 0x569a08 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
pub fn stub_569a08() -> ! {
    todo!("0x569a08 RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")
}

// 0x569afc — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
pub fn stub_569afc() -> ! {
    todo!("0x569afc RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x569b64 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::signalProducedIncremented(RBX::NormalId,float)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
pub fn stub_569b64() -> ! {
    todo!("0x569b64 RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::signalProducedIncremented(RBX::NormalId,float)")
}

// 0x569b7c — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId,float)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f
pub fn stub_569b7c() -> ! {
    todo!("0x569b7c RBX::Reflection::RemoteEventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId,float)")
}

// 0x569ce8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_569ce8() -> ! {
    todo!("0x569ce8 rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")
}
