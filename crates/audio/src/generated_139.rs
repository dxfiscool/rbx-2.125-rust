//! audio generated_139 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x1cae0..0x1ee58 EA-sorted asc filler after 0x1cacc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
pub fn stub_1cae0(controller: &crate::generated_138::AudioHomeViewController, logged_in: bool) {
    // IDA 0x1cae0 (`-[HomeViewController buttonForWebDidTouchUpInside:]`):
    // segue when logged in, else a login alert. Same as the platform
    // 0x1cae0 anchor.
    controller.button_for_web_did_touch_up_inside(logged_in);
}

// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
pub fn stub_1cbac(controller: &crate::generated_138::AudioHomeViewController) {
    // IDA 0x1cbac (`-[HomeViewController btnTouchPlayButtonDisabled:]`):
    // `UnsupportedDevicePlayError` alert. Same as the platform 0x1cbac
    // anchor.
    controller.btn_touch_play_button_disabled();
}

// 0x1cc1c — +[HomeViewController getUrlForButtonTag:recordPageView:]
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:]")]
pub fn stub_1cc1c(
    base_url: &str,
    search_url: &str,
    tablet: bool,
    tag: i32,
    record_page_view: bool,
) -> Option<String> {
    // IDA 0x1cc1c (`+[HomeViewController getUrlForButtonTag:recordPageView:]`):
    // forwards with the empty query. Same as the platform 0x1cc1c
    // anchor.
    crate::generated_138::AudioHomeViewController::url_for_button_tag_no_query(
        base_url,
        search_url,
        tablet,
        tag,
        record_page_view,
    )
}

// 0x1cc54 — +[HomeViewController getUrlForButtonTag:recordPageView:query:]
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_1cc54(
    base_url: &str,
    search_url: &str,
    tablet: bool,
    tag: i32,
    record_page_view: bool,
    query: &str,
) -> Option<String> {
    // IDA 0x1cc54 (`+[HomeViewController getUrlForButtonTag:recordPageView:query:]`):
    // URL + page table. Same as the platform 0x1cc54 anchor.
    crate::generated_138::AudioHomeViewController::url_for_button_tag(
        base_url,
        search_url,
        tablet,
        tag,
        record_page_view,
        query,
    )
}

// 0x1cfe8 — -[HomeViewController prepareForSegue:sender:]
#[doc(alias = "-[HomeViewController prepareForSegue:sender:]")]
pub fn stub_1cfe8(
    controller: &crate::generated_138::AudioHomeViewController,
    dest_is_nav_bar: bool,
    sender: crate::generated_138::AudioHomeSegueSender,
    base_url: &str,
    search_url: &str,
    tablet: bool,
) -> Option<String> {
    // IDA 0x1cfe8 (`-[HomeViewController prepareForSegue:sender:]`):
    // jump-id or sender URL plus the preloaded web view. Same as the
    // platform 0x1cfe8 anchor.
    controller.prepare_for_segue(dest_is_nav_bar, sender, base_url, search_url, tablet)
}

// 0x1d238 — -[HomeViewController viewMustSegueAfterLoad]
#[doc(alias = "-[HomeViewController viewMustSegueAfterLoad]")]
pub fn stub_1d238(controller: &crate::generated_138::AudioHomeViewController) {
    // IDA 0x1d238 (`-[HomeViewController viewMustSegueAfterLoad]`): sets
    // the flag `viewDidAppear:` consumes. Same as the platform 0x1d238
    // anchor.
    controller.view_must_segue_after_load();
}

// 0x1d248 — -[HomeViewController setJumpToPlaceID:]
#[doc(alias = "-[HomeViewController setJumpToPlaceID:]")]
pub fn stub_1d248(controller: &crate::generated_138::AudioHomeViewController, place_id: i32) {
    // IDA 0x1d248 (`-[HomeViewController setJumpToPlaceID:]`): stores the
    // id `prepareForSegue:` consumes. Same as the platform 0x1d248
    // anchor.
    controller.set_jump_to_place_id(place_id);
}

// 0x1d258 — -[HomeViewController blueFrame]
#[doc(alias = "-[HomeViewController blueFrame]")]
pub fn stub_1d258(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d258 (`-[HomeViewController blueFrame]`): returns the
    // retained slot. Same as the platform 0x1d258 anchor.
    controller.blue_frame()
}

