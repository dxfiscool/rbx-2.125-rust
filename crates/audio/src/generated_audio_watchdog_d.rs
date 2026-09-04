//! audio generated_audio_watchdog_d — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio 2544/2544 complete, gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio | rbx_core::SharedPtr not boost
//! Range 0x521aec..0x524670 | existing 30013 -> 30113 distinct
//! Batch: 100 stubs | 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// was: `RBX::Adorn` — opaque render adorn handle threaded through the
/// GuiDrawImage/UnifiedImageWidget ctors (IDA 0x522130/0x522c04, host: unused).
#[derive(Clone, Copy, Default)]
pub struct Adorn;

/// was: `RBX::Canvas` — 2-float coordinate frame passed to
/// `GuiItem::getPosition/getSize` (IDA 0x522260/0x522e8c: the two floats
/// copied at 0x5222d8..0x5222de, host: origin offset).
#[derive(Clone, Copy, Default)]
pub struct Canvas {
    pub offset: [f32; 2],
}

/// was: `RBX::Layout` — consumed by `RelativePanel::init` (IDA 0x52397e);
/// ctor stores it (IDA 0x5238e4, host: moved in).
#[derive(Clone, Default)]
pub struct Layout;

/// was: `RBX::GuiItem` — base of every widget below. `size` is the two-dword
/// pair read at +104/+108 by `getSize` (IDA 0x522e8c); `title` is the
/// Instance name word copied at +24 by `getTitle` (IDA 0x5222f4); `parent`
/// backs the `getGuiParent()` assert in `getPosition` (IDA 0x522260).
#[derive(Clone, Default)]
pub struct GuiItem {
    pub parent: Option<SharedPtr<GuiItem>>,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub title: String,
}

/// was: `RBX::UnifiedWidget` — GuiItem base plus init (IDA 0x522320).
#[derive(Clone, Default)]
pub struct UnifiedWidget {
    pub base: GuiItem,
}

/// was: `RBX::GuiDrawImage` — empty strings/null names plus zeroed quads at
/// construction, then `setImageFromName` (IDA 0x522c04, host: name/id stored,
/// zero image size).
#[derive(Clone, Default)]
pub struct GuiDrawImage {
    pub image_name: String,
    pub image_id: u32,
    pub image_size: [f32; 2],
}

/// was: `RBX::UnifiedImageWidget` — UnifiedWidget base, GuiDrawImage
/// subobject at +116, name copy at +200, flags at +204 (IDA 0x522130).
#[derive(Clone, Default)]
pub struct UnifiedImageWidget {
    pub base: UnifiedWidget,
    pub image: GuiDrawImage,
    pub image_name: String,
    pub flags: i32,
}

/// was: `RBX::TopMenuBar` — GuiItem base plus init; `visible` is the byte at
/// +132 read by `isVisible` (IDA 0x523ac8).
#[derive(Clone, Default)]
pub struct TopMenuBar {
    pub base: GuiItem,
    pub visible: bool,
}

/// was: `RBX::RelativePanel` — TopMenuBar base plus the stored layout
/// (IDA 0x5238e4).
#[derive(Clone, Default)]
pub struct RelativePanel {
    pub base: TopMenuBar,
    pub layout: Layout,
}

/// was: `RBX::TextDisplay` — downcast target of 0x521af0 (host: GuiItem base).
#[derive(Clone, Default)]
pub struct TextDisplay {
    pub base: GuiItem,
}

/// was: `RBX::EquationDisplay` — shared_ptr ctor target (IDA 0x521b38,
/// host: Arc adoption).
#[derive(Clone, Default)]
pub struct EquationDisplay;

/// was: `RBX::ChatButton` — shared_ptr ctor target (IDA 0x522ff0).
#[derive(Clone, Default)]
pub struct ChatButton;

/// was: `RBX::ChatWidget` — shared_ptr ctor target (IDA 0x5232ec).
#[derive(Clone, Default)]
pub struct ChatWidget;

/// was: `RBX::ChatOutput` — shared_ptr ctor target (IDA 0x5235e8).
#[derive(Clone, Default)]
pub struct ChatOutput;

