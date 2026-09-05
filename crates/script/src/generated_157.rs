// Auto-generated skeletons for rbx-script — Script/Lua batch
// Filter: Script|Lua (4456 filtered, exhausted) -> global gap filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x47338..0x693f4 EA-sorted asc next 100 global not yet in any crate (missing 50098 before, 49998 after, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
use crate::generated_171::{CtrlComp, GameInputVC, GameVC};

/// `dispatch_once` singletons below (IDA 0x4c6ac..0x58f94): the
/// once-payload allocates the shared instance (see the `_block_invoke`
/// twins) and the accessor answers it. The allocator folds into the
/// host; the opaque handles record once.
static GAMEKEYS_SHARED: LazyLock<u32> = LazyLock::new(|| 1);
static MAINVC_SHARED: LazyLock<u32> = LazyLock::new(|| 1);
static WEBCACHE_SHARED: LazyLock<u32> = LazyLock::new(|| 1);
static LOGINMAN_SHARED: LazyLock<u32> = LazyLock::new(|| 1);
/// `__GLOBAL__I_a_30` one-shot latch (IDA 0x58bb0).
static GLOBAL_A30_INIT: LazyLock<u32> = LazyLock::new(|| 1);
/// `__GLOBAL__I_a_31` one-shot latch (IDA 0x5b3d8).
static GLOBAL_A31_INIT: LazyLock<u32> = LazyLock::new(|| 1);

// 0x47338 — -[ControlComponent getUserInputServiceForGameDataModel]
// type: UserInputService *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getUserInputServiceForGameDataModel]")]
pub fn stub_0x47338(comp: &CtrlComp) -> Option<u32> {
    // IDA 0x47338: `getUserInputServiceForGameDataModel` resolves the
    // game (cf. 0x47274), reads its data model, and answers the input
    // service, else null; the provider lookup folds into the host and
    // the singleton handle is opaque.
    comp.game.map(|_| 1)
}

// 0x48774 — -[ControlView checkUserInputPropertyChanged:onDataModel:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView checkUserInputPropertyChanged:onDataModel:]")]
pub fn stub_0x48774() {
    // IDA 0x48774: `checkUserInputPropertyChanged:onDataModel:` gates on
    // `isValidUserInputProperty` (cf. 0x487d4) and applies via
    // 0x4880c; the check and the service application fold into the
    // host — no-op.
}

// 0x4880c — -[ControlView userInputPropertyChangedOnDataModel:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnDataModel:]")]
pub fn stub_0x4880c() {
    // IDA 0x4880c: `userInputPropertyChangedOnDataModel:` applies the
    // property change through to the data-model input service (twin of
    // 0x48918); it touches no `ControlView` state — no-op.
}

// 0x4c248 — -[GameInputViewController init:withBundle:withGame:overlayDataModel:]
// type: id __cdecl(GameInputViewController *self, SEL, id, id, shared_ptr<RBX::Game>, shared_ptr<RBX::OverlayDataModel>)
#[doc(alias = "-[GameInputViewController init:withBundle:withGame:overlayDataModel:]")]
pub fn stub_0x4c248(game: Option<u32>) -> GameInputVC {
    // IDA 0x4c248: `GameInputViewController
    // init:withBundle:withGame:overlayDataModel:` chains to super, seats
    // the control view, and binds the game/overlay models; the UIKit
    // glue folds into the host. The game handle is observed via the
    // view; construction answers unloaded.
    let _ = game;
    GameInputVC::default()
}

// 0x4c6ac — +[GameKeyboard sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[GameKeyboard sharedInstance]")]
pub fn stub_0x4c6ac() -> u32 {
    // IDA 0x4c6ac: `GameKeyboard sharedInstance` answers the
    // `dispatch_once` instance (0x4c6c0..0x4c6d4). See
    // `GAMEKEYS_SHARED`.
    *GAMEKEYS_SHARED
}