// 0x1d268 — -[HomeViewController setBlueFrame:]
#[doc(alias = "-[HomeViewController setBlueFrame:]")]
pub fn stub_1d268(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d268 (`-[HomeViewController setBlueFrame:]`): retained ivar
    // store. Same as the platform 0x1d268 anchor.
    controller.set_blue_frame(value);
}

// 0x1d28c — -[HomeViewController imgAvatar]
#[doc(alias = "-[HomeViewController imgAvatar]")]
pub fn stub_1d28c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d28c (`-[HomeViewController imgAvatar]`). Same as the
    // platform 0x1d28c anchor.
    controller.img_avatar()
}

// 0x1d29c — -[HomeViewController setImgAvatar:]
#[doc(alias = "-[HomeViewController setImgAvatar:]")]
pub fn stub_1d29c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d29c (`-[HomeViewController setImgAvatar:]`). Same as the
    // platform 0x1d29c anchor.
    controller.set_img_avatar(value);
}

// 0x1d2c0 — -[HomeViewController lblPlayerName]
#[doc(alias = "-[HomeViewController lblPlayerName]")]
pub fn stub_1d2c0(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d2c0 (`-[HomeViewController lblPlayerName]`). Same as the
    // platform 0x1d2c0 anchor.
    controller.lbl_player_name()
}

// 0x1d2d0 — -[HomeViewController setLblPlayerName:]
#[doc(alias = "-[HomeViewController setLblPlayerName:]")]
pub fn stub_1d2d0(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d2d0 (`-[HomeViewController setLblPlayerName:]`). Same as
    // the platform 0x1d2d0 anchor.
    controller.set_lbl_player_name(value);
}

// 0x1d2f4 — -[HomeViewController placeId]
#[doc(alias = "-[HomeViewController placeId]")]
pub fn stub_1d2f4(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d2f4 (`-[HomeViewController placeId]`). Same as the
    // platform 0x1d2f4 anchor.
    controller.place_id()
}

// 0x1d304 — -[HomeViewController setPlaceId:]
#[doc(alias = "-[HomeViewController setPlaceId:]")]
pub fn stub_1d304(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d304 (`-[HomeViewController setPlaceId:]`). Same as the
    // platform 0x1d304 anchor.
    controller.set_place_id(value);
}

// 0x1d328 — -[HomeViewController portId]
#[doc(alias = "-[HomeViewController portId]")]
pub fn stub_1d328(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d328 (`-[HomeViewController portId]`). Same as the platform
    // 0x1d328 anchor.
    controller.port_id()
}

// 0x1d338 — -[HomeViewController setPortId:]
#[doc(alias = "-[HomeViewController setPortId:]")]
pub fn stub_1d338(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d338 (`-[HomeViewController setPortId:]`). Same as the
    // platform 0x1d338 anchor.
    controller.set_port_id(value);
}

// 0x1d35c — -[HomeViewController ipId]
#[doc(alias = "-[HomeViewController ipId]")]
pub fn stub_1d35c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d35c (`-[HomeViewController ipId]`). Same as the platform
    // 0x1d35c anchor.
    controller.ip_id()
}

// 0x1d36c — -[HomeViewController setIpId:]
#[doc(alias = "-[HomeViewController setIpId:]")]
pub fn stub_1d36c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d36c (`-[HomeViewController setIpId:]`). Same as the
    // platform 0x1d36c anchor.
    controller.set_ip_id(value);
}

// 0x1d390 — -[HomeViewController btnPlaceLauncher]
#[doc(alias = "-[HomeViewController btnPlaceLauncher]")]
pub fn stub_1d390(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d390 (`-[HomeViewController btnPlaceLauncher]`). Same as
    // the platform 0x1d390 anchor.
    controller.btn_place_launcher()
}

// 0x1d3a0 — -[HomeViewController setBtnPlaceLauncher:]
#[doc(alias = "-[HomeViewController setBtnPlaceLauncher:]")]
pub fn stub_1d3a0(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d3a0 (`-[HomeViewController setBtnPlaceLauncher:]`). Same
    // as the platform 0x1d3a0 anchor.
    controller.set_btn_place_launcher(value);
}

// 0x1d3c4 — -[HomeViewController btnGames]
#[doc(alias = "-[HomeViewController btnGames]")]
pub fn stub_1d3c4(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d3c4 (`-[HomeViewController btnGames]`). Same as the
    // platform 0x1d3c4 anchor.
    controller.btn_games()
}

// 0x1d3d4 — -[HomeViewController setBtnGames:]
#[doc(alias = "-[HomeViewController setBtnGames:]")]
pub fn stub_1d3d4(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d3d4 (`-[HomeViewController setBtnGames:]`). Same as the
    // platform 0x1d3d4 anchor.
    controller.set_btn_games(value);
}

// 0x1d3f8 — -[HomeViewController btnDebugSettings]
#[doc(alias = "-[HomeViewController btnDebugSettings]")]
pub fn stub_1d3f8(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d3f8 (`-[HomeViewController btnDebugSettings]`). Same as
    // the platform 0x1d3f8 anchor.
    controller.btn_debug_settings()
}

// 0x1d408 — -[HomeViewController setBtnDebugSettings:]
#[doc(alias = "-[HomeViewController setBtnDebugSettings:]")]
pub fn stub_1d408(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d408 (`-[HomeViewController setBtnDebugSettings:]`). Same
    // as the platform 0x1d408 anchor.
    controller.set_btn_debug_settings(value);
}

// 0x1d42c — -[HomeViewController lblRobux]
#[doc(alias = "-[HomeViewController lblRobux]")]
pub fn stub_1d42c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d42c (`-[HomeViewController lblRobux]`). Same as the
    // platform 0x1d42c anchor.
    controller.lbl_robux()
}

// 0x1d43c — -[HomeViewController setLblRobux:]
#[doc(alias = "-[HomeViewController setLblRobux:]")]
pub fn stub_1d43c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d43c (`-[HomeViewController setLblRobux:]`). Same as the
    // platform 0x1d43c anchor.
    controller.set_lbl_robux(value);
}

// 0x1d460 — -[HomeViewController lblTix]
#[doc(alias = "-[HomeViewController lblTix]")]
pub fn stub_1d460(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d460 (`-[HomeViewController lblTix]`). Same as the platform
    // 0x1d460 anchor.
    controller.lbl_tix()
}

// 0x1d470 — -[HomeViewController setLblTix:]
#[doc(alias = "-[HomeViewController setLblTix:]")]
pub fn stub_1d470(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d470 (`-[HomeViewController setLblTix:]`). Same as the
    // platform 0x1d470 anchor.
    controller.set_lbl_tix(value);
}

// 0x1d494 — -[HomeViewController btnMessages]
#[doc(alias = "-[HomeViewController btnMessages]")]
pub fn stub_1d494() -> ! {
    todo!("0x1d494 -[HomeViewController btnMessages]")
}

// 0x1d4a4 — -[HomeViewController setBtnMessages:]
#[doc(alias = "-[HomeViewController setBtnMessages:]")]
pub fn stub_1d4a4() -> ! {
    todo!("0x1d4a4 -[HomeViewController setBtnMessages:]")
}

// 0x1d4c8 — -[HomeViewController gameLabel]
#[doc(alias = "-[HomeViewController gameLabel]")]
pub fn stub_1d4c8() -> ! {
    todo!("0x1d4c8 -[HomeViewController gameLabel]")
}

// 0x1d4d8 — -[HomeViewController setGameLabel:]
#[doc(alias = "-[HomeViewController setGameLabel:]")]
pub fn stub_1d4d8() -> ! {
    todo!("0x1d4d8 -[HomeViewController setGameLabel:]")
}

// 0x1d4fc — -[HomeViewController catalogLabel]
#[doc(alias = "-[HomeViewController catalogLabel]")]
pub fn stub_1d4fc() -> ! {
    todo!("0x1d4fc -[HomeViewController catalogLabel]")
}

// 0x1d50c — -[HomeViewController setCatalogLabel:]
#[doc(alias = "-[HomeViewController setCatalogLabel:]")]
pub fn stub_1d50c() -> ! {
    todo!("0x1d50c -[HomeViewController setCatalogLabel:]")
}

