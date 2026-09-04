//! audio generated_138 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x18ca0..0x1cacc EA-sorted asc filler after 0x18c98, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_137::AudioAppirater;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x18ca0 — +[Appirater appLaunched]
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_18ca0(current_version: &str, now_secs: f64) {
    // IDA 0x18ca0 (`+[Appirater appLaunched]`): forwards `YES` to
    // `appLaunched:` (`stub_18cc0`). Same as the platform 0x18ca0 anchor.
    stub_18cc0(true, current_version, now_secs);
}

// 0x18cc0 — +[Appirater appLaunched:]
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_18cc0(first_launch: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18cc0 (`+[Appirater appLaunched:]`): captures the flag into the
    // stack block and `dispatch_async`es it to a global queue. The queue
    // hop collapses; the block is `stub_18d10`. Same as the platform 0x18cc0
    // anchor.
    stub_18d10(first_launch, current_version, now_secs);
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_18d10(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18d10 (`__25+[Appirater appLaunched:]_block_invoke`):
    // `sharedInstance` then `incrementAndRate:` with the captured flag.
    // Same as the platform 0x18d10 anchor.
    AudioAppirater::shared_note_app_launched();
    crate::generated_137::stub_18b18(can_rate, current_version, now_secs);
}

// 0x18d4c — -[Appirater hideRatingAlert]
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_18d4c() -> bool {
    // IDA 0x18d4c (`-[Appirater hideRatingAlert]`): dismisses `ratingAlert`
    // when visible; the `_debug` `NSLog` has no host sink. Reports whether
    // an alert was dismissed. Same as the platform 0x18d4c anchor.
    AudioAppirater::shared_hide_rating_alert()
}

// 0x18dbc — +[Appirater appWillResignActive]
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_18dbc() {
    // IDA 0x18dbc (`+[Appirater appWillResignActive]`): `_debug` `NSLog`
    // (no host sink), then `hideRatingAlert` on `sharedInstance`. Same as
    // the platform 0x18dbc anchor.
    AudioAppirater::shared_hide_rating_alert();
}

// 0x18e0c — +[Appirater appEnteredForeground:]
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_18e0c(entered: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e0c (`+[Appirater appEnteredForeground:]`): same shape as
    // 0x18cc0 — capture the flag, `dispatch_async` to a global queue; the
    // block is `stub_18e5c`. Same as the platform 0x18e0c anchor.
    stub_18e5c(entered, current_version, now_secs);
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_18e5c(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e5c (`__34+[Appirater appEnteredForeground:]_block_invoke`):
    // `sharedInstance` then `incrementAndRate:`. Same as the platform
    // 0x18e5c anchor.
    AudioAppirater::shared_note_entered_foreground();
    crate::generated_137::stub_18b18(can_rate, current_version, now_secs);
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_18e98(significant: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e98 (`+[Appirater userDidSignificantEvent:]`): same dispatch
    // shape over `incrementSignificantEventAndRate:`; the block is
    // `stub_18ee8`. Same as the platform 0x18e98 anchor.
    stub_18ee8(significant, current_version, now_secs);
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_18ee8(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18ee8 (`__37+[Appirater userDidSignificantEvent:]_block_invoke`):
    // `sharedInstance` then `incrementSignificantEventAndRate:`. Same as the
    // platform 0x18ee8 anchor.
    crate::generated_137::stub_18bdc(can_rate, current_version, now_secs);
}

// 0x18f24 — +[Appirater rateApp]
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_18f24() -> String {
    // IDA 0x18f24 (`+[Appirater rateApp]`): review URL from the template
    // with `APP_ID` replaced, flag `kAppiraterRatedCurrentVersion`,
    // `openURL:`. Returns the opened URL. Same as the platform 0x18f24
    // anchor.
    AudioAppirater::shared_rate_app()
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028(button_index: i32, now_secs: f64) {
    // IDA 0x19028 (`-[Appirater alertView:clickedButtonAtIndex:]`): the
    // three-way button switch with delegate callbacks; see
    // `AudioAppirater::alert_view_clicked_button`. Same as the platform
    // 0x19028 anchor.
    AudioAppirater::shared_alert_button(button_index, now_secs);
}

// 0x191d4 — -[Appirater ratingAlert]
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_191d4() -> u64 {
    // IDA 0x191d4 (`-[Appirater ratingAlert]`): returns the `ratingAlert`
    // ivar. Same as the platform 0x191d4 anchor; `0` is `nil`.
    AudioAppirater::shared_rating_alert()
}

// 0x191e4 — -[Appirater setRatingAlert:]
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_191e4(alert: u64) {
    // IDA 0x191e4 (`-[Appirater setRatingAlert:]`): retained-property store
    // via `objc_setProperty`. Same as the platform 0x191e4 anchor.
    AudioAppirater::shared_set_rating_alert(alert);
}

// 0x19208 — -[Appirater delegate]
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_19208() -> u64 {
    // IDA 0x19208 (`-[Appirater delegate]`): returns the `_delegate` ivar.
    // Same as the platform 0x19208 anchor; `0` is `nil`.
    AudioAppirater::shared_delegate()
}

// 0x19218 — -[Appirater setDelegate:]
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_19218(delegate: u64) {
    // IDA 0x19218 (`-[Appirater setDelegate:]`): plain ivar store. Same as
    // the platform 0x19218 anchor.
    AudioAppirater::shared_set_delegate(delegate);
}

// 0x19228 — -[AppDelegate init]
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> ! {
    todo!("0x19228 -[AppDelegate init]")
}

// 0x19254 — -[AppDelegate dealloc]
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254() -> ! {
    todo!("0x19254 -[AppDelegate dealloc]")
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4() -> ! {
    todo!("0x192b4 -[AppDelegate application:didFinishLaunchingWithOptions:]")
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec() -> ! {
    todo!("0x194ec ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() -> ! {
    todo!("0x19514 ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0() -> ! {
    todo!("0x195a0 -[AppDelegate applicationWillResignActive:]")
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4() -> ! {
    todo!("0x196e4 -[AppDelegate applicationDidEnterBackground:]")
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30() -> ! {
    todo!("0x19a30 -[AppDelegate applicationDidReceiveMemoryWarning:]")
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60() -> ! {
    todo!("0x19b60 -[AppDelegate applicationWillEnterForeground:]")
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc() -> ! {
    todo!("0x19cdc -[AppDelegate applicationDidBecomeActive:]")
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34() -> ! {
    todo!("0x19f34 ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c() -> ! {
    todo!("0x19f7c -[AppDelegate applicationWillTerminate:]")
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098() -> ! {
    todo!("0x1a098 _topMostController(UIViewController *)")
}

// 0x1a124 — __Z17topMostControllerv
#[doc(alias = "topMostController(void)")]
pub fn stub_1a124() -> ! {
    todo!("0x1a124 topMostController(void)")
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174() -> ! {
    todo!("0x1a174 -[AppDelegate application:openURL:sourceApplication:annotation:]")
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234() -> ! {
    todo!("0x1a234 -[AppDelegate TryLaunchPlace:]")
}

// 0x1a494 — -[AppDelegate bgTask]
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494() -> ! {
    todo!("0x1a494 -[AppDelegate bgTask]")
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8() -> ! {
    todo!("0x1a4a8 -[AppDelegate setBgTask:]")
}

// 0x1a4c0 — -[AppDelegate window]
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0() -> ! {
    todo!("0x1a4c0 -[AppDelegate window]")
}

// 0x1a4d0 — -[AppDelegate setWindow:]
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0() -> ! {
    todo!("0x1a4d0 -[AppDelegate setWindow:]")
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4() -> ! {
    todo!("0x1a4f4 -[AppDelegate .cxx_destruct]")
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc() -> ! {
    todo!("0x1a5bc -[AppDelegate .cxx_construct]")
}

// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "__GLOBAL__I_a_1")]
pub fn stub_1a5d0() -> ! {
    todo!("0x1a5d0 global constructor keyed to_a_1")
}

// 0x1a768 — _main
#[doc(alias = "_main")]
pub fn stub_1a768() -> ! {
    todo!("0x1a768 _main")
}

// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "__GLOBAL__I_a_2")]
pub fn stub_1a7d4() -> ! {
    todo!("0x1a7d4 global constructor keyed to_a_2")
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970() -> ! {
    todo!("0x1a970 -[DebugSettingsViewController initWithCoder:]")
}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20() -> ! {
    todo!("0x1ab20 -[DebugSettingsViewController dealloc]")
}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c() -> ! {
    todo!("0x1ab6c -[DebugSettingsViewController reloadOldData]")
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70() -> ! {
    todo!("0x1ab70 -[DebugSettingsViewController viewDidLoad]")
}

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0() -> ! {
    todo!("0x1abb0 -[DebugSettingsViewController setDisplayUI]")
}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80() -> ! {
    todo!("0x1ac80 -[DebugSettingsViewController displayPickerDoneClicked:]")
}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78() -> ! {
    todo!("0x1ad78 ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")
}

// 0x1ae78 — ___copy_helper_block__0
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_1ae78() -> ! {
    todo!("0x1ae78 ___copy_helper_block__0")
}

// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_1aea8() -> ! {
    todo!("0x1aea8 ___destroy_helper_block__0")
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0() -> ! {
    todo!("0x1aed0 -[DebugSettingsViewController displayTouchUp:]")
}

// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_1afa0() -> ! {
    todo!("0x1afa0 ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")
}

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_1b11c() -> ! {
    todo!("0x1b11c ___copy_helper_block_66")
}

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_1b14c() -> ! {
    todo!("0x1b14c ___destroy_helper_block_67")
}

// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
pub fn stub_1b170() -> ! {
    todo!("0x1b170 -[DebugSettingsViewController didReceiveMemoryWarning]")
}

// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_1b19c() -> ! {
    todo!("0x1b19c -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
pub fn stub_1b224() -> ! {
    todo!("0x1b224 -[DebugSettingsViewController viewWillAppear:]")
}

// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
pub fn stub_1b2a8() -> ! {
    todo!("0x1b2a8 -[DebugSettingsViewController doneTouchUp:]")
}

// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
pub fn stub_1b2bc() -> ! {
    todo!("0x1b2bc -[DebugSettingsViewController numberOfComponentsInPickerView:]")
}

// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_1b2c0() -> ! {
    todo!("0x1b2c0 -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")
}

// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_1b2e0() -> ! {
    todo!("0x1b2e0 -[DebugSettingsViewController pickerView:titleForRow:forComponent:]")
}

// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_1b300() -> ! {
    todo!("0x1b300 -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")
}

// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
pub fn stub_1b304() -> ! {
    todo!("0x1b304 -[DebugSettingsViewController .cxx_construct]")
}

// 0x1b308 — __GLOBAL__I_a_3
#[doc(alias = "__GLOBAL__I_a_3")]
pub fn stub_1b308() -> ! {
    todo!("0x1b308 global constructor keyed to_a_3")
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_1b3d0() -> ! {
    todo!("0x1b3d0 -[HomeViewController initWithCoder:]")
}

// 0x1b4b0 — -[HomeViewController dealloc]
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_1b4b0() -> ! {
    todo!("0x1b4b0 -[HomeViewController dealloc]")
}

// 0x1b75c — -[HomeViewController viewDidLoad]
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_1b75c() -> ! {
    todo!("0x1b75c -[HomeViewController viewDidLoad]")
}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_1bae4() -> ! {
    todo!("0x1bae4 ___33-[HomeViewController viewDidLoad]_block_invoke")
}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_1bb64() -> ! {
    todo!("0x1bb64 ___33-[HomeViewController viewDidLoad]_block_invoke_2")
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_1bb88() -> ! {
    todo!("0x1bb88 ___copy_helper_block__1")
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_1bb94() -> ! {
    todo!("0x1bb94 ___destroy_helper_block__1")
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_1bb9c() -> ! {
    todo!("0x1bb9c ___copy_helper_block_80")
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_1bba8() -> ! {
    todo!("0x1bba8 ___destroy_helper_block_81")
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_1bbb0() -> ! {
    todo!("0x1bbb0 -[HomeViewController keyboardDidShow:]")
}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_1bbd0() -> ! {
    todo!("0x1bbd0 -[HomeViewController keyboardDidHide:]")
}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_1bbf0() -> ! {
    todo!("0x1bbf0 -[HomeViewController dismissKeyboard]")
}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_1bc10() -> ! {
    todo!("0x1bc10 -[HomeViewController localizeAndStyleLabels]")
}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_1bf0c() -> ! {
    todo!("0x1bf0c -[HomeViewController updateUserInfoDisplay:]")
}

// 0x1c134 — -[HomeViewController viewDidUnload]
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_1c134() -> ! {
    todo!("0x1c134 -[HomeViewController viewDidUnload]")
}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_1c2bc() -> ! {
    todo!("0x1c2bc -[HomeViewController handleSignupNotification:]")
}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_1c37c() -> ! {
    todo!("0x1c37c -[HomeViewController logoutTouchUp:]")
}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_1c4b0() -> ! {
    todo!("0x1c4b0 -[HomeViewController alertView:didDismissWithButtonIndex:]")
}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_1c5c8() -> ! {
    todo!("0x1c5c8 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_1c5f4() -> ! {
    todo!("0x1c5f4 ___copy_helper_block_224")
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_1c600() -> ! {
    todo!("0x1c600 ___destroy_helper_block_225")
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_1c608() -> ! {
    todo!("0x1c608 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_1c734() -> ! {
    todo!("0x1c734 ___copy_helper_block_246")
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_1c740() -> ! {
    todo!("0x1c740 ___destroy_helper_block_247")
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_1c748() -> ! {
    todo!("0x1c748 -[HomeViewController viewWillAppear:]")
}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_1c788() -> ! {
    todo!("0x1c788 -[HomeViewController showCorrectLoggedInState]")
}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_1c860() -> ! {
    todo!("0x1c860 ___46-[HomeViewController showCorrectLoggedInState]_block_invoke")
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_1c874() -> ! {
    todo!("0x1c874 ___copy_helper_block_261")
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_1c880() -> ! {
    todo!("0x1c880 ___destroy_helper_block_262")
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_1c888() -> ! {
    todo!("0x1c888 -[HomeViewController viewDidAppear:]")
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_1c8e8() -> ! {
    todo!("0x1c8e8 -[HomeViewController handleStartGameFailure]")
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_1c958() -> ! {
    todo!("0x1c958 -[HomeViewController handleStartGameSuccess]")
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_1c95c() -> ! {
    todo!("0x1c95c -[HomeViewController placeIdClicked:]")
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_1ca9c() -> ! {
    todo!("0x1ca9c -[HomeViewController searchEditingDidEnd:]")
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_1caa0() -> ! {
    todo!("0x1caa0 -[HomeViewController searchDidEndOnExit:]")
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_1cac8() -> ! {
    todo!("0x1cac8 -[HomeViewController signUpButtonDidTouchUpInside:]")
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_1cacc() -> ! {
    todo!("0x1cacc -[HomeViewController logInButtonDidTouchUpInside:]")
}
