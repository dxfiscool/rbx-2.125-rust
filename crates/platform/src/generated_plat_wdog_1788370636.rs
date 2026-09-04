//! platform — generated_plat_wdog_1788370636 — 110 stubs EA-sorted asc gap-filler distinct | Source ida/export.json | Filter iOS/ObjC/Controller/View (RobloxView,GameViewController,iOS) 1881 total, 1881 remaining before batch | range 0x19028..0x1d5dc | rbx_core::SharedPtr not boost
//! Batch: 110 stubs | range 0x19028..0x1d5dc | EA-sorted asc gap-filler distinct not yet in any crate | rbx_core::SharedPtr not boost | // 0xADDR — mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

static DELEGATE: std::sync::LazyLock<crate::view_controllers::AppDelegate> =
    std::sync::LazyLock::new(crate::view_controllers::AppDelegate::default);
static HOME: std::sync::LazyLock<crate::roblox_view::HomeViewControllerState> =
    std::sync::LazyLock::new(crate::roblox_view::HomeViewControllerState::new);

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028(button_index: i32, now_secs: f64) {
    // delegate of crate::view_controllers::Appirater (IDA 0x19028)
    crate::view_controllers::Appirater::shared_instance()
        .alert_view_clicked_button(button_index, now_secs);
}

// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> crate::view_controllers::ObjCId {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19228)
    &*DELEGATE as *const crate::view_controllers::AppDelegate as crate::view_controllers::ObjCId
}

// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19254)
    crate::view_controllers::AppDelegate::init().dealloc();
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4() -> bool {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x192b4)
    DELEGATE.application_did_finish_launching()
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec() {
    // delegate of crate::view_controllers (IDA 0x194ec)
    crate::view_controllers::did_finish_launching_flurry_block();
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() {
    // delegate of crate::view_controllers (IDA 0x19514)
    crate::view_controllers::did_finish_launching_appirater_block();
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x195a0)
    DELEGATE.application_will_resign_active();
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x196e4)
    DELEGATE.application_did_enter_background();
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19a30)
    DELEGATE.application_did_receive_memory_warning();
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19b60)
    DELEGATE.application_will_enter_foreground();
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19cdc)
    DELEGATE.application_did_become_active();
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34() {
    // delegate of crate::view_controllers (IDA 0x19f34)
    crate::view_controllers::did_become_active_fetch_settings_block();
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x19f7c)
    DELEGATE.application_will_terminate();
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
#[doc(alias = "__Z18_topMostControllerP16UIViewController")]
pub fn stub_1a098(
    graph: &crate::view_controllers::ViewControllerGraph,
    root: crate::view_controllers::ObjCId,
) -> Option<crate::view_controllers::ObjCId> {
    // delegate of crate::view_controllers (IDA 0x1a098)
    crate::view_controllers::top_most_controller(graph, root)
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174(
    url_absolute_string: &str,
    url_host: &str,
    url_path: &str,
    source_application: &str,
    annotation: &str,
) -> bool {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a174)
    DELEGATE.application_open_url(
        url_absolute_string,
        url_host,
        url_path,
        source_application,
        annotation,
    )
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234(
    place_id: i32,
    top_controller_class: &str,
) -> crate::view_controllers::LaunchAction {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a234)
    DELEGATE.try_launch_place(place_id, top_controller_class)
}

// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494() -> u32 {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a494)
    DELEGATE.bg_task()
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8(task: u32) {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a4a8)
    DELEGATE.set_bg_task(task);
}

// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0() -> Option<crate::view_controllers::ObjCId> {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a4c0)
    DELEGATE.window()
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0(window: Option<crate::view_controllers::ObjCId>) {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a4d0)
    DELEGATE.set_window(window);
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a4f4)
    DELEGATE.cxx_destruct();
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc() {
    // delegate of crate::view_controllers::AppDelegate (IDA 0x1a5bc)
    DELEGATE.cxx_construct();
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970(
    super_ok: bool,
    idiom_pad: bool,
    screen_bounds: Option<(f64, f64, f64, f64)>,
) -> Option<crate::generated_176::DebugSettingsViewController> {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1a970)
    crate::generated_176::DebugSettingsViewController::init_with_coder(
        super_ok,
        idiom_pad,
        screen_bounds,
    )
}
// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20(controller: crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1ab20)
    controller.dealloc();
}
// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c(controller: &crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1ab6c)
    controller.reload_old_data();
}
// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70(controller: &crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1ab70)
    controller.view_did_load();
}
// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0(
    controller: &crate::generated_176::DebugSettingsViewController,
) -> &'static str {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1abb0)
    controller.set_display_ui()
}
// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80(
    controller: &crate::generated_176::DebugSettingsViewController,
    selected_row: i32,
) -> &'static str {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1ac80)
    controller.display_picker_done_clicked(selected_row)
}
// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78(controller: &crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1ad78)
    controller.display_picker_animation_frame();
}
// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0(controller: &crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1aed0)
    controller.display_touch_up();
}
// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_1afa0() -> ! {
    todo!("0x1afa0 ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")
}

// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
pub fn stub_1b170() -> ! {
    todo!("0x1b170 -[DebugSettingsViewController didReceiveMemoryWarning]")
}

// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_1b19c() -> ! {
    todo!("0x1b19c -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
pub fn stub_1b224() -> ! {
    todo!("0x1b224 -[DebugSettingsViewController viewWillAppear:]")
}

// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
pub fn stub_1b2a8() -> ! {
    todo!("0x1b2a8 -[DebugSettingsViewController doneTouchUp:]")
}

// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
pub fn stub_1b2bc() -> ! {
    todo!("0x1b2bc -[DebugSettingsViewController numberOfComponentsInPickerView:]")
}

// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_1b2c0() -> ! {
    todo!("0x1b2c0 -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")
}

// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_1b2e0() -> ! {
    todo!("0x1b2e0 -[DebugSettingsViewController pickerView:titleForRow:forComponent:]")
}

// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_1b300() -> ! {
    todo!("0x1b300 -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")
}

// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
pub fn stub_1b304(controller: &crate::generated_176::DebugSettingsViewController) {
    // delegate of crate::generated_176::DebugSettingsViewController (IDA 0x1b304)
    controller.cxx_construct();
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_1b3d0() -> bool {
    // delegate of crate::roblox_view (IDA 0x1b3d0)
    HOME.init_with_coder()
}

// 0x1b4b0 — -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_1b4b0() {
    // delegate of crate::roblox_view (IDA 0x1b4b0)
    HOME.dealloc();
}

