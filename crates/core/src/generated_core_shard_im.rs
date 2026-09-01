//! core shard IM — 100 core stubs EA-sorted, continuation after IL 0x698334 (EA-sorted ascending, next 100 uncovered).
//!
//! Source: `ida/export.json` filtered where demangled/mangled contains RBX::|boost, excludes Reflection|DataModel|Ogre|RakNet|Lua, EA-sorted, next 100 uncovered after 0x698334.
//!
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x698354 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ButtonBindingWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_698354() -> ! {
    todo!("0x698354 boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x69836c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ButtonBindingWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_69836c() -> ! {
    todo!("0x69836c boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(void)const")]
// 0x699120 — __ZNK3RBX15ServiceProvider4findINS_9WorkspaceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(void)const
pub fn stub_699120() -> ! {
    todo!("0x699120 RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(void)const")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Workspace>(void)")]
// 0x6992a0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9WorkspaceEEEvv
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::Workspace>(void)
pub fn stub_6992a0() -> ! {
    todo!("0x6992a0 void RBX::ServiceProvider::callDoGetClassIndex<RBX::Workspace>(void)")
}

#[doc(alias = "RBX::GuiItem::askAddChild(RBX::Instance const*)const")]
// 0x699320 — __ZNK3RBX7GuiItem11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this, const RBX::Instance *)
// was: RBX::GuiItem::askAddChild(RBX::Instance const*)const
pub fn stub_699320() -> ! {
    todo!("0x699320 RBX::GuiItem::askAddChild(RBX::Instance const*)const")
}

#[doc(alias = "RBX::ObjectValue::setValue(RBX::Instance *)")]
// 0x69d4c8 — __ZN3RBX11ObjectValue8setValueEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ObjectValue *__hidden this, RBX::Instance *)
// was: RBX::ObjectValue::setValue(RBX::Instance *)
pub fn stub_69d4c8() -> ! {
    todo!("0x69d4c8 RBX::ObjectValue::setValue(RBX::Instance *)")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69dc6c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69dc6c() -> ! {
    todo!("0x69dc6c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69dd1c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69dd1c() -> ! {
    todo!("0x69dd1c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69ddcc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69ddcc() -> ! {
    todo!("0x69ddcc __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e08c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69e08c() -> ! {
    todo!("0x69e08c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e13c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69e13c() -> ! {
    todo!("0x69e13c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e1ec — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69e1ec() -> ! {
    todo!("0x69e1ec __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e29c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_69e29c() -> ! {
    todo!("0x69e29c __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x69f524 — __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_69f524() -> ! {
    todo!("0x69f524 __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIdLZNS3_23sDoubleConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x69f6d4 — __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIdLZNS3_23sDoubleConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIdLZNS3_23sDoubleConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_69f6d4() -> ! {
    todo!("0x69f6d4 __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIdLZNS3_23sDoubleConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x69f7dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_69f7dc() -> ! {
    todo!("0x69f7dc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x69f7e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_69f7e0() -> ! {
    todo!("0x69f7e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x69f7e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_69f7e4() -> ! {
    todo!("0x69f7e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x69f804 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_69f804() -> ! {
    todo!("0x69f804 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x69f81c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_69f81c() -> ! {
    todo!("0x69f81c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIdLZNS2_23sDoubleConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a09c4 — __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6a09c4() -> ! {
    todo!("0x6a09c4 __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIiLZNS3_20sIntConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6a0b74 — __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIiLZNS3_20sIntConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIiLZNS3_20sIntConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6a0b74() -> ! {
    todo!("0x6a0b74 __ZN5boost6detail12shared_countC2IPN3RBX16ConstrainedValueIiLZNS3_20sIntConstrainedValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6a0c7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6a0c7c() -> ! {
    todo!("0x6a0c7c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6a0c80 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6a0c80() -> ! {
    todo!("0x6a0c80 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6a0c84 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6a0c84() -> ! {
    todo!("0x6a0c84 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6a0ca4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6a0ca4() -> ! {
    todo!("0x6a0ca4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6a0cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6a0cbc() -> ! {
    todo!("0x6a0cbc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ConstrainedValueIiLZNS2_20sIntConstrainedValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a20f0 — __ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6a20f0() -> ! {
    todo!("0x6a20f0 __ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_6RbxRayELZNS3_9sRayValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6a22a0 — __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_6RbxRayELZNS3_9sRayValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_6RbxRayELZNS3_9sRayValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6a22a0() -> ! {
    todo!("0x6a22a0 __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_6RbxRayELZNS3_9sRayValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6a23a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6a23a8() -> ! {
    todo!("0x6a23a8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6a23ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6a23ac() -> ! {
    todo!("0x6a23ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6a23b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6a23b0() -> ! {
    todo!("0x6a23b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6a23d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6a23d0() -> ! {
    todo!("0x6a23d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6a23e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6a23e8() -> ! {
    todo!("0x6a23e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_6RbxRayELZNS2_9sRayValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a37f0 — __ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6a37f0() -> ! {
    todo!("0x6a37f0 __ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_10BrickColorELZNS3_16sBrickColorValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6a39a0 — __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_10BrickColorELZNS3_16sBrickColorValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_10BrickColorELZNS3_16sBrickColorValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6a39a0() -> ! {
    todo!("0x6a39a0 __ZN5boost6detail12shared_countC2IPN3RBX5ValueINS3_10BrickColorELZNS3_16sBrickColorValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6a3aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6a3aa8() -> ! {
    todo!("0x6a3aa8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6a3aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6a3aac() -> ! {
    todo!("0x6a3aac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6a3ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6a3ab0() -> ! {
    todo!("0x6a3ab0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6a3ad0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6a3ad0() -> ! {
    todo!("0x6a3ad0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6a3ae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6a3ae8() -> ! {
    todo!("0x6a3ae8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueINS2_10BrickColorELZNS2_16sBrickColorValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a91ac — __ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6a91ac() -> ! {
    todo!("0x6a91ac __ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5ValueIbLZNS3_10sBoolValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6a935c — __ZN5boost6detail12shared_countC2IPN3RBX5ValueIbLZNS3_10sBoolValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ValueIbLZNS3_10sBoolValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6a935c() -> ! {
    todo!("0x6a935c __ZN5boost6detail12shared_countC2IPN3RBX5ValueIbLZNS3_10sBoolValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6a9464 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6a9464() -> ! {
    todo!("0x6a9464 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6a9468 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6a9468() -> ! {
    todo!("0x6a9468 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6a946c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6a946c() -> ! {
    todo!("0x6a946c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6a948c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6a948c() -> ! {
    todo!("0x6a948c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6a94a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6a94a4() -> ! {
    todo!("0x6a94a4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIbLZNS2_10sBoolValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6aa60c — __ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6aa60c() -> ! {
    todo!("0x6aa60c __ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5ValueIdLZNS3_12sDoubleValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6aa7bc — __ZN5boost6detail12shared_countC2IPN3RBX5ValueIdLZNS3_12sDoubleValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ValueIdLZNS3_12sDoubleValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6aa7bc() -> ! {
    todo!("0x6aa7bc __ZN5boost6detail12shared_countC2IPN3RBX5ValueIdLZNS3_12sDoubleValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6aa8c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6aa8c4() -> ! {
    todo!("0x6aa8c4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6aa8c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6aa8c8() -> ! {
    todo!("0x6aa8c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6aa8cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6aa8cc() -> ! {
    todo!("0x6aa8cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6aa8ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6aa8ec() -> ! {
    todo!("0x6aa8ec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6aa904 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6aa904() -> ! {
    todo!("0x6aa904 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIdLZNS2_12sDoubleValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6aba68 — __ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_6aba68() -> ! {
    todo!("0x6aba68 __ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5ValueIiLZNS3_9sIntValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x6abc18 — __ZN5boost6detail12shared_countC2IPN3RBX5ValueIiLZNS3_9sIntValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ValueIiLZNS3_9sIntValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6abc18() -> ! {
    todo!("0x6abc18 __ZN5boost6detail12shared_countC2IPN3RBX5ValueIiLZNS3_9sIntValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x6abd20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6abd20() -> ! {
    todo!("0x6abd20 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x6abd24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6abd24() -> ! {
    todo!("0x6abd24 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x6abd28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6abd28() -> ! {
    todo!("0x6abd28 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x6abd48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6abd48() -> ! {
    todo!("0x6abd48 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x6abd60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6abd60() -> ! {
    todo!("0x6abd60 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ValueIiLZNS2_9sIntValueEEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::getDisabled(void)const")]
// 0x6be4c4 — __ZNK3RBX8SeatImplINS_12PartInstanceEE11getDisabledEv
// was: RBX::SeatImpl<RBX::PartInstance>::getDisabled(void)const
pub fn stub_6be4c4() -> ! {
    todo!("0x6be4c4 RBX::SeatImpl<RBX::PartInstance>::getDisabled(void)const")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::setDisabled(bool const&)")]
// 0x6be4cc — __ZN3RBX8SeatImplINS_12PartInstanceEE11setDisabledERKb
// was: RBX::SeatImpl<RBX::PartInstance>::setDisabled(bool const&)
pub fn stub_6be4cc() -> ! {
    todo!("0x6be4cc RBX::SeatImpl<RBX::PartInstance>::setDisabled(bool const&)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x6be5ac — __ZN3RBX8SeatImplINS_12PartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
// was: RBX::SeatImpl<RBX::PartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_6be5ac() -> ! {
    todo!("0x6be5ac RBX::SeatImpl<RBX::PartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

#[doc(alias = "RBX::ActionStation<RBX::PartInstance>::setName(std::string const&)")]
// 0x6be874 — __ZN3RBX13ActionStationINS_12PartInstanceEE7setNameERKSs
// was: RBX::ActionStation<RBX::PartInstance>::setName(std::string const&)
pub fn stub_6be874() -> ! {
    todo!("0x6be874 RBX::ActionStation<RBX::PartInstance>::setName(std::string const&)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onChildAdded(RBX::Instance *)")]
// 0x6be88c — __ZN3RBX8SeatImplINS_12PartInstanceEE12onChildAddedEPNS_8InstanceE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: RBX::SeatImpl<RBX::PartInstance>::onChildAdded(RBX::Instance *)
pub fn stub_6be88c() -> ! {
    todo!("0x6be88c RBX::SeatImpl<RBX::PartInstance>::onChildAdded(RBX::Instance *)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onChildRemoved(RBX::Instance *)")]
// 0x6bea64 — __ZN3RBX8SeatImplINS_12PartInstanceEE14onChildRemovedEPNS_8InstanceE
// was: RBX::SeatImpl<RBX::PartInstance>::onChildRemoved(RBX::Instance *)
pub fn stub_6bea64() -> ! {
    todo!("0x6bea64 RBX::SeatImpl<RBX::PartInstance>::onChildRemoved(RBX::Instance *)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onSeatedChanged(bool,RBX::Humanoid *)")]
// 0x6becf0 — __ZN3RBX8SeatImplINS_12PartInstanceEE15onSeatedChangedEbPNS_8HumanoidE
// was: RBX::SeatImpl<RBX::PartInstance>::onSeatedChanged(bool,RBX::Humanoid *)
pub fn stub_6becf0() -> ! {
    todo!("0x6becf0 RBX::SeatImpl<RBX::PartInstance>::onSeatedChanged(bool,RBX::Humanoid *)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")]
// 0x6bf028 — __ZN3RBX8SeatImplINS_12PartInstanceEED1Ev
// was: RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()
pub fn stub_6bf028() -> ! {
    todo!("0x6bf028 RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")]
// 0x6bf038 — __ZN3RBX8SeatImplINS_12PartInstanceEED0Ev
// was: RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()
pub fn stub_6bf038() -> ! {
    todo!("0x6bf038 RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")
}

#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")]
// 0x6bf0e4 — __ZThn132_N3RBX8SeatImplINS_12PartInstanceEED1Ev
// was: `non-virtual thunk to'RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()
pub fn stub_6bf0e4() -> ! {
    todo!("0x6bf0e4 non-virtual thunk toRBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")
}

#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")]
// 0x6bf0f8 — __ZThn132_N3RBX8SeatImplINS_12PartInstanceEED0Ev
// was: `non-virtual thunk to'RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()
pub fn stub_6bf0f8() -> ! {
    todo!("0x6bf0f8 non-virtual thunk toRBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")
}

#[doc(alias = "RBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf1a8 — __ZN3RBX13ActionStationINS_12PartInstanceEED1Ev
// was: RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf1a8() -> ! {
    todo!("0x6bf1a8 RBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "RBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf1bc — __ZN3RBX13ActionStationINS_12PartInstanceEED0Ev
// was: RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf1bc() -> ! {
    todo!("0x6bf1bc RBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf26c — __ZThn132_N3RBX13ActionStationINS_12PartInstanceEED1Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf26c() -> ! {
    todo!("0x6bf26c non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf280 — __ZThn132_N3RBX13ActionStationINS_12PartInstanceEED0Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf280() -> ! {
    todo!("0x6bf280 non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf28c — __ZThn32_N3RBX13ActionStationINS_12PartInstanceEED1Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf28c() -> ! {
    todo!("0x6bf28c non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf2a0 — __ZThn36_N3RBX13ActionStationINS_12PartInstanceEED1Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf2a0() -> ! {
    todo!("0x6bf2a0 non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf2b4 — __ZThn32_N3RBX13ActionStationINS_12PartInstanceEED0Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf2b4() -> ! {
    todo!("0x6bf2b4 non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
// 0x6bf2bc — __ZThn36_N3RBX13ActionStationINS_12PartInstanceEED0Ev
// was: `non-virtual thunk to'RBX::ActionStation<RBX::PartInstance>::~ActionStation()
pub fn stub_6bf2bc() -> ! {
    todo!("0x6bf2bc non-virtual thunk toRBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::isChildSeatWeld(RBX::Instance *)")]
// 0x6bf2c4 — __ZN3RBX8SeatImplINS_12PartInstanceEE15isChildSeatWeldEPNS_8InstanceE
// type: int __fastcall(int, RBX::Instance *this)
// was: RBX::SeatImpl<RBX::PartInstance>::isChildSeatWeld(RBX::Instance *)
pub fn stub_6bf2c4() -> ! {
    todo!("0x6bf2c4 RBX::SeatImpl<RBX::PartInstance>::isChildSeatWeld(RBX::Instance *)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::humanoidFromWeld(RBX::Weld *)")]
// 0x6bf30c — __ZN3RBX8SeatImplINS_12PartInstanceEE16humanoidFromWeldEPNS_4WeldE
// type: int __fastcall(int, RBX::JointInstance *this)
// was: RBX::SeatImpl<RBX::PartInstance>::humanoidFromWeld(RBX::Weld *)
pub fn stub_6bf30c() -> ! {
    todo!("0x6bf30c RBX::SeatImpl<RBX::PartInstance>::humanoidFromWeld(RBX::Weld *)")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>> const&)")]
// 0x6bf324 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_12PartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>> const&)
pub fn stub_6bf324() -> ! {
    todo!("0x6bf324 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>> const&)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onEvent_humanoidDoneSitting(void)")]
// 0x6bf398 — __ZN3RBX8SeatImplINS_12PartInstanceEE27onEvent_humanoidDoneSittingEv
// was: RBX::SeatImpl<RBX::PartInstance>::onEvent_humanoidDoneSitting(void)
pub fn stub_6bf398() -> ! {
    todo!("0x6bf398 RBX::SeatImpl<RBX::PartInstance>::onEvent_humanoidDoneSitting(void)")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::findSeatWeld(void)")]
// 0x6bf3b8 — __ZN3RBX8SeatImplINS_12PartInstanceEE12findSeatWeldEv
// type: int __fastcall(RBX::Instance *this)
// was: RBX::SeatImpl<RBX::PartInstance>::findSeatWeld(void)
pub fn stub_6bf3b8() -> ! {
    todo!("0x6bf3b8 RBX::SeatImpl<RBX::PartInstance>::findSeatWeld(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()")]
// 0x6bf3ec — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_12PartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()
pub fn stub_6bf3ec() -> ! {
    todo!("0x6bf3ec rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()")]
// 0x6bf418 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_12PartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()
pub fn stub_6bf418() -> ! {
    todo!("0x6bf418 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)")]
// 0x6bf4ec — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_12PartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)
pub fn stub_6bf4ec() -> ! {
    todo!("0x6bf4ec rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)")]
// 0x6bf4f4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_12PartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)
pub fn stub_6bf4f4() -> ! {
    todo!("0x6bf4f4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::call(void)")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>::operator()(void)")]
// 0x6bf4fc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_12PartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>::operator()(void)
pub fn stub_6bf4fc() -> ! {
    todo!("0x6bf4fc boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>::operator()(void)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()")]
// 0x6bf514 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_12PartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()
pub fn stub_6bf514() -> ! {
    todo!("0x6bf514 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()")]
// 0x6bf540 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_12PartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()
pub fn stub_6bf540() -> ! {
    todo!("0x6bf540 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>,0,void ()(void)>::~callable()")
}

#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::destroyOtherWeld(rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)")]
// 0x6bf614 — __ZN3RBX8SeatImplINS_12PartInstanceEE16destroyOtherWeldEN5boost10shared_ptrINS_8InstanceEEEPNS_4WeldE
// was: RBX::SeatImpl<RBX::PartInstance>::destroyOtherWeld(boost::shared_ptr<RBX::Instance>,RBX::Weld *)
pub fn stub_6bf614() -> ! {
    todo!("0x6bf614 RBX::SeatImpl<RBX::PartInstance>::destroyOtherWeld(rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// 0x6bf63c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8SeatImplINS3_12PartInstanceEEEEENS_3argILi1EEENS2_IPNS3_4WeldEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_6bf63c() -> ! {
    todo!("0x6bf63c void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::PartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)const")]
// 0x6bf718 — __ZNK5boost4_mfi3mf2IvN3RBX8SeatImplINS2_12PartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_4WeldEEclEPS5_S8_SA_
// was: boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::PartInstance>*,boost::shared_ptr<RBX::Instance>,RBX::Weld *)const
pub fn stub_6bf718() -> ! {
    todo!("0x6bf718 boost::_mfi::mf2<void,RBX::SeatImpl<RBX::PartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::PartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)const")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat> RBX::Creatable<RBX::Instance>::create<RBX::VehicleSeat>(void)")]
// 0x6bfa6c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::VehicleSeat> RBX::Creatable<RBX::Instance>::create<RBX::VehicleSeat>(void)
pub fn stub_6bfa6c() -> ! {
    todo!("0x6bfa6c rbx_core::SharedPtr<RBX::VehicleSeat> RBX::Creatable<RBX::Instance>::create<RBX::VehicleSeat>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x6bfb20 — __ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6bfb20() -> ! {
    todo!("0x6bfb20 rbx_core::SharedPtr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x6bfcd4 — __ZN5boost6detail12shared_countC2IPN3RBX11VehicleSeatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6bfcd4() -> ! {
    todo!("0x6bfcd4 boost::detail::shared_count::shared_count<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x6bfddc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11VehicleSeatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_6bfddc() -> ! {
    todo!("0x6bfddc boost::detail::sp_counted_impl_pd<RBX::VehicleSeat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}