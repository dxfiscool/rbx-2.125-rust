//! Auto-generated reflB watchdog 1788389058 — 120 stubs EA-sorted asc 0x823b10..0x83f320 (Reflection exhausted, gap-fill 120 from 18910 remaining, global dedup)
//! Source: ida/export.json (85545 funcs) EA asc not in /tmp/global_eas.txt nor existing reflection stubs
//! Format: // 0xADDR — mangled + #[doc(alias)] + todo!("0xADDR") using rbx_core::SharedPtr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x823b10 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_")]
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")]
pub fn stub_0x823b10() -> ! {
    todo!("0x823b10")
}

// 0x823d98 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv")]
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")]
pub fn stub_0x823d98() -> ! {
    todo!("0x823d98")
}

// 0x82e808 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE
// type: int __fastcall(int, void *)
#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE")]
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")]
pub fn stub_0x82e808() -> ! {
    todo!("0x82e808")
}

// 0x82e834 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv")]
#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")]
pub fn stub_0x82e834() -> ! {
    todo!("0x82e834")
}

// 0x833a9c — __ZN3RBX12TweenServiceC1Ev
// type: _DWORD __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZN3RBX12TweenServiceC1Ev")]
#[doc(alias = "RBX::TweenService::TweenService(void)")]
pub fn stub_0x833a9c() -> ! {
    todo!("0x833a9c")
}

// 0x833aa0 — __ZN3RBX12TweenServiceC2Ev
// type: _DWORD __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZN3RBX12TweenServiceC2Ev")]
#[doc(alias = "RBX::TweenService::TweenService(void)")]
pub fn stub_0x833aa0() -> ! {
    todo!("0x833aa0")
}

// 0x833ce4 — __ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE")]
#[doc(alias = "RBX::TweenService::addTweeningObject(boost::weak_ptr<RBX::GuiObject>)")]
pub fn stub_0x833ce4() -> ! {
    todo!("0x833ce4")
}

// 0x833d10 — __ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "RBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x833d10() -> ! {
    todo!("0x833d10")
}

// 0x833e78 — __ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "__ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x833e78() {
    // IDA 0x833e78: non-virtual thunk to `RBX::TweenService::onHeartbeat( int a1, int a2, int a3, int a4, struct _Unwind_Exception *a5, int a6, ` — this/arg-adjust + tail-call (arg a1 -= 96) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x833e80 — __ZN3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZN3RBX12TweenServiceD1Ev")]
