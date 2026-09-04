//! rendering shard 478 — 100 stubs 0x7a19f4..0x87b74b EA-sorted asc global gap filler (Ogre|G3D|Gfx|Render|Adorn, 15618 total, 0 in global 64586, 100 this batch = 100 new for global dedup, rbx_core::SharedPtr not boost)
//! Source: ida/export.json (85545 funcs) EA asc global dedup — next 100 rendering not in /tmp/global_eas.txt
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use crate::generated_03::BillboardRenderCell;
use crate::generated_141::AdornHandle;
use std::cell::Cell;

/// `RBX::Adorn` 2D virtuals touched by this shard; vtable slots from IDA:
/// +32 viewport extent (`0x7aa7e0`), +48 stroked rect (`0x7adf32`),
/// +64 filled rect (`0x7ade1a`), +76 font draw (`0x7ad36e`),
/// +80 `drawFont2D` (`0x7a9c44`).
pub trait Adorn2d {
    fn viewport_extent(&self) -> [f32; 4];
    fn fill_rect(&mut self, rect: [f32; 4], color: [f32; 4]);
    fn stroke_rect(&mut self, rect: [f32; 4], thickness: f32, color: [f32; 4]);
    fn draw_font_2d(&mut self, args: &FontDraw2dArgs) -> i32;
    /// `RBX::renderClassicChatBox` sink (IDA `0x7a253c`).
    fn draw_chat_box(&mut self, pos: [f32; 2], lines: usize, backdrop: [f32; 4]);
}

/// `drawFont2D` argument bundle (IDA `0x7a9c00`: text, pos, size, two colors,
/// font, x/y align, offset, clip).
#[derive(Clone, Debug, Default)]
pub struct FontDraw2dArgs {
    pub text: String,
    pub pos: [f32; 2],
    pub size: f32,
    pub color: [f32; 4],
    pub shadow: [f32; 4],
    pub font: u32,
    pub x_align: i32,
    pub y_align: i32,
    pub offset: [f32; 2],
    /// Default `-1.0` pad word (`0xBF800000`, IDA `0x7ad39a`).
    pub pad: [f32; 4],
}

impl FontDraw2dArgs {
    pub fn new(text: &str, pos: [f32; 2], color: [f32; 4], shadow: [f32; 4], x_align: i32) -> Self {
        Self {
            text: text.to_owned(),
            pos,
            color,
            shadow,
            x_align,
            pad: [-1.0; 4],
            ..Default::default()
        }
    }
}

/// `render2d` virtual at vtable +164 (`0x7ad70e`, `0x7adffa`, `0x7ade76`).
pub trait Render2dChild {
    fn render2d(&self, adorn: &mut dyn Adorn2d);
}

/// Instance child window: `numChildren` + `getGuiItem(i)` with the null skip
/// (`0x7ad6f0..0x7ad714`, `0x7adfdc..0x7ae000`, `0x7ade5c..0x7ade7c`).
pub trait GuiChildHost {
    fn child_count(&self) -> usize;
    fn child(&self, index: usize) -> Option<SharedPtr<dyn Render2dChild>>;
}

/// Shared tail of `GuiRoot::render2d`, `TopMenuBar::render2d` and
/// `UnifiedWidget::render2dChildren`: `numChildren` is re-read every step so
/// a child that adds/removes siblings is observed (IDA refreshes `result`
/// at `0x7ad714` / `0x7ae000` / `0x7ade7c`).
pub fn render_child_list(host: &dyn GuiChildHost, adorn: &mut dyn Adorn2d) {
    let mut i = 0;
    while i < host.child_count() {
        if let Some(item) = host.child(i) {
            item.render2d(adorn);
        }
        i += 1;
    }
}

/// Loopback `render2d` child behind the `+164` dispatch in the child loops.
#[derive(Debug, Default)]
pub struct GuiRenderCounter {
    pub draws: Cell<usize>,
}

impl Render2dChild for GuiRenderCounter {
    fn render2d(&self, _adorn: &mut dyn Adorn2d) {
        self.draws.set(self.draws.get() + 1);
    }
}

/// `RBX::ChatOutput` chat-style selector: mode word at `*(this + 28)` / `+220`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChatStyleMode(pub Option<u32>);

impl ChatStyleMode {
    /// IDA `0x7a1a0c`: `(mode - 1) < 2` — bubble style for modes 1..=2.
    pub fn bubble_enabled(self) -> bool {
        matches!(self.0, Some(m) if m.wrapping_sub(1) < 2)
    }
    /// IDA `0x7a1a1e..0x7a1a26`: `(mode & !2) != 0`.
    pub fn classic_enabled(self) -> bool {
        matches!(self.0, Some(m) if (m & 0xFFFF_FFFD) != 0)
    }
}

/// Speaker behind one `CharacterChats` entry, classified by
/// `ClassDescriptor::isA` against Model/Part (IDA `0x7a1c5a`, `0x7a1f4a`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpeakerRef {
    #[default]
    Unknown,
    Model(u32),
    Part(u32),
}

impl SpeakerRef {
    pub fn id(self) -> u32 {
        match self {
            SpeakerRef::Model(id) | SpeakerRef::Part(id) => id,
            SpeakerRef::Unknown => 0,
        }
    }
}

/// One `CharacterChats` map entry (`this + 72` tree, IDA `0x7a1a38`).
#[derive(Default)]
pub struct BubbleChatEntry {
    /// Billboard whose render function is cleared then rebound.
    pub billboard: BillboardRenderCell,
    /// The two drawn-flag bytes zeroed at `0x7a1bc4`/`0x7a1bc8`.
    pub drawn: [bool; 2],
    pub speaker: SpeakerRef,
    pub queued_lines: usize,
}

/// `ChatOutput` bubble-style world: `"Head"` part resolved via
/// `findConstFirstChildByName` (`0x7a1f18`), `Workspace` via
/// `ServiceProvider::find<Workspace>` (`0x7a1f8e`), camera frame via the
/// workspace vtable `+196` (`0x7a1dd8`).
#[derive(Default)]
pub struct ChatBubbleWorld {
    pub entries: Vec<BubbleChatEntry>,
    pub head_part: Option<u32>,
    pub workspace: Option<u32>,
}