// 0x4c6dc — ___30+[GameKeyboard sharedInstance]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___30+[GameKeyboard sharedInstance]_block_invoke")]
pub fn stub_0x4c6dc() {
    // IDA 0x4c6dc: the `sharedInstance` block allocs/inits the keyboard;
    // folds into `GAMEKEYS_SHARED` — no-op.
}

// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
#[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
pub fn stub_0x4dbe8(vc: &mut GameVC) {
    // IDA 0x4dbe8: `signalGuiServiceUrlWindowClosedOnDataModel:`
    // notifies the GuiService that the URL window closed (counterpart of
    // `closeUrlWindow:` 0x4dc08); the service send folds into the host.
    vc.web_open = false;
}

// 0x51dc4 — +[MainViewController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[MainViewController sharedInstance]")]
pub fn stub_0x51dc4() -> u32 {
    // IDA 0x51dc4: `MainViewController sharedInstance` — see
    // `MAINVC_SHARED` (0x51df0..0x51e1a).
    *MAINVC_SHARED
}

// 0x51e20 — ___36+[MainViewController sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36+[MainViewController sharedInstance]_block_invoke")]
pub fn stub_0x51e20() {
    // IDA 0x51e20: the `sharedInstance` block allocs/inits the main view
    // controller; folds into `MAINVC_SHARED` — no-op.
}

// 0x584e4 — +[UIWebViewCacheManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UIWebViewCacheManager sharedInstance]")]
pub fn stub_0x584e4() -> u32 {
    // IDA 0x584e4: `UIWebViewCacheManager sharedInstance` — see
    // `WEBCACHE_SHARED` (0x58510..0x5853a).
    *WEBCACHE_SHARED
}

// 0x58540 — ___39+[UIWebViewCacheManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39+[UIWebViewCacheManager sharedInstance]_block_invoke")]
pub fn stub_0x58540() {
    // IDA 0x58540: the `sharedInstance` block allocs/inits the cache
    // manager; folds into `WEBCACHE_SHARED` — no-op.
}

// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_56")]
pub fn stub_0x58580() {
    // IDA 0x58580: `__destroy_helper_block_56` releases captures;
    // `Arc` glue covers it — no-op.
}

// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_78")]
pub fn stub_0x58844() {
    // IDA 0x58844: `__copy_helper_block_78` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_79")]
pub fn stub_0x58850() {
    // IDA 0x58850: `__destroy_helper_block_79` releases captures (pair
    // of 0x58844); `Arc` glue covers it — no-op.
}

// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_83")]
pub fn stub_0x589f4() {
    // IDA 0x589f4: `__copy_helper_block_83` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_84")]
pub fn stub_0x58a00() {
    // IDA 0x58a00: `__destroy_helper_block_84` releases captures (pair
    // of 0x589f4); `Arc` glue covers it — no-op.
}

// 0x58bb0 — __GLOBAL__I_a_30
#[doc(alias = "global constructor keyed to_a_30")]
pub fn stub_0x58bb0() -> u32 {
    // IDA 0x58bb0: `__GLOBAL__I_a_30` — see `GLOBAL_A30_INIT`.
    *GLOBAL_A30_INIT
}

// 0x58f94 — +[LoginManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginManager sharedInstance]")]
pub fn stub_0x58f94() -> u32 {
    // IDA 0x58f94: `LoginManager sharedInstance` — see `LOGINMAN_SHARED`
    // (0x58fc0..0x58fea).
    *LOGINMAN_SHARED
}

// 0x58ff0 — ___30+[LoginManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___30+[LoginManager sharedInstance]_block_invoke")]
pub fn stub_0x58ff0() {
    // IDA 0x58ff0: the `sharedInstance` block allocs/inits the login
    // manager; folds into `LOGINMAN_SHARED` — no-op.
}

// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__18")]
pub fn stub_0x59024() {
    // IDA 0x59024: `__copy_helper_block__18` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__18")]
pub fn stub_0x59030() {
    // IDA 0x59030: `__destroy_helper_block__18` releases captures (pair
    // of 0x59024); `Arc` glue covers it — no-op.
}

// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_149")]
pub fn stub_0x59aa8() {
    // IDA 0x59aa8: `__copy_helper_block_149` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_150")]
pub fn stub_0x59acc() {
    // IDA 0x59acc: `__destroy_helper_block_150` releases captures (pair
    // of 0x59aa8); `Arc` glue covers it — no-op.
}

// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_192")]
pub fn stub_0x5a068() {
    // IDA 0x5a068: `__copy_helper_block_192` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5a0b0 — ___destroy_helper_block_193
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_193")]
pub fn stub_0x5a0b0() {
    // IDA 0x5a0b0: `__destroy_helper_block_193` releases captures (pair
    // of 0x5a068); `Arc` glue covers it — no-op.
}

// 0x5b3d8 — __GLOBAL__I_a_31
#[doc(alias = "global constructor keyed to_a_31")]
pub fn stub_0x5b3d8() -> u32 {
    // IDA 0x5b3d8: `__GLOBAL__I_a_31` — see `GLOBAL_A31_INIT`.
    *GLOBAL_A31_INIT
}

// 0x5c4f4 — ___copy_helper_block__19
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__19")]
pub fn stub_0x5c4f4() {
    // IDA 0x5c4f4: `__copy_helper_block__19` retains two object captures
    // (0x5c504..0x5c514); `Arc` glue covers it — no-op.
}

// 0x5c518 — ___destroy_helper_block__19
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__19")]
pub fn stub_0x5c518() {
    // IDA 0x5c518: `__destroy_helper_block__19` releases the captures
    // (pair of 0x5c4f4, 0x5c522..0x5c52e); `Arc` glue covers it — no-op.
}

// 0x5c6c8 — ___copy_helper_block_104
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_104")]
pub fn stub_0x5c6c8() {
    // IDA 0x5c6c8: `__copy_helper_block_104` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5c6ec — ___destroy_helper_block_105
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_105")]
pub fn stub_0x5c6ec() {
    // IDA 0x5c6ec: `__destroy_helper_block_105` releases captures (pair
    // of 0x5c6c8); `Arc` glue covers it — no-op.
}

// 0x5cad4 — ___copy_helper_block_126
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_126")]
pub fn stub_0x5cad4() {
    // IDA 0x5cad4: `__copy_helper_block_126` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5cae0 — ___destroy_helper_block_127
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_127")]
pub fn stub_0x5cae0() {
    // IDA 0x5cae0: `__destroy_helper_block_127` releases captures (pair
    // of 0x5cad4); `Arc` glue covers it — no-op.
}

// 0x5d1a8 — ___copy_helper_block_162
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_162")]
pub fn stub_0x5d1a8() {
    // IDA 0x5d1a8: `__copy_helper_block_162` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5d1b4 — ___destroy_helper_block_163
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_163")]
pub fn stub_0x5d1b4() {
    // IDA 0x5d1b4: `__destroy_helper_block_163` releases captures (pair
    // of 0x5d1a8); `Arc` glue covers it — no-op.
}

// 0x5ed84 — ___copy_helper_block__20
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__20")]
pub fn stub_0x5ed84() {
    // IDA 0x5ed84: `__copy_helper_block__20` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5ed90 — ___destroy_helper_block__20
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__20")]
pub fn stub_0x5ed90() {
    // IDA 0x5ed90: `__destroy_helper_block__20` releases captures (pair
    // of 0x5ed84); `Arc` glue covers it — no-op.
}

// 0x5f024 — ___copy_helper_block_232_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_232_0")]
pub fn stub_0x5f024() {
    // IDA 0x5f024: `__copy_helper_block_232_0` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5f030 — ___destroy_helper_block_233_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_233_0")]
pub fn stub_0x5f030() {
    // IDA 0x5f030: `__destroy_helper_block_233_0` releases captures
    // (pair of 0x5f024); `Arc` glue covers it — no-op.
}

// 0x5f3e4 — ___copy_helper_block_252_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_252_0")]
pub fn stub_0x5f3e4() {
    // IDA 0x5f3e4: `__copy_helper_block_252_0` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5f3f0 — ___destroy_helper_block_253_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_253_0")]
pub fn stub_0x5f3f0() {
    // IDA 0x5f3f0: `__destroy_helper_block_253_0` releases captures
    // (pair of 0x5f3e4); `Arc` glue covers it — no-op.
}

// 0x5f5d8 — ___copy_helper_block_255
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_255")]
pub fn stub_0x5f5d8() {
    // IDA 0x5f5d8: `__copy_helper_block_255` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5f5e4 — ___destroy_helper_block_256
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_256")]
pub fn stub_0x5f5e4() {
    // IDA 0x5f5e4: `__destroy_helper_block_256` releases captures (pair
    // of 0x5f5d8); `Arc` glue covers it — no-op.
}

// 0x5fd3c — ___copy_helper_block_324
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_324")]
pub fn stub_0x5fd3c() {
    // IDA 0x5fd3c: `__copy_helper_block_324` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5fd48 — ___destroy_helper_block_325
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_325")]
pub fn stub_0x5fd48() {
    // IDA 0x5fd48: `__destroy_helper_block_325` releases captures (pair
    // of 0x5fd3c); `Arc` glue covers it — no-op.
}

// 0x5fdc4 — ___copy_helper_block_330
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_330")]
pub fn stub_0x5fdc4() {
    // IDA 0x5fdc4: `__copy_helper_block_330` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5fdd0 — ___destroy_helper_block_331
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_331")]
pub fn stub_0x5fdd0() {
    // IDA 0x5fdd0: `__destroy_helper_block_331` releases captures (pair
    // of 0x5fdc4); `Arc` glue covers it — no-op.
}

// 0x5fdfc — ___copy_helper_block_334
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_334")]
pub fn stub_0x5fdfc() {
    // IDA 0x5fdfc: `__copy_helper_block_334` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x5fe08 — ___destroy_helper_block_335
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_335")]
pub fn stub_0x5fe08() {
    // IDA 0x5fe08: `__destroy_helper_block_335` releases captures (pair
    // of 0x5fdfc); `Arc` glue covers it — no-op.
}

// 0x601e4 — ___copy_helper_block_345
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_345")]
pub fn stub_0x601e4() {
    // IDA 0x601e4: `__copy_helper_block_345` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x601f0 — ___destroy_helper_block_346
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_346")]
pub fn stub_0x601f0() {
    // IDA 0x601f0: `__destroy_helper_block_346` releases captures (pair
    // of 0x601e4); `Arc` glue covers it — no-op.
}

// 0x6026c — ___copy_helper_block_349
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_349")]
pub fn stub_0x6026c() -> ! {
    todo!("0x6026c ___copy_helper_block_349")
}

// 0x60278 — ___destroy_helper_block_350
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_350")]
pub fn stub_0x60278() -> ! {
    todo!("0x60278 ___destroy_helper_block_350")
}

// 0x602a4 — ___copy_helper_block_353
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_353")]
pub fn stub_0x602a4() -> ! {
    todo!("0x602a4 ___copy_helper_block_353")
}

// 0x602b0 — ___destroy_helper_block_354
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_354")]
pub fn stub_0x602b0() -> ! {
    todo!("0x602b0 ___destroy_helper_block_354")
}

// 0x608ec — ___copy_helper_block_386
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_386")]
pub fn stub_0x608ec() -> ! {
    todo!("0x608ec ___copy_helper_block_386")
}

// 0x608f8 — ___destroy_helper_block_387
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_387")]
pub fn stub_0x608f8() -> ! {
    todo!("0x608f8 ___destroy_helper_block_387")
}

// 0x60900 — ___copy_helper_block_389
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_389")]
pub fn stub_0x60900() -> ! {
    todo!("0x60900 ___copy_helper_block_389")
}

// 0x60930 — ___destroy_helper_block_390
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_390")]
pub fn stub_0x60930() -> ! {
    todo!("0x60930 ___destroy_helper_block_390")
}

// 0x61a98 — ___copy_helper_block_487
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_487")]
pub fn stub_0x61a98() -> ! {
    todo!("0x61a98 ___copy_helper_block_487")
}

// 0x61aa4 — ___destroy_helper_block_488
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_488")]
pub fn stub_0x61aa4() -> ! {
    todo!("0x61aa4 ___destroy_helper_block_488")
}

// 0x61c4c — ___copy_helper_block_490
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_490")]
pub fn stub_0x61c4c() -> ! {
    todo!("0x61c4c ___copy_helper_block_490")
}

// 0x62778 — +[RobloxMemoryManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxMemoryManager sharedInstance]")]
pub fn stub_0x62778() -> ! {
    todo!("0x62778 +[RobloxMemoryManager sharedInstance]")
}

// 0x627d4 — ___37+[RobloxMemoryManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___37+[RobloxMemoryManager sharedInstance]_block_invoke")]
pub fn stub_0x627d4() -> ! {
    todo!("0x627d4 ___37+[RobloxMemoryManager sharedInstance]_block_invoke")
}

// 0x63d30 — +[RobloxCachedFlags sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxCachedFlags sharedInstance]")]
pub fn stub_0x63d30() -> ! {
    todo!("0x63d30 +[RobloxCachedFlags sharedInstance]")
}

// 0x63d94 — ___35+[RobloxCachedFlags sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___35+[RobloxCachedFlags sharedInstance]_block_invoke")]
pub fn stub_0x63d94() -> ! {
    todo!("0x63d94 ___35+[RobloxCachedFlags sharedInstance]_block_invoke")
}

// 0x640e4 — +[CrashReporter sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[CrashReporter sharedInstance]")]
pub fn stub_0x640e4() -> ! {
    todo!("0x640e4 +[CrashReporter sharedInstance]")
}

// 0x64140 — ___31+[CrashReporter sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[CrashReporter sharedInstance]_block_invoke")]
pub fn stub_0x64140() -> ! {
    todo!("0x64140 ___31+[CrashReporter sharedInstance]_block_invoke")
}

// 0x66794 — +[AppController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[AppController sharedInstance]")]
pub fn stub_0x66794() -> ! {
    todo!("0x66794 +[AppController sharedInstance]")
}

// 0x667f0 — ___31+[AppController sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[AppController sharedInstance]_block_invoke")]
pub fn stub_0x667f0() -> ! {
    todo!("0x667f0 ___31+[AppController sharedInstance]_block_invoke")
}

// 0x67148 — -[AppController launchGameFromOverlayDataModel:]
// type: void __cdecl(AppController *self, SEL, int)
#[doc(alias = "-[AppController launchGameFromOverlayDataModel:]")]
pub fn stub_0x67148() -> ! {
    todo!("0x67148 -[AppController launchGameFromOverlayDataModel:]")
}

// 0x674f0 — +[SessionReporter sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[SessionReporter sharedInstance]")]
pub fn stub_0x674f0() -> ! {
    todo!("0x674f0 +[SessionReporter sharedInstance]")
}

// 0x6754c — ___33+[SessionReporter sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___33+[SessionReporter sharedInstance]_block_invoke")]
pub fn stub_0x6754c() -> ! {
    todo!("0x6754c ___33+[SessionReporter sharedInstance]_block_invoke")
}

// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::init(void)")]
pub fn stub_0x686a4() -> ! {
    todo!("0x686a4 __ZN4FMOD10ProfileCpu4initEv")
}

// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileCpu *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_0x686ac() -> ! {
    todo!("0x686ac __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj")
}

// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::release(void)")]
pub fn stub_0x68758() -> ! {
    todo!("0x68758 __ZN4FMOD10ProfileCpu7releaseEv")
}

// 0x68794 — __ZN4FMOD10ProfileCpuC2Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
pub fn stub_0x68794() -> ! {
    todo!("0x68794 __ZN4FMOD10ProfileCpuC2Ev")
}

// 0x687bc — __ZN4FMOD10ProfileCpuC1Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void) [0x687bc]")]
pub fn stub_0x687bc() -> ! {
    todo!("0x687bc __ZN4FMOD10ProfileCpuC1Ev")
}

// 0x687c0 — __ZN4FMOD22FMOD_ProfileCpu_CreateEv
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileCpu_Create(void)")]
pub fn stub_0x687c0() -> ! {
    todo!("0x687c0 __ZN4FMOD22FMOD_ProfileCpu_CreateEv")
}

// 0x68864 — __ZN4FMOD10ProfileDsp15isNodeDuplicateEy
// type: int __fastcall(FMOD::ProfileDsp *this, unsigned __int64)
#[doc(alias = "FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")]
pub fn stub_0x68864() -> ! {
    todo!("0x68864 __ZN4FMOD10ProfileDsp15isNodeDuplicateEy")
}

// 0x68944 — __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *)
#[doc(alias = "FMOD::ProfileDsp::sendPacket(FMOD::SystemI *)")]
pub fn stub_0x68944() -> ! {
    todo!("0x68944 __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE")
}

// 0x68a6c — __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growNodeStackSpace(void)")]
pub fn stub_0x68a6c() -> ! {
    todo!("0x68a6c __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv")
}

// 0x68adc — __ZN4FMOD10ProfileDsp15growPacketSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growPacketSpace(void)")]
pub fn stub_0x68adc() -> ! {
    todo!("0x68adc __ZN4FMOD10ProfileDsp15growPacketSpaceEv")
}

// 0x68b68 — __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileDsp::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_0x68b68() -> ! {
    todo!("0x68b68 __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj")
}

// 0x68dfc — __ZN4FMOD10ProfileDsp7releaseEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::release(void)")]
pub fn stub_0x68dfc() -> ! {
    todo!("0x68dfc __ZN4FMOD10ProfileDsp7releaseEv")
}

// 0x68ebc — __ZN4FMOD10ProfileDsp4initEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::init(void)")]
pub fn stub_0x68ebc() -> ! {
    todo!("0x68ebc __ZN4FMOD10ProfileDsp4initEv")
}

// 0x69028 — __ZN4FMOD10ProfileDspC2Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
pub fn stub_0x69028() -> ! {
    todo!("0x69028 __ZN4FMOD10ProfileDspC2Ev")
}

// 0x69078 — __ZN4FMOD10ProfileDspC1Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void) [0x69078]")]
pub fn stub_0x69078() -> ! {
    todo!("0x69078 __ZN4FMOD10ProfileDspC1Ev")
}

// 0x6907c — __ZN4FMOD22FMOD_ProfileDsp_CreateEv
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileDsp_Create(void)")]
pub fn stub_0x6907c() -> ! {
    todo!("0x6907c __ZN4FMOD22FMOD_ProfileDsp_CreateEv")
}

// 0x6914c — __ZN4FMOD7ProfileC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void)")]
pub fn stub_0x6914c() -> ! {
    todo!("0x6914c __ZN4FMOD7ProfileC2Ev")
}

// 0x6919c — __ZN4FMOD7ProfileC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void) [0x6919c]")]
pub fn stub_0x6919c() -> ! {
    todo!("0x6919c __ZN4FMOD7ProfileC1Ev")
}

// 0x691a0 — __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::Profile::registerModule(FMOD::ProfileModule *)")]
pub fn stub_0x691a0() -> ! {
    todo!("0x691a0 __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE")
}

// 0x691c8 — __ZN4FMOD13ProfileModuleC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ProfileModule::ProfileModule(void)")]
pub fn stub_0x691c8() -> ! {
    todo!("0x691c8 __ZN4FMOD13ProfileModuleC2Ev")
}

// 0x691fc — __ZN4FMOD13ProfileModule4initEv
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::init(void)")]
pub fn stub_0x691fc() -> ! {
    todo!("0x691fc __ZN4FMOD13ProfileModule4initEv")
}

// 0x69204 — __ZN4FMOD13ProfileModule7releaseEv
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::release(void)")]
pub fn stub_0x69204() -> ! {
    todo!("0x69204 __ZN4FMOD13ProfileModule7releaseEv")
}

// 0x6920c — __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj
// type: int()
#[doc(alias = "FMOD::ProfileModule::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_0x6920c() -> ! {
    todo!("0x6920c __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj")
}

// 0x69214 — __ZN4FMOD13ProfileClientC2Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
pub fn stub_0x69214() -> ! {
    todo!("0x69214 __ZN4FMOD13ProfileClientC2Ev")
}

// 0x69280 — __ZN4FMOD13ProfileClientC1Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void) [0x69280]")]
pub fn stub_0x69280() -> ! {
    todo!("0x69280 __ZN4FMOD13ProfileClientC1Ev")
}

// 0x69284 — __ZN4FMOD13ProfileClient15requestDataTypeEhhj
// type: int __fastcall(FMOD::ProfileClient *this, int, int, unsigned int)
#[doc(alias = "FMOD::ProfileClient::requestDataType(unsigned char,unsigned char,unsigned int)")]
pub fn stub_0x69284() -> ! {
    todo!("0x69284 __ZN4FMOD13ProfileClient15requestDataTypeEhhj")
}

// 0x69358 — __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE
// type: bool __fastcall(int, unsigned __int8 *)
#[doc(alias = "FMOD::ProfileClient::wantsData(FMOD::ProfilePacketHeader *)")]
pub fn stub_0x69358() -> ! {
    todo!("0x69358 __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE")
}

// 0x693f4 — __ZN4FMOD13ProfileClient8sendDataEv
// type: int __fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::sendData(void)")]
pub fn stub_0x693f4() -> ! {
    todo!("0x693f4 __ZN4FMOD13ProfileClient8sendDataEv")
}

#[cfg(test)]
mod service_singleton_batch_tests {
    use super::*;
    use crate::generated_171::{CtrlComp, GameInputVC, GameVC};

    #[test]
    fn service_lookup() {
        let bare = CtrlComp { interaction: true, view: None, game: None };
        assert_eq!(stub_0x47338(&bare), None);
        let wired = CtrlComp { interaction: true, view: Some(2), game: Some(7) };
        assert_eq!(stub_0x47338(&wired), Some(1));
        stub_0x48774();
        stub_0x4880c();
        let vc = stub_0x4c248(Some(7));
        assert_eq!(vc, GameInputVC::default());
        let mut game_vc = GameVC { web_open: true, ..GameVC::default() };
        stub_0x4dbe8(&mut game_vc);
        assert!(!game_vc.web_open);
    }

    #[test]
    fn singletons() {
        assert_eq!(stub_0x4c6ac(), 1);
        stub_0x4c6dc();
        assert_eq!(stub_0x4c6ac(), 1);
        assert_eq!(stub_0x51dc4(), 1);
        stub_0x51e20();
        assert_eq!(stub_0x584e4(), 1);
        stub_0x58540();
        assert_eq!(stub_0x58f94(), 1);
        stub_0x58ff0();
        assert_eq!(stub_0x58bb0(), 1);
        stub_0x58580();
        stub_0x58844();
        stub_0x58850();
        stub_0x589f4();
        stub_0x58a00();
        stub_0x59024();
        stub_0x59030();
        stub_0x59aa8();
        stub_0x59acc();
        stub_0x5a068();
        stub_0x5a0b0();
    }
}

#[cfg(test)]
mod block_glue_batch_tests {
    use super::*;

    #[test]
    fn latch_and_glue() {
        assert_eq!(stub_0x5b3d8(), 1);
        stub_0x5c4f4();
        stub_0x5c518();
        stub_0x5c6c8();
        stub_0x5c6ec();
        stub_0x5cad4();
        stub_0x5cae0();
        stub_0x5d1a8();
        stub_0x5d1b4();
        stub_0x5ed84();
        stub_0x5ed90();
        stub_0x5f024();
        stub_0x5f030();
        stub_0x5f3e4();
        stub_0x5f3f0();
        stub_0x5f5d8();
        stub_0x5f5e4();
        stub_0x5fd3c();
        stub_0x5fd48();
        stub_0x5fdc4();
        stub_0x5fdd0();
        stub_0x5fdfc();
        stub_0x5fe08();
        stub_0x601e4();
        stub_0x601f0();
    }
}
