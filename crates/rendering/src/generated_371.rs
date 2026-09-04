//! rendering shard 371 — 100 stubs 0x522260..0x52536c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 40360->40460 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x522260..0x52536c (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x522260 — __ZNK3RBX7GuiItem11getPositionENS_6CanvasE
#[doc(alias = "__ZNK3RBX7GuiItem11getPositionENS_6CanvasE")]
#[doc(alias = "RBX::GuiItem::getPosition(RBX::Canvas)const")]
// was: __ZNK3RBX7GuiItem11getPositionENS_6CanvasE
// IDA 0x522260: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_522260() {
}

// 0x5222f0 — __ZNK3RBX13UnifiedWidget11getFontSizeEv
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZNK3RBX13UnifiedWidget11getFontSizeEv")]
#[doc(alias = "RBX::UnifiedWidget::getFontSize(void)const")]
// was: __ZNK3RBX13UnifiedWidget11getFontSizeEv
// IDA 0x5222f0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5222f0() {
}

// 0x5222f4 — __ZN3RBX7GuiItem8getTitleEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "__ZN3RBX7GuiItem8getTitleEv")]
#[doc(alias = "RBX::GuiItem::getTitle(void)")]
// was: __ZN3RBX7GuiItem8getTitleEv
// IDA 0x5222f4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5222f4() {
}

// 0x52230c — __ZNK3RBX18UnifiedImageWidget7getSizeENS_6CanvasE
#[doc(alias = "__ZNK3RBX18UnifiedImageWidget7getSizeENS_6CanvasE")]
#[doc(alias = "RBX::UnifiedImageWidget::getSize(RBX::Canvas)const")]
// was: __ZNK3RBX18UnifiedImageWidget7getSizeENS_6CanvasE
// IDA 0x52230c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52230c() {
}

// 0x522318 — __ZN3RBX13UnifiedWidget18onMenuStateChangedEv
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZN3RBX13UnifiedWidget18onMenuStateChangedEv")]
#[doc(alias = "RBX::UnifiedWidget::onMenuStateChanged(void)")]
// was: __ZN3RBX13UnifiedWidget18onMenuStateChangedEv
// IDA 0x522318: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_522318() {
}

// 0x52231c — __ZThn32_NK3RBX7GuiItem12getClassNameEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "__ZThn32_NK3RBX7GuiItem12getClassNameEv")]
#[doc(alias = "non-virtual thunk toRBX::GuiItem::getClassName(void)const")]
// was: __ZThn32_NK3RBX7GuiItem12getClassNameEv
// IDA 0x52231c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52231c() {
}

// 0x522320 — __ZN3RBX13UnifiedWidgetC2Ev
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZN3RBX13UnifiedWidgetC2Ev")]
#[doc(alias = "RBX::UnifiedWidget::UnifiedWidget(void)")]
// was: __ZN3RBX13UnifiedWidgetC2Ev
// IDA 0x522320: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_522320() {
}

// 0x522408 — __ZN3RBX18UnifiedImageWidgetD1Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZN3RBX18UnifiedImageWidgetD1Ev")]
#[doc(alias = "RBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZN3RBX18UnifiedImageWidgetD1Ev
// IDA 0x522408: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522408() {
}

// 0x522500 — __ZN3RBX18UnifiedImageWidgetD0Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZN3RBX18UnifiedImageWidgetD0Ev")]
#[doc(alias = "RBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZN3RBX18UnifiedImageWidgetD0Ev
// IDA 0x522500: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522500() {
}

// 0x522608 — __ZThn32_N3RBX18UnifiedImageWidgetD1Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18UnifiedImageWidgetD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZThn32_N3RBX18UnifiedImageWidgetD1Ev
// IDA 0x522608: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522608() {
}

// 0x5226fc — __ZThn32_N3RBX18UnifiedImageWidgetD0Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18UnifiedImageWidgetD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZThn32_N3RBX18UnifiedImageWidgetD0Ev
// IDA 0x5226fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5226fc() {
}

// 0x522808 — __ZThn36_N3RBX18UnifiedImageWidgetD1Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18UnifiedImageWidgetD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZThn36_N3RBX18UnifiedImageWidgetD1Ev
// IDA 0x522808: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522808() {
}

// 0x5228fc — __ZThn36_N3RBX18UnifiedImageWidgetD0Ev
// type: void __fastcall(RBX::UnifiedImageWidget *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18UnifiedImageWidgetD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
// was: __ZThn36_N3RBX18UnifiedImageWidgetD0Ev
// IDA 0x5228fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5228fc() {
}

// 0x522a08 — __ZN3RBX12GuiDrawImageD2Ev
// type: void __fastcall(RBX::GuiDrawImage *__hidden this)
#[doc(alias = "__ZN3RBX12GuiDrawImageD2Ev")]
#[doc(alias = "RBX::GuiDrawImage::~GuiDrawImage()")]
// was: __ZN3RBX12GuiDrawImageD2Ev
// IDA 0x522a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522a08() {
}