// 0x1d530 — -[HomeViewController inventoryLabel]
#[doc(alias = "-[HomeViewController inventoryLabel]")]
pub fn stub_1d530() -> ! {
    todo!("0x1d530 -[HomeViewController inventoryLabel]")
}

// 0x1d540 — -[HomeViewController setInventoryLabel:]
#[doc(alias = "-[HomeViewController setInventoryLabel:]")]
pub fn stub_1d540() -> ! {
    todo!("0x1d540 -[HomeViewController setInventoryLabel:]")
}

// 0x1d564 — -[HomeViewController buildersClubLabel]
#[doc(alias = "-[HomeViewController buildersClubLabel]")]
pub fn stub_1d564() -> ! {
    todo!("0x1d564 -[HomeViewController buildersClubLabel]")
}

// 0x1d574 — -[HomeViewController setBuildersClubLabel:]
#[doc(alias = "-[HomeViewController setBuildersClubLabel:]")]
pub fn stub_1d574() -> ! {
    todo!("0x1d574 -[HomeViewController setBuildersClubLabel:]")
}

// 0x1d598 — -[HomeViewController profileLabel]
#[doc(alias = "-[HomeViewController profileLabel]")]
pub fn stub_1d598() -> ! {
    todo!("0x1d598 -[HomeViewController profileLabel]")
}

// 0x1d5a8 — -[HomeViewController setProfileLabel:]
#[doc(alias = "-[HomeViewController setProfileLabel:]")]
pub fn stub_1d5a8() -> ! {
    todo!("0x1d5a8 -[HomeViewController setProfileLabel:]")
}

// 0x1d5cc — -[HomeViewController messagesLabel]
#[doc(alias = "-[HomeViewController messagesLabel]")]
pub fn stub_1d5cc() -> ! {
    todo!("0x1d5cc -[HomeViewController messagesLabel]")
}

// 0x1d5dc — -[HomeViewController setMessagesLabel:]
#[doc(alias = "-[HomeViewController setMessagesLabel:]")]
pub fn stub_1d5dc() -> ! {
    todo!("0x1d5dc -[HomeViewController setMessagesLabel:]")
}

// 0x1d600 — -[HomeViewController btnPlayDisabled]
#[doc(alias = "-[HomeViewController btnPlayDisabled]")]
pub fn stub_1d600() -> ! {
    todo!("0x1d600 -[HomeViewController btnPlayDisabled]")
}

// 0x1d610 — -[HomeViewController setBtnPlayDisabled:]
#[doc(alias = "-[HomeViewController setBtnPlayDisabled:]")]
pub fn stub_1d610() -> ! {
    todo!("0x1d610 -[HomeViewController setBtnPlayDisabled:]")
}

// 0x1d634 — -[HomeViewController communityLabel]
#[doc(alias = "-[HomeViewController communityLabel]")]
pub fn stub_1d634() -> ! {
    todo!("0x1d634 -[HomeViewController communityLabel]")
}

// 0x1d644 — -[HomeViewController setCommunityLabel:]
#[doc(alias = "-[HomeViewController setCommunityLabel:]")]
pub fn stub_1d644() -> ! {
    todo!("0x1d644 -[HomeViewController setCommunityLabel:]")
}

// 0x1d668 — -[HomeViewController communityButton]
#[doc(alias = "-[HomeViewController communityButton]")]
pub fn stub_1d668() -> ! {
    todo!("0x1d668 -[HomeViewController communityButton]")
}

// 0x1d678 — -[HomeViewController setCommunityButton:]
#[doc(alias = "-[HomeViewController setCommunityButton:]")]
pub fn stub_1d678() -> ! {
    todo!("0x1d678 -[HomeViewController setCommunityButton:]")
}

// 0x1d69c — -[HomeViewController buttonView]
#[doc(alias = "-[HomeViewController buttonView]")]
pub fn stub_1d69c() -> ! {
    todo!("0x1d69c -[HomeViewController buttonView]")
}

// 0x1d6ac — -[HomeViewController setButtonView:]
#[doc(alias = "-[HomeViewController setButtonView:]")]
pub fn stub_1d6ac() -> ! {
    todo!("0x1d6ac -[HomeViewController setButtonView:]")
}