/// `RBX::ChatOutput` classic-style inputs (IDA `0x7a2400`).
#[derive(Clone, Debug, Default)]
pub struct ClassicChatState {
    /// `FFlag::NativeChatRendering` (IDA `0x7a242a`; `BEQ` skips the body).
    pub native_chat_rendering: bool,
    /// `a3`: map `+0x114` entry when set, plain `+0xEC` deque otherwise.
    pub show_name_column: bool,
    pub named_lines: usize,
    pub plain_lines: usize,
    /// `translucentBackdrop` tint feeding `renderClassicChatBox`.
    pub backdrop: [f32; 4],
    pub position: [f32; 2],
}

impl ClassicChatState {
    /// Line deque selected by `a3` (IDA `0x7a2476`: `BNE` skips the map lookup
    /// when the flag is clear).
    pub fn active_lines(&self) -> usize {
        if self.show_name_column {
            self.named_lines
        } else {
            self.plain_lines
        }
    }
    /// `getMyRect` over the viewport-derived canvas (IDA `0x7a2468`).
    pub fn my_rect(&self, canvas: [f32; 2]) -> [f32; 4] {
        [self.position[0], self.position[1], canvas[0], canvas[1]]
    }
}

/// `RBX::ChatOutput` state fanning out to both style passes (IDA `0x7a19f4`).
#[derive(Default)]
pub struct ChatOutputState {
    pub mode: ChatStyleMode,
    pub bubbles: ChatBubbleWorld,
    pub classic: ClassicChatState,
}

/// `RBX::AdornBillboarder` draw gate (IDA `0x7a3f74`): `isVisibleAndValid`
/// decides between forwarding through the billboard sub-object (`+28`,
/// vtable slot 20) and clearing the out param.
#[derive(Debug, Default)]
pub struct BillboardDrawState {
    pub visible_and_valid: bool,
    pub forwarded_draws: u32,
}

/// `RBX::UnifiedImageWidget` fields for `render2dMe` (IDA `0x7aa7a8`):
/// image store at `+116`, name at `+200`, draw-mode word at `+28`.
#[derive(Clone, Debug, Default)]
pub struct UnifiedImageState {
    pub visible: bool,
    pub image_name: String,
    pub kind: u32,
}

/// `RBX::GuiDrawImage` image slot (`+116` store, IDA `0x7aa7d0`).
#[derive(Clone, Debug, Default)]
pub struct GuiDrawImageState {
    pub last_name: String,
    pub draws: u32,
}

impl GuiDrawImageState {
    /// `setImageFromName` (IDA `0x7aa7d0`): records the name; fails on an
    /// empty name so the caller keeps the previous frame.
    pub fn set_from_name(&mut self, _adorn: &mut dyn Adorn2d, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        self.last_name = name.to_owned();
        true
    }
    /// `getMyRect` over the viewport-derived canvas (IDA `0x7aa80c`).
    pub fn my_rect(&self, canvas: [f32; 2]) -> [f32; 4] {
        [0.0, 0.0, canvas[0], canvas[1]]
    }
    /// Full `GuiDrawImage::render2d` overload selected by the `0x7b15fc` /
    /// `0x7b163c` forwarders.
    pub fn draw(
        &mut self,
        _adorn: &mut dyn Adorn2d,
        _filter: bool,
        _rect: &[f32; 4],
        _mode: u32,
        _tile: u32,
    ) -> bool {
        self.draws += 1;
        true
    }
}

/// IDA `0x7aa810..0x7aa822`: mode 2 when `kind - 2 < 2`, else `kind == 1`.
pub fn image_draw_mode(kind: u32) -> u32 {
    if kind.wrapping_sub(2) < 2 {
        2
    } else {
        u32::from(kind == 1)
    }
}

/// `XAlign` values (IDA `0x7ad344` / `0x7ad358` / `LABEL_7`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelAnchor {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

/// `GuiItem` label fields shared by `EquationDisplay::render2d` (`0x7abef4`)
/// and `TextDisplay::render2d` (`0x7ae9e8`): text at `+28`, colors at
/// `+120`/`+136`, `XAlign` at `+38`.
#[derive(Clone, Debug, Default)]
pub struct LabeledWidget {
    /// Visibility virtual, vtable +148.
    pub visible: bool,
    pub text: String,
    pub rect: [f32; 4],
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub align: LabelAnchor,
}

/// `G3D::Color4::clear()` reference tint (IDA `0x7addc4`).
pub const CLEAR_TINT: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
/// `G3D::Color3::white()` + opaque alpha (IDA `0x7adf0e..0x7adf1e`).
pub const WHITE_TINT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// `G3D::Color3::black()` + opaque alpha (IDA `0x7adf6a..0x7adf88`).
pub const BLACK_TINT: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// `getMyRect2D` over the viewport-derived canvas (IDA `0x7ade4e`).
pub fn my_rect_2d(canvas: [f32; 2]) -> [f32; 4] {
    [0.0, 0.0, canvas[0], canvas[1]]
}

/// Viewport extent minus origin (the `VSUB` pair at `0x7aa800`, `0x7ade42`).
pub fn canvas_from_extent(extent: [f32; 4]) -> [f32; 2] {
    [extent[2] - extent[0], extent[3] - extent[1]]
}

/// `RBX::TopMenuBar` tint at `+112` (IDA `0x7adda8`).
#[derive(Default)]
pub struct TopMenuBarState {
    /// Visibility virtual, vtable +148.
    pub visible: bool,
    pub tint: [f32; 4],
    pub children: Vec<SharedPtr<dyn Render2dChild>>,
}

impl GuiChildHost for TopMenuBarState {
    fn child_count(&self) -> usize {
        self.children.len()
    }
    fn child(&self, index: usize) -> Option<SharedPtr<dyn Render2dChild>> {
        self.children.get(index).cloned()
    }
}

/// `RBX::UnifiedWidget` shared state: `+28` token word selects the
/// `menuSelect` tint in `render2dMe` (`0x7adeee`) and gates
/// `render2dChildren` on `>= 2` (`0x7adfd8`); label comes from `fw(this)`
/// (`0x7adf94`) with `black`/`clear` colors (`0x7adf96..0x7adfaa`).
#[derive(Default)]
pub struct UnifiedWidgetState {
    pub token: u32,
    /// `dword_1329C78..84` selected tint (IDA `0x7adf00..0x7adf0a`).
    pub highlight: [f32; 4],
    pub label: LabeledWidget,
    pub menu_selects: u32,
    pub children: Vec<SharedPtr<dyn Render2dChild>>,
}

impl UnifiedWidgetState {
    /// `GuiItem::menuSelect` (IDA `0x7adef0`).
    pub fn on_menu_select(&mut self) {
        self.menu_selects += 1;
    }
}