// 0x522de8 — __ZN3RBX13UnifiedWidgetD1Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZN3RBX13UnifiedWidgetD1Ev")]
#[doc(alias = "RBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZN3RBX13UnifiedWidgetD1Ev
// IDA 0x522de8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_522de8() {
}

// 0x522dec — __ZN3RBX13UnifiedWidgetD0Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZN3RBX13UnifiedWidgetD0Ev")]
#[doc(alias = "RBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZN3RBX13UnifiedWidgetD0Ev
// IDA 0x522dec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522dec() {
}

// 0x522e8c — __ZNK3RBX7GuiItem7getSizeENS_6CanvasE
#[doc(alias = "__ZNK3RBX7GuiItem7getSizeENS_6CanvasE")]
#[doc(alias = "RBX::GuiItem::getSize(RBX::Canvas)const")]
// was: __ZNK3RBX7GuiItem7getSizeENS_6CanvasE
// IDA 0x522e8c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_522e8c() {
}

// 0x522e98 — __ZThn32_N3RBX13UnifiedWidgetD1Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13UnifiedWidgetD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZThn32_N3RBX13UnifiedWidgetD1Ev
// IDA 0x522e98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522e98() {
}

// 0x522ea0 — __ZThn32_N3RBX13UnifiedWidgetD0Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13UnifiedWidgetD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZThn32_N3RBX13UnifiedWidgetD0Ev
// IDA 0x522ea0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522ea0() {
}

// 0x522f44 — __ZThn36_N3RBX13UnifiedWidgetD1Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13UnifiedWidgetD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZThn36_N3RBX13UnifiedWidgetD1Ev
// IDA 0x522f44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522f44() {
}

// 0x522f4c — __ZThn36_N3RBX13UnifiedWidgetD0Ev
// type: void __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13UnifiedWidgetD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
// was: __ZThn36_N3RBX13UnifiedWidgetD0Ev
// IDA 0x522f4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_522f4c() {
}

// 0x522ff0 — __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x522ff0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_522ff0() {
}

// 0x5231a0 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5231a0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5231a0() {
}

// 0x5232a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5232a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5232a8() {
}

// 0x5232ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5232ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5232ac() {
}

// 0x5232b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5232b0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5232b0() {
}

// 0x5232d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5232d0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5232d0() {
}

// 0x5232e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5232e8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5232e8() {
}

// 0x5232ec — __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5232ec: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5232ec() {
}

// 0x52349c — __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x52349c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52349c() {
}

// 0x5235a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5235a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5235a4() {
}

// 0x5235a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5235a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5235a8() {
}

// 0x5235ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5235ac: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5235ac() {
}

// 0x5235cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5235cc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5235cc() {
}

// 0x5235e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5235e4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5235e4() {
}

// 0x5235e8 — __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5235e8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5235e8() {
}

// 0x523798 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x523798: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523798() {
}

// 0x5238a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5238a0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5238a0() {
}

// 0x5238a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5238a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5238a4() {
}

// 0x5238a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5238a8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5238a8() {
}

// 0x5238c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5238c8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5238c8() {
}

// 0x5238e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5238e0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5238e0() {
}

// 0x5238e4 — __ZN3RBX13RelativePanelC2ERKNS_6LayoutE
#[doc(alias = "__ZN3RBX13RelativePanelC2ERKNS_6LayoutE")]
#[doc(alias = "RBX::RelativePanel::RelativePanel(RBX::Layout const&)")]
// was: __ZN3RBX13RelativePanelC2ERKNS_6LayoutE
// IDA 0x5238e4: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5238e4() {
}

// 0x5239dc — __ZN3RBX10TopMenuBarC2Ev
// type: _DWORD __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZN3RBX10TopMenuBarC2Ev")]
#[doc(alias = "RBX::TopMenuBar::TopMenuBar(void)")]
// was: __ZN3RBX10TopMenuBarC2Ev
// IDA 0x5239dc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5239dc() {
}

// 0x523ac0 — __ZN3RBX7GuiItem11onLoseFocusEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "__ZN3RBX7GuiItem11onLoseFocusEv")]
#[doc(alias = "RBX::GuiItem::onLoseFocus(void)")]
// was: __ZN3RBX7GuiItem11onLoseFocusEv
// IDA 0x523ac0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_523ac0() {
}

// 0x523ac4 — __ZNK3RBX7GuiItem11getFontSizeEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "__ZNK3RBX7GuiItem11getFontSizeEv")]
#[doc(alias = "RBX::GuiItem::getFontSize(void)const")]
// was: __ZNK3RBX7GuiItem11getFontSizeEv
// IDA 0x523ac4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523ac4() {
}