// 0x1d6d0 — -[HomeViewController searchTextField]
#[doc(alias = "-[HomeViewController searchTextField]")]
pub fn stub_1d6d0() -> ! {
    todo!("0x1d6d0 -[HomeViewController searchTextField]")
}

// 0x1d6e0 — -[HomeViewController setSearchTextField:]
#[doc(alias = "-[HomeViewController setSearchTextField:]")]
pub fn stub_1d6e0() -> ! {
    todo!("0x1d6e0 -[HomeViewController setSearchTextField:]")
}

// 0x1d704 — -[HomeViewController loggedInView]
#[doc(alias = "-[HomeViewController loggedInView]")]
pub fn stub_1d704() -> ! {
    todo!("0x1d704 -[HomeViewController loggedInView]")
}

// 0x1d714 — -[HomeViewController setLoggedInView:]
#[doc(alias = "-[HomeViewController setLoggedInView:]")]
pub fn stub_1d714() -> ! {
    todo!("0x1d714 -[HomeViewController setLoggedInView:]")
}

// 0x1d738 — -[HomeViewController notLoggedInView]
#[doc(alias = "-[HomeViewController notLoggedInView]")]
pub fn stub_1d738() -> ! {
    todo!("0x1d738 -[HomeViewController notLoggedInView]")
}

// 0x1d748 — -[HomeViewController setNotLoggedInView:]
#[doc(alias = "-[HomeViewController setNotLoggedInView:]")]
pub fn stub_1d748() -> ! {
    todo!("0x1d748 -[HomeViewController setNotLoggedInView:]")
}

// 0x1d76c — -[HomeViewController signUpButtonLabel]
#[doc(alias = "-[HomeViewController signUpButtonLabel]")]
pub fn stub_1d76c() -> ! {
    todo!("0x1d76c -[HomeViewController signUpButtonLabel]")
}

// 0x1d77c — -[HomeViewController setSignUpButtonLabel:]
#[doc(alias = "-[HomeViewController setSignUpButtonLabel:]")]
pub fn stub_1d77c() -> ! {
    todo!("0x1d77c -[HomeViewController setSignUpButtonLabel:]")
}

// 0x1d7a0 — -[HomeViewController loginButtonLabel]
#[doc(alias = "-[HomeViewController loginButtonLabel]")]
pub fn stub_1d7a0() -> ! {
    todo!("0x1d7a0 -[HomeViewController loginButtonLabel]")
}

// 0x1d7b0 — -[HomeViewController setLoginButtonLabel:]
#[doc(alias = "-[HomeViewController setLoginButtonLabel:]")]
pub fn stub_1d7b0() -> ! {
    todo!("0x1d7b0 -[HomeViewController setLoginButtonLabel:]")
}

// 0x1d7d4 — -[HomeViewController welcomeToRobloxTextView]
#[doc(alias = "-[HomeViewController welcomeToRobloxTextView]")]
pub fn stub_1d7d4() -> ! {
    todo!("0x1d7d4 -[HomeViewController welcomeToRobloxTextView]")
}

// 0x1d7e4 — -[HomeViewController setWelcomeToRobloxTextView:]
#[doc(alias = "-[HomeViewController setWelcomeToRobloxTextView:]")]
pub fn stub_1d7e4() -> ! {
    todo!("0x1d7e4 -[HomeViewController setWelcomeToRobloxTextView:]")
}

// 0x1d808 — -[HomeViewController youAreCurrentlyLoggedInAsTextView]
#[doc(alias = "-[HomeViewController youAreCurrentlyLoggedInAsTextView]")]
pub fn stub_1d808() -> ! {
    todo!("0x1d808 -[HomeViewController youAreCurrentlyLoggedInAsTextView]")
}

// 0x1d818 — -[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]
#[doc(alias = "-[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]")]
pub fn stub_1d818() -> ! {
    todo!("0x1d818 -[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]")
}

// 0x1d83c — -[HomeViewController versionLabel]
#[doc(alias = "-[HomeViewController versionLabel]")]
pub fn stub_1d83c() -> ! {
    todo!("0x1d83c -[HomeViewController versionLabel]")
}