/// was: `RBX::GuiImageButton` — shared_ptr ctor target (IDA 0x5244c0).
#[derive(Clone, Default)]
pub struct GuiImageButton;

/// was: `RBX::Reflection::DescribedBase` (+ `RBX::TextDisplay`) — minimal host
/// carrier for `dynamic_pointer_cast<TextDisplay, DescribedBase>`
/// (IDA 0x521af0). The kind tag plays `typeinfo`; the stored value plays the
/// adjusted pointer the aliasing share adopts.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GuiDescribedKind {
    #[default]
    Unknown,
    TextDisplay,
}

/// was: `RBX::Reflection::DescribedBase` value carrier for 0x521af0.
#[derive(Clone, Default)]
pub struct GuiDescribedBase {
    pub kind: GuiDescribedKind,
    pub text: Option<TextDisplay>,
}

/// was: `RBX::GuiBuilder::Data` — map value in the GuiBuilder name index
/// searched by `_Rb_tree::find` (IDA 0x524088, host: BTreeMap value).
#[derive(Clone, Default)]
pub struct GuiBuilderData;

/// was: `RBX::sCoreGuiService` — the "CoreGui" class-name string backing
/// isNullClassName/declare/doDeclare (IDA 0x5240d8/0x524178/0x5241bc).
pub const S_CORE_GUI_SERVICE: Option<&'static str> = Some("CoreGui");

// 0x521aec — __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_521aec() {
    // IDA 0x521aec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x521af0 — __ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> boost::dynamic_pointer_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE")]
pub fn stub_521af0(base: Option<SharedPtr<GuiDescribedBase>>) -> Option<SharedPtr<TextDisplay>> {
    // IDA 0x521af0: null pi (0x521b1c) or failed __dynamic_cast
    // DescribedBase→TextDisplay → empty out (0x521b30); success stores the
    // adjusted pointer and shares the count (0x521b1e..0x521b26, host: Arc).
    // Host: the kind tag plays typeinfo; the stored value plays the adjusted
    // pointer the aliasing share adopts.
    let base = base?;
    if base.kind != GuiDescribedKind::TextDisplay {
        return None;
    }
    let text = base.text.clone()?;
    Some(SharedPtr::new(text))
}

// 0x521b38 — __ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_521b38(display: EquationDisplay) -> SharedPtr<EquationDisplay> {
    // IDA 0x521b38: store px (0x521b58), shared_count<Creatable::Deleter>
    // (0x521b60), then _internal_accept_owner at +40 when non-null
    // (0x521b8e..0x521b9e, host: Arc construction adopts owners).
    SharedPtr::new(display)
}

// 0x521c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::EquationDisplay,RBX::EquationDisplay>(rbx_core::SharedPtr<RBX::EquationDisplay> const*,RBX::EquationDisplay *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_521c00() {
    // IDA 0x521c00: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x521ce8 — __ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_521ce8() {
    // IDA 0x521ce8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x521df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_521df0() {
    // IDA 0x521df0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x521df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_521df4() {
    // IDA 0x521df4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x521df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_521df8() {
    // IDA 0x521df8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x521e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_521e18() {
    // IDA 0x521e18: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x521e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_521e30() {
    // IDA 0x521e30: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x521e34 — __ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_521e34(display: TextDisplay) -> SharedPtr<TextDisplay> {
    // IDA 0x521e34: store px (0x521e54), shared_count<Creatable::Deleter>
    // (0x521e5c), then _internal_accept_owner at +40 when non-null
    // (0x521e8a..0x521e9a, host: Arc construction adopts owners).
    SharedPtr::new(display)
}

// 0x521efc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextDisplay,RBX::TextDisplay>(rbx_core::SharedPtr<RBX::TextDisplay> const*,RBX::TextDisplay *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_521efc() {
    // IDA 0x521efc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x521fe4 — __ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_521fe4() {
    // IDA 0x521fe4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5220ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_5220ec() {
    // IDA 0x5220ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5220f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_5220f0() {
    // IDA 0x5220f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5220f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_5220f4() {
    // IDA 0x5220f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x522114 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_522114() {
    // IDA 0x522114: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x52212c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_52212c() {
    // IDA 0x52212c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x522130 — __ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi
