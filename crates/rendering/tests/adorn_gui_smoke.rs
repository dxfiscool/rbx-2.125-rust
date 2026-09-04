use rbx_core::SharedPtr;
use rbx_rendering::generated_478 as g;
use std::sync::Arc;

struct Host {
    kids: Vec<SharedPtr<dyn g::Render2dChild>>,
}
impl g::GuiChildHost for Host {
    fn child_count(&self) -> usize {
        self.kids.len()
    }
    fn child(&self, i: usize) -> Option<SharedPtr<dyn g::Render2dChild>> {
        self.kids.get(i).cloned()
    }
}

#[test]
fn chat_style_mode_bits() {
    assert!(!g::ChatStyleMode(None).bubble_enabled());
    assert!(!g::ChatStyleMode(None).classic_enabled());
    assert!(g::ChatStyleMode(Some(1)).bubble_enabled());
    assert!(g::ChatStyleMode(Some(2)).bubble_enabled());
    assert!(!g::ChatStyleMode(Some(3)).bubble_enabled());
    // 0x7a1a1e: bit1 cleared then nonzero check.
    assert!(!g::ChatStyleMode(Some(2)).classic_enabled());
    assert!(g::ChatStyleMode(Some(1)).classic_enabled());
    assert!(g::ChatStyleMode(Some(4)).classic_enabled());
}

#[test]
fn render2d_fans_out_to_both_styles() {
    let mut out = g::ChatOutputState {
        mode: g::ChatStyleMode(Some(1)),
        bubbles: g::ChatBubbleWorld {
            entries: vec![g::BubbleChatEntry {
                speaker: g::SpeakerRef::Part(7),
                queued_lines: 2,
                ..Default::default()
            }],
            head_part: Some(7),
            workspace: Some(9),
        },
        classic: g::ClassicChatState {
            native_chat_rendering: true,
            named_lines: 3,
            position: [4.0, 5.0],
            backdrop: [0.0, 0.0, 0.0, 0.5],
            ..Default::default()
        },
    };
    let mut adorn = g::TestAdorn::default();
    g::stub_0x7a19f4(&mut out, &mut adorn);
    // Bubble phase bound the imposter; classic phase drew the box.
    assert!(out.bubbles.entries[0].billboard.render_fn.is_some());
    assert_eq!(adorn.chat_boxes.len(), 1);
    assert_eq!(adorn.chat_boxes[0].1, 3);
}

#[test]
fn bubble_phase_clears_then_binds() {
    let mut world = g::ChatBubbleWorld {
        entries: vec![g::BubbleChatEntry {
            queued_lines: 1,
            drawn: [true, true],
            ..Default::default()
        }],
        ..Default::default()
    };
    world.entries[0].billboard.render_fn = Some(Box::new(|_, _| {}));
    // No head/workspace and no force: cleared, not rebound.
    g::stub_0x7a1a38(&mut world, false);
    assert!(world.entries[0].billboard.render_fn.is_none());
    assert_eq!(world.entries[0].drawn, [false, false]);
    // Forced: rebound.
    g::stub_0x7a1a38(&mut world, true);
    assert!(world.entries[0].billboard.render_fn.is_some());
}

#[test]
fn classic_style_flag_gate_and_deque_select() {
    let mut adorn = g::TestAdorn {
        viewport: [0.0, 0.0, 800.0, 600.0],
        ..Default::default()
    };
    let off = g::ClassicChatState::default();
    g::stub_0x7a2400(&off, &mut adorn, true);
    assert!(adorn.chat_boxes.is_empty());
    let named = g::ClassicChatState {
        native_chat_rendering: true,
        named_lines: 5,
        plain_lines: 2,
        ..Default::default()
    };
    g::stub_0x7a2400(&named, &mut adorn, true);
    g::stub_0x7a2400(&named, &mut adorn, false);
    assert_eq!(adorn.chat_boxes.len(), 2);
    assert_eq!(adorn.chat_boxes[0].1, 5);
    assert_eq!(adorn.chat_boxes[1].1, 2);
}

#[test]
fn billboard_gate_clears_out_param() {
    let mut board = g::BillboardDrawState::default();
    let mut drawn = 9;
    assert!(!g::stub_0x7a3f74(&mut board, &mut drawn));
    assert_eq!(drawn, 0);
    board.visible_and_valid = true;
    assert!(g::stub_0x7a3f74(&mut board, &mut drawn));
    assert_eq!(board.forwarded_draws, 1);
}