// 0x1d84c — -[HomeViewController setVersionLabel:]
#[doc(alias = "-[HomeViewController setVersionLabel:]")]
pub fn stub_1d84c() -> ! {
    todo!("0x1d84c -[HomeViewController setVersionLabel:]")
}

// 0x1d870 — __GLOBAL__I_a_4
#[doc(alias = "__GLOBAL__I_a_4")]
pub fn stub_1d870() -> ! {
    todo!("0x1d870 global constructor keyed to_a_4")
}

// 0x1da08 — -[NSString stringWithPercentEscape]
#[doc(alias = "-[NSString stringWithPercentEscape]")]
pub fn stub_1da08() -> ! {
    todo!("0x1da08 -[NSString stringWithPercentEscape]")
}

// 0x1da5c — +[LoginViewController sharedInstance]
#[doc(alias = "+[LoginViewController sharedInstance]")]
pub fn stub_1da5c() -> ! {
    todo!("0x1da5c +[LoginViewController sharedInstance]")
}

// 0x1da6c — -[LoginViewController initWithCoder:]
#[doc(alias = "-[LoginViewController initWithCoder:]")]
pub fn stub_1da6c() -> ! {
    todo!("0x1da6c -[LoginViewController initWithCoder:]")
}

// 0x1dbd4 — -[LoginViewController dealloc]
#[doc(alias = "-[LoginViewController dealloc]")]
pub fn stub_1dbd4() -> ! {
    todo!("0x1dbd4 -[LoginViewController dealloc]")
}

// 0x1dd84 — -[LoginViewController populateEnvironmentPicker]
#[doc(alias = "-[LoginViewController populateEnvironmentPicker]")]
pub fn stub_1dd84() -> ! {
    todo!("0x1dd84 -[LoginViewController populateEnvironmentPicker]")
}

// 0x1e0d8 — -[LoginViewController pickerView:didSelectRow:inComponent:]
#[doc(alias = "-[LoginViewController pickerView:didSelectRow:inComponent:]")]
pub fn stub_1e0d8() -> ! {
    todo!("0x1e0d8 -[LoginViewController pickerView:didSelectRow:inComponent:]")
}

// 0x1e13c — ___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke
#[doc(alias = "___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke")]
pub fn stub_1e13c() -> ! {
    todo!("0x1e13c ___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke")
}

// 0x1e170 — -[LoginViewController numberOfComponentsInPickerView:]
#[doc(alias = "-[LoginViewController numberOfComponentsInPickerView:]")]
pub fn stub_1e170() -> ! {
    todo!("0x1e170 -[LoginViewController numberOfComponentsInPickerView:]")
}

// 0x1e174 — -[LoginViewController pickerView:numberOfRowsInComponent:]
#[doc(alias = "-[LoginViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_1e174() -> ! {
    todo!("0x1e174 -[LoginViewController pickerView:numberOfRowsInComponent:]")
}

// 0x1e194 — -[LoginViewController pickerView:titleForRow:forComponent:]
#[doc(alias = "-[LoginViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_1e194() -> ! {
    todo!("0x1e194 -[LoginViewController pickerView:titleForRow:forComponent:]")
}

// 0x1e1b4 — -[LoginViewController viewWillAppear:]
#[doc(alias = "-[LoginViewController viewWillAppear:]")]
pub fn stub_1e1b4() -> ! {
    todo!("0x1e1b4 -[LoginViewController viewWillAppear:]")
}

