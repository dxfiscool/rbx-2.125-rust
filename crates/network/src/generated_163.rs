//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 0 remaining before batch; filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x4c440..0x50c98 | existing 18129 -> 18249 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// Static-init states for `_a_21` / `__GLOBAL__I_a_20` (IDA 0x4d6d4/0x4d398).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA21 {
 pub done: bool,
}
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA20 {
 pub done: bool,
}

/// `GameKeyboard` text state (IDA 0x4cbf8 et al.).
#[derive(Clone, Debug, Default)]
pub struct GameKeyboard {
 pub text: String,
 pub placeholder: String,
 pub current_text_box: Option<usize>,
 pub hidden: bool,
}

/// Static-init state for `__GLOBAL__I_a_19` (IDA 0x4c498).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA19 {
 pub done: bool,
}

// 0x4c440 — -[GameInputViewController viewDidLoad]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidLoad]")]
pub fn stub_4c440(super_call: &mut dyn FnMut()) {
    // IDA 0x4c440: super viewDidLoad.
    super_call();
}

// 0x4c46c — -[GameInputViewController viewDidUnload]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidUnload]")]
pub fn stub_4c46c(super_call: &mut dyn FnMut()) {
    // IDA 0x4c46c: super viewDidUnload.
    super_call();
}

