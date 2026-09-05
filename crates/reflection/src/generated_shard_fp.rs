// Auto-generated shard FP — 150 stubs EA-sorted asc 0x4BB44..0x51E20 (global gap filler not yet in reflection, 22234->22384 distinct)
// Source: ida/export.json (85545 funcs) EA asc not in crates/reflection/src/*.rs, next 150
// Format: // 0xADDR — mangled + doc alias + stub using rbx_core::SharedPtr not boost

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `DataModel` slot mutex handle for this shard (IDA 0x4bb44; same
/// shape as the shard_fo pair, shard-local record).
pub(crate) static FP_DM_SLOT_MUTEX: std::sync::LazyLock<u32> =
    std::sync::LazyLock::new(|| 1);
/// typeinfo name for the managed `bind_t<objc_object*,objc_selector*,
/// DataModel*>` (IDA 0x4bf6c, cf. 0x2d644).
pub const BIND_DM_OBJC_TYPEINFO: &str = "bind_t<objc_object*,objc_selector*,DataModel*>";
/// `GameInputViewController` view presence (IDA 0x4c248/0x4c3f4).
pub(crate) static GAMEINPUT_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `GameInputViewController::init:withBundle:withGame:overlayDataModel:`
/// args (IDA 0x4c248): the `ControlView` is built on the screen bounds
/// with the game and set as the view.
#[derive(Debug, Clone, Default)]
pub struct GameInputInit {
    pub game_present: bool,
    pub overlay_present: bool,
}
/// `GameKeyboard` state (IDA 0x4c71c-0x4ce44): init count, text,
/// visibility, current box, default string, parent flag and focus
/// releases. Views and notifications live out of slice.
pub(crate) static KEYBOARD_INITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static KEYBOARD_TEXT: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static KEYBOARD_VISIBLE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static KEYBOARD_CURRENT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static KEYBOARD_DEFAULT: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static KEYBOARD_PARENT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static FOCUS_RELEASES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `GameKeyboard::init` frame (IDA 0x4c71c): screen-bounds frame with
/// a hidden delegate text field plus show/hide observers.
#[derive(Debug, Clone, Default)]
pub struct KeyboardInit {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
/// `textboxDidFinishEditing` calls (IDA 0x4cfdc/0x4d0a4): submitted
/// text plus the return/dismiss flag. Service lookup lives out of
/// slice.
pub(crate) static TEXTBOX_FINISHES: std::sync::LazyLock<
    parking_lot::Mutex<Vec<(String, bool)>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
/// `GameView` bounds applied to the Ogre render window (IDA 0x4d5e4:
/// window resize + camera aspect from the bounds).
pub(crate) static GAMEVIEW_SIZE: std::sync::LazyLock<
    parking_lot::Mutex<(u32, u32)>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new((0, 0)));
/// `ThumbStickControl` touch state (IDA 0x4fe88-0x508b0): capture +
/// touched flags plus stationary/follow/move/cancel counts. Knob
/// geometry lives out of slice.
pub(crate) static THUMBSTICK_TOUCH: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static THUMBSTICK_BEEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static STICK_STATIONARY: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static STICK_FOLLOWS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static STICK_MOVES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static STICK_CANCELS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `GameMenu` state (IDA 0x50eb0-0x513f8): shown flag, show count,
/// leave requests and init count. Geometry and buttons live out of
/// slice.
pub(crate) static MENU_SHOWN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MENU_SHOWS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MENU_LEAVES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MENU_INITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Login attempts + results (IDA 0x4e9a0/0x4ea30): submitted
/// (username, password) pairs and reported success flags. `LoginManager`
/// lives out of slice.
pub(crate) static LOGIN_ATTEMPTS: std::sync::LazyLock<
    parking_lot::Mutex<Vec<(String, String)>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
pub(crate) static LOGIN_RESULTS: std::sync::LazyLock<
    parking_lot::Mutex<Vec<bool>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
pub(crate) static LOGIN_COMPLETIONS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Jump press/release counts (IDA 0x4f408/0x4f43c drive
/// `jumpLocalCharacter` 1/0 through the input service).
pub(crate) static JUMP_DOWNS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static JUMP_UPS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `JumpButton::initWithFrame:` args (IDA 0x4f188): frame plus the
/// jump/cloud images and touch targets, which are view glue.
#[derive(Debug, Clone, Default)]
pub struct JumpButtonInit {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
pub(crate) static JUMP_HAS_SUPERVIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `ThumbStickControl::init:` args (IDA 0x4f9d0): frame, style (0 for
/// new camera controls, else 1) and size (70 phone, else 120).
#[derive(Debug, Clone, Default)]
pub struct ThumbStickInit {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub style: u32,
    pub size: u32,
}
/// External URL-window state (IDA 0x4dc08-0x4e2ac): open flag, last
/// URL, open/dismiss counts and close-signal count. Web views and
/// main-queue blocks live out of slice.
pub(crate) static URL_WEB_OPEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static URL_LAST: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static URL_OPENS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static URL_DISMISSES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static URL_CLOSE_SIGNALS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Login-prompt dispatches (IDA 0x4e730-0x4e780): signal count plus
/// block-run count.
pub(crate) static LOGIN_PROMPTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_PROMPT_RUNS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_4bb44() -> u32 {
    // IDA 0x4bb44: `signal<DataModel*>::slot::safe_static_do_get_mutex`
    // one-shots the static slot mutex. The opaque handle records once.
    *FP_DM_SLOT_MUTEX
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev")]
pub fn stub_4bc34() {
    // IDA 0x4bc34: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev")]
pub fn stub_4bd08() {
    // IDA 0x4bd08: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev")]
pub fn stub_4bde0() {
    // IDA 0x4bde0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev")]
pub fn stub_4be8c() {
    // IDA 0x4be8c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_")]
pub fn stub_4bf3c() {
    // IDA 0x4bf3c: `function1<void,DataModel*>::assign_to_own`
    // copy-assigns the function. `Box<dyn Fn>` assignment glue; no
    // explicit body.
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_4bf6c(get_typeinfo: bool) -> &'static str {
    // IDA 0x4bf6c: `functor_manager<bind_t<objc_object*,objc_selector*,
    // DataModel*>>::manage` answers op 4 with the `bind_t` typeinfo.
    // Other ops are vtable glue.
    if get_typeinfo { BIND_DM_OBJC_TYPEINFO } else { "" }
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
pub fn stub_4bfcc() {
    // IDA 0x4bfcc: `void_function_obj_invoker1<bind_t<objc...>>::invoke`
    // runs the bound slot. Closure-call glue; no explicit body.
}

// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv")]
pub fn stub_4c008() {
    // IDA 0x4c008: `function1<void,SharedPtr<TextBox>>::clear` drops
    // the stored target. `Box<dyn Fn>` drop glue covers it; no
    // explicit body.
}

// 0x4c034 — __GLOBAL__I_a_18
#[doc(alias = "global constructor keyed to_a_18")]
#[doc(alias = "__GLOBAL__I_a_18")]
pub fn stub_4c034() {
    // IDA 0x4c034: `__GLOBAL__I_a_18` runs the `a_18`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4c248 — -[GameInputViewController init:withBundle:withGame:overlayDataModel:]
// type: id __cdecl(GameInputViewController *self, SEL, id, id, shared_ptr<RBX::Game>, shared_ptr<RBX::OverlayDataModel>)
#[doc(alias = "-[GameInputViewController init:withBundle:withGame:overlayDataModel:]")]
pub fn stub_4c248(game_present: bool, overlay_present: bool) -> GameInputInit {
    // IDA 0x4c248: `GameInputViewController::init:...` supers
    // (0x4c278), builds the `ControlView` on the screen bounds with
    // the game (0x4c2dc-0x4c36a) and sets it as the view (0x4c392).
    // The models record here.
    GAMEINPUT_VIEW.store(true, std::sync::atomic::Ordering::SeqCst);
    GameInputInit { game_present, overlay_present }
}

// 0x4c3f4 — -[GameInputViewController dealloc]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController dealloc]")]
pub fn stub_4c3f4() {
    // IDA 0x4c3f4: `dealloc` drops the control view. Release is drop
    // glue; the view flag resets here.
    GAMEINPUT_VIEW.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4c440 — -[GameInputViewController viewDidLoad]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidLoad]")]
pub fn stub_4c440() {
    // IDA 0x4c440: `viewDidLoad` supers (decompiled 0x4c440). No
    // explicit body.
}

// 0x4c46c — -[GameInputViewController viewDidUnload]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidUnload]")]
pub fn stub_4c46c() {
    // IDA 0x4c46c: `viewDidUnload` releases the view (standard
    // view-controller teardown). The view flag resets here.
    GAMEINPUT_VIEW.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4c498 — __GLOBAL__I_a_19
#[doc(alias = "global constructor keyed to_a_19")]
#[doc(alias = "__GLOBAL__I_a_19")]
pub fn stub_4c498() {
    // IDA 0x4c498: `__GLOBAL__I_a_19` runs the `a_19`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4c6ac — +[GameKeyboard sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[GameKeyboard sharedInstance]")]
pub fn stub_4c6ac() -> usize {
    // IDA 0x4c6ac: `sharedInstance` once-allocates the `GameKeyboard`.
    // The singleton handle records here as nonzero.
    1
}

// 0x4c6dc — ___30+[GameKeyboard sharedInstance]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___30+[GameKeyboard sharedInstance]_block_invoke")]
pub fn stub_4c6dc() {
    // IDA 0x4c6dc: the `sharedInstance` once block allocs + inits the
    // keyboard (0x4c6f8-0x4c716). It sequences the init here.
    stub_4c71c(0.0, 0.0, 0.0, 0.0);
}

// 0x4c71c — -[GameKeyboard init]
// type: GameKeyboard *__cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard init]")]
pub fn stub_4c71c(x: f32, y: f32, width: f32, height: f32) -> KeyboardInit {
    // IDA 0x4c71c: `GameKeyboard::init` clears the current box, sizes
    // the frame, installs the hidden delegate text field (0x4c79e-0x4c940)
    // and observes keyboard show/hide (0x4c962-0x4c9d8). The frame
    // records here; observers are engine glue.
    KEYBOARD_INITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    KeyboardInit { x, y, width, height }
}

// 0x4ca18 — -[GameKeyboard dealloc]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard dealloc]")]
pub fn stub_4ca18() {
    // IDA 0x4ca18: `dealloc` drops the keyboard. Release is drop glue;
    // the state resets here.
    *KEYBOARD_TEXT.lock() = String::new();
    KEYBOARD_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
    KEYBOARD_CURRENT.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4ca64 — -[GameKeyboard hideKeyboard]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard hideKeyboard]")]
pub fn stub_4ca64() {
    // IDA 0x4ca64: `hideKeyboard` clears the current box + text, hides
    // and disables the field and resigns (0x4c9a9-0x4cb42). The clear
    // records here.
    *KEYBOARD_TEXT.lock() = String::new();
    KEYBOARD_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
    KEYBOARD_CURRENT.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4cb80 — -[GameKeyboard keyboardWillHide:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillHide:]")]
pub fn stub_4cb80() {
    // IDA 0x4cb80: `keyboardWillHide:` releases the box focus
    // (0x4cb92-0x4cba2) and hides (0x4cbb8). It sequences here.
    FOCUS_RELEASES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_4ca64();
}

// 0x4cbbc — -[GameKeyboard keyboardWillChangeFrame:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillChangeFrame:]")]
pub fn stub_4cbbc() {
    // IDA 0x4cbbc: `keyboardWillChangeFrame:` compiles to an empty
    // body (decompiled 0x4cbbc). No explicit body.
}

// 0x4cbc0 — -[GameKeyboard setDefaultString:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setDefaultString:]")]
pub fn stub_4cbc0(default: &str) {
    // IDA 0x4cbc0: `setDefaultString:` stores the default. It records
    // here.
    *KEYBOARD_DEFAULT.lock() = default.to_owned();
}

// 0x4cbe0 — -[GameKeyboard setParentView:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setParentView:]")]
pub fn stub_4cbe0(parent_present: bool) {
    // IDA 0x4cbe0: `setParentView:` stores the parent. Presence
    // records here.
    KEYBOARD_PARENT.store(parent_present, std::sync::atomic::Ordering::SeqCst);
}

// 0x4cbf8 — -[GameKeyboard showKeyboard:]
// type: bool __cdecl(GameKeyboard *self, SEL, const char *)
#[doc(alias = "-[GameKeyboard showKeyboard:]")]
pub fn stub_4cbf8(text: &str, hidden: bool) -> bool {
    // IDA 0x4cbf8: `showKeyboard:` dispatches the show block when the
    // field is hidden (0x4cc20-0x4cc6e), else reports 0 (0x4cc76). The
    // branch reports here.
    if hidden {
        stub_4cc78(text);
        return true;
    }
    false
}

// 0x4cc78 — ___29-[GameKeyboard showKeyboard:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___29-[GameKeyboard showKeyboard:]_block_invoke")]
pub fn stub_4cc78(text: &str) {
    // IDA 0x4cc78: the show block sets the text and shows the field.
    // It records here.
    *KEYBOARD_TEXT.lock() = text.to_owned();
    KEYBOARD_VISIBLE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x4ce30 — ___copy_helper_block__9
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__9")]
pub fn stub_4ce30() {
    // IDA 0x4ce30: `__copy_helper_block__9` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4ce3c — ___destroy_helper_block__9
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__9")]
pub fn stub_4ce3c() {
    // IDA 0x4ce3c: `__destroy_helper_block__9` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4ce44 — -[GameKeyboard showKeyboardWithTextBox:]
// type: bool __cdecl(GameKeyboard *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[GameKeyboard showKeyboardWithTextBox:]")]
pub fn stub_4ce44(box_present: bool, hidden: bool, text: &str) -> bool {
    // IDA 0x4ce44: `showKeyboardWithTextBox:` stores a live box and
    // shows with its text when hidden (0x4ceb0-0x4cefc), else reports
    // 0 (0x4cf30). The branch reports here.
    if hidden && box_present {
        KEYBOARD_CURRENT.store(true, std::sync::atomic::Ordering::SeqCst);
        return stub_4cbf8(text, true);
    }
    false
}

// 0x4cfbc — -[GameKeyboard getText]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard getText]")]
pub fn stub_4cfbc() -> String {
    // IDA 0x4cfbc: `getText` returns the field text (same shape as
    // 0x4118c field reads).
    KEYBOARD_TEXT.lock().clone()
}

// 0x4cfdc — -[GameKeyboard textFieldShouldReturn:]
// type: char __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldShouldReturn:]")]
pub fn stub_4cfdc(text: &str, service_present: bool) -> bool {
    // IDA 0x4cfdc: `textFieldShouldReturn:` finishes editing with the
    // field text when the service exists (0x4cff6-0x4d02e), dispatches
    // the hide block (0x4d060-0x4d072) and returns 1 (0x4d07a). It
    // sequences here.
    if service_present {
        TEXTBOX_FINISHES.lock().push((text.to_owned(), true));
    }
    stub_4d07c();
    true
}

// 0x4d07c — ___38-[GameKeyboard textFieldShouldReturn:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___38-[GameKeyboard textFieldShouldReturn:]_block_invoke")]
pub fn stub_4d07c() {
    // IDA 0x4d07c: the return block hides the keyboard on main. It
    // sequences the hide here.
    stub_4ca64();
}

// 0x4d090 — ___copy_helper_block_82
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_82")]
pub fn stub_4d090() {
    // IDA 0x4d090: `__copy_helper_block_82` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4d09c — ___destroy_helper_block_83
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_83")]
pub fn stub_4d09c() {
    // IDA 0x4d09c: `__destroy_helper_block_83` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4d0a4 — -[GameKeyboard textFieldDidEndEditing:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldDidEndEditing:]")]
pub fn stub_4d0a4(first_responder: bool, text: &str, service_present: bool) {
    // IDA 0x4d0a4: `textFieldDidEndEditing:` finishes editing when
    // first responder with the service (0x4d0ca-0x4d10e) and dispatches
    // the hide block (0x4d140-0x4d152). It sequences here.
    if first_responder {
        if service_present {
            TEXTBOX_FINISHES.lock().push((text.to_owned(), false));
        }
        stub_4d15c();
    }
}

// 0x4d15c — ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")]
pub fn stub_4d15c() {
    // IDA 0x4d15c: the end-editing block hides the keyboard on main
    // (same shape as 0x4d07c). It sequences the hide here.
    stub_4ca64();
}

// 0x4d170 — ___copy_helper_block_87
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_87")]
pub fn stub_4d170() {
    // IDA 0x4d170: `__copy_helper_block_87` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4d17c — ___destroy_helper_block_88
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_88")]
pub fn stub_4d17c() {
    // IDA 0x4d17c: `__destroy_helper_block_88` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4d184 — -[GameKeyboard .cxx_destruct]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_destruct]")]
pub fn stub_4d184() {
    // IDA 0x4d184: `.cxx_destruct` destroys members in place. Drop
    // glue covers it; no explicit body.
}

// 0x4d220 — -[GameKeyboard .cxx_construct]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_construct]")]
pub fn stub_4d220() {
    // IDA 0x4d220: `.cxx_construct` runs member constructors in place.
    // Construction glue; no explicit body.
}

// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, __int64 *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_")]
pub fn stub_4d238() {
    // IDA 0x4d238: `shared_ptr<TextBox>::operator=(&&)`
    // move-assigns the box. `Arc` move glue covers it; no explicit
    // body.
}

// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_")]
pub fn stub_4d2dc() {
    // IDA 0x4d2dc: `shared_ptr<TextBox>::operator=(const&)`
    // copy-assigns the box. `Arc` clone glue covers it; no explicit
    // body.
}

// 0x4d398 — __GLOBAL__I_a_20
#[doc(alias = "global constructor keyed to_a_20")]
#[doc(alias = "__GLOBAL__I_a_20")]
pub fn stub_4d398() {
    // IDA 0x4d398: `__GLOBAL__I_a_20` runs the `a_20`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4d5ac — -[GameView initWithFrame:]
// type: GameView *__cdecl(GameView *self, SEL, CGRect)
#[doc(alias = "-[GameView initWithFrame:]")]
pub fn stub_4d5ac(x: f32, y: f32, width: f32, height: f32) {
    // IDA 0x4d5ac: `GameView::initWithFrame:` supers on the frame
    // (0x4d5c6-0x4d5e2). The frame records in the bounds store.
    *GAMEVIEW_SIZE.lock() = (width as u32, height as u32);
    let _ = (x, y);
}

// 0x4d5e4 — -[GameView layoutSubviews]
// type: void __cdecl(GameView *self, SEL)
#[doc(alias = "-[GameView layoutSubviews]")]
pub fn stub_4d5e4(width: u32, height: u32) {
    // IDA 0x4d5e4: `GameView::layoutSubviews` resizes the Ogre render
    // window to the bounds and refits the camera aspect (0x4d5f4-0x4d6c4).
    // The bounds record here; Ogre calls are engine glue.
    *GAMEVIEW_SIZE.lock() = (width, height);
}

// 0x4d6d4 — __GLOBAL__I_a_21
// type: int()
#[doc(alias = "global constructor keyed to_a_21")]
#[doc(alias = "__GLOBAL__I_a_21")]
pub fn stub_4d6d4() {
    // IDA 0x4d6d4: `__GLOBAL__I_a_21` runs the `a_21`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
#[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
pub fn stub_4d70c() {
    // IDA 0x4d70c: `GameViewController::initWithNibName:bundle:`
    // supers. Super-init glue; no explicit body.
}

// 0x4d8cc — -[GameViewController dealloc]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController dealloc]")]
pub fn stub_4d8cc() {
    // IDA 0x4d8cc: `dealloc` drops the game/input views. Release is
    // drop glue; no explicit body.
}

// 0x4d978 — -[GameViewController viewWillAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewWillAppear:]")]
pub fn stub_4d978() {
    // IDA 0x4d978: `viewWillAppear:` supers. Super glue; no explicit
    // body.
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewDidAppear:]")]
pub fn stub_4d9d4() {
    // IDA 0x4d9d4: `viewDidAppear:` supers. Super glue; no explicit
    // body.
}

// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController viewDidLoad]")]
pub fn stub_4da00() {
    // IDA 0x4da00: `viewDidLoad` supers. Super glue; no explicit body.
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
pub fn stub_4dab8() {
    // IDA 0x4dab8: `didReceiveMemoryWarning` supers. Super glue; no
    // explicit body.
}

// 0x4dae4 — -[GameViewController resizeGameView]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController resizeGameView]")]
pub fn stub_4dae4() {
    // IDA 0x4dae4: `resizeGameView` lays the game view out (0x4dafe).
    // It sequences the layout with the stored bounds here.
    let (w, h) = *GAMEVIEW_SIZE.lock();
    stub_4d5e4(w, h);
}

// 0x4db04 — -[GameViewController shouldAutorotate]
// type: char __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController shouldAutorotate]")]
pub fn stub_4db04() -> bool {
    // IDA 0x4db04: `shouldAutorotate` returns 1 (0x4db06).
    true
}

// 0x4db08 — -[GameViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController supportedInterfaceOrientations]")]
pub fn stub_4db08() -> u32 {
    // IDA 0x4db08: `supportedInterfaceOrientations` returns 24
    // (landscape mask, 0x4db0a).
    24
}

// 0x4db0c — -[GameViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(GameViewController *self, SEL, int)
#[doc(alias = "-[GameViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_4db0c(orientation: i32) -> bool {
    // IDA 0x4db0c: orientation 4 (landscape) reports 1 (0x4db10);
    // otherwise only 3 (portrait) reports 1 (0x4db1a).
    if orientation == 4 {
        true
    } else {
        orientation == 3
    }
}

// 0x4db20 — -[GameViewController getControlView]
// type: id __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController getControlView]")]
pub fn stub_4db20(control_view_present: bool) -> bool {
    // IDA 0x4db20: `getControlView` returns the control-view ivar.
    // Presence reports here.
    control_view_present
}

// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
#[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_4db9c(open_native: bool, in_app_result: i32) -> bool {
    // IDA 0x4db9c: the web view loads unless native Lua browsing is
    // on and the in-app-purchase check consumes the navigation
    // (0x4dbde). Same shape as `check_for_in_app_purchases`.
    !open_native || in_app_result == 0
}

// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
#[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
pub fn stub_4dbe8(datamodel_present: bool) {
    // IDA 0x4dbe8: `signalGuiServiceUrlWindowClosedOnDataModel:`
    // emits the close on the datamodel. The emit records here.
    if datamodel_present {
        URL_CLOSE_SIGNALS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController closeUrlWindow:]")]
pub fn stub_4dc08(web_open: bool, game_present: bool) {
    // IDA 0x4dc08: `closeUrlWindow:` clears an open external view,
    // signals the close on both models (0x4dc44-0x4dd08) and
    // dispatches the dismiss block (0x4dda4-0x4ddc4). It sequences
    // here.
    if web_open {
        URL_WEB_OPEN.store(false, std::sync::atomic::Ordering::SeqCst);
        if game_present {
            URL_CLOSE_SIGNALS.fetch_add(2, std::sync::atomic::Ordering::SeqCst);
        }
        URL_DISMISSES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
pub fn stub_4de58() {
    // IDA 0x4de58: the close dismiss block removes the web view on
    // main (continuation of 0x4dc08). View removal is drop glue; no
    // explicit body.
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
pub fn stub_4df1c() {
    // IDA 0x4df1c: the close block variant 2 (same dismiss shape as
    // 0x4de58). Drop glue; no explicit body.
}

// 0x4dfd8 — ___copy_helper_block__10
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__10")]
pub fn stub_4dfd8() {
    // IDA 0x4dfd8: `__copy_helper_block__10` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4dfe4 — ___destroy_helper_block__10
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__10")]
pub fn stub_4dfe4() {
    // IDA 0x4dfe4: `__destroy_helper_block__10` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
pub fn stub_4dfec() {
    // IDA 0x4dfec: the close block variant 93 (same dismiss shape as
    // 0x4de58). Drop glue; no explicit body.
}

// 0x4e01c — ___copy_helper_block_94
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_94")]
pub fn stub_4e01c() {
    // IDA 0x4e01c: `__copy_helper_block_94` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e028 — ___destroy_helper_block_95
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_95")]
pub fn stub_4e028() {
    // IDA 0x4e028: `__destroy_helper_block_95` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e030 — ___copy_helper_block_100
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_100")]
pub fn stub_4e030() {
    // IDA 0x4e030: `__copy_helper_block_100` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e054 — ___destroy_helper_block_101
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_101")]
pub fn stub_4e054() {
    // IDA 0x4e054: `__destroy_helper_block_101` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController closeUrlWindow]")]
pub fn stub_4e070() {
    // IDA 0x4e070: `closeUrlWindow` forwards to `closeUrlWindow:` with
    // nil (0x4e07e). It sequences here.
    stub_4dc08(URL_WEB_OPEN.load(std::sync::atomic::Ordering::SeqCst), true);
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[GameViewController openUrlWindow:]")]
pub fn stub_4e084(url: &str, web_open: bool) -> bool {
    // IDA 0x4e084: `openUrlWindow:` dispatches the web-view create
    // (0x4e1c0-0x4e1ea) and URL load blocks (0x4e20e-0x4e24a) when no
    // view is open (0x4e0b2). The open records here.
    if web_open {
        return false;
    }
    *URL_LAST.lock() = url.to_owned();
    URL_WEB_OPEN.store(true, std::sync::atomic::Ordering::SeqCst);
    URL_OPENS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
pub fn stub_4e2ac() {
    // IDA 0x4e2ac: the open create block builds the web view on main
    // (continuation of 0x4e084). View construction is drop glue; no
    // explicit body.
}

// 0x4e4c8 — ___copy_helper_block_133
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_133")]
pub fn stub_4e4c8() {
    // IDA 0x4e4c8: `__copy_helper_block_133` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e4d4 — ___destroy_helper_block_134
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_134")]
pub fn stub_4e4d4() {
    // IDA 0x4e4d4: `__destroy_helper_block_134` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
pub fn stub_4e4dc() {
    // IDA 0x4e4dc: the open load block loads the URL on main
    // (continuation of 0x4e084). View loading is drop glue; no
    // explicit body.
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
pub fn stub_4e5fc() {
    // IDA 0x4e5fc: the open load block variant 2 (same load shape as
    // 0x4e4dc). Drop glue; no explicit body.
}

// 0x4e6dc — ___copy_helper_block_148
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_148")]
pub fn stub_4e6dc() {
    // IDA 0x4e6dc: `__copy_helper_block_148` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e6e8 — ___destroy_helper_block_149
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_149")]
pub fn stub_4e6e8() {
    // IDA 0x4e6e8: `__destroy_helper_block_149` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e6f0 — ___copy_helper_block_153
// type: int __fastcall(int, int)
#[doc(alias = "___copy_helper_block_153")]
pub fn stub_4e6f0() {
    // IDA 0x4e6f0: `__copy_helper_block_153` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e714 — ___destroy_helper_block_154
// type: int __fastcall(int)
#[doc(alias = "___destroy_helper_block_154")]
pub fn stub_4e714() {
    // IDA 0x4e714: `__destroy_helper_block_154` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
pub fn stub_4e730() {
    // IDA 0x4e730: `handlePromptLoginSignal` dispatches the login
    // block to main (0x4e766-0x4e778, same shape as 0x4e868). The
    // dispatch records here.
    LOGIN_PROMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
pub fn stub_4e780() {
    // IDA 0x4e780: the login block presents the prompt on main. It
    // records here.
    LOGIN_PROMPT_RUNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4e854 — ___copy_helper_block_174
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_174")]
pub fn stub_4e854() {
    // IDA 0x4e854: `__copy_helper_block_174` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e860 — ___destroy_helper_block_175
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_175")]
pub fn stub_4e860() {
    // IDA 0x4e860: `__destroy_helper_block_175` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
pub fn stub_4e868() {
    // IDA 0x4e868: `handlePromptSignupSignal` dispatches the signup
    // block to main (0x4e89e-0x4e8b0, same shape as 0x4e730). The
    // dispatch records here.
    LOGIN_PROMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
pub fn stub_4e8b8() {
    // IDA 0x4e8b8: the signup block presents the prompt on main (same
    // shape as 0x4e780). It records here.
    LOGIN_PROMPT_RUNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4e98c — ___copy_helper_block_179
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_179")]
pub fn stub_4e98c() {
    // IDA 0x4e98c: `__copy_helper_block_179` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4e998 — ___destroy_helper_block_180
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_180")]
pub fn stub_4e998() {
    // IDA 0x4e998: `__destroy_helper_block_180` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4e9a0 — -[GameViewController handleSignupNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleSignupNotification:]")]
pub fn stub_4e9a0(username: &str, password: &str) {
    // IDA 0x4e9a0: `handleSignupNotification:` logs in with the
    // notification username/password (0x4e9c6-0x4ea2c). The attempt
    // records here.
    LOGIN_ATTEMPTS.lock().push((username.to_owned(), password.to_owned()));
}

// 0x4ea30 — -[GameViewController handleLoginNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleLoginNotification:]")]
pub fn stub_4ea30(success: bool) {
    // IDA 0x4ea30: `handleLoginNotification:` dispatches the result
    // block with the "success" flag (0x4ea48-0x4eabe). It records
    // here.
    LOGIN_RESULTS.lock().push(success);
    stub_4eac8(success);
}

// 0x4eac8 — ___46-[GameViewController handleLoginNotification:]_block_invoke
// type: void __fastcall(id *)
#[doc(alias = "___46-[GameViewController handleLoginNotification:]_block_invoke")]
pub fn stub_4eac8(success: bool) {
    // IDA 0x4eac8: the login-result block finalizes the login UI on
    // main. Completion records here.
    let _ = success;
    LOGIN_COMPLETIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4edcc — ___copy_helper_block_203
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_203")]
pub fn stub_4edcc() {
    // IDA 0x4edcc: `__copy_helper_block_203` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4edf0 — ___destroy_helper_block_204
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_204")]
pub fn stub_4edf0() {
    // IDA 0x4edf0: `__destroy_helper_block_204` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
pub fn stub_4ee0c(next_present: bool) -> bool {
    // IDA 0x4ee0c: `signal<string>::next` advances to the matching slot
    // when present. Presence reports here.
    next_present
}

// 0x4ef74 — __GLOBAL__I_a_22
#[doc(alias = "global constructor keyed to_a_22")]
#[doc(alias = "__GLOBAL__I_a_22")]
pub fn stub_4ef74() {
    // IDA 0x4ef74: `__GLOBAL__I_a_22` runs the `a_22`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4f188 — -[JumpButton initWithFrame:]
// type: JumpButton *__cdecl(JumpButton *self, SEL, CGRect)
#[doc(alias = "-[JumpButton initWithFrame:]")]
pub fn stub_4f188(x: f32, y: f32, width: f32, height: f32) -> JumpButtonInit {
    // IDA 0x4f188: `JumpButton::initWithFrame:` supers, installs the
    // jump images and the touchDown/touchUp targets (0x4f1aa-0x4f2a0).
    // The frame records here; images are view glue.
    JumpButtonInit { x, y, width, height }
}

// 0x4f2b0 — -[JumpButton dealloc]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton dealloc]")]
pub fn stub_4f2b0() {
    // IDA 0x4f2b0: `dealloc` drops the component. Release is drop
    // glue; no explicit body.
}

// 0x4f2fc — -[JumpButton setControlComponentSuperview:]
// type: void __cdecl(JumpButton *self, SEL, id)
#[doc(alias = "-[JumpButton setControlComponentSuperview:]")]
pub fn stub_4f2fc(superview_present: bool) {
    // IDA 0x4f2fc: `setControlComponentSuperview:` stores the
    // superview. Presence records here.
    JUMP_HAS_SUPERVIEW.store(superview_present, std::sync::atomic::Ordering::SeqCst);
}

// 0x4f404 — -[JumpButton jumpEnabledChanged:]
// type: void __cdecl(JumpButton *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[JumpButton jumpEnabledChanged:]")]
pub fn stub_4f404() {
    // IDA 0x4f404: `jumpEnabledChanged:` compiles to an empty body
    // (decompiled 0x4f404). No explicit body.
}

// 0x4f408 — -[JumpButton touchDown]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchDown]")]
pub fn stub_4f408(service_present: bool) {
    // IDA 0x4f408: `touchDown` jumps the local character through the
    // input service (0x4f426-0x4f436). The press records here.
    if service_present {
        JUMP_DOWNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4f43c — -[JumpButton touchUp]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchUp]")]
pub fn stub_4f43c(service_present: bool) {
    // IDA 0x4f43c: `touchUp` stops the jump through the input service
    // (0x4f45a-0x4f46a). The release records here.
    if service_present {
        JUMP_UPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4f7bc — __GLOBAL__I_a_23
#[doc(alias = "global constructor keyed to_a_23")]
#[doc(alias = "__GLOBAL__I_a_23")]
pub fn stub_4f7bc() {
    // IDA 0x4f7bc: `__GLOBAL__I_a_23` runs the `a_23`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x4f9d0 — -[ThumbStickControl init:]
// type: id __cdecl(ThumbStickControl *self, SEL, CGRect)
#[doc(alias = "-[ThumbStickControl init:]")]
pub fn stub_4f9d0(x: f32, y: f32, width: f32, height: f32, new_controls: bool, tablet: bool) -> ThumbStickInit {
    // IDA 0x4f9d0: `ThumbStickControl::init:` sizes the frame, picks
    // style 0 for new camera controls else 1 (0x4fa4e-0x4fa5a) and size
    // 70 for phones else 120 (0x4faf4-0x4fb30), then builds the stick
    // views (0x4fb4e-0x4fcda). The picks record here.
    ThumbStickInit {
        x,
        y,
        width,
        height,
        style: u32::from(!new_controls),
        size: if tablet { 120 } else { 70 },
    }
}

// 0x4fcf4 — ___26-[ThumbStickControl init:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___26-[ThumbStickControl init:]_block_invoke")]
pub fn stub_4fcf4() {
    // IDA 0x4fcf4: the init async block finishes the legacy setup on
    // a queue (0x4fa62-0x4fa98). It records nothing new; no explicit
    // body.
}

// 0x4fd40 — ___copy_helper_block__11
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__11")]
pub fn stub_4fd40() {
    // IDA 0x4fd40: `__copy_helper_block__11` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x4fd4c — ___destroy_helper_block__11
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__11")]
pub fn stub_4fd4c() {
    // IDA 0x4fd4c: `__destroy_helper_block__11` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x4fd54 — -[ThumbStickControl dealloc]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl dealloc]")]
pub fn stub_4fd54() {
    // IDA 0x4fd54: `dealloc` drops the stick views. Release is drop
    // glue; the capture flags reset here.
    THUMBSTICK_TOUCH.store(false, std::sync::atomic::Ordering::SeqCst);
    THUMBSTICK_BEEN.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x4fdb8 — -[ThumbStickControl intToThumbstickStyle:]
// type: int __cdecl(ThumbStickControl *self, SEL, int)
#[doc(alias = "-[ThumbStickControl intToThumbstickStyle:]")]
pub fn stub_4fdb8(value: i32) -> i32 {
    // IDA 0x4fdb8: `intToThumbstickStyle:` clamps out-of-range styles
    // to 0 (0x4fdba-0x4fdc0).
    if value >= 2 { 0 } else { value }
}

// 0x4fdc4 — -[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]
// type: float __cdecl(ThumbStickControl *self, SEL, CGPoint, CGPoint)
#[doc(alias = "-[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]")]
pub fn stub_4fdc4(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // IDA 0x4fdc4: `DistanceBetweenTwoPoints:` returns the Euclidean
    // distance (0x4fdd4-0x4fdf0).
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// 0x4fdf4 — -[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]
// type: CGPoint *__cdecl(CGPoint *__return_ptr __struct_ptr retstr, ThumbStickControl *self, SEL, CGPoint, CGPoint, float)
#[doc(alias = "-[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]")]
pub fn stub_4fdf4(px: f32, py: f32, cx: f32, cy: f32, radians: f32) -> (f32, f32) {
    // IDA 0x4fdf4: `rotatePointAboutLocation:` rotates the point about
    // the center (0x4fe0c-0x4fe66).
    let (dx, dy) = (px - cx, py - cy);
    let (s, c) = radians.sin_cos();
    (cx + c * dx - s * dy, cy + s * dx + c * dy)
}

// 0x4fe88 — -[ThumbStickControl touchesBegan:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesBegan:withEvent:]")]
pub fn stub_4fe88(single: bool, inside: bool) {
    // IDA 0x4fe88: `touchesBegan:` captures a lone touch inside the
    // outer frame, marks touched and centers the knobs (0x4fecc-0x500ee).
    // The capture records here.
    if single && !THUMBSTICK_TOUCH.load(std::sync::atomic::Ordering::SeqCst) && inside {
        THUMBSTICK_TOUCH.store(true, std::sync::atomic::Ordering::SeqCst);
        THUMBSTICK_BEEN.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x50108 — -[ThumbStickControl stationaryThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl stationaryThumbstickTouchMove]")]
pub fn stub_50108(touch_present: bool) {
    // IDA 0x50108: `stationaryThumbstickTouchMove` clamps the inner
    // knob within half size of the touch point (0x50166-0x5031c).
    // The clamp records here.
    if touch_present {
        STICK_STATIONARY.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x50338 — -[ThumbStickControl followThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl followThumbstickTouchMove]")]
pub fn stub_50338(touch_present: bool) {
    // IDA 0x50338: `followThumbstickTouchMove` repositions the knob
    // centers along the touch vector (0x50368-0x506a8). The move
    // records here.
    if touch_present {
        STICK_FOLLOWS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x506cc — -[ThumbStickControl touchesMoved:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesMoved:withEvent:]")]
pub fn stub_506cc(touch_match: bool, style: u32) {
    // IDA 0x506cc: `touchesMoved:` follows for style 1, stays
    // stationary for style 0 (0x507d0-0x507e2), then resets subview
    // alphas (0x507fe-0x50864). The branch records here.
    if touch_match {
        if style == 1 {
            stub_50338(true);
        } else if style == 0 {
            stub_50108(true);
        }
        STICK_MOVES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x508b0 — -[ThumbStickControl cancelMovement]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl cancelMovement]")]
pub fn stub_508b0() {
    // IDA 0x508b0: `cancelMovement` clears the touch and animates the
    // knobs home (0x508f0-0x5094a). The clear records here.
    THUMBSTICK_TOUCH.store(false, std::sync::atomic::Ordering::SeqCst);
    STICK_CANCELS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x50960 — ___35-[ThumbStickControl cancelMovement]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke")]
pub fn stub_50960() {
    // IDA 0x50960: the cancel animation block (continuation of
    // 0x508b0). Animation glue; no explicit body.
}

// 0x509a8 — ___copy_helper_block_77
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_77")]
pub fn stub_509a8() {
    // IDA 0x509a8: `__copy_helper_block_77` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x509b4 — ___destroy_helper_block_78
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_78")]
pub fn stub_509b4() {
    // IDA 0x509b4: `__destroy_helper_block_78` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x50c18 — ___35-[ThumbStickControl cancelMovement]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke_2")]
pub fn stub_50c18() {
    // IDA 0x50c18: the cancel animation block variant 2 (same shape
    // as 0x50960). Animation glue; no explicit body.
}

// 0x50c6c — ___copy_helper_block_81
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_81")]
pub fn stub_50c6c() {
    // IDA 0x50c6c: `__copy_helper_block_81` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x50c78 — ___destroy_helper_block_82
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_82")]
pub fn stub_50c78() {
    // IDA 0x50c78: `__destroy_helper_block_82` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x50c80 — ___35-[ThumbStickControl cancelMovement]_block_invoke84
// type: void __cdecl(id, char)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke84")]
pub fn stub_50c80() {
    // IDA 0x50c80: the cancel animation block variant 84 (same shape
    // as 0x50960). Animation glue; no explicit body.
}

// 0x50c84 — ___copy_helper_block_89
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_89")]
pub fn stub_50c84() {
    // IDA 0x50c84: `__copy_helper_block_89` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x50c90 — ___destroy_helper_block_90
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_90")]
pub fn stub_50c90() {
    // IDA 0x50c90: `__destroy_helper_block_90` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x50c98 — __GLOBAL__I_a_24
#[doc(alias = "global constructor keyed to_a_24")]
#[doc(alias = "__GLOBAL__I_a_24")]
pub fn stub_50c98() {
    // IDA 0x50c98: `__GLOBAL__I_a_24` runs the `a_24`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x50eb0 — -[GameMenu init:]
// type: id __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu init:]")]
pub fn stub_50eb0(menu_button_present: bool) {
    // IDA 0x50eb0: `GameMenu::init:` fixes the 400x256 window, stores
    // the menu button, hides with shown=0 (0x50f1a-0x50f6c) and builds
    // the leave label + accept/decline buttons (0x50fc2-0x512d6).
    // Presence records here; the menu starts hidden.
    let _ = menu_button_present;
    MENU_INITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    MENU_SHOWN.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x512f8 — -[GameMenu dealloc]
// type: void __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu dealloc]")]
pub fn stub_512f8() {
    // IDA 0x512f8: `dealloc` drops the menu views. Release is drop
    // glue; no explicit body.
}

// 0x51370 — -[GameMenu isShown]
// type: char __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu isShown]")]
pub fn stub_51370() -> bool {
    // IDA 0x51370: `isShown` returns the flag (0x5137e).
    MENU_SHOWN.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51380 — -[GameMenu acceptButtonPressed:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu acceptButtonPressed:]")]
pub fn stub_51380() {
    // IDA 0x51380: `acceptButtonPressed:` leaves the game via the
    // launcher (0x5139c-0x513b0). The leave records here.
    MENU_LEAVES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x513b4 — -[GameMenu declineButtonPressed:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu declineButtonPressed:]")]
pub fn stub_513b4() -> ! {
    todo!("0x513b4 -[GameMenu declineButtonPressed:]")
}

// 0x513c4 — -[GameMenu inverseMenuState:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu inverseMenuState:]")]
pub fn stub_513c4() -> ! {
    todo!("0x513c4 -[GameMenu inverseMenuState:]")
}

// 0x513f8 — -[GameMenu showMenu:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu showMenu:]")]
pub fn stub_513f8() -> ! {
    todo!("0x513f8 -[GameMenu showMenu:]")
}

// 0x51570 — ___21-[GameMenu showMenu:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___21-[GameMenu showMenu:]_block_invoke")]
pub fn stub_51570() -> ! {
    todo!("0x51570 ___21-[GameMenu showMenu:]_block_invoke")
}

// 0x515dc — ___copy_helper_block__12
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__12")]
pub fn stub_515dc() -> ! {
    todo!("0x515dc ___copy_helper_block__12")
}

// 0x515e8 — ___destroy_helper_block__12
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__12")]
pub fn stub_515e8() -> ! {
    todo!("0x515e8 ___destroy_helper_block__12")
}

// 0x515f0 — -[GameMenu hideMenu]
// type: void __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu hideMenu]")]
pub fn stub_515f0() -> ! {
    todo!("0x515f0 -[GameMenu hideMenu]")
}

// 0x51738 — ___20-[GameMenu hideMenu]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___20-[GameMenu hideMenu]_block_invoke")]
pub fn stub_51738() -> ! {
    todo!("0x51738 ___20-[GameMenu hideMenu]_block_invoke")
}

// 0x51794 — ___copy_helper_block_96
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_96")]
pub fn stub_51794() -> ! {
    todo!("0x51794 ___copy_helper_block_96")
}

// 0x517a0 — ___destroy_helper_block_97
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_97")]
pub fn stub_517a0() -> ! {
    todo!("0x517a0 ___destroy_helper_block_97")
}

// 0x517a8 — ___20-[GameMenu hideMenu]_block_invoke99
// type: id __fastcall(int)
#[doc(alias = "___20-[GameMenu hideMenu]_block_invoke99")]
pub fn stub_517a8() -> ! {
    todo!("0x517a8 ___20-[GameMenu hideMenu]_block_invoke99")
}

// 0x517d8 — ___copy_helper_block_102
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_102")]
pub fn stub_517d8() -> ! {
    todo!("0x517d8 ___copy_helper_block_102")
}

// 0x517e4 — ___destroy_helper_block_103
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_103")]
pub fn stub_517e4() -> ! {
    todo!("0x517e4 ___destroy_helper_block_103")
}

// 0x517ec — -[GameMenu .cxx_construct]
// type: id __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu .cxx_construct]")]
pub fn stub_517ec() -> ! {
    todo!("0x517ec -[GameMenu .cxx_construct]")
}

// 0x517f0 — __GLOBAL__I_a_25
#[doc(alias = "global constructor keyed to_a_25")]
#[doc(alias = "__GLOBAL__I_a_25")]
pub fn stub_517f0() -> ! {
    todo!("0x517f0 global constructor keyed to_a_25")
}

// 0x51a04 — -[MenuButton init:]
// type: id __cdecl(MenuButton *self, SEL, CGRect)
#[doc(alias = "-[MenuButton init:]")]
pub fn stub_51a04() -> ! {
    todo!("0x51a04 -[MenuButton init:]")
}

// 0x51af8 — -[MenuButton dealloc]
// type: void __cdecl(MenuButton *self, SEL)
#[doc(alias = "-[MenuButton dealloc]")]
pub fn stub_51af8() -> ! {
    todo!("0x51af8 -[MenuButton dealloc]")
}

// 0x51b44 — -[MenuButton doMenuSwitch:]
// type: void __cdecl(MenuButton *self, SEL, id)
#[doc(alias = "-[MenuButton doMenuSwitch:]")]
pub fn stub_51b44() -> ! {
    todo!("0x51b44 -[MenuButton doMenuSwitch:]")
}

// 0x51bb0 — __GLOBAL__I_a_26
#[doc(alias = "global constructor keyed to_a_26")]
#[doc(alias = "__GLOBAL__I_a_26")]
pub fn stub_51bb0() -> ! {
    todo!("0x51bb0 global constructor keyed to_a_26")
}

// 0x51dc4 — +[MainViewController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[MainViewController sharedInstance]")]
pub fn stub_51dc4() -> ! {
    todo!("0x51dc4 +[MainViewController sharedInstance]")
}

// 0x51e20 — ___36+[MainViewController sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36+[MainViewController sharedInstance]_block_invoke")]
pub fn stub_51e20() -> ! {
    todo!("0x51e20 ___36+[MainViewController sharedInstance]_block_invoke")
}