#[test]
fn adorn_base_noops_and_forward() {
    g::stub_0x7a9bf8();
    g::stub_0x7a9bfc();
    g::stub_0x7aecd8();
    let handle = rbx_rendering::generated_141::AdornHandle::default();
    g::stub_0x7a9b58(handle);
    let mut adorn = g::TestAdorn::default();
    let args = g::FontDraw2dArgs::new("hi", [1.0, 2.0], [1.0; 4], [0.0; 4], 2);
    assert_eq!(g::stub_0x7a9c00(&mut adorn, &args), 0);
    assert_eq!(adorn.fonts.len(), 1);
    assert_eq!(adorn.fonts[0].pad, [-1.0; 4]);
}

#[test]
fn unified_image_gate_and_draw_mode() {
    assert_eq!(g::image_draw_mode(2), 2);
    assert_eq!(g::image_draw_mode(3), 2);
    assert_eq!(g::image_draw_mode(1), 1);
    assert_eq!(g::image_draw_mode(0), 0);
    let state = g::UnifiedImageState {
        visible: true,
        image_name: "img".into(),
        kind: 2,
    };
    let mut image = g::GuiDrawImageState::default();
    let mut adorn = g::TestAdorn::default();
    assert!(g::stub_0x7aa7a8(&state, &mut image, &mut adorn));
    assert_eq!(image.last_name, "img");
    assert_eq!(image.draws, 1);
    let hidden = g::UnifiedImageState {
        visible: false,
        ..Default::default()
    };
    assert!(!g::stub_0x7aa7a8(&hidden, &mut image, &mut adorn));
    let mut image2 = g::GuiDrawImageState::default();
    let noname = g::UnifiedImageState {
        visible: true,
        ..Default::default()
    };
    assert!(!g::stub_0x7aa7a8(&noname, &mut image2, &mut adorn));
}

#[test]
fn label_empty_out_and_anchors() {
    let mut adorn = g::TestAdorn::default();
    let rect = [0.0, 0.0, 100.0, 20.0];
    assert!(!g::stub_0x7ad2b0(
        &mut adorn,
        "",
        &rect,
        &[1.0; 4],
        &[0.0; 4],
        g::LabelAnchor::Right
    ));
    adorn.font_result = 1;
    assert!(g::stub_0x7ad2b0(
        &mut adorn,
        "x",
        &rect,
        &[1.0; 4],
        &[0.0; 4],
        g::LabelAnchor::Left
    ));
    assert!(g::stub_0x7ad2b0(
        &mut adorn,
        "x",
        &rect,
        &[1.0; 4],
        &[0.0; 4],
        g::LabelAnchor::Center
    ));
    assert!(g::stub_0x7ad2b0(
        &mut adorn,
        "x",
        &rect,
        &[1.0; 4],
        &[0.0; 4],
        g::LabelAnchor::Right
    ));
    // Centered at 50, nudged by ∓0.1 width.
    assert_eq!(adorn.fonts[0].pos[0], 0.0 + 100.0 * -0.1);
    assert_eq!(adorn.fonts[1].pos[0], 0.0 + 100.0 * 0.1);
    assert_eq!(adorn.fonts[2].pos[0], 50.0);
    assert_eq!(adorn.fonts[2].pos[1], 10.0);
}

#[test]
fn canvas_pixel_math_matches_ida_constants() {
    // y <= 0.75x branch: (1.33y, y) scaled by 0.01·size.
    let out = g::stub_0x7ad5d4([100, 100], [10.0, 5.0]);
    assert!((out[0] - 100.0 * 0.01 * 1.33000004 * 5.0).abs() < 1e-3);
    assert!((out[1] - 100.0 * 0.01 * 5.0).abs() < 1e-3);
    // Else branch: (x, 0.75x).
    let out = g::stub_0x7ad5d4([100, 200], [10.0, 9.0]);
    assert!((out[0] - 100.0 * 0.01 * 10.0).abs() < 1e-3);
    assert!((out[1] - 200.0 * 0.01 * 0.75 * 10.0).abs() < 1e-3);
}