impl GuiChildHost for UnifiedWidgetState {
    fn child_count(&self) -> usize {
        self.children.len()
    }
    fn child(&self, index: usize) -> Option<SharedPtr<dyn Render2dChild>> {
        self.children.get(index).cloned()
    }
}

/// Loopback `Adorn2d` recording every call, for tests and loopback use.
#[derive(Debug, Default)]
pub struct TestAdorn {
    pub viewport: [f32; 4],
    pub fills: Vec<([f32; 4], [f32; 4])>,
    pub strokes: Vec<([f32; 4], f32, [f32; 4])>,
    pub fonts: Vec<FontDraw2dArgs>,
    pub chat_boxes: Vec<([f32; 2], usize, [f32; 4])>,
    pub font_result: i32,
}

impl Adorn2d for TestAdorn {
    fn viewport_extent(&self) -> [f32; 4] {
        self.viewport
    }
    fn fill_rect(&mut self, rect: [f32; 4], color: [f32; 4]) {
        self.fills.push((rect, color));
    }
    fn stroke_rect(&mut self, rect: [f32; 4], thickness: f32, color: [f32; 4]) {
        self.strokes.push((rect, thickness, color));
    }
    fn draw_font_2d(&mut self, args: &FontDraw2dArgs) -> i32 {
        self.fonts.push(args.clone());
        self.font_result
    }
    fn draw_chat_box(&mut self, pos: [f32; 2], lines: usize, backdrop: [f32; 4]) {
        self.chat_boxes.push((pos, lines, backdrop));
    }
}


// 0x7a19f4 — __ZN3RBX10ChatOutput8render2dEPNS_5AdornE
#[doc(alias = "RBX::ChatOutput::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX10ChatOutput8render2dEPNS_5AdornE")]
// IDA 0x7a19f4: bubble pass gated by `(mode - 1) < 2` (0x7a1a0c), classic pass
// gated by `(mode & !2) != 0` (0x7a1a1e..0x7a1a26); returns the classic result.
pub fn stub_0x7a19f4(output: &mut ChatOutputState, adorn: &mut dyn Adorn2d) {
    stub_0x7a1a38(&mut output.bubbles, output.mode.bubble_enabled());
    stub_0x7a2400(&output.classic, adorn, output.mode.classic_enabled());
}

// 0x7a1a38 — __ZN3RBX10ChatOutput20render2d_bubbleStyleEPNS_5AdornEb
#[doc(alias = "RBX::ChatOutput::render2d_bubbleStyle(RBX::Adorn *,bool)")]
#[doc(alias = "__ZN3RBX10ChatOutput20render2d_bubbleStyleEPNS_5AdornEb")]
// IDA 0x7a1a38: two phases over the `CharacterChats` map (`this + 72`).
// Phase 1 (0x7a1b86..0x7a1bc8) drops every live billboard render function
// (`setRenderFunction(empty)` + `clear`) and zeroes the drawn flags.
// Phase 2 (0x7a1bf0..0x7a1eac) resolves each queued entry's speaker
// (Model/Part `isA` + `"Head"` child + `ServiceProvider::find<Workspace>`)
// and — when head part + workspace are live, or `force` (a3) skips the
// visible-line scan (0x7a1d80) — binds `renderBubbleImposters` with weak
// instance/part refs (0x7a1e68..0x7a1eac).
pub fn stub_0x7a1a38(world: &mut ChatBubbleWorld, force: bool) {
    for entry in world.entries.iter_mut() {
        entry.billboard.render_fn = None;
        entry.drawn = [false, false];
    }
    for entry in world.entries.iter_mut() {
        if entry.queued_lines == 0 {
            continue;
        }
        let bound = match (world.head_part, world.workspace) {
            (Some(_), Some(_)) => true,
            _ => force,
        };
        if !bound {
            continue;
        }
        let weak_instance = entry.speaker.id();
        entry.billboard.render_fn = Some(Box::new(move |_gui: usize, _adorn: usize| {
            let _ = weak_instance;
        }));
    }
}

// 0x7a2400 — __ZN3RBX10ChatOutput21render2d_classicStyleEPNS_5AdornEb
#[doc(alias = "RBX::ChatOutput::render2d_classicStyle(RBX::Adorn *,bool)")]
#[doc(alias = "__ZN3RBX10ChatOutput21render2d_classicStyleEPNS_5AdornEb")]
// IDA 0x7a2400: `FFlag::NativeChatRendering` gate (0x7a242a, `BEQ` skips the
// body); viewport extent via the Adorn +32 virtual, `getMyRect` (0x7a2468),
// then the a3-selected deque (`+0x114` map entry when set, `+0xEC` fields
// otherwise, 0x7a2476 `BNE`) drawn with the `translucentBackdrop` tint via
// `renderClassicChatBox` (0x7a253c).
pub fn stub_0x7a2400(state: &ClassicChatState, adorn: &mut dyn Adorn2d, show_name: bool) {
    if !state.native_chat_rendering {
        return;
    }
    let canvas = canvas_from_extent(adorn.viewport_extent());
    let _rect = state.my_rect(canvas);
    let mut view = state.clone();
    view.show_name_column = show_name;
    adorn.draw_chat_box(view.position, view.active_lines(), view.backdrop);
}

// 0x7a3f74 — __ZN3RBX16AdornBillboarder10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
#[doc(alias = "RBX::AdornBillboarder::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX16AdornBillboarder10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE")]
// IDA 0x7a3f74: `isVisibleAndValid` (0x7a3f88) decides between forwarding
// through the billboard sub-object (`+28`, vtable slot 20 at 0x7a3fd6) and
// clearing the out param (0x7a3fde).
pub fn stub_0x7a3f74(board: &mut BillboardDrawState, drawn: &mut i32) -> bool {
    if board.visible_and_valid {
        board.forwarded_draws += 1;
        return true;
    }
    *drawn = 0;
    false
}

// 0x7a9b58 — __ZN3RBX5AdornD0Ev
#[doc(alias = "RBX::Adorn::~Adorn()")]
#[doc(alias = "__ZN3RBX5AdornD0Ev")]
// IDA 0x7a9b58: D0 deleting destructor — `Adorn::~Adorn(this)` (0x7a9ba8)
// then `operator delete` (0x7a9bae); maps to dropping the owned handle.
pub fn stub_0x7a9b58(handle: AdornHandle) {
    drop(handle);
}

// 0x7a9bf8 — __ZN3RBX5Adorn17prepareRenderPassEv
#[doc(alias = "RBX::Adorn::prepareRenderPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn17prepareRenderPassEv")]
// IDA 0x7a9bf8: base `prepareRenderPass` — empty body, single `BX` return.
pub fn stub_0x7a9bf8() {}