// 0x1e2c4 — ___38-[LoginViewController viewWillAppear:]_block_invoke
#[doc(alias = "___38-[LoginViewController viewWillAppear:]_block_invoke")]
pub fn stub_1e2c4() -> ! {
    todo!("0x1e2c4 ___38-[LoginViewController viewWillAppear:]_block_invoke")
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
pub fn stub_1e2d8() -> ! {
    todo!("0x1e2d8 ___copy_helper_block__2")
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
pub fn stub_1e2e4() -> ! {
    todo!("0x1e2e4 ___destroy_helper_block__2")
}

// 0x1e2ec — -[LoginViewController viewDidLoad]
#[doc(alias = "-[LoginViewController viewDidLoad]")]
pub fn stub_1e2ec() -> ! {
    todo!("0x1e2ec -[LoginViewController viewDidLoad]")
}

// 0x1e898 — ___34-[LoginViewController viewDidLoad]_block_invoke
#[doc(alias = "___34-[LoginViewController viewDidLoad]_block_invoke")]
pub fn stub_1e898() -> ! {
    todo!("0x1e898 ___34-[LoginViewController viewDidLoad]_block_invoke")
}

// 0x1e8cc — -[LoginViewController viewDidUnload]
#[doc(alias = "-[LoginViewController viewDidUnload]")]
pub fn stub_1e8cc() -> ! {
    todo!("0x1e8cc -[LoginViewController viewDidUnload]")
}

// 0x1e9d0 — -[LoginViewController handleSignupNotification:]
#[doc(alias = "-[LoginViewController handleSignupNotification:]")]
pub fn stub_1e9d0() -> ! {
    todo!("0x1e9d0 -[LoginViewController handleSignupNotification:]")
}

// 0x1eaa0 — ___48-[LoginViewController handleSignupNotification:]_block_invoke
#[doc(alias = "___48-[LoginViewController handleSignupNotification:]_block_invoke")]
pub fn stub_1eaa0() -> ! {
    todo!("0x1eaa0 ___48-[LoginViewController handleSignupNotification:]_block_invoke")
}

// 0x1eb08 — ___copy_helper_block_226
#[doc(alias = "___copy_helper_block_226")]
pub fn stub_1eb08() -> ! {
    todo!("0x1eb08 ___copy_helper_block_226")
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
pub fn stub_1eb38() -> ! {
    todo!("0x1eb38 ___destroy_helper_block_227")
}

// 0x1eb5c — -[LoginViewController gotLoginFailedNotification:]
#[doc(alias = "-[LoginViewController gotLoginFailedNotification:]")]
pub fn stub_1eb5c() -> ! {
    todo!("0x1eb5c -[LoginViewController gotLoginFailedNotification:]")
}

// 0x1ebdc — ___50-[LoginViewController gotLoginFailedNotification:]_block_invoke
#[doc(alias = "___50-[LoginViewController gotLoginFailedNotification:]_block_invoke")]
pub fn stub_1ebdc() -> ! {
    todo!("0x1ebdc ___50-[LoginViewController gotLoginFailedNotification:]_block_invoke")
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
pub fn stub_1ec44() -> ! {
    todo!("0x1ec44 ___copy_helper_block_234")
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
pub fn stub_1ec68() -> ! {
    todo!("0x1ec68 ___destroy_helper_block_235")
}

// 0x1ec84 — -[LoginViewController gotLoginSuccessfulNotification:]
#[doc(alias = "-[LoginViewController gotLoginSuccessfulNotification:]")]
pub fn stub_1ec84() -> ! {
    todo!("0x1ec84 -[LoginViewController gotLoginSuccessfulNotification:]")
}

// 0x1ed04 — ___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke
#[doc(alias = "___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke")]
pub fn stub_1ed04() -> ! {
    todo!("0x1ed04 ___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke")
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
pub fn stub_1ed30() -> ! {
    todo!("0x1ed30 ___copy_helper_block_242")
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
pub fn stub_1ed3c() -> ! {
    todo!("0x1ed3c ___destroy_helper_block_243")
}

// 0x1ed44 — -[LoginViewController showLoggingIn]
#[doc(alias = "-[LoginViewController showLoggingIn]")]
pub fn stub_1ed44() -> ! {
    todo!("0x1ed44 -[LoginViewController showLoggingIn]")
}

// 0x1edbc — ___36-[LoginViewController showLoggingIn]_block_invoke
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke")]
pub fn stub_1edbc() -> ! {
    todo!("0x1edbc ___36-[LoginViewController showLoggingIn]_block_invoke")
}

// 0x1ee58 — ___36-[LoginViewController showLoggingIn]_block_invoke_2
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke_2")]
pub fn stub_1ee58() -> ! {
    todo!("0x1ee58 ___36-[LoginViewController showLoggingIn]_block_invoke_2")
}