#[doc(alias = "RBX::UnifiedImageWidget::UnifiedImageWidget(RBX::Adorn *,std::string const&,int)")]
#[doc(alias = "__ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi")]
pub fn stub_522130(adorn: Adorn, name: &str, id: i32) -> UnifiedImageWidget {
    // IDA 0x522130: UnifiedWidget C2 base (0x52215a), vtable installs
    // (0x522170..0x52219a, host: type wiring), GuiDrawImage subobject at +116
    // (0x5221d0), name copy at +200 (0x5221de), id at +204 (0x5221e6).
    let image = stub_522c04(adorn, name, id as u32);
    UnifiedImageWidget { base: stub_522320(), image, image_name: name.to_owned(), flags: id }
}

// 0x522258 — __ZNK3RBX7GuiItem12getClassNameEv
#[doc(alias = "RBX::GuiItem::getClassName(void)const")]
#[doc(alias = "__ZNK3RBX7GuiItem12getClassNameEv")]
pub fn stub_522258(_item: &GuiItem) -> &'static str {
    // IDA 0x522258: thunk (B.W) into RBX::Name::getNullName — the base
    // GuiItem carries no class name, so the null name (host: "") is returned.
    ""
}

// 0x52225c — __ZN3RBX13UnifiedWidget12canLoseFocusEv
#[doc(alias = "RBX::UnifiedWidget::canLoseFocus(void)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget12canLoseFocusEv")]
pub fn stub_52225c(_widget: &UnifiedWidget) -> bool {
    // IDA 0x52225c: MOVS R0, #1; BX LR.
    true
}

// 0x522260 — __ZNK3RBX7GuiItem11getPositionENS_6CanvasE
#[doc(alias = "RBX::GuiItem::getPosition(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX7GuiItem11getPositionENS_6CanvasE")]
pub fn stub_522260(item: &GuiItem, canvas: Canvas) -> [f32; 2] {
    // IDA 0x522260: FLog::Asserts-gated getGuiParent() ReleaseAssert (GUI.h:64,
    // 0x52226a..0x5222c2); parent = getGuiParent() (0x5222c4); then the parent
    // vtable +140 slot with the canvas floats (0x5222d4..0x5222ee, host:
    // canvas-relative projection below).
    let _parent = item.parent.as_ref().expect("RBX::GuiItem::getGuiParent()");
    [item.position[0] + canvas.offset[0], item.position[1] + canvas.offset[1]]
}

// 0x5222f0 — __ZNK3RBX13UnifiedWidget11getFontSizeEv
#[doc(alias = "RBX::UnifiedWidget::getFontSize(void)const")]
#[doc(alias = "__ZNK3RBX13UnifiedWidget11getFontSizeEv")]
pub fn stub_5222f0(_widget: &UnifiedWidget) -> i32 {
    // IDA 0x5222f0: MOVS R0, #8; BX LR.
    8
}

// 0x5222f4 — __ZN3RBX7GuiItem8getTitleEv
#[doc(alias = "RBX::GuiItem::getTitle(void)")]
#[doc(alias = "__ZN3RBX7GuiItem8getTitleEv")]
pub fn stub_5222f4(item: &GuiItem) -> String {
    // IDA 0x5222f4: Instance::fw (0x5222fc) then string copy of the name word
    // at +24 (0x52230a, host: stored title).
    item.title.clone()
}

// 0x52230c — __ZNK3RBX18UnifiedImageWidget7getSizeENS_6CanvasE
#[doc(alias = "RBX::UnifiedImageWidget::getSize(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX18UnifiedImageWidget7getSizeENS_6CanvasE")]
pub fn stub_52230c(widget: &UnifiedImageWidget) -> [f32; 2] {
    // IDA 0x52230c: tail-calls GuiDrawImage::getImageSize on the subobject at
    // +116 (R1 += 0x74, 0x52230e..0x522316, host: stored image size).
    widget.image.image_size
}

// 0x522318 — __ZN3RBX13UnifiedWidget18onMenuStateChangedEv
#[doc(alias = "RBX::UnifiedWidget::onMenuStateChanged(void)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget18onMenuStateChangedEv")]
pub fn stub_522318(_widget: &UnifiedWidget) {
    // IDA 0x522318: BX LR — empty override.
}