// 0x7a9bfc — __ZN3RBX5Adorn13preSubmitPassEv
#[doc(alias = "RBX::Adorn::preSubmitPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn13preSubmitPassEv")]
// IDA 0x7a9bfc: base `preSubmitPass` — empty body, single `BX` return.
pub fn stub_0x7a9bfc() {}

// 0x7a9c00 — __ZN3RBX5Adorn10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
#[doc(alias = "RBX::Adorn::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX5Adorn10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE")]
// IDA 0x7a9c00: base `drawFont2D` forwards every argument to vtable slot 20
// (Adorn +80, 0x7a9c44).
pub fn stub_0x7a9c00(adorn: &mut dyn Adorn2d, args: &FontDraw2dArgs) -> i32 {
    adorn.draw_font_2d(args)
}

// 0x7aa7a8 — __ZN3RBX18UnifiedImageWidget10render2dMeEPNS_5AdornE
#[doc(alias = "RBX::UnifiedImageWidget::render2dMe(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX18UnifiedImageWidget10render2dMeEPNS_5AdornE")]
// IDA 0x7aa7a8: visibility gate (slot +148, 0x7aa7ba), then
// `setImageFromName(+116 image, +200 name)` must succeed (0x7aa7d0); the
// viewport extent minus origin feeds `getMyRect` (0x7aa7e0..0x7aa80c) and the
// `+28` kind word selects the draw mode (0x7aa810..0x7aa822) for the full
// `GuiDrawImage::render2d` overload (0x7aa832).
pub fn stub_0x7aa7a8(
    state: &UnifiedImageState,
    image: &mut GuiDrawImageState,
    adorn: &mut dyn Adorn2d,
) -> bool {
    if !state.visible {
        return false;
    }
    if !image.set_from_name(adorn, &state.image_name) {
        return false;
    }
    let rect = image.my_rect(canvas_from_extent(adorn.viewport_extent()));
    image.draw(adorn, true, &rect, image_draw_mode(state.kind), 0)
}

// 0x7abe70 — __ZN3RBX15EquationDisplay8render2dEPNS_5AdornE
#[doc(alias = "RBX::EquationDisplay::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX15EquationDisplay8render2dEPNS_5AdornE")]
// IDA 0x7abe70: visibility gate (slot +148, 0x7abec8), then `getLabel`
// (0x7abed6) rendered via `GuiItem::label2d` with the `+120`/`+136` colors
// and `+38` align (0x7abef4); the label string is released on the way out
// (0x7abf06..0x7abf4c).
pub fn stub_0x7abe70(widget: &LabeledWidget, adorn: &mut dyn Adorn2d) -> bool {
    if !widget.visible {
        return false;
    }
    stub_0x7ad2b0(
        adorn,
        &widget.text,
        &widget.rect,
        &widget.color_a,
        &widget.color_b,
        widget.align,
    )
}

// 0x7ad2b0 — __ZNK3RBX7GuiItem7label2dEPNS_5AdornERKSsRKN3G3D6Color4ES8_NS_4Text6XAlignE
#[doc(alias = "RBX::GuiItem::label2d(RBX::Adorn *,std::string const&,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign)const")]
#[doc(alias = "__ZNK3RBX7GuiItem7label2dEPNS_5AdornERKSsRKN3G3D6Color4ES8_NS_4Text6XAlignE")]
// IDA 0x7ad2b0: empty-string early-out on the `std::string` length
// (0x7ad2c8/0x7ad2ce); viewport extent minus origin feeds `getMyRect`
// (0x7ad2dc..0x7ad308); the label is centered (`* 0.5`, 0x7ad338) then nudged
// by align — `-0.1` width for `Left` (0x7ad34e), `+0.1` for `Center`
// (0x7ad362), as-is for `Right` — before the slot +76 draw (0x7ad3c6) with
// the `-1.0` pad words (0x7ad39a).
pub fn stub_0x7ad2b0(
    adorn: &mut dyn Adorn2d,
    text: &str,
    rect: &[f32; 4],
    color_a: &[f32; 4],
    color_b: &[f32; 4],
    align: LabelAnchor,
) -> bool {
    if text.is_empty() {
        return false;
    }
    let width = rect[2] - rect[0];
    let mut pos = [
        (rect[0] + rect[2]) * 0.5,
        (rect[1] + rect[3]) * 0.5,
    ];
    match align {
        LabelAnchor::Left => pos[0] = rect[0] + width * -0.1,
        LabelAnchor::Center => pos[0] = rect[0] + width * 0.1,
        LabelAnchor::Right => {}
    }
    let args = FontDraw2dArgs::new(text, pos, *color_a, *color_b, align as i32);
    adorn.draw_font_2d(&args) != 0
}

// 0x7ad5d4 — __ZNK3RBX6Canvas11toPixelSizeERKN3G3D7Vector2E
#[doc(alias = "RBX::Canvas::toPixelSize(G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX6Canvas11toPixelSizeERKN3G3D7Vector2E")]
// IDA 0x7ad5d4: `0.01` unit scale (0x3C23D70A, 0x7ad5d8), `0.75` aspect knee
// (0x7ad5dc); when `y <= 0.75 * x` the scale is `(1.33 * y, y)`
// (0x3FAA3D71, 0x7ad60a), else `(x, 0.75 * x)`; the output is
// `size * 0.01 * scale` per axis (0x7ad61e..0x7ad622).
pub fn stub_0x7ad5d4(size: [i32; 2], vec: [f32; 2]) -> [f32; 2] {
    const UNIT: f32 = f32::from_bits(0x3C23_D70A);
    const KNEE: f32 = 0.75;
    const COMP: f32 = f32::from_bits(0x3FAA_3D71);
    let (sx, sy) = if vec[1] <= KNEE * vec[0] {
        (COMP * vec[1], vec[1])
    } else {
        (vec[0], KNEE * vec[0])
    };
    [
        size[0] as f32 * UNIT * sx,
        size[1] as f32 * UNIT * sy,
    ]
}

// 0x7ad6e8 — __ZN3RBX7GuiRoot8render2dEPNS_5AdornE
#[doc(alias = "RBX::GuiRoot::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7GuiRoot8render2dEPNS_5AdornE")]
// IDA 0x7ad6e8: iterate `numChildren` (0x7ad6f0), `getGuiItem(i)` with the
// null skip (0x7ad700..0x7ad704), slot +164 dispatch (0x7ad70e).
pub fn stub_0x7ad6e8(host: &dyn GuiChildHost, adorn: &mut dyn Adorn2d) {
    render_child_list(host, adorn);
}