// 0x523ac8 — __ZNK3RBX10TopMenuBar9isVisibleEv
// type: _DWORD __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZNK3RBX10TopMenuBar9isVisibleEv")]
#[doc(alias = "RBX::TopMenuBar::isVisible(void)const")]
// was: __ZNK3RBX10TopMenuBar9isVisibleEv
// IDA 0x523ac8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523ac8() {
}

// 0x523ad0 — __ZN3RBX10TopMenuBarD1Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZN3RBX10TopMenuBarD1Ev")]
#[doc(alias = "RBX::TopMenuBar::~TopMenuBar()")]
// was: __ZN3RBX10TopMenuBarD1Ev
// IDA 0x523ad0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_523ad0() {
}

// 0x523ad4 — __ZN3RBX10TopMenuBarD0Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZN3RBX10TopMenuBarD0Ev")]
#[doc(alias = "RBX::TopMenuBar::~TopMenuBar()")]
// was: __ZN3RBX10TopMenuBarD0Ev
// IDA 0x523ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_523ad4() {
}

// 0x523b74 — __ZThn32_N3RBX10TopMenuBarD1Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10TopMenuBarD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// was: __ZThn32_N3RBX10TopMenuBarD1Ev
// IDA 0x523b74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_523b74() {
}

// 0x523b7c — __ZThn32_N3RBX10TopMenuBarD0Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10TopMenuBarD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// was: __ZThn32_N3RBX10TopMenuBarD0Ev
// IDA 0x523b7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_523b7c() {
}

// 0x523c20 — __ZThn36_N3RBX10TopMenuBarD1Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10TopMenuBarD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// was: __ZThn36_N3RBX10TopMenuBarD1Ev
// IDA 0x523c20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_523c20() {
}

// 0x523c28 — __ZThn36_N3RBX10TopMenuBarD0Ev
// type: void __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10TopMenuBarD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// was: __ZThn36_N3RBX10TopMenuBarD0Ev
// IDA 0x523c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_523c28() {
}

// 0x523ccc — __ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x523ccc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523ccc() {
}

// 0x523e7c — __ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x523e7c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523e7c() {
}

// 0x523f84 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x523f84: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_523f84() {
}

// 0x523f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x523f88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_523f88() {
}

// 0x523f8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x523f8c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523f8c() {
}

// 0x523fac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x523fac: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523fac() {
}

// 0x523fc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x523fc4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523fc4() {
}

// 0x523fc8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// IDA 0x523fc8: 64 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523fc8() {
}

// 0x524088 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// IDA 0x524088: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524088() {
}

// 0x5240d8 — __ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv
// IDA 0x5240d8: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5240d8() {
}

// 0x524178 — __ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v
// IDA 0x524178: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524178() {
}

// 0x5241bc — __ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v
// IDA 0x5241bc: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5241bc() {
}

// 0x5242a0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CoreGuiService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv
// IDA 0x5242a0: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5242a0() {
}

// 0x524378 — __ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x524378: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524378() {
}

// 0x524480 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x524480: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_524480() {
}

// 0x524484 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x524484: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524484() {
}

// 0x5244a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5244a4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5244a4() {
}

// 0x5244bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5244bc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5244bc() {
}

// 0x5244c0 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5244c0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5244c0() {
}

// 0x524670 — __ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x524670: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524670() {
}

// 0x524778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x524778: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_524778() {
}

// 0x52477c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x52477c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52477c() {
}

// 0x524780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x524780: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524780() {
}

// 0x5247a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5247a0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5247a0() {
}

// 0x5247b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5247b8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5247b8() {
}

// 0x5247bc — __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5247bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5247bc() {
}

// 0x52496c — __ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x52496c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52496c() {
}

// 0x524a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x524a74: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_524a74() {
}

// 0x524a78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x524a78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_524a78() {
}

// 0x524a7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x524a7c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524a7c() {
}

// 0x524a9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x524a9c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524a9c() {
}

// 0x524ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x524ab4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524ab4() {
}

// 0x524ab8 — __ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x524ab8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524ab8() {
}

// 0x524c68 — __ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x524c68: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524c68() {
}

// 0x524d70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x524d70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_524d70() {
}

// 0x524d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x524d74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_524d74() {
}

// 0x524d78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x524d78: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524d78() {
}

// 0x524d98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x524d98: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524d98() {
}

// 0x524db0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x524db0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524db0() {
}

// 0x524db4 — __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD2Ev
// IDA 0x524db4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_524db4() {
}

// 0x524e50 — __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x524e50: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524e50() {
}

// 0x524ebc — __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv
// IDA 0x524ebc: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524ebc() {
}

// 0x525000 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv
// IDA 0x525000: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_525000() {
}

// 0x5250b0 — __ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5250b0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5250b0() {
}

// 0x525260 — __ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x525260: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_525260() {
}

// 0x525368 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x525368: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_525368() {
}

// 0x52536c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x52536c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52536c() {
}