// 0x1b75c — -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_1b75c(bundle_version: &str) {
    // delegate of crate::roblox_view (IDA 0x1b75c)
    HOME.view_did_load(bundle_version);
}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_1bae4(search_url_len: usize) -> bool {
    // delegate of crate::roblox_view (IDA 0x1bae4)
    HOME.view_did_load_search_block(search_url_len)
}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_1bb64() {
    // delegate of crate::roblox_view (IDA 0x1bb64)
    HOME.view_did_load_search_apply();
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_1bbb0() {
    // delegate of crate::roblox_view (IDA 0x1bbb0)
    HOME.keyboard_did_show();
}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_1bbd0() {
    // delegate of crate::roblox_view (IDA 0x1bbd0)
    HOME.keyboard_did_hide();
}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_1bbf0() {
    // delegate of crate::roblox_view (IDA 0x1bbf0)
    HOME.dismiss_keyboard();
}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_1bc10() {
    // delegate of crate::roblox_view (IDA 0x1bc10)
    HOME.localize_and_style_labels();
}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_1bf0c(refresh: bool) {
    // delegate of crate::roblox_view (IDA 0x1bf0c)
    HOME.update_user_info_display(refresh);
}

// 0x1c134 — -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_1c134() {
    // delegate of crate::roblox_view (IDA 0x1c134)
    HOME.view_did_unload();
}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_1c2bc(username: &str, password: &str) {
    // delegate of crate::roblox_view (IDA 0x1c2bc)
    HOME.handle_signup_notification(username, password);
}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_1c37c() {
    // delegate of crate::roblox_view (IDA 0x1c37c)
    HOME.logout_touch_up();
}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_1c4b0(button_index: i32) -> bool {
    // delegate of crate::roblox_view (IDA 0x1c4b0)
    HOME.alert_view_did_dismiss(button_index)
}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_1c5c8() {
    // delegate of crate::roblox_view (IDA 0x1c5c8)
    HOME.alert_animation_step();
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_1c608(presented: bool, animating: bool, foreground_x: f32, background_x: f32) {
    // delegate of crate::roblox_view (IDA 0x1c608)
    HOME.alert_completion(presented, animating, foreground_x, background_x);
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_1c748(animated: bool) {
    // delegate of crate::roblox_view (IDA 0x1c748)
    HOME.view_will_appear(animated);
}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_1c788(logged_in: bool) {
    // delegate of crate::roblox_view (IDA 0x1c788)
    HOME.show_correct_logged_in_state(logged_in);
}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_1c860() {
    // delegate of crate::roblox_view (IDA 0x1c860)
    HOME.logged_in_state_refresh_block();
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_1c888(animated: bool) {
    // delegate of crate::roblox_view (IDA 0x1c888)
    HOME.view_did_appear(animated);
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_1c8e8() {
    // delegate of crate::roblox_view (IDA 0x1c8e8)
    HOME.handle_start_game_failure();
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_1c958() {
    // delegate of crate::roblox_view (IDA 0x1c958)
    HOME.handle_start_game_success();
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_1c95c(place_text: &str, port_text: &str, ip_text: &str) {
    // delegate of crate::roblox_view (IDA 0x1c95c)
    HOME.place_id_clicked(place_text, port_text, ip_text);
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_1ca9c() {
    // delegate of crate::roblox_view (IDA 0x1ca9c)
    HOME.search_editing_did_end();
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_1caa0() {
    // delegate of crate::roblox_view (IDA 0x1caa0)
    HOME.search_did_end_on_exit();
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_1cac8() -> ! {
    todo!("0x1cac8 -[HomeViewController signUpButtonDidTouchUpInside:]")
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_1cacc() -> ! {
    todo!("0x1cacc -[HomeViewController logInButtonDidTouchUpInside:]")
}

// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
pub fn stub_1cae0() -> ! {
    todo!("0x1cae0 -[HomeViewController buttonForWebDidTouchUpInside:]")
}

// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
pub fn stub_1cbac() -> ! {
    todo!("0x1cbac -[HomeViewController btnTouchPlayButtonDisabled:]")
}

// 0x1cc1c — +[HomeViewController getUrlForButtonTag:recordPageView:]
// type: id __cdecl(id, SEL, int, char)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:]")]
pub fn stub_1cc1c() -> ! {
    todo!("0x1cc1c +[HomeViewController getUrlForButtonTag:recordPageView:]")
}

// 0x1cc54 — +[HomeViewController getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_1cc54() -> ! {
    todo!("0x1cc54 +[HomeViewController getUrlForButtonTag:recordPageView:query:]")
}

// 0x1cfe8 — -[HomeViewController prepareForSegue:sender:]
// type: void __cdecl(HomeViewController *self, SEL, id, id)
#[doc(alias = "-[HomeViewController prepareForSegue:sender:]")]
pub fn stub_1cfe8() -> ! {
    todo!("0x1cfe8 -[HomeViewController prepareForSegue:sender:]")
}

// 0x1d238 — -[HomeViewController viewMustSegueAfterLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewMustSegueAfterLoad]")]
pub fn stub_1d238() -> ! {
    todo!("0x1d238 -[HomeViewController viewMustSegueAfterLoad]")
}

// 0x1d248 — -[HomeViewController setJumpToPlaceID:]
// type: void __cdecl(HomeViewController *self, SEL, int)
#[doc(alias = "-[HomeViewController setJumpToPlaceID:]")]
pub fn stub_1d248() -> ! {
    todo!("0x1d248 -[HomeViewController setJumpToPlaceID:]")
}

// 0x1d258 — -[HomeViewController blueFrame]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController blueFrame]")]
pub fn stub_1d258() -> ! {
    todo!("0x1d258 -[HomeViewController blueFrame]")
}

// 0x1d268 — -[HomeViewController setBlueFrame:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBlueFrame:]")]
pub fn stub_1d268() -> ! {
    todo!("0x1d268 -[HomeViewController setBlueFrame:]")
}

// 0x1d28c — -[HomeViewController imgAvatar]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController imgAvatar]")]
pub fn stub_1d28c() -> ! {
    todo!("0x1d28c -[HomeViewController imgAvatar]")
}

// 0x1d29c — -[HomeViewController setImgAvatar:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setImgAvatar:]")]
pub fn stub_1d29c() -> ! {
    todo!("0x1d29c -[HomeViewController setImgAvatar:]")
}

// 0x1d2c0 — -[HomeViewController lblPlayerName]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblPlayerName]")]
pub fn stub_1d2c0() -> ! {
    todo!("0x1d2c0 -[HomeViewController lblPlayerName]")
}

// 0x1d2d0 — -[HomeViewController setLblPlayerName:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblPlayerName:]")]
pub fn stub_1d2d0() -> ! {
    todo!("0x1d2d0 -[HomeViewController setLblPlayerName:]")
}

// 0x1d2f4 — -[HomeViewController placeId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController placeId]")]
pub fn stub_1d2f4() -> ! {
    todo!("0x1d2f4 -[HomeViewController placeId]")
}

// 0x1d304 — -[HomeViewController setPlaceId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPlaceId:]")]
pub fn stub_1d304() -> ! {
    todo!("0x1d304 -[HomeViewController setPlaceId:]")
}

// 0x1d328 — -[HomeViewController portId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController portId]")]
pub fn stub_1d328() -> ! {
    todo!("0x1d328 -[HomeViewController portId]")
}

// 0x1d338 — -[HomeViewController setPortId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPortId:]")]
pub fn stub_1d338() -> ! {
    todo!("0x1d338 -[HomeViewController setPortId:]")
}

// 0x1d35c — -[HomeViewController ipId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController ipId]")]
pub fn stub_1d35c() -> ! {
    todo!("0x1d35c -[HomeViewController ipId]")
}

// 0x1d36c — -[HomeViewController setIpId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setIpId:]")]
pub fn stub_1d36c() -> ! {
    todo!("0x1d36c -[HomeViewController setIpId:]")
}

// 0x1d390 — -[HomeViewController btnPlaceLauncher]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnPlaceLauncher]")]
pub fn stub_1d390() -> ! {
    todo!("0x1d390 -[HomeViewController btnPlaceLauncher]")
}

// 0x1d3a0 — -[HomeViewController setBtnPlaceLauncher:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnPlaceLauncher:]")]
pub fn stub_1d3a0() -> ! {
    todo!("0x1d3a0 -[HomeViewController setBtnPlaceLauncher:]")
}

// 0x1d3c4 — -[HomeViewController btnGames]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnGames]")]
pub fn stub_1d3c4() -> ! {
    todo!("0x1d3c4 -[HomeViewController btnGames]")
}

// 0x1d3d4 — -[HomeViewController setBtnGames:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnGames:]")]
pub fn stub_1d3d4() -> ! {
    todo!("0x1d3d4 -[HomeViewController setBtnGames:]")
}

// 0x1d3f8 — -[HomeViewController btnDebugSettings]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnDebugSettings]")]
pub fn stub_1d3f8() -> ! {
    todo!("0x1d3f8 -[HomeViewController btnDebugSettings]")
}

// 0x1d408 — -[HomeViewController setBtnDebugSettings:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnDebugSettings:]")]
pub fn stub_1d408() -> ! {
    todo!("0x1d408 -[HomeViewController setBtnDebugSettings:]")
}

// 0x1d42c — -[HomeViewController lblRobux]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblRobux]")]
pub fn stub_1d42c() -> ! {
    todo!("0x1d42c -[HomeViewController lblRobux]")
}

// 0x1d43c — -[HomeViewController setLblRobux:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblRobux:]")]
pub fn stub_1d43c() -> ! {
    todo!("0x1d43c -[HomeViewController setLblRobux:]")
}

// 0x1d460 — -[HomeViewController lblTix]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblTix]")]
pub fn stub_1d460() -> ! {
    todo!("0x1d460 -[HomeViewController lblTix]")
}

// 0x1d470 — -[HomeViewController setLblTix:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblTix:]")]
pub fn stub_1d470() -> ! {
    todo!("0x1d470 -[HomeViewController setLblTix:]")
}

// 0x1d494 — -[HomeViewController btnMessages]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnMessages]")]
pub fn stub_1d494() -> ! {
    todo!("0x1d494 -[HomeViewController btnMessages]")
}

// 0x1d4a4 — -[HomeViewController setBtnMessages:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnMessages:]")]
pub fn stub_1d4a4() -> ! {
    todo!("0x1d4a4 -[HomeViewController setBtnMessages:]")
}

// 0x1d4c8 — -[HomeViewController gameLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController gameLabel]")]
pub fn stub_1d4c8() -> ! {
    todo!("0x1d4c8 -[HomeViewController gameLabel]")
}

// 0x1d4d8 — -[HomeViewController setGameLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setGameLabel:]")]
pub fn stub_1d4d8() -> ! {
    todo!("0x1d4d8 -[HomeViewController setGameLabel:]")
}

// 0x1d4fc — -[HomeViewController catalogLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController catalogLabel]")]
pub fn stub_1d4fc() -> ! {
    todo!("0x1d4fc -[HomeViewController catalogLabel]")
}

// 0x1d50c — -[HomeViewController setCatalogLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCatalogLabel:]")]
pub fn stub_1d50c() -> ! {
    todo!("0x1d50c -[HomeViewController setCatalogLabel:]")
}

// 0x1d530 — -[HomeViewController inventoryLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController inventoryLabel]")]
pub fn stub_1d530() -> ! {
    todo!("0x1d530 -[HomeViewController inventoryLabel]")
}

// 0x1d540 — -[HomeViewController setInventoryLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setInventoryLabel:]")]
pub fn stub_1d540() -> ! {
    todo!("0x1d540 -[HomeViewController setInventoryLabel:]")
}

// 0x1d564 — -[HomeViewController buildersClubLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController buildersClubLabel]")]
pub fn stub_1d564() -> ! {
    todo!("0x1d564 -[HomeViewController buildersClubLabel]")
}

// 0x1d574 — -[HomeViewController setBuildersClubLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBuildersClubLabel:]")]
pub fn stub_1d574() -> ! {
    todo!("0x1d574 -[HomeViewController setBuildersClubLabel:]")
}

// 0x1d598 — -[HomeViewController profileLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController profileLabel]")]
pub fn stub_1d598() -> ! {
    todo!("0x1d598 -[HomeViewController profileLabel]")
}

// 0x1d5a8 — -[HomeViewController setProfileLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setProfileLabel:]")]
pub fn stub_1d5a8() -> ! {
    todo!("0x1d5a8 -[HomeViewController setProfileLabel:]")
}

// 0x1d5cc — -[HomeViewController messagesLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController messagesLabel]")]
pub fn stub_1d5cc() -> ! {
    todo!("0x1d5cc -[HomeViewController messagesLabel]")
}

// 0x1d5dc — -[HomeViewController setMessagesLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setMessagesLabel:]")]
pub fn stub_1d5dc() -> ! {
    todo!("0x1d5dc -[HomeViewController setMessagesLabel:]")
}