// 0x7ad720 — __ZN3RBX7GuiRoot12render2dItemEPNS_5AdornEPNS_7GuiItemE
#[doc(alias = "RBX::GuiRoot::render2dItem(RBX::Adorn *,RBX::GuiItem *)")]
#[doc(alias = "__ZN3RBX7GuiRoot12render2dItemEPNS_5AdornEPNS_7GuiItemE")]
// IDA 0x7ad720: direct tail-forward to the item's slot +164 (`render2d`).
pub fn stub_0x7ad720(item: &SharedPtr<dyn Render2dChild>, adorn: &mut dyn Adorn2d) {
    item.render2d(adorn);
}

// 0x7adda8 — __ZN3RBX10TopMenuBar8render2dEPNS_5AdornE
#[doc(alias = "RBX::TopMenuBar::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX10TopMenuBar8render2dEPNS_5AdornE")]
// IDA 0x7adda8: visibility gate (slot +148, 0x7addbe); when the `+112` tint
// differs from `Color4::clear()` (0x7ade0e) the `getMyRect2D` rect is filled
// via the Adorn +64 virtual (0x7ade1a..0x7ade58); then the child loop
// (0x7ade5c..0x7ade7c).
pub fn stub_0x7adda8(bar: &TopMenuBarState, adorn: &mut dyn Adorn2d) {
    if !bar.visible {
        return;
    }
    if bar.tint != CLEAR_TINT {
        let rect = my_rect_2d(canvas_from_extent(adorn.viewport_extent()));
        adorn.fill_rect(rect, bar.tint);
    }
    render_child_list(bar, adorn);
}

// 0x7adea4 — __ZN3RBX13UnifiedWidget10render2dMeEPNS_5AdornE
#[doc(alias = "RBX::UnifiedWidget::render2dMe(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget10render2dMeEPNS_5AdornE")]
// IDA 0x7adea4: `getMyRect2D` over the viewport canvas (0x7adeb6..0x7adee6);
// nonzero `+28` selects `menuSelect` + the `dword_1329C78` tint
// (0x7adeee..0x7adf0a), otherwise `white` + alpha 1 (0x7adf0e..0x7adf1e);
// fill via Adorn +64 (0x7adf2a), `black` outline via Adorn +48 (0x7adf32..),
// then `label2d` on `fw(this)` with `black`/`clear` and align 2
// (0x7adf94..0x7adfc8).
pub fn stub_0x7adea4(state: &mut UnifiedWidgetState, adorn: &mut dyn Adorn2d) {
    let rect = my_rect_2d(canvas_from_extent(adorn.viewport_extent()));
    let fill = if state.token != 0 {
        state.on_menu_select();
        state.highlight
    } else {
        WHITE_TINT
    };
    adorn.fill_rect(rect, fill);
    adorn.stroke_rect(rect, 1.0, BLACK_TINT);
    let _ = stub_0x7ad2b0(
        adorn,
        &state.label.text.clone(),
        &state.label.rect,
        &BLACK_TINT,
        &CLEAR_TINT,
        LabelAnchor::Right,
    );
}

// 0x7adfcc — __ZN3RBX13UnifiedWidget16render2dChildrenEPNS_5AdornE
#[doc(alias = "RBX::UnifiedWidget::render2dChildren(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget16render2dChildrenEPNS_5AdornE")]
// IDA 0x7adfcc: children render only when the `+28` kind word is `>= 2`
// (0x7adfd8); the `numChildren`/`getGuiItem`/slot +164 loop is shared.
pub fn stub_0x7adfcc(state: &UnifiedWidgetState, adorn: &mut dyn Adorn2d) {
    if state.token < 2 {
        return;
    }
    render_child_list(state, adorn);
}

// 0x7ae00c — __ZN3RBX13UnifiedWidget8render2dEPNS_5AdornE
#[doc(alias = "RBX::UnifiedWidget::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget8render2dEPNS_5AdornE")]
// IDA 0x7ae00c: visibility gate (slot +148, 0x7ae01c); `render2dMe`
// (slot +180, 0x7ae02e) then `render2dChildren` (0x7ae038).
pub fn stub_0x7ae00c(state: &mut UnifiedWidgetState, adorn: &mut dyn Adorn2d) -> bool {
    if !state.label.visible {
        return false;
    }
    stub_0x7adea4(state, adorn);
    stub_0x7adfcc(state, adorn);
    true
}

// 0x7ae9b8 — __ZN3RBX11TextDisplay8render2dEPNS_5AdornE
#[doc(alias = "RBX::TextDisplay::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11TextDisplay8render2dEPNS_5AdornE")]
// IDA 0x7ae9b8: visibility gate (slot +148, 0x7ae9ca), then
// `GuiItem::label2d` with the `+28` text, `+120`/`+136` colors and `+38`
// align (0x7ae9e8).
pub fn stub_0x7ae9b8(widget: &LabeledWidget, adorn: &mut dyn Adorn2d) -> bool {
    if !widget.visible {
        return false;
    }
    stub_0x7ad2b0(
        adorn,
        &widget.text,
        &widget.rect,
        &widget.color_a,
        &widget.color_b,
        widget.align,
    )
}

// 0x7aecd8 — __ZN3RBX7GuiItem8render2dEPNS_5AdornE
#[doc(alias = "RBX::GuiItem::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7GuiItem8render2dEPNS_5AdornE")]
// IDA 0x7aecd8: base `GuiItem::render2d` — empty body, single `BX` return.
pub fn stub_0x7aecd8() {}

// 0x7afdbc — __ZN3RBX12GuiDrawImage8setImageEPNS_5AdornERKNS_9TextureIdEjPN3G3D7Vector2E
#[doc(alias = "RBX::GuiDrawImage::setImage(RBX::Adorn *,RBX::TextureId const&,unsigned int,G3D::Vector2 *)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8setImageEPNS_5AdornERKNS_9TextureIdEjPN3G3D7Vector2E")]
// IDA 0x7afdbc: 1505 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7afdbc() {
}

// 0x7b0fbc — __ZN3RBX12GuiDrawImage16setImageFromNameEPNS_5AdornERKSsj
#[doc(alias = "RBX::GuiDrawImage::setImageFromName(RBX::Adorn *,std::string const&,unsigned int)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage16setImageFromNameEPNS_5AdornERKSsj")]
// IDA 0x7b0fbc: 277 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b0fbc() {
}