// 0x52231c — __ZThn32_NK3RBX7GuiItem12getClassNameEv
#[doc(alias = "non-virtual thunk toRBX::GuiItem::getClassName(void)const")]
#[doc(alias = "__ZThn32_NK3RBX7GuiItem12getClassNameEv")]
pub fn stub_52231c() {
    // IDA 0x52231c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522320 — __ZN3RBX13UnifiedWidgetC2Ev
#[doc(alias = "RBX::UnifiedWidget::UnifiedWidget(void)")]
#[doc(alias = "__ZN3RBX13UnifiedWidgetC2Ev")]
pub fn stub_522320() -> UnifiedWidget {
    // IDA 0x522320: GuiItem C2 base (0x522340), vtable installs
    // (0x522356..0x522384, host: type wiring), init (0x5223aa, host: defaults).
    UnifiedWidget::default()
}

// 0x522408 — __ZN3RBX18UnifiedImageWidgetD1Ev
#[doc(alias = "RBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZN3RBX18UnifiedImageWidgetD1Ev")]
pub fn stub_522408() {
    // IDA 0x522408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522500 — __ZN3RBX18UnifiedImageWidgetD0Ev
#[doc(alias = "RBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZN3RBX18UnifiedImageWidgetD0Ev")]
pub fn stub_522500() {
    // IDA 0x522500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522608 — __ZThn32_N3RBX18UnifiedImageWidgetD1Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZThn32_N3RBX18UnifiedImageWidgetD1Ev")]
pub fn stub_522608() {
    // IDA 0x522608: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5226fc — __ZThn32_N3RBX18UnifiedImageWidgetD0Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZThn32_N3RBX18UnifiedImageWidgetD0Ev")]
pub fn stub_5226fc() {
    // IDA 0x5226fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522808 — __ZThn36_N3RBX18UnifiedImageWidgetD1Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZThn36_N3RBX18UnifiedImageWidgetD1Ev")]
pub fn stub_522808() {
    // IDA 0x522808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5228fc — __ZThn36_N3RBX18UnifiedImageWidgetD0Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedImageWidget::~UnifiedImageWidget()")]
#[doc(alias = "__ZThn36_N3RBX18UnifiedImageWidgetD0Ev")]
pub fn stub_5228fc() {
    // IDA 0x5228fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522a08 — __ZN3RBX12GuiDrawImageD2Ev
#[doc(alias = "RBX::GuiDrawImage::~GuiDrawImage()")]
#[doc(alias = "__ZN3RBX12GuiDrawImageD2Ev")]
pub fn stub_522a08() {
    // IDA 0x522a08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522c04 — __ZN3RBX12GuiDrawImageC2EPNS_5AdornERKSsj
#[doc(alias = "RBX::GuiDrawImage::GuiDrawImage(RBX::Adorn *,std::string const&,unsigned int)")]
#[doc(alias = "__ZN3RBX12GuiDrawImageC2EPNS_5AdornERKSsj")]
pub fn stub_522c04(_adorn: Adorn, name: &str, id: u32) -> GuiDrawImage {
    // IDA 0x522c04: two empty strings + two null names (0x522c46..0x522c8c),
    // zeroed size/pos quads (0x522c9a..0x522cb8, host: zero image_size), then
    // setImageFromName(adorn, name, id) (0x522cc4, host: name/id stored).
    GuiDrawImage { image_name: name.to_owned(), image_id: id, image_size: [0.0, 0.0] }
}

// 0x522de8 — __ZN3RBX13UnifiedWidgetD1Ev
#[doc(alias = "RBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZN3RBX13UnifiedWidgetD1Ev")]
pub fn stub_522de8() {
    // IDA 0x522de8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522dec — __ZN3RBX13UnifiedWidgetD0Ev
#[doc(alias = "RBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZN3RBX13UnifiedWidgetD0Ev")]
pub fn stub_522dec() {
    // IDA 0x522dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522e8c — __ZNK3RBX7GuiItem7getSizeENS_6CanvasE
#[doc(alias = "RBX::GuiItem::getSize(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX7GuiItem7getSizeENS_6CanvasE")]
pub fn stub_522e8c(item: &GuiItem, _canvas: Canvas) -> [f32; 2] {
    // IDA 0x522e8c: copies the two dwords at +104/+108 into the out pair
    // (0x522e8e..0x522e94, host: stored size; canvas unused).
    item.size
}

// 0x522e98 — __ZThn32_N3RBX13UnifiedWidgetD1Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZThn32_N3RBX13UnifiedWidgetD1Ev")]
pub fn stub_522e98() {
    // IDA 0x522e98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522ea0 — __ZThn32_N3RBX13UnifiedWidgetD0Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZThn32_N3RBX13UnifiedWidgetD0Ev")]
pub fn stub_522ea0() {
    // IDA 0x522ea0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522f44 — __ZThn36_N3RBX13UnifiedWidgetD1Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZThn36_N3RBX13UnifiedWidgetD1Ev")]
pub fn stub_522f44() {
    // IDA 0x522f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522f4c — __ZThn36_N3RBX13UnifiedWidgetD0Ev
#[doc(alias = "non-virtual thunk toRBX::UnifiedWidget::~UnifiedWidget()")]
#[doc(alias = "__ZThn36_N3RBX13UnifiedWidgetD0Ev")]
pub fn stub_522f4c() {
    // IDA 0x522f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x522ff0 — __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_522ff0(button: ChatButton) -> SharedPtr<ChatButton> {
    // IDA 0x522ff0: store px (0x523010), shared_count<Creatable::Deleter>
    // (0x523018), then _internal_accept_owner at +40 when non-null
    // (0x523046..0x523056, host: Arc construction adopts owners).
    SharedPtr::new(button)
}

// 0x5230b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatButton,RBX::ChatButton>(rbx_core::SharedPtr<RBX::ChatButton> const*,RBX::ChatButton *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_5230b8() {
    // IDA 0x5230b8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x5231a0 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_5231a0() {
    // IDA 0x5231a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5232a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_5232a8() {
    // IDA 0x5232a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5232ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_5232ac() {
    // IDA 0x5232ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5232b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_5232b0() {
    // IDA 0x5232b0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5232d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_5232d0() {
    // IDA 0x5232d0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5232e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5232e8() {
    // IDA 0x5232e8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5232ec — __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_5232ec(widget: ChatWidget) -> SharedPtr<ChatWidget> {
    // IDA 0x5232ec: store px (0x52330c), shared_count<Creatable::Deleter>
    // (0x523314), then _internal_accept_owner at +40 when non-null
    // (0x523342..0x523352, host: Arc construction adopts owners).
    SharedPtr::new(widget)
}

// 0x5233b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatWidget,RBX::ChatWidget>(rbx_core::SharedPtr<RBX::ChatWidget> const*,RBX::ChatWidget *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_5233b4() {
    // IDA 0x5233b4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x52349c — __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_52349c() {
    // IDA 0x52349c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5235a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_5235a4() {
    // IDA 0x5235a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5235a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_5235a8() {
    // IDA 0x5235a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5235ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_5235ac() {
    // IDA 0x5235ac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5235cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_5235cc() {
    // IDA 0x5235cc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5235e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5235e4() {
    // IDA 0x5235e4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5235e8 — __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_5235e8(output: ChatOutput) -> SharedPtr<ChatOutput> {
    // IDA 0x5235e8: store px (0x523608), shared_count<Creatable::Deleter>
    // (0x523610), then _internal_accept_owner at +40 when non-null
    // (0x52363e..0x52364e, host: Arc construction adopts owners).
    SharedPtr::new(output)
}

// 0x5236b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatOutput,RBX::ChatOutput>(rbx_core::SharedPtr<RBX::ChatOutput> const*,RBX::ChatOutput *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_5236b0() {
    // IDA 0x5236b0: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x523798 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_523798() {
    // IDA 0x523798: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5238a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_5238a0() {
    // IDA 0x5238a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5238a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_5238a4() {
    // IDA 0x5238a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5238a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_5238a8() {
    // IDA 0x5238a8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5238c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_5238c8() {
    // IDA 0x5238c8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5238e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5238e0() {
    // IDA 0x5238e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5238e4 — __ZN3RBX13RelativePanelC2ERKNS_6LayoutE
#[doc(alias = "RBX::RelativePanel::RelativePanel(RBX::Layout const&)")]
#[doc(alias = "__ZN3RBX13RelativePanelC2ERKNS_6LayoutE")]
pub fn stub_5238e4(layout: Layout) -> RelativePanel {
    // IDA 0x5238e4: TopMenuBar C2 base (0x523906), vtable installs
    // (0x52391c..0x52393a, host: type wiring), zeroed words at +144/+146
    // (0x52394a..0x523954, host: defaults), init(layout) (0x52397e, host:
    // layout moved in).
    RelativePanel { base: stub_5239dc(), layout }
}

// 0x5239dc — __ZN3RBX10TopMenuBarC2Ev
#[doc(alias = "RBX::TopMenuBar::TopMenuBar(void)")]
#[doc(alias = "__ZN3RBX10TopMenuBarC2Ev")]
pub fn stub_5239dc() -> TopMenuBar {
    // IDA 0x5239dc: GuiItem C2 base (0x5239fc), vtable installs
    // (0x523a12..0x523a3e, host: type wiring), init (0x523a64, host: defaults).
    TopMenuBar::default()
}

// 0x523ac0 — __ZN3RBX7GuiItem11onLoseFocusEv
#[doc(alias = "RBX::GuiItem::onLoseFocus(void)")]
#[doc(alias = "__ZN3RBX7GuiItem11onLoseFocusEv")]
pub fn stub_523ac0(_item: &GuiItem) {
    // IDA 0x523ac0: BX LR — empty override.
}

// 0x523ac4 — __ZNK3RBX7GuiItem11getFontSizeEv
#[doc(alias = "RBX::GuiItem::getFontSize(void)const")]
#[doc(alias = "__ZNK3RBX7GuiItem11getFontSizeEv")]
pub fn stub_523ac4(_item: &GuiItem) -> i32 {
    // IDA 0x523ac4: MOVS R0, #0xC; BX LR.
    12
}

// 0x523ac8 — __ZNK3RBX10TopMenuBar9isVisibleEv
#[doc(alias = "RBX::TopMenuBar::isVisible(void)const")]
#[doc(alias = "__ZNK3RBX10TopMenuBar9isVisibleEv")]
pub fn stub_523ac8(bar: &TopMenuBar) -> bool {
    // IDA 0x523ac8: LDRB R0, [R0, #0x84] — visibility byte at +132.
    bar.visible
}

// 0x523ad0 — __ZN3RBX10TopMenuBarD1Ev
#[doc(alias = "RBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZN3RBX10TopMenuBarD1Ev")]
pub fn stub_523ad0() {
    // IDA 0x523ad0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523ad4 — __ZN3RBX10TopMenuBarD0Ev
#[doc(alias = "RBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZN3RBX10TopMenuBarD0Ev")]
pub fn stub_523ad4() {
    // IDA 0x523ad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523b74 — __ZThn32_N3RBX10TopMenuBarD1Ev
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZThn32_N3RBX10TopMenuBarD1Ev")]
pub fn stub_523b74() {
    // IDA 0x523b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523b7c — __ZThn32_N3RBX10TopMenuBarD0Ev
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZThn32_N3RBX10TopMenuBarD0Ev")]
pub fn stub_523b7c() {
    // IDA 0x523b7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523c20 — __ZThn36_N3RBX10TopMenuBarD1Ev
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZThn36_N3RBX10TopMenuBarD1Ev")]
pub fn stub_523c20() {
    // IDA 0x523c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523c28 — __ZThn36_N3RBX10TopMenuBarD0Ev
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
#[doc(alias = "__ZThn36_N3RBX10TopMenuBarD0Ev")]
pub fn stub_523c28() {
    // IDA 0x523c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523ccc — __ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_523ccc(panel: RelativePanel) -> SharedPtr<RelativePanel> {
    // IDA 0x523ccc: store px (0x523cec), shared_count<Creatable::Deleter>
    // (0x523cf4), then _internal_accept_owner at +40 when non-null
    // (0x523d22..0x523d32, host: Arc construction adopts owners).
    SharedPtr::new(panel)
}

// 0x523d94 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RelativePanel,RBX::RelativePanel>(rbx_core::SharedPtr<RBX::RelativePanel> const*,RBX::RelativePanel *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_523d94() {
    // IDA 0x523d94: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x523e7c — __ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_523e7c() {
    // IDA 0x523e7c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x523f84 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_523f84() {
    // IDA 0x523f84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_523f88() {
    // IDA 0x523f88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x523f8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_523f8c() {
    // IDA 0x523f8c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x523fac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_523fac() {
    // IDA 0x523fac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x523fc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_523fc4() {
    // IDA 0x523fc4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x523fc8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_523fc8() {
    // IDA 0x523fc8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x524088 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
pub fn stub_524088<'a>(map: &'a BTreeMap<String, GuiBuilderData>, key: &str) -> Option<&'a GuiBuilderData> {
    // IDA 0x524088: lower_bound walk with string::compare (0x5240a4..0x5240bc),
    // then the equality re-check (0x5240cc); end() when missing (0x5240d6,
    // host: None). Host: BTreeMap::get is the same lower_bound + equality check.
    map.get(key)
}

// 0x5240d8 — __ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv")]
pub fn stub_5240d8() -> bool {
    // IDA 0x5240d8: FLog::Asserts-gated className().empty() == (sClassName ==
    // NULL) check (0x5240fa..0x52412c, host: statically consistent); returns
    // sCoreGuiService == NULL (0x524174, host: the static is "CoreGui").
    S_CORE_GUI_SERVICE.is_none()
}

// 0x524178 — __ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v")]
pub fn stub_524178() -> &'static str {
    // IDA 0x524178: null sCoreGuiService string → getNullName
    // (0x5241b2..0x5241b6, host: ""); else boost::call_once(flag,
    // callDoDeclare) (0x52418e..0x5241a6) then the doDeclare shim (0x5241ae,
    // host: delegate below).
    match S_CORE_GUI_SERVICE {
        Some(_) => stub_5241bc(),
        None => "",
    }
}

// 0x5241bc — __ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v")]
pub fn stub_5241bc() -> &'static str {
    // IDA 0x5241bc: __cxa_guard_acquire/release once-init (0x524218..0x524244)
    // of the function-local static via Name::declare(&sCoreGuiService, 1)
    // (0x524240, host: LazyLock over the same value).
    static NAME: LazyLock<&'static str> = LazyLock::new(|| S_CORE_GUI_SERVICE.unwrap_or(""));
    *NAME
}

// 0x5242a0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CoreGuiService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv")]
pub fn stub_5242a0() -> u64 {
    // IDA 0x5242a0: guard-variable once-init (0x5242fc..0x52431c) with
    // index = ServiceProvider::newIndex(1) (0x524318, host: process-wide
    // counter below).
    static INDEX: LazyLock<u64> = LazyLock::new(new_service_index);
    *INDEX
}

/// Host half of `ServiceProvider::newIndex` (IDA 0x524318): process-wide
/// counter handing out fresh class indices.
fn new_service_index() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

// 0x524378 — __ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_524378() {
    // IDA 0x524378: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524480 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_524480() {
    // IDA 0x524480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524484 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_524484() {
    // IDA 0x524484: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5244a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_5244a4() {
    // IDA 0x5244a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5244bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5244bc() {
    // IDA 0x5244bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5244c0 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_5244c0(button: GuiImageButton) -> SharedPtr<GuiImageButton> {
    // IDA 0x5244c0: store px (0x5244e0), shared_count<Creatable::Deleter>
    // (0x5244e8), then _internal_accept_owner at +40 when non-null
    // (0x524516..0x524526, host: Arc construction adopts owners).
    SharedPtr::new(button)
}

// 0x524588 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiImageButton,RBX::GuiImageButton>(rbx_core::SharedPtr<RBX::GuiImageButton> const*,RBX::GuiImageButton *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_524588() {
    // IDA 0x524588: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x524670 — __ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_524670() {
    // IDA 0x524670: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