#[doc(alias = "RBX::TweenService::~TweenService()")]
pub fn stub_0x833e80() {
    // IDA 0x833e80: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x833f94 — __ZN3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZN3RBX12TweenServiceD0Ev")]
#[doc(alias = "RBX::TweenService::~TweenService()")]
pub fn stub_0x833f94() {
    // IDA 0x833f94: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8340e8 — __ZThn32_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12TweenServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x8340e8() {
    // IDA 0x8340e8: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x8341fc — __ZThn32_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12TweenServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x8341fc() {
    // IDA 0x8341fc: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x83434c — __ZThn36_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12TweenServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x83434c() {
    // IDA 0x83434c: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x834460 — __ZThn36_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12TweenServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x834460() {
    // IDA 0x834460: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x834588 — __ZThn96_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn96_N3RBX12TweenServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x834588() {
    // IDA 0x834588: __ZThn96 thunk (D1 base dtor): `this -= 96`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x83469c — __ZThn96_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "__ZThn96_N3RBX12TweenServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
pub fn stub_0x83469c() {
    // IDA 0x83469c: __ZThn96 thunk (D0 deleting dtor): `this -= 96`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8347c4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)")]
pub fn stub_0x8347c4() -> ! {
    todo!("0x8347c4")
}

// 0x8347e0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert_unique(boost::weak_ptr<RBX::GuiObject> const&)")]
pub fn stub_0x8347e0() -> ! {
    todo!("0x8347e0")
}

// 0x834848 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,boost::weak_ptr<RBX::GuiObject> const&)")]
pub fn stub_0x834848() -> ! {
    todo!("0x834848")
}

// 0x834894 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_create_node(boost::weak_ptr<RBX::GuiObject> const&)")]
pub fn stub_0x834894() -> ! {
    todo!("0x834894")
}

// 0x8349b8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::find(boost::weak_ptr<RBX::GuiObject> const&)")]
pub fn stub_0x8349b8() -> ! {
    todo!("0x8349b8")
}

// 0x834bf4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
#[doc(alias = "std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)")]
pub fn stub_0x834bf4() -> ! {
    todo!("0x834bf4")
}

// 0x834df0 — __ZN3RBX15NotificationBoxC1Ev
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZN3RBX15NotificationBoxC1Ev")]
#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
pub fn stub_0x834df0() -> ! {
    todo!("0x834df0")
}

// 0x834df4 — __ZN3RBX15NotificationBoxC2Ev
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZN3RBX15NotificationBoxC2Ev")]
#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
pub fn stub_0x834df4() -> ! {
    todo!("0x834df4")
}

// 0x835030 — __ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")]
#[doc(alias = "RBX::NotificationBox::addNotification(boost::weak_ptr<RBX::NotificationObject>)")]
pub fn stub_0x835030() -> ! {
    todo!("0x835030")
}

// 0x835248 — __ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")]
#[doc(alias = "RBX::NotificationBox::removeNotification(boost::weak_ptr<RBX::NotificationObject>)")]
pub fn stub_0x835248() -> ! {
    todo!("0x835248")
}

// 0x835270 — __ZN3RBX15NotificationBox13organizeStackEv
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZN3RBX15NotificationBox13organizeStackEv")]
#[doc(alias = "RBX::NotificationBox::organizeStack(void)")]
pub fn stub_0x835270() -> ! {
    todo!("0x835270")
}

// 0x835498 — __ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "__ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "RBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x835498() -> ! {
    todo!("0x835498")
}

// 0x83554c — __ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "__ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x83554c() {
    // IDA 0x83554c: non-virtual thunk to `RBX::NotificationBox::onHeartbeat(int a1) { return RBX::NotificationBox::onHeartbeat(a1 - 536); } "` — this/arg-adjust + tail-call (arg a1 -= 536) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x835554 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_")]
#[doc(alias = "std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::remove(boost::shared_ptr<RBX::NotificationObject> const&)")]
pub fn stub_0x835554() -> ! {
    todo!("0x835554")
}

// 0x83557c — __ZN3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZN3RBX15NotificationBoxD1Ev")]
#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x83557c() {
    // IDA 0x83557c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8356a4 — __ZN3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZN3RBX15NotificationBoxD0Ev")]
#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x8356a4() {
    // IDA 0x8356a4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8357f4 — __ZThn32_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn32_N3RBX15NotificationBoxD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x8357f4() {
    // IDA 0x8357f4: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x835918 — __ZThn32_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn32_N3RBX15NotificationBoxD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x835918() {
    // IDA 0x835918: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x835a60 — __ZThn36_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn36_N3RBX15NotificationBoxD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x835a60() {
    // IDA 0x835a60: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x835b84 — __ZThn36_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn36_N3RBX15NotificationBoxD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x835b84() {
    // IDA 0x835b84: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x835cbc — __ZThn536_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn536_N3RBX15NotificationBoxD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x835cbc() {
    // IDA 0x835cbc: __ZThn536 thunk (D1 base dtor): `this -= 536`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x835de4 — __ZThn536_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "__ZThn536_N3RBX15NotificationBoxD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
pub fn stub_0x835de4() {
    // IDA 0x835de4: __ZThn536 thunk (D0 deleting dtor): `this -= 536`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x836528 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv")]
#[doc(alias = "std::_List_base<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_clear(void)")]
pub fn stub_0x836528() -> ! {
    todo!("0x836528")
}

// 0x836550 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// type: int __fastcall(int, std::_List_node_base *this)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")]
#[doc(alias = "std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::NotificationObject>>)")]
pub fn stub_0x836550() -> ! {
    todo!("0x836550")
}

// 0x836570 — __ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
#[doc(alias = "boost::weak_ptr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(boost::shared_ptr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)")]
pub fn stub_0x836570() -> ! {
    todo!("0x836570")
}

// 0x8365c0 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_")]
#[doc(alias = "std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_create_node(boost::shared_ptr<RBX::NotificationObject> const&)")]
pub fn stub_0x8365c0() -> ! {
    todo!("0x8365c0")
}

// 0x836ab4 — __ZN3RBX18NotificationObjectC1Ev
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZN3RBX18NotificationObjectC1Ev")]
#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
pub fn stub_0x836ab4() -> ! {
    todo!("0x836ab4")
}

// 0x836ab8 — __ZN3RBX18NotificationObjectC2Ev
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZN3RBX18NotificationObjectC2Ev")]
#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
pub fn stub_0x836ab8() -> ! {
    todo!("0x836ab8")
}

// 0x836e60 — __ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE")]
#[doc(alias = "RBX::NotificationObject::initialize(std::string,std::string,std::string,int,boost::function<void ()(void)>)")]
pub fn stub_0x836e60() -> ! {
    todo!("0x836e60")
}

// 0x8373fc — __ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE
#[doc(alias = "__ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE")]
#[doc(alias = "RBX::NotificationObject::processMouseEvent(RBX::GuiEvent const&)")]
pub fn stub_0x8373fc() -> ! {
    todo!("0x8373fc")
}

// 0x837548 — __ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::TextLabel>::operator=(boost::shared_ptr<RBX::TextLabel> const&)")]
pub fn stub_0x837548() -> ! {
    todo!("0x837548")
}

// 0x837580 — __ZN5boost10shared_ptrIN3RBX10ImageLabelEEaSERKS3_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ImageLabelEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::ImageLabel>::operator=(boost::shared_ptr<RBX::ImageLabel> const&)")]
pub fn stub_0x837580() -> ! {
    todo!("0x837580")
}

// 0x8375b8 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEaSERKS3_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::GuiImageButton>::operator=(boost::shared_ptr<RBX::GuiImageButton> const&)")]
pub fn stub_0x8375b8() -> ! {
    todo!("0x8375b8")
}

// 0x8375f0 — __ZN3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZN3RBX18NotificationObjectD1Ev")]
#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x8375f0() {
    // IDA 0x8375f0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8375f4 — __ZN3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZN3RBX18NotificationObjectD0Ev")]
#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x8375f4() {
    // IDA 0x8375f4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8376a4 — __ZThn32_N3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18NotificationObjectD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x8376a4() {
    // IDA 0x8376a4: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x8376ac — __ZThn32_N3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18NotificationObjectD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x8376ac() {
    // IDA 0x8376ac: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x837760 — __ZThn36_N3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18NotificationObjectD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x837760() {
    // IDA 0x837760: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x837768 — __ZThn36_N3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18NotificationObjectD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x837768() {
    // IDA 0x837768: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8389b8 — __ZN3RBX5FrameD1Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZN3RBX5FrameD1Ev")]
#[doc(alias = "RBX::Frame::~Frame()")]
pub fn stub_0x8389b8() {
    // IDA 0x8389b8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x838ab0 — __ZN3RBX5FrameD0Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZN3RBX5FrameD0Ev")]
#[doc(alias = "RBX::Frame::~Frame()")]
pub fn stub_0x838ab0() {
    // IDA 0x838ab0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x838bc8 — __ZThn32_N3RBX5FrameD1Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZThn32_N3RBX5FrameD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
pub fn stub_0x838bc8() {
    // IDA 0x838bc8: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x838cbc — __ZThn32_N3RBX5FrameD0Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZThn32_N3RBX5FrameD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
pub fn stub_0x838cbc() {
    // IDA 0x838cbc: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x838dd8 — __ZThn36_N3RBX5FrameD1Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZThn36_N3RBX5FrameD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
pub fn stub_0x838dd8() {
    // IDA 0x838dd8: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x838ecc — __ZThn36_N3RBX5FrameD0Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "__ZThn36_N3RBX5FrameD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
pub fn stub_0x838ecc() {
    // IDA 0x838ecc: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x83904c — __ZN3RBX18NotificationObjectD2Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "__ZN3RBX18NotificationObjectD2Ev")]
#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
pub fn stub_0x83904c() {
    // IDA 0x83904c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x839508 — __ZN3RBX13FriendService25setCreateFriendRequestUrlESs
#[doc(alias = "__ZN3RBX13FriendService25setCreateFriendRequestUrlESs")]
#[doc(alias = "RBX::FriendService::setCreateFriendRequestUrl(std::string)")]
pub fn stub_0x839508() -> ! {
    todo!("0x839508")
}

// 0x839660 — __ZN3RBX13FriendService25setDeleteFriendRequestUrlESs
#[doc(alias = "__ZN3RBX13FriendService25setDeleteFriendRequestUrlESs")]
#[doc(alias = "RBX::FriendService::setDeleteFriendRequestUrl(std::string)")]
pub fn stub_0x839660() -> ! {
    todo!("0x839660")
}

// 0x8397b8 — __ZN3RBX13FriendService16setMakeFriendUrlESs
#[doc(alias = "__ZN3RBX13FriendService16setMakeFriendUrlESs")]
#[doc(alias = "RBX::FriendService::setMakeFriendUrl(std::string)")]
pub fn stub_0x8397b8() -> ! {
    todo!("0x8397b8")
}

// 0x839910 — __ZN3RBX13FriendService17setBreakFriendUrlESs
#[doc(alias = "__ZN3RBX13FriendService17setBreakFriendUrlESs")]
#[doc(alias = "RBX::FriendService::setBreakFriendUrl(std::string)")]
pub fn stub_0x839910() -> ! {
    todo!("0x839910")
}

// 0x839a68 — __ZN3RBX13FriendService16setGetFriendsUrlESs
#[doc(alias = "__ZN3RBX13FriendService16setGetFriendsUrlESs")]
#[doc(alias = "RBX::FriendService::setGetFriendsUrl(std::string)")]
pub fn stub_0x839a68() -> ! {
    todo!("0x839a68")
}

// 0x839bc0 — __ZN3RBX13FriendService9setEnableEb
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, bool)
#[doc(alias = "__ZN3RBX13FriendService9setEnableEb")]
#[doc(alias = "RBX::FriendService::setEnable(bool)")]
pub fn stub_0x839bc0() -> ! {
    todo!("0x839bc0")
}

// 0x839bc8 — __ZN3RBX13FriendService19setFriendsOnlineUrlESs
#[doc(alias = "__ZN3RBX13FriendService19setFriendsOnlineUrlESs")]
#[doc(alias = "RBX::FriendService::setFriendsOnlineUrl(std::string)")]
pub fn stub_0x839bc8() -> ! {
    todo!("0x839bc8")
}

// 0x839bd0 — __ZN3RBX13FriendServiceC1Ev
// type: _DWORD __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZN3RBX13FriendServiceC1Ev")]
#[doc(alias = "RBX::FriendService::FriendService(void)")]
pub fn stub_0x839bd0() -> ! {
    todo!("0x839bd0")
}

// 0x839bd4 — __ZN3RBX13FriendServiceC2Ev
// type: _DWORD __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZN3RBX13FriendServiceC2Ev")]
#[doc(alias = "RBX::FriendService::FriendService(void)")]
pub fn stub_0x839bd4() -> ! {
    todo!("0x839bd4")
}

// 0x83a7a4 — __ZN3RBXL17countNumberParamsERKSs
// type: _DWORD __fastcall(RBX *__hidden this, const std::string *)
#[doc(alias = "__ZN3RBXL17countNumberParamsERKSs")]
#[doc(alias = "RBX::countNumberParams(std::string const&)")]
pub fn stub_0x83a7a4() -> ! {
    todo!("0x83a7a4")
}

// 0x83a9bc — __ZN3RBX13FriendService30friendStatusReplicationChangedEiiNS0_12FriendStatusE
#[doc(alias = "__ZN3RBX13FriendService30friendStatusReplicationChangedEiiNS0_12FriendStatusE")]
#[doc(alias = "RBX::FriendService::friendStatusReplicationChanged(int,int,RBX::FriendService::FriendStatus)")]
pub fn stub_0x83a9bc() -> ! {
    todo!("0x83a9bc")
}

// 0x83aa1c — __ZN3RBX13FriendService29friendEventReplicationChangedEiiNS0_15FriendEventTypeE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX13FriendService29friendEventReplicationChangedEiiNS0_15FriendEventTypeE")]
#[doc(alias = "RBX::FriendService::friendEventReplicationChanged(int,int,RBX::FriendService::FriendEventType)")]
pub fn stub_0x83aa1c() -> ! {
    todo!("0x83aa1c")
}

// 0x83aa40 — __ZN3RBX13FriendService34issueFriendRequestOrMakeFriendshipEii
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, int, int)
#[doc(alias = "__ZN3RBX13FriendService34issueFriendRequestOrMakeFriendshipEii")]
#[doc(alias = "RBX::FriendService::issueFriendRequestOrMakeFriendship(int,int)")]
pub fn stub_0x83aa40() -> ! {
    todo!("0x83aa40")
}

// 0x83b410 — __ZNK3RBX13FriendService15getFriendStatusEii
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, int, int)
#[doc(alias = "__ZNK3RBX13FriendService15getFriendStatusEii")]
#[doc(alias = "RBX::FriendService::getFriendStatus(int,int)const")]
pub fn stub_0x83b410() -> ! {
    todo!("0x83b410")
}

// 0x83b4b8 — __ZN3RBXL16DontCareResponseEPSsPSt9exception
#[doc(alias = "__ZN3RBXL16DontCareResponseEPSsPSt9exception")]
#[doc(alias = "RBX::DontCareResponse(std::string *,std::exception *)")]
pub fn stub_0x83b4b8() -> ! {
    todo!("0x83b4b8")
}

// 0x83b4bc — __ZN3RBX13FriendService29storeAndReplicateFriendStatusEiiNS0_12FriendStatusE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX13FriendService29storeAndReplicateFriendStatusEiiNS0_12FriendStatusE")]
#[doc(alias = "RBX::FriendService::storeAndReplicateFriendStatus(int,int,RBX::FriendService::FriendStatus)")]
pub fn stub_0x83b4bc() -> ! {
    todo!("0x83b4bc")
}

// 0x83b54c — __ZN3RBX13FriendService36rejectFriendRequestOrBreakFriendshipEii
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, int, int)
#[doc(alias = "__ZN3RBX13FriendService36rejectFriendRequestOrBreakFriendshipEii")]
#[doc(alias = "RBX::FriendService::rejectFriendRequestOrBreakFriendship(int,int)")]
pub fn stub_0x83b54c() -> ! {
    todo!("0x83b54c")
}

// 0x83bf28 — __ZN3RBX13FriendService25ProcessBulkFriendResponseEN5boost8weak_ptrIS0_EEiSt3setIiSt4lessIiESaIiEEPSsPSt9exception
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, char, int, int, void *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX13FriendService25ProcessBulkFriendResponseEN5boost8weak_ptrIS0_EEiSt3setIiSt4lessIiESaIiEEPSsPSt9exception")]
#[doc(alias = "RBX::FriendService::ProcessBulkFriendResponse(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *)")]
pub fn stub_0x83bf28() -> ! {
    todo!("0x83bf28")
}

// 0x83c430 — __ZN3RBX13FriendService18StoreFriendsHelperEN5boost8weak_ptrIS0_EEiNS1_10shared_ptrISt3mapIiNS0_12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX13FriendService18StoreFriendsHelperEN5boost8weak_ptrIS0_EEiNS1_10shared_ptrISt3mapIiNS0_12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEE")]
#[doc(alias = "RBX::FriendService::StoreFriendsHelper(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)")]
pub fn stub_0x83c430() -> ! {
    todo!("0x83c430")
}

// 0x83c52c — __ZN3RBX13FriendService11playerAddedEi
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, int)
#[doc(alias = "__ZN3RBX13FriendService11playerAddedEi")]
#[doc(alias = "RBX::FriendService::playerAdded(int)")]
pub fn stub_0x83c52c() -> ! {
    todo!("0x83c52c")
}

// 0x83cc44 — __ZN3RBX13FriendService14playerRemovingEi
// type: _DWORD __fastcall(RBX::FriendService *__hidden this, int)
#[doc(alias = "__ZN3RBX13FriendService14playerRemovingEi")]
#[doc(alias = "RBX::FriendService::playerRemoving(int)")]
pub fn stub_0x83cc44() -> ! {
    todo!("0x83cc44")
}

// 0x83d3c8 — __ZNSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEEixERS6_
#[doc(alias = "__ZNSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEEixERS6_")]
#[doc(alias = "std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::operator[](int const&)")]
pub fn stub_0x83d3c8() -> ! {
    todo!("0x83d3c8")
}

// 0x83d420 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS3_12FriendStatusESt4lessIiESaISt4pairIKiS7_EEEEES4_iSF_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS3_12FriendStatusESt4lessIiESaISt4pairIKiS7_EEEEES4_iSF_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list_av_3<boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)")]
pub fn stub_0x83d420() -> ! {
    todo!("0x83d420")
}

// 0x83d694 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionS4_iS9_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_ENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESP_SR_SS_ST_SU_SV_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionS4_iS9_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_ENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESP_SR_SS_ST_SU_SV_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list_av_5<boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x83d694() -> ! {
    todo!("0x83d694")
}

// 0x83d8bc — __ZN3RBX9weak_fromINS_13FriendServiceEEEN5boost8weak_ptrIT_EEPS4_
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX9weak_fromINS_13FriendServiceEEEN5boost8weak_ptrIT_EEPS4_")]
#[doc(alias = "boost::weak_ptr<RBX::FriendService> RBX::weak_from<RBX::FriendService>(RBX::FriendService*)")]
pub fn stub_0x83d8bc() -> ! {
    todo!("0x83d8bc")
}

// 0x83dab4 — __ZNSt3mapIiS_IiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEES4_SaIS5_IS6_S9_EEEixERS6_
#[doc(alias = "__ZNSt3mapIiS_IiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEES4_SaIS5_IS6_S9_EEEixERS6_")]
#[doc(alias = "std::map<int,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator[](int const&)")]
pub fn stub_0x83dab4() -> ! {
    todo!("0x83dab4")
}

// 0x83e298 — __ZN3RBX13FriendServiceD1Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZN3RBX13FriendServiceD1Ev")]
#[doc(alias = "RBX::FriendService::~FriendService()")]
pub fn stub_0x83e298() {
    // IDA 0x83e298: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x83e29c — __ZN3RBX13FriendServiceD0Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZN3RBX13FriendServiceD0Ev")]
#[doc(alias = "RBX::FriendService::~FriendService()")]
pub fn stub_0x83e29c() {
    // IDA 0x83e29c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x83e34c — __ZThn32_N3RBX13FriendServiceD1Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13FriendServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
pub fn stub_0x83e34c() {
    // IDA 0x83e34c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x83e354 — __ZThn32_N3RBX13FriendServiceD0Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13FriendServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
pub fn stub_0x83e354() {
    // IDA 0x83e354: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x83e408 — __ZThn36_N3RBX13FriendServiceD1Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13FriendServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
pub fn stub_0x83e408() {
    // IDA 0x83e408: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x83e410 — __ZThn36_N3RBX13FriendServiceD0Ev
// type: void __fastcall(RBX::FriendService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13FriendServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::FriendService::~FriendService()")]
pub fn stub_0x83e410() {
    // IDA 0x83e410: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x83e4b4 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::resize(unsigned long,RBX::FriendService::FriendEventType)")]
pub fn stub_0x83e4b4() -> ! {
    todo!("0x83e4b4")
}

// 0x83e4e8 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::push_back(RBX::FriendService::FriendEventType const&)")]
pub fn stub_0x83e4e8() -> ! {
    todo!("0x83e4e8")
}

// 0x83e510 — __ZNSt3mapIPKN3RBX4NameENS0_13FriendService15FriendEventTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13FriendService15FriendEventTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendEventType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x83e510() -> ! {
    todo!("0x83e510")
}

// 0x83e568 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
pub fn stub_0x83e568() -> ! {
    todo!("0x83e568")
}

// 0x83e61c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
pub fn stub_0x83e61c() -> ! {
    todo!("0x83e61c")
}

// 0x83e674 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
pub fn stub_0x83e674() -> ! {
    todo!("0x83e674")
}

// 0x83e6dc — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,RBX::FriendService::FriendEventType const&)")]
pub fn stub_0x83e6dc() -> ! {
    todo!("0x83e6dc")
}

// 0x83e7c0 — __ZNSt12_Vector_baseIN3RBX13FriendService15FriendEventTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13FriendService15FriendEventTypeESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_allocate(unsigned long)")]
pub fn stub_0x83e7c0() -> ! {
    todo!("0x83e7c0")
}

// 0x83e7d8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService15FriendEventTypeES6_EET0_T_S8_S7_
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService15FriendEventTypeES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::FriendService::FriendEventType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *>(RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *)")]
pub fn stub_0x83e7d8() -> ! {
    todo!("0x83e7d8")
}

// 0x83e814 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,unsigned long,RBX::FriendService::FriendEventType const&)")]
pub fn stub_0x83e814() -> ! {
    todo!("0x83e814")
}

// 0x83e9a4 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE6resizeEmS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::resize(unsigned long,RBX::FriendService::FriendStatus)")]
pub fn stub_0x83e9a4() -> ! {
    todo!("0x83e9a4")
}

// 0x83e9d8 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE9push_backERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::push_back(RBX::FriendService::FriendStatus const&)")]
pub fn stub_0x83e9d8() -> ! {
    todo!("0x83e9d8")
}

// 0x83ea00 — __ZNSt3mapIPKN3RBX4NameENS0_13FriendService12FriendStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13FriendService12FriendStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x83ea00() -> ! {
    todo!("0x83ea00")
}

// 0x83ea58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
pub fn stub_0x83ea58() -> ! {
    todo!("0x83ea58")
}

// 0x83eb0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
pub fn stub_0x83eb0c() -> ! {
    todo!("0x83eb0c")
}

// 0x83eb64 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
pub fn stub_0x83eb64() -> ! {
    todo!("0x83eb64")
}

// 0x83ebcc — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,RBX::FriendService::FriendStatus const&)")]
pub fn stub_0x83ebcc() -> ! {
    todo!("0x83ebcc")
}

// 0x83ecb0 — __ZNSt12_Vector_baseIN3RBX13FriendService12FriendStatusESaIS2_EE11_M_allocateEm
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13FriendService12FriendStatusESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_allocate(unsigned long)")]
pub fn stub_0x83ecb0() -> ! {
    todo!("0x83ecb0")
}

// 0x83ecc8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService12FriendStatusES6_EET0_T_S8_S7_
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService12FriendStatusES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::FriendService::FriendStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *>(RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *)")]
pub fn stub_0x83ecc8() -> ! {
    todo!("0x83ecc8")
}

// 0x83ed04 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,unsigned long,RBX::FriendService::FriendStatus const&)")]
pub fn stub_0x83ed04() -> ! {
    todo!("0x83ed04")
}

// 0x83f028 — __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_")]
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,RBX::FriendService::FriendStatus)>::operator()(int,int,RBX::FriendService::FriendStatus)")]
pub fn stub_0x83f028() -> ! {
    todo!("0x83f028")
}

// 0x83f174 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> &)")]
pub fn stub_0x83f174() -> ! {
    todo!("0x83f174")
}

// 0x83f2d4 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::on_error(std::exception &)")]
pub fn stub_0x83f2d4() -> ! {
    todo!("0x83f2d4")
}

// 0x83f2fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSERKSA_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSERKSA_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> const&)")]
pub fn stub_0x83f2fc() -> ! {
    todo!("0x83f2fc")
}

// 0x83f320 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::safe_static_init_mutex(void)")]
pub fn stub_0x83f320() -> ! {
    todo!("0x83f320")
}