// 0x7b15fc — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectENS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectENS_3Gui11WidgetStateEb")]
// IDA 0x7b15fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b15fc() {
}

// 0x7b163c — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectERKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectERKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb")]
// IDA 0x7b163c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b163c() {
}

// 0x7b1658 — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectES5_RKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectES5_RKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb")]
// IDA 0x7b1658: 437 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b1658() {
}

// 0x7b1a9c — __ZN3RBX12GuiDrawImage9computeUVERN3G3D7Vector2ES3_RKS2_S5_S5_
#[doc(alias = "RBX::GuiDrawImage::computeUV(G3D::Vector2 &,G3D::Vector2 &,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Vector2 const&)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage9computeUVERN3G3D7Vector2ES3_RKS2_S5_S5_")]
// IDA 0x7b1a9c: 82 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b1a9c() {
}

// 0x7b32c4 — __ZN3RBX6Widget8render2dEPNS_5AdornE
#[doc(alias = "RBX::Widget::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX6Widget8render2dEPNS_5AdornE")]
// IDA 0x7b32c4: 228 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b32c4() {
}

// 0x7b6384 — __ZN3RBX8Humanoid14setWalkToPointERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setWalkToPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14setWalkToPointERKN3G3D7Vector3E")]
// IDA 0x7b6384: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b6384() {
}

// 0x7b6464 — __ZN3RBX8Humanoid19setTargetPointLocalERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setTargetPointLocal(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid19setTargetPointLocalERKN3G3D7Vector3E")]
// IDA 0x7b6464: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b6464() {
}

// 0x7b65ec — __ZN3RBX8Humanoid16setWalkDirectionERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setWalkDirection(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid16setWalkDirectionERKN3G3D7Vector3E")]
// IDA 0x7b65ec: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7b65ec() {
}

// 0x7bacac — __ZNK3RBX8Humanoid14hasWalkToPointERN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::hasWalkToPoint(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX8Humanoid14hasWalkToPointERN3G3D7Vector3E")]
// IDA 0x7bacac: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bacac() {
}

// 0x7bb51c — __ZN3RBX8Humanoid14setTargetPointERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setTargetPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14setTargetPointERKN3G3D7Vector3E")]
// IDA 0x7bb51c: 71 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bb51c() {
}

// 0x7bc6bc — __ZN3RBX8Humanoid14renderWaypointEPNS_5AdornERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::renderWaypoint(RBX::Adorn *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14renderWaypointEPNS_5AdornERKN3G3D7Vector3E")]
// IDA 0x7bc6bc: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bc6bc() {
}

// 0x7bc720 — __ZN3RBX8Humanoid13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::Humanoid::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX8Humanoid13render3dAdornEPNS_5AdornE")]
// IDA 0x7bc720: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bc720() {
}

// 0x7bc79c — __ZThn268_N3RBX8Humanoid13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn268_N3RBX8Humanoid13render3dAdornEPNS_5AdornE")]
// IDA 0x7bc79c: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bc79c() {
}

// 0x7bc7a4 — __ZN3RBX8Humanoid17renderMultiplayerEPNS_5AdornERKNS_6CameraE
#[doc(alias = "RBX::Humanoid::renderMultiplayer(RBX::Adorn *,RBX::Camera const&)")]
#[doc(alias = "__ZN3RBX8Humanoid17renderMultiplayerEPNS_5AdornERKNS_6CameraE")]
// IDA 0x7bc7a4: 387 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bc7a4() {
}

// 0x7bcc68 — __ZNK3RBX8Humanoid22render3dSortedPositionEv
#[doc(alias = "RBX::Humanoid::render3dSortedPosition(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid22render3dSortedPositionEv")]
// IDA 0x7bcc68: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcc68() {
}

// 0x7bcc9c — __ZThn268_NK3RBX8Humanoid22render3dSortedPositionEv
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dSortedPosition(void)const")]
#[doc(alias = "__ZThn268_NK3RBX8Humanoid22render3dSortedPositionEv")]
// IDA 0x7bcc9c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcc9c() {
}

// 0x7bccac — __ZN3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE
#[doc(alias = "RBX::Humanoid::render3dSortedAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE")]
// IDA 0x7bccac: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bccac() {
}

// 0x7bcce0 — __ZThn268_N3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dSortedAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn268_N3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE")]
// IDA 0x7bcce0: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcce0() {
}

// 0x7bce4c — __ZN3RBX8Humanoid17getRenderLocationEv
#[doc(alias = "RBX::Humanoid::getRenderLocation(void)")]
#[doc(alias = "__ZN3RBX8Humanoid17getRenderLocationEv")]
// IDA 0x7bce4c: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bce4c() {
}

// 0x7bcee4 — __ZThn292_N3RBX8Humanoid17getRenderLocationEv
#[doc(alias = "non-virtual thunk toRBX::Humanoid::getRenderLocation(void)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid17getRenderLocationEv")]
// IDA 0x7bcee4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcee4() {
}

// 0x7bcef4 — __ZN3RBX8Humanoid13getRenderSizeEv
#[doc(alias = "RBX::Humanoid::getRenderSize(void)")]
#[doc(alias = "__ZN3RBX8Humanoid13getRenderSizeEv")]
// IDA 0x7bcef4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcef4() {
}

// 0x7bcf1c — __ZThn292_N3RBX8Humanoid13getRenderSizeEv
#[doc(alias = "non-virtual thunk toRBX::Humanoid::getRenderSize(void)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid13getRenderSizeEv")]
// IDA 0x7bcf1c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bcf1c() {
}

// 0x7bd338 — __ZN3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb")]
// IDA 0x7bd338: 164 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bd338() {
}

// 0x7bd574 — __ZThn292_N3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
#[doc(alias = "non-virtual thunk toRBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb")]
// IDA 0x7bd574: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bd574() {
}

// 0x7bf638 — __ZNK3RBX8Humanoid19shouldRender3dAdornEv
#[doc(alias = "RBX::Humanoid::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid19shouldRender3dAdornEv")]
// IDA 0x7bf638: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bf638() {
}

// 0x7bf63c — __ZNK3RBX8Humanoid25shouldRender3dSortedAdornEv
#[doc(alias = "RBX::Humanoid::shouldRender3dSortedAdorn(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid25shouldRender3dSortedAdornEv")]
// IDA 0x7bf63c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bf63c() {
}

// 0x7bf69c — __ZThn268_NK3RBX8Humanoid19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk toRBX::Humanoid::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn268_NK3RBX8Humanoid19shouldRender3dAdornEv")]
// IDA 0x7bf69c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bf69c() {
}