#[test]
fn child_loops_skip_null_and_recount() {
    let counter = Arc::new(g::GuiRenderCounter::default());
    let live: SharedPtr<dyn g::Render2dChild> = counter.clone();
    let host = Host {
        kids: vec![live.clone(), live.clone()],
    };
    let mut adorn = g::TestAdorn::default();
    g::stub_0x7ad6e8(&host, &mut adorn);
    g::stub_0x7ad720(&live, &mut adorn);
    assert_eq!(counter.draws.get(), 3);
    let empty = Host { kids: vec![] };
    g::stub_0x7ad6e8(&empty, &mut adorn);
    assert_eq!(counter.draws.get(), 3);
}

#[test]
fn top_menu_tint_gate_and_children() {
    let counter = Arc::new(g::GuiRenderCounter::default());
    let live: SharedPtr<dyn g::Render2dChild> = counter.clone();
    let bar = g::TopMenuBarState {
        visible: true,
        tint: g::CLEAR_TINT,
        children: vec![live.clone()],
    };
    let mut adorn = g::TestAdorn {
        viewport: [0.0, 0.0, 800.0, 600.0],
        ..Default::default()
    };
    g::stub_0x7adda8(&bar, &mut adorn);
    assert!(adorn.fills.is_empty());
    assert_eq!(counter.draws.get(), 1);
    let tinted = g::TopMenuBarState {
        visible: true,
        tint: [1.0, 0.0, 0.0, 1.0],
        children: vec![],
    };
    g::stub_0x7adda8(&tinted, &mut adorn);
    assert_eq!(adorn.fills.len(), 1);
    assert_eq!(adorn.fills[0].0, [0.0, 0.0, 800.0, 600.0]);
    let hidden = g::TopMenuBarState::default();
    g::stub_0x7adda8(&hidden, &mut adorn);
    assert_eq!(adorn.fills.len(), 1);
}

#[test]
fn unified_widget_me_children_and_full() {
    let counter = Arc::new(g::GuiRenderCounter::default());
    let live: SharedPtr<dyn g::Render2dChild> = counter.clone();
    let mut state = g::UnifiedWidgetState {
        token: 2,
        highlight: [0.5, 0.5, 0.5, 1.0],
        label: g::LabeledWidget {
            visible: true,
            text: "w".into(),
            rect: [0.0, 0.0, 10.0, 10.0],
            ..Default::default()
        },
        children: vec![live.clone()],
        ..Default::default()
    };
    let mut adorn = g::TestAdorn {
        viewport: [0.0, 0.0, 100.0, 100.0],
        ..Default::default()
    };
    g::stub_0x7adea4(&mut state, &mut adorn);
    assert_eq!(state.menu_selects, 1);
    assert_eq!(adorn.fills[0].1, [0.5, 0.5, 0.5, 1.0]);
    assert_eq!(adorn.strokes.len(), 1);
    assert_eq!(adorn.fonts.len(), 1);
    assert_eq!(adorn.fonts[0].x_align, 2);
    g::stub_0x7adfcc(&state, &mut adorn);
    assert_eq!(counter.draws.get(), 1);
    state.token = 1;
    g::stub_0x7adfcc(&state, &mut adorn);
    assert_eq!(counter.draws.get(), 1);
    assert!(g::stub_0x7ae00c(&mut state, &mut adorn));
    state.label.visible = false;
    assert!(!g::stub_0x7ae00c(&mut state, &mut adorn));
}

#[test]
fn text_and_equation_displays_share_label_path() {
    let widget = g::LabeledWidget {
        visible: true,
        text: "e=mc2".into(),
        rect: [0.0, 0.0, 40.0, 10.0],
        color_a: [1.0; 4],
        color_b: [0.0; 4],
        align: g::LabelAnchor::Right,
    };
    let mut adorn = g::TestAdorn::default();
    adorn.font_result = 7;
    assert_eq!(g::stub_0x7abe70(&widget, &mut adorn), true);
    assert_eq!(g::stub_0x7ae9b8(&widget, &mut adorn), true);
    assert_eq!(adorn.fonts.len(), 2);
    let hidden = g::LabeledWidget::default();
    assert!(!g::stub_0x7abe70(&hidden, &mut adorn));
    assert!(!g::stub_0x7ae9b8(&hidden, &mut adorn));
}