// 0x4c498 — __GLOBAL__I_a_19
// demangled: global constructor keyed to_a_19
#[doc(alias = "global constructor keyed to_a_19")]
pub fn stub_4c498(state: &mut GlobalInitA19, init: &mut dyn FnMut()) {
    // IDA 0x4c498: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x4c6ac — +[GameKeyboard sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[GameKeyboard sharedInstance]")]
pub fn stub_4c6ac(slot: &mut Option<usize>, alloc: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4c6ac: dispatch_once sharedInstance.
    if let Some(v) = *slot {
        return v;
    }
    let v = alloc();
    *slot = Some(v);
    v
}

// 0x4c6dc — ___30+[GameKeyboard sharedInstance]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___30+[GameKeyboard sharedInstance]_block_invoke")]
pub fn stub_4c6dc(alloc: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4c6dc: sharedInstance block — alloc + init.
    alloc()
}

// 0x4c71c — -[GameKeyboard init]
// type: GameKeyboard *__cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard init]")]
pub fn stub_4c71c(ok: bool, setup: &mut dyn FnMut()) -> bool {
    // IDA 0x4c71c: GameKeyboard init — text field + notifications (below truncation).
    if !ok {
        return false;
    }
    setup();
    true
}

// 0x4ca18 — -[GameKeyboard dealloc]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard dealloc]")]
pub fn stub_4ca18(release: &mut dyn FnMut(), teardown: &mut dyn FnMut()) {
    // IDA 0x4ca18: release textView; super dealloc.
    release();
    teardown();
}

// 0x4ca64 — -[GameKeyboard hideKeyboard]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard hideKeyboard]")]
pub fn stub_4ca64(hide: &mut dyn FnMut()) {
    // IDA 0x4ca64: hideKeyboard animation (below truncation).
    hide();
}

// 0x4cb80 — -[GameKeyboard keyboardWillHide:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillHide:]")]
pub fn stub_4cb80(has_box: bool, release_focus: &mut dyn FnMut(), hide: &mut dyn FnMut()) {
    // IDA 0x4cb80: externalReleaseFocus when set; hideKeyboard.
    if has_box {
        release_focus();
    }
    hide();
}

// 0x4cbbc — -[GameKeyboard keyboardWillChangeFrame:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillChangeFrame:]")]
pub fn stub_4cbbc() {
    // IDA 0x4cbbc: empty keyboardWillChangeFrame body.
}

// 0x4cbc0 — -[GameKeyboard setDefaultString:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setDefaultString:]")]
pub fn stub_4cbc0(kb: &mut GameKeyboard, text: String) {
    // IDA 0x4cbc0: textView placeholder = string.
    kb.placeholder = text;
}

// 0x4cbe0 — -[GameKeyboard setParentView:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setParentView:]")]
pub fn stub_4cbe0(add: &mut dyn FnMut()) {
    // IDA 0x4cbe0: parent addSubview:self.
    add();
}

// 0x4cbf8 — -[GameKeyboard showKeyboard:]
// type: bool __cdecl(GameKeyboard *self, SEL, const char *)
#[doc(alias = "-[GameKeyboard showKeyboard:]")]
pub fn stub_4cbf8(kb: &GameKeyboard, text: &str, show: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x4cbf8: hidden ? dispatch show block : NO.
    if kb.hidden {
        show(text);
        true
    } else {
        false
    }
}

// 0x4cc78 — ___29-[GameKeyboard showKeyboard:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___29-[GameKeyboard showKeyboard:]_block_invoke")]
pub fn stub_4cc78(show: &mut dyn FnMut()) {
    // IDA 0x4cc78: show block — configure + display text field (below truncation).
    show();
}

// 0x4ce30 — ___copy_helper_block__9
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__9")]
pub fn stub_4ce30(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x4ce30: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x4ce3c — ___destroy_helper_block__9
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__9")]
pub fn stub_4ce3c(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x4ce3c: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x4ce44 — -[GameKeyboard showKeyboardWithTextBox:]
// type: bool __cdecl(GameKeyboard *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[GameKeyboard showKeyboardWithTextBox:]")]
pub fn stub_4ce44(kb: &mut GameKeyboard, text_box: usize, show: &mut dyn FnMut(usize) -> bool) -> bool {
    // IDA 0x4ce44: showKeyboardWithTextBox (below truncation).
    kb.current_text_box = Some(text_box);
    show(text_box)
}

// 0x4cfbc — -[GameKeyboard getText]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard getText]")]
pub fn stub_4cfbc(kb: &GameKeyboard) -> &str {
    // IDA 0x4cfbc: return textView text.
    &kb.text
}

// 0x4cfdc — -[GameKeyboard textFieldShouldReturn:]
// type: char __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldShouldReturn:]")]
pub fn stub_4cfdc(has_service: bool, text: &str, finish: &mut dyn FnMut(&str), hide: &mut dyn FnMut()) -> bool {
    // IDA 0x4cfdc: textboxDidFinishEditing when service; dispatch hide; YES.
    if has_service {
        finish(text);
    }
    hide();
    true
}

// 0x4d07c — ___38-[GameKeyboard textFieldShouldReturn:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___38-[GameKeyboard textFieldShouldReturn:]_block_invoke")]
pub fn stub_4d07c(hide: &mut dyn FnMut()) {
    // IDA 0x4d07c: block — hideKeyboard.
    hide();
}

// 0x4d090 — ___copy_helper_block_82
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_82")]
pub fn stub_4d090(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x4d090: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x4d09c — ___destroy_helper_block_83
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_83")]
pub fn stub_4d09c(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x4d09c: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x4d0a4 — -[GameKeyboard textFieldDidEndEditing:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldDidEndEditing:]")]
pub fn stub_4d0a4(is_first: bool, has_service: bool, text: &str, finish: &mut dyn FnMut(&str), hide: &mut dyn FnMut()) {
    // IDA 0x4d0a4: firstResponder ? finish editing : skip; dispatch hide.
    if is_first && has_service {
        finish(text);
    }
    hide();
}

// 0x4d15c — ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")]
pub fn stub_4d15c(hide: &mut dyn FnMut()) {
    // IDA 0x4d15c: block — hideKeyboard.
    hide();
}

// 0x4d170 — ___copy_helper_block_87
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_87")]
pub fn stub_4d170(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x4d170: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x4d17c — ___destroy_helper_block_88
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_88")]
pub fn stub_4d17c(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x4d17c: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x4d184 — -[GameKeyboard .cxx_destruct]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_destruct]")]
pub fn stub_4d184(kb: &mut GameKeyboard, release: &mut dyn FnMut(usize)) {
    // IDA 0x4d184: cxx_destruct — release currentTextBox.
    if let Some(t) = kb.current_text_box.take() {
        release(t);
    }
}

// 0x4d220 — -[GameKeyboard .cxx_construct]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_construct]")]
pub fn stub_4d220(kb: &mut GameKeyboard) {
    // IDA 0x4d220: cxx_construct — zero currentTextBox.
    kb.current_text_box = None;
}

// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// demangled: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, __int64 *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
pub fn stub_4d238(dst: &mut Option<usize>, src: &mut Option<usize>, release: &mut dyn FnMut(usize)) {
    // IDA 0x4d238: move-assign — steal src pair (src zeroed); release old dst.
    let old = std::mem::replace(dst, src.take());
    if let Some(p) = old {
        release(p);
    }
}

// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// demangled: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
pub fn stub_4d2dc(dst: &mut Option<usize>, src: Option<usize>, retain: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) {
    // IDA 0x4d2dc: shared_ptr<TextBox> copy-assign — retain src; release old (below truncation).
    if let Some(s) = src {
        retain(s);
    }
    let old = std::mem::replace(dst, src);
    if let Some(p) = old {
        release(p);
    }
}

// 0x4d398 — __GLOBAL__I_a_20
// demangled: global constructor keyed to_a_20
#[doc(alias = "global constructor keyed to_a_20")]
pub fn stub_4d398(state: &mut GlobalInitA20, init: &mut dyn FnMut()) {
    // IDA 0x4d398: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x4d5ac — -[GameView initWithFrame:]
// type: GameView *__cdecl(GameView *self, SEL, CGRect)
#[doc(alias = "-[GameView initWithFrame:]")]
pub fn stub_4d5ac(view: usize, init_super: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x4d5ac: super initWithFrame.
    init_super(view)
}

// 0x4d5e4 — -[GameView layoutSubviews]
// type: void __cdecl(GameView *self, SEL)
#[doc(alias = "-[GameView layoutSubviews]")]
pub fn stub_4d5e4(layout: &mut dyn FnMut()) {
    // IDA 0x4d5e4: GameView layoutSubviews — Ogre viewport resize (below truncation).
    layout();
}

// 0x4d6d4 — __GLOBAL__I_a_21
// demangled: global constructor keyed to_a_21
// type: int()
#[doc(alias = "global constructor keyed to_a_21")]
pub fn stub_4d6d4(state: &mut GlobalInitA21, init: &mut dyn FnMut()) {
    // IDA 0x4d6d4: ios_base::Init + atexit.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
#[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
pub fn stub_4d70c(ok: bool, setup: &mut dyn FnMut()) -> bool {
    // IDA 0x4d70c: GameViewController initWithNibName — view + observers (below truncation).
    if !ok {
        return false;
    }
    setup();
    true
}

// 0x4d8cc — -[GameViewController dealloc]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController dealloc]")]
pub fn stub_4d8cc(has_webview: bool, remove: &mut dyn FnMut(), teardown: &mut dyn FnMut()) {
    // IDA 0x4d8cc: remove webview; removeObserver; super dealloc.
    if has_webview {
        remove();
    }
    teardown();
}

// 0x4d978 — -[GameViewController viewWillAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewWillAppear:]")]
pub fn stub_4d978(super_call: &mut dyn FnMut(bool), hide_bar: &mut dyn FnMut()) {
    // IDA 0x4d978: super viewWillAppear; statusBarHidden = YES.
    super_call(true);
    hide_bar();
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewDidAppear:]")]
pub fn stub_4d9d4(super_call: &mut dyn FnMut(bool)) {
    // IDA 0x4d9d4: super viewDidAppear.
    super_call(true);
}

// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController viewDidLoad]")]
pub fn stub_4da00(setup: &mut dyn FnMut()) {
    // IDA 0x4da00: super viewDidLoad + user-agent dict (below truncation).
    setup();
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
pub fn stub_4dab8(super_call: &mut dyn FnMut()) {
    // IDA 0x4dab8: super didReceiveMemoryWarning (EAGLViewController).
    super_call();
}

// 0x4dae4 — -[GameViewController resizeGameView]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController resizeGameView]")]
pub fn stub_4dae4(layout: &mut dyn FnMut()) {
    // IDA 0x4dae4: resizeGameView -> layoutSubviews.
    layout();
}

// 0x4db04 — -[GameViewController shouldAutorotate]
// type: char __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController shouldAutorotate]")]
pub fn stub_4db04() -> bool {
    // IDA 0x4db04: shouldAutorotate returns YES.
    true
}

// 0x4db08 — -[GameViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController supportedInterfaceOrientations]")]
pub fn stub_4db08() -> u32 {
    // IDA 0x4db08: supportedInterfaceOrientations = 24 (landscape).
    24
}

// 0x4db0c — -[GameViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(GameViewController *self, SEL, int)
#[doc(alias = "-[GameViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_4db0c(orientation: i32) -> bool {
    // IDA 0x4db0c: YES when landscape (3/4).
    orientation == 4 || orientation == 3
}

// 0x4db20 — -[GameViewController getControlView]
// type: id __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController getControlView]")]
pub fn stub_4db20(first: Option<usize>) -> Option<usize> {
    // IDA 0x4db20: first subview or nil.
    first
}

// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
#[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_4db9c(open_native: bool, purchase_ok: bool) -> bool {
    // IDA 0x4db9c: !OpenNativeBrowser || no in-app purchase.
    !open_native || purchase_ok
}

// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
#[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
pub fn stub_4dbe8(has_model: bool, has_service: bool, fire: &mut dyn FnMut()) {
    // IDA 0x4dbe8: nil model/service -> return; else fire GuiService url-closed signal.
    if has_model && has_service {
        fire();
    }
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController closeUrlWindow:]")]
pub fn stub_4dc08(close: &mut dyn FnMut()) {
    // IDA 0x4dc08: closeUrlWindow (below truncation).
    close();
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
pub fn stub_4de58(animate: &mut dyn FnMut()) {
    // IDA 0x4de58: closeUrlWindow animation block (below truncation).
    animate();
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
pub fn stub_4df1c(animate: &mut dyn FnMut()) {
    // IDA 0x4df1c: closeUrlWindow animation block 2 (below truncation).
    animate();
}

// 0x4dfd8 — ___copy_helper_block__10
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__10")]
pub fn stub_4dfd8() -> ! {
    todo!("0x4dfd8 ___copy_helper_block__10")
}

// 0x4dfe4 — ___destroy_helper_block__10
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__10")]
pub fn stub_4dfe4() -> ! {
    todo!("0x4dfe4 ___destroy_helper_block__10")
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
pub fn stub_4dfec() -> ! {
    todo!("0x4dfec ___37-[GameViewController closeUrlWindow:]_block_invoke93")
}

// 0x4e01c — ___copy_helper_block_94
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_94")]
pub fn stub_4e01c() -> ! {
    todo!("0x4e01c ___copy_helper_block_94")
}

// 0x4e028 — ___destroy_helper_block_95
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_95")]
pub fn stub_4e028() -> ! {
    todo!("0x4e028 ___destroy_helper_block_95")
}

// 0x4e030 — ___copy_helper_block_100
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_100")]
pub fn stub_4e030() -> ! {
    todo!("0x4e030 ___copy_helper_block_100")
}

// 0x4e054 — ___destroy_helper_block_101
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_101")]
pub fn stub_4e054() -> ! {
    todo!("0x4e054 ___destroy_helper_block_101")
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController closeUrlWindow]")]
pub fn stub_4e070() -> ! {
    todo!("0x4e070 -[GameViewController closeUrlWindow]")
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[GameViewController openUrlWindow:]")]
pub fn stub_4e084() -> ! {
    todo!("0x4e084 -[GameViewController openUrlWindow:]")
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
pub fn stub_4e2ac() -> ! {
    todo!("0x4e2ac ___36-[GameViewController openUrlWindow:]_block_invoke")
}

// 0x4e4c8 — ___copy_helper_block_133
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_133")]
pub fn stub_4e4c8() -> ! {
    todo!("0x4e4c8 ___copy_helper_block_133")
}

// 0x4e4d4 — ___destroy_helper_block_134
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_134")]
pub fn stub_4e4d4() -> ! {
    todo!("0x4e4d4 ___destroy_helper_block_134")
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
pub fn stub_4e4dc() -> ! {
    todo!("0x4e4dc ___36-[GameViewController openUrlWindow:]_block_invoke136")
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
pub fn stub_4e5fc() -> ! {
    todo!("0x4e5fc ___36-[GameViewController openUrlWindow:]_block_invoke_2")
}

// 0x4e6dc — ___copy_helper_block_148
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_148")]
pub fn stub_4e6dc() -> ! {
    todo!("0x4e6dc ___copy_helper_block_148")
}

// 0x4e6e8 — ___destroy_helper_block_149
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_149")]
pub fn stub_4e6e8() -> ! {
    todo!("0x4e6e8 ___destroy_helper_block_149")
}

// 0x4e6f0 — ___copy_helper_block_153
// type: int __fastcall(int, int)
#[doc(alias = "___copy_helper_block_153")]
pub fn stub_4e6f0() -> ! {
    todo!("0x4e6f0 ___copy_helper_block_153")
}

// 0x4e714 — ___destroy_helper_block_154
// type: int __fastcall(int)
#[doc(alias = "___destroy_helper_block_154")]
pub fn stub_4e714() -> ! {
    todo!("0x4e714 ___destroy_helper_block_154")
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
pub fn stub_4e730() -> ! {
    todo!("0x4e730 -[GameViewController handlePromptLoginSignal]")
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
pub fn stub_4e780() -> ! {
    todo!("0x4e780 ___45-[GameViewController handlePromptLoginSignal]_block_invoke")
}

// 0x4e854 — ___copy_helper_block_174
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_174")]
pub fn stub_4e854() -> ! {
    todo!("0x4e854 ___copy_helper_block_174")
}

// 0x4e860 — ___destroy_helper_block_175
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_175")]
pub fn stub_4e860() -> ! {
    todo!("0x4e860 ___destroy_helper_block_175")
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
pub fn stub_4e868() -> ! {
    todo!("0x4e868 -[GameViewController handlePromptSignupSignal]")
}

// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
pub fn stub_4e8b8() -> ! {
    todo!("0x4e8b8 ___46-[GameViewController handlePromptSignupSignal]_block_invoke")
}

// 0x4e98c — ___copy_helper_block_179
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_179")]
pub fn stub_4e98c() -> ! {
    todo!("0x4e98c ___copy_helper_block_179")
}

// 0x4e998 — ___destroy_helper_block_180
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_180")]
pub fn stub_4e998() -> ! {
    todo!("0x4e998 ___destroy_helper_block_180")
}

// 0x4e9a0 — -[GameViewController handleSignupNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleSignupNotification:]")]
pub fn stub_4e9a0() -> ! {
    todo!("0x4e9a0 -[GameViewController handleSignupNotification:]")
}

// 0x4ea30 — -[GameViewController handleLoginNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleLoginNotification:]")]
pub fn stub_4ea30() -> ! {
    todo!("0x4ea30 -[GameViewController handleLoginNotification:]")
}

// 0x4eac8 — ___46-[GameViewController handleLoginNotification:]_block_invoke
// type: void __fastcall(id *)
#[doc(alias = "___46-[GameViewController handleLoginNotification:]_block_invoke")]
pub fn stub_4eac8() -> ! {
    todo!("0x4eac8 ___46-[GameViewController handleLoginNotification:]_block_invoke")
}

// 0x4edcc — ___copy_helper_block_203
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_203")]
pub fn stub_4edcc() -> ! {
    todo!("0x4edcc ___copy_helper_block_203")
}

// 0x4edf0 — ___destroy_helper_block_204
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_204")]
pub fn stub_4edf0() -> ! {
    todo!("0x4edf0 ___destroy_helper_block_204")
}

// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// demangled: rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
pub fn stub_4ee0c() -> ! {
    todo!("0x4ee0c rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)")
}

// 0x4ef74 — __GLOBAL__I_a_22
// demangled: global constructor keyed to_a_22
#[doc(alias = "global constructor keyed to_a_22")]
pub fn stub_4ef74() -> ! {
    todo!("0x4ef74 global constructor keyed to_a_22")
}

// 0x4f188 — -[JumpButton initWithFrame:]
// type: JumpButton *__cdecl(JumpButton *self, SEL, CGRect)
#[doc(alias = "-[JumpButton initWithFrame:]")]
pub fn stub_4f188() -> ! {
    todo!("0x4f188 -[JumpButton initWithFrame:]")
}

// 0x4f2b0 — -[JumpButton dealloc]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton dealloc]")]
pub fn stub_4f2b0() -> ! {
    todo!("0x4f2b0 -[JumpButton dealloc]")
}

// 0x4f2fc — -[JumpButton setControlComponentSuperview:]
// type: void __cdecl(JumpButton *self, SEL, id)
#[doc(alias = "-[JumpButton setControlComponentSuperview:]")]
pub fn stub_4f2fc() -> ! {
    todo!("0x4f2fc -[JumpButton setControlComponentSuperview:]")
}

// 0x4f404 — -[JumpButton jumpEnabledChanged:]
// type: void __cdecl(JumpButton *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[JumpButton jumpEnabledChanged:]")]
pub fn stub_4f404() -> ! {
    todo!("0x4f404 -[JumpButton jumpEnabledChanged:]")
}

// 0x4f408 — -[JumpButton touchDown]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchDown]")]
pub fn stub_4f408() -> ! {
    todo!("0x4f408 -[JumpButton touchDown]")
}

// 0x4f43c — -[JumpButton touchUp]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchUp]")]
pub fn stub_4f43c() -> ! {
    todo!("0x4f43c -[JumpButton touchUp]")
}

// 0x4f470 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
pub fn stub_4f470() -> ! {
    todo!("0x4f470 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")
}

// 0x4f4e4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_4f4e4() -> ! {
    todo!("0x4f4e4 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

// 0x4f590 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_4f590() -> ! {
    todo!("0x4f590 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

// 0x4f640 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_4f640() -> ! {
    todo!("0x4f640 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x4f650 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_4f650() -> ! {
    todo!("0x4f650 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0x4f660 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_4f660() -> ! {
    todo!("0x4f660 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x4f70c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_4f70c() -> ! {
    todo!("0x4f70c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x4f7bc — __GLOBAL__I_a_23
// demangled: global constructor keyed to_a_23
#[doc(alias = "global constructor keyed to_a_23")]
pub fn stub_4f7bc() -> ! {
    todo!("0x4f7bc global constructor keyed to_a_23")
}

// 0x4f9d0 — -[ThumbStickControl init:]
// type: id __cdecl(ThumbStickControl *self, SEL, CGRect)
#[doc(alias = "-[ThumbStickControl init:]")]
pub fn stub_4f9d0() -> ! {
    todo!("0x4f9d0 -[ThumbStickControl init:]")
}

// 0x4fcf4 — ___26-[ThumbStickControl init:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___26-[ThumbStickControl init:]_block_invoke")]
pub fn stub_4fcf4() -> ! {
    todo!("0x4fcf4 ___26-[ThumbStickControl init:]_block_invoke")
}

// 0x4fd40 — ___copy_helper_block__11
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__11")]
pub fn stub_4fd40() -> ! {
    todo!("0x4fd40 ___copy_helper_block__11")
}

// 0x4fd4c — ___destroy_helper_block__11
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__11")]
pub fn stub_4fd4c() -> ! {
    todo!("0x4fd4c ___destroy_helper_block__11")
}

// 0x4fd54 — -[ThumbStickControl dealloc]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl dealloc]")]
pub fn stub_4fd54() -> ! {
    todo!("0x4fd54 -[ThumbStickControl dealloc]")
}

// 0x4fdb8 — -[ThumbStickControl intToThumbstickStyle:]
// type: int __cdecl(ThumbStickControl *self, SEL, int)
#[doc(alias = "-[ThumbStickControl intToThumbstickStyle:]")]
pub fn stub_4fdb8() -> ! {
    todo!("0x4fdb8 -[ThumbStickControl intToThumbstickStyle:]")
}

// 0x4fdc4 — -[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]
// type: float __cdecl(ThumbStickControl *self, SEL, CGPoint, CGPoint)
#[doc(alias = "-[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]")]
pub fn stub_4fdc4() -> ! {
    todo!("0x4fdc4 -[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]")
}

// 0x4fdf4 — -[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]
// type: CGPoint *__cdecl(CGPoint *__return_ptr __struct_ptr retstr, ThumbStickControl *self, SEL, CGPoint, CGPoint, float)
#[doc(alias = "-[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]")]
pub fn stub_4fdf4() -> ! {
    todo!("0x4fdf4 -[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]")
}

// 0x4fe88 — -[ThumbStickControl touchesBegan:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesBegan:withEvent:]")]
pub fn stub_4fe88() -> ! {
    todo!("0x4fe88 -[ThumbStickControl touchesBegan:withEvent:]")
}

// 0x50108 — -[ThumbStickControl stationaryThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl stationaryThumbstickTouchMove]")]
pub fn stub_50108() -> ! {
    todo!("0x50108 -[ThumbStickControl stationaryThumbstickTouchMove]")
}

// 0x50338 — -[ThumbStickControl followThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl followThumbstickTouchMove]")]
pub fn stub_50338() -> ! {
    todo!("0x50338 -[ThumbStickControl followThumbstickTouchMove]")
}

// 0x506cc — -[ThumbStickControl touchesMoved:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesMoved:withEvent:]")]
pub fn stub_506cc() -> ! {
    todo!("0x506cc -[ThumbStickControl touchesMoved:withEvent:]")
}

// 0x508b0 — -[ThumbStickControl cancelMovement]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl cancelMovement]")]
pub fn stub_508b0() -> ! {
    todo!("0x508b0 -[ThumbStickControl cancelMovement]")
}

// 0x50960 — ___35-[ThumbStickControl cancelMovement]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke")]
pub fn stub_50960() -> ! {
    todo!("0x50960 ___35-[ThumbStickControl cancelMovement]_block_invoke")
}

// 0x509a8 — ___copy_helper_block_77
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_77")]
pub fn stub_509a8() -> ! {
    todo!("0x509a8 ___copy_helper_block_77")
}

// 0x509b4 — ___destroy_helper_block_78
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_78")]
pub fn stub_509b4() -> ! {
    todo!("0x509b4 ___destroy_helper_block_78")
}

// 0x50c18 — ___35-[ThumbStickControl cancelMovement]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke_2")]
pub fn stub_50c18() -> ! {
    todo!("0x50c18 ___35-[ThumbStickControl cancelMovement]_block_invoke_2")
}

// 0x50c6c — ___copy_helper_block_81
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_81")]
pub fn stub_50c6c() -> ! {
    todo!("0x50c6c ___copy_helper_block_81")
}

// 0x50c78 — ___destroy_helper_block_82
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_82")]
pub fn stub_50c78() -> ! {
    todo!("0x50c78 ___destroy_helper_block_82")
}

// 0x50c80 — ___35-[ThumbStickControl cancelMovement]_block_invoke84
// type: void __cdecl(id, char)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke84")]
pub fn stub_50c80() -> ! {
    todo!("0x50c80 ___35-[ThumbStickControl cancelMovement]_block_invoke84")
}

// 0x50c84 — ___copy_helper_block_89
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_89")]
pub fn stub_50c84() -> ! {
    todo!("0x50c84 ___copy_helper_block_89")
}

// 0x50c90 — ___destroy_helper_block_90
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_90")]
pub fn stub_50c90() -> ! {
    todo!("0x50c90 ___destroy_helper_block_90")
}

// 0x50c98 — __GLOBAL__I_a_24
// demangled: global constructor keyed to_a_24
#[doc(alias = "global constructor keyed to_a_24")]
pub fn stub_50c98() -> ! {
    todo!("0x50c98 global constructor keyed to_a_24")
}