// 0x7bf6a0 — __ZThn268_NK3RBX8Humanoid25shouldRender3dSortedAdornEv
#[doc(alias = "non-virtual thunk toRBX::Humanoid::shouldRender3dSortedAdorn(void)const")]
#[doc(alias = "__ZThn268_NK3RBX8Humanoid25shouldRender3dSortedAdornEv")]
// IDA 0x7bf6a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7bf6a0() {
}

// 0x7cdba4 — __ZN3RBX5HUMAN13HumanoidState13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::HUMAN::HumanoidState::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState13render3dAdornEPNS_5AdornE")]
// IDA 0x7cdba4: 701 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cdba4() {
}

// 0x7ce378 — __ZN3RBX5HUMAN13HumanoidState10findLadderEPNS_5AdornE
#[doc(alias = "RBX::HUMAN::HumanoidState::findLadder(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10findLadderEPNS_5AdornE")]
// IDA 0x7ce378: 400 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ce378() {
}

// 0x7d09a8 — __ZN3RBX5HUMAN13HumanoidState8tryFloorERKNS_6RbxRayERN3G3D7Vector3EfPNS_8AssemblyE
#[doc(alias = "RBX::HUMAN::HumanoidState::tryFloor(RBX::RbxRay const&,G3D::Vector3 &,float,RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState8tryFloorERKNS_6RbxRayERN3G3D7Vector3EfPNS_8AssemblyE")]
// IDA 0x7d09a8: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7d09a8() {
}

// 0x7d1004 — __ZN3RBX5HUMAN13HumanoidState25findPrimitiveInLadderZoneEPNS_5AdornE
#[doc(alias = "RBX::HUMAN::HumanoidState::findPrimitiveInLadderZone(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState25findPrimitiveInLadderZoneEPNS_5AdornE")]
// IDA 0x7d1004: 186 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7d1004() {
}

// 0x7d1230 — __ZN3RBX5HUMAN13HumanoidState15doLadderRaycastEPNS_15GeometryServiceERKNS_6RbxRayEPNS_8HumanoidEPPNS_9PrimitiveEPN3G3D7Vector3E
#[doc(alias = "RBX::HUMAN::HumanoidState::doLadderRaycast(RBX::GeometryService *,RBX::RbxRay const&,RBX::Humanoid *,RBX::Primitive **,G3D::Vector3 *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState15doLadderRaycastEPNS_15GeometryServiceERKNS_6RbxRayEPNS_8HumanoidEPPNS_9PrimitiveEPN3G3D7Vector3E")]
// IDA 0x7d1230: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7d1230() {
}

// 0x816a34 — __ZN3RBX7Region216getRelativeErrorERKN3G3D7Vector2ERKNS0_13WeightedPointE
#[doc(alias = "RBX::Region2::getRelativeError(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&)")]
#[doc(alias = "__ZN3RBX7Region216getRelativeErrorERKN3G3D7Vector2ERKNS0_13WeightedPointE")]
// IDA 0x816a34: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816a34() {
}

// 0x816a6c — __ZN3RBX7Region212pointInRangeERKN3G3D7Vector2ERKNS0_13WeightedPointEf
#[doc(alias = "RBX::Region2::pointInRange(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,float)")]
#[doc(alias = "__ZN3RBX7Region212pointInRangeERKN3G3D7Vector2ERKNS0_13WeightedPointEf")]
// IDA 0x816a6c: 19 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816a6c() {
}

// 0x816ab0 — __ZNK3RBX7Region28containsERKN3G3D7Vector2Ef
#[doc(alias = "RBX::Region2::contains(G3D::Vector2 const&,float)const")]
#[doc(alias = "__ZNK3RBX7Region28containsERKN3G3D7Vector2Ef")]
// IDA 0x816ab0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816ab0() {
}

// 0x816b04 — __ZNK3RBX7Region215findCloserOtherERKN3G3D7Vector2Ef
#[doc(alias = "RBX::Region2::findCloserOther(G3D::Vector2 const&,float)const")]
#[doc(alias = "__ZNK3RBX7Region215findCloserOtherERKN3G3D7Vector2Ef")]
// IDA 0x816b04: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816b04() {
}

// 0x816b54 — __ZN3RBX7Region218closerToOtherPointERKN3G3D7Vector2ERKNS0_13WeightedPointES7_f
#[doc(alias = "RBX::Region2::closerToOtherPoint(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,RBX::Region2::WeightedPoint const&,float)")]
#[doc(alias = "__ZN3RBX7Region218closerToOtherPointERKN3G3D7Vector2ERKNS0_13WeightedPointES7_f")]
// IDA 0x816b54: 43 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816b54() {
}

// 0x816d20 — __ZN3RBX7Region3C1ERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::Region3::Region3(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Region3C1ERKN3G3D7Vector3ES4_")]
// IDA 0x816d20: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x816d20() {
}

// 0x83548c — __ZN3RBX15NotificationBox8render2dEPNS_5AdornE
#[doc(alias = "RBX::NotificationBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX15NotificationBox8render2dEPNS_5AdornE")]
// IDA 0x83548c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x83548c() {
}

// 0x835490 — __ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE")]
// IDA 0x835490: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x835490() {
}

// 0x83753c — __ZN3RBX18NotificationObject8render2dEPNS_5AdornE
#[doc(alias = "RBX::NotificationObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX18NotificationObject8render2dEPNS_5AdornE")]
// IDA 0x83753c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x83753c() {
}

// 0x837540 — __ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE")]
// IDA 0x837540: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x837540() {
}

// 0x84fa9c — __ZN3RBX18RenderHooksService14captureMetricsEv
#[doc(alias = "RBX::RenderHooksService::captureMetrics(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService14captureMetricsEv")]
// IDA 0x84fa9c: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fa9c() {
}

// 0x84fab0 — __ZN3RBX18RenderHooksService12resizeWindowEii
#[doc(alias = "RBX::RenderHooksService::resizeWindow(int,int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12resizeWindowEii")]
// IDA 0x84fab0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fab0() {
}

// 0x84fac0 — __ZN3RBX18RenderHooksService12enableAdornsEb
#[doc(alias = "RBX::RenderHooksService::enableAdorns(bool)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12enableAdornsEb")]
// IDA 0x84fac0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fac0() {
}

// 0x84fad0 — __ZN3RBX18RenderHooksService10printSceneEv
#[doc(alias = "RBX::RenderHooksService::printScene(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService10printSceneEv")]
// IDA 0x84fad0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fad0() {
}

// 0x84fae0 — __ZN3RBX18RenderHooksServiceC1Ev
#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceC1Ev")]
// IDA 0x84fae0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x84fae0() {
}

// 0x84fae4 — __ZN3RBX18RenderHooksServiceC2Ev
#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceC2Ev")]
// IDA 0x84fae4: 350 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fae4() {
}

// 0x84fea4 — __ZN3RBX18RenderHooksService13reloadShadersEv
#[doc(alias = "RBX::RenderHooksService::reloadShaders(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService13reloadShadersEv")]
// IDA 0x84fea4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fea4() {
}

// 0x84fed0 — __ZN3RBX18RenderHooksService11enableQueueEi
#[doc(alias = "RBX::RenderHooksService::enableQueue(int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11enableQueueEi")]
// IDA 0x84fed0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84fed0() {
}

// 0x84ff18 — __ZN3RBX18RenderHooksService12disableQueueEi
#[doc(alias = "RBX::RenderHooksService::disableQueue(int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12disableQueueEi")]
// IDA 0x84ff18: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ff18() {
}

// 0x84ff68 — __ZN3RBX18RenderHooksService14getPresentTimeEv
#[doc(alias = "RBX::RenderHooksService::getPresentTime(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService14getPresentTimeEv")]
// IDA 0x84ff68: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ff68() {
}

// 0x84ff98 — __ZN3RBX18RenderHooksService11getGPUDelayEv
#[doc(alias = "RBX::RenderHooksService::getGPUDelay(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11getGPUDelayEv")]
// IDA 0x84ff98: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ff98() {
}

// 0x84ffa4 — __ZN3RBX18RenderHooksService12getRenderAveEv
#[doc(alias = "RBX::RenderHooksService::getRenderAve(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12getRenderAveEv")]
// IDA 0x84ffa4: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ffa4() {
}

// 0x84ffb0 — __ZN3RBX18RenderHooksService16getRenderConfMinEv
#[doc(alias = "RBX::RenderHooksService::getRenderConfMin(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService16getRenderConfMinEv")]
// IDA 0x84ffb0: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ffb0() {
}

// 0x84ffbc — __ZN3RBX18RenderHooksService16getRenderConfMaxEv
#[doc(alias = "RBX::RenderHooksService::getRenderConfMax(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService16getRenderConfMaxEv")]
// IDA 0x84ffbc: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ffbc() {
}

// 0x84ffc8 — __ZN3RBX18RenderHooksService12getRenderStdEv
#[doc(alias = "RBX::RenderHooksService::getRenderStd(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12getRenderStdEv")]
// IDA 0x84ffc8: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ffc8() {
}

// 0x84ffd4 — __ZN3RBX18RenderHooksService11getDeltaAveEv
#[doc(alias = "RBX::RenderHooksService::getDeltaAve(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11getDeltaAveEv")]
// IDA 0x84ffd4: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x84ffd4() {
}

// 0x850020 — __ZN3RBX18RenderHooksServiceD1Ev
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD1Ev")]
// IDA 0x850020: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x850020() {
}

// 0x850024 — __ZN3RBX18RenderHooksServiceD0Ev
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD0Ev")]
// IDA 0x850024: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x850024() {
}

// 0x8500ec — __ZThn32_N3RBX18RenderHooksServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn32_N3RBX18RenderHooksServiceD1Ev")]
// IDA 0x8500ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x8500ec() {
}

// 0x8500f4 — __ZThn32_N3RBX18RenderHooksServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn32_N3RBX18RenderHooksServiceD0Ev")]
// IDA 0x8500f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x8500f4() {
}

// 0x8501c0 — __ZThn36_N3RBX18RenderHooksServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn36_N3RBX18RenderHooksServiceD1Ev")]
// IDA 0x8501c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x8501c0() {
}

// 0x8501c8 — __ZThn36_N3RBX18RenderHooksServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn36_N3RBX18RenderHooksServiceD0Ev")]
// IDA 0x8501c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x8501c8() {
}

// 0x85026c — __ZN3RBX18RenderHooksServiceD2Ev
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD2Ev")]
// IDA 0x85026c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x85026c() {
}

// 0x855e20 — __ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc
#[doc(alias = "RBX::ClientAppSettings::ReadValueAxisAdornmentGrabSize(char const*)")]
#[doc(alias = "__ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc")]
// IDA 0x855e20: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x855e20() {
}

// 0x85df88 — __ZN3RBX12TextureTrail14setTextureSizeEN3G3D7Vector2E
#[doc(alias = "RBX::TextureTrail::setTextureSize(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX12TextureTrail14setTextureSizeEN3G3D7Vector2E")]
// IDA 0x85df88: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x85df88() {
}

// 0x85e2bc — __ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE")]
// IDA 0x85e2bc: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x85e2bc() {
}

// 0x85e7f0 — __ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE")]
// IDA 0x85e7f0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x85e7f0() {
}

// 0x867a90 — __ZN3RBX9FloorWire14setTextureSizeEN3G3D7Vector2E
#[doc(alias = "RBX::FloorWire::setTextureSize(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX9FloorWire14setTextureSizeEN3G3D7Vector2E")]
// IDA 0x867a90: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x867a90() {
}

// 0x867de4 — __ZN3RBX9FloorWire13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX9FloorWire13render3dAdornEPNS_5AdornE")]
// IDA 0x867de4: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x867de4() {
}

// 0x8685d8 — __ZThn96_N3RBX9FloorWire13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX9FloorWire13render3dAdornEPNS_5AdornE")]
// IDA 0x8685d8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8685d8() {
}

// 0x868ce0 — __ZNK3RBX9GuiBase3d19shouldRender3dAdornEv
#[doc(alias = "RBX::GuiBase3d::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase3d19shouldRender3dAdornEv")]
// IDA 0x868ce0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x868ce0() {
}

// 0x8691c8 — __ZThn96_NK3RBX9GuiBase3d19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase3d19shouldRender3dAdornEv")]
// IDA 0x8691c8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8691c8() {
}

// 0x86f1ac — __ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE
#[doc(alias = "RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)")]
#[doc(alias = "__ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE")]
// IDA 0x86f1ac: 36 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x86f1ac() {
}

// 0x87b3bc — __ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb
#[doc(alias = "RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb")]
// IDA 0x87b3bc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b3bc() {
}

// 0x87b414 — __ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb
#[doc(alias = "RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb")]
// IDA 0x87b414: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b414() {
}
