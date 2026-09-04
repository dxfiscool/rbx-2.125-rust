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
pub fn stub_1d494(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d494 (`-[HomeViewController btnMessages]`): returns the
    // retained slot. Same as the platform 0x1d494 anchor.
    controller.btn_messages()
}

// 0x1d4a4 — -[HomeViewController setBtnMessages:]
#[doc(alias = "-[HomeViewController setBtnMessages:]")]
pub fn stub_1d4a4(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d4a4 (`-[HomeViewController setBtnMessages:]`). Same as the
    // platform 0x1d4a4 anchor.
    controller.set_btn_messages(value);
}

// 0x1d4c8 — -[HomeViewController gameLabel]
#[doc(alias = "-[HomeViewController gameLabel]")]
pub fn stub_1d4c8(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d4c8 (`-[HomeViewController gameLabel]`). Same as the
    // platform 0x1d4c8 anchor.
    controller.game_label()
}

// 0x1d4d8 — -[HomeViewController setGameLabel:]
#[doc(alias = "-[HomeViewController setGameLabel:]")]
pub fn stub_1d4d8(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d4d8 (`-[HomeViewController setGameLabel:]`). Same as the
    // platform 0x1d4d8 anchor.
    controller.set_game_label(value);
}

// 0x1d4fc — -[HomeViewController catalogLabel]
#[doc(alias = "-[HomeViewController catalogLabel]")]
pub fn stub_1d4fc(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d4fc (`-[HomeViewController catalogLabel]`). Same as the
    // platform 0x1d4fc anchor.
    controller.catalog_label()
}

// 0x1d50c — -[HomeViewController setCatalogLabel:]
#[doc(alias = "-[HomeViewController setCatalogLabel:]")]
pub fn stub_1d50c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d50c (`-[HomeViewController setCatalogLabel:]`). Same as the
    // platform 0x1d50c anchor.
    controller.set_catalog_label(value);
}

// 0x1d530 — -[HomeViewController inventoryLabel]
#[doc(alias = "-[HomeViewController inventoryLabel]")]
pub fn stub_1d530(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d530 (`-[HomeViewController inventoryLabel]`). Same as the
    // platform 0x1d530 anchor.
    controller.inventory_label()
}

// 0x1d540 — -[HomeViewController setInventoryLabel:]
#[doc(alias = "-[HomeViewController setInventoryLabel:]")]
pub fn stub_1d540(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d540 (`-[HomeViewController setInventoryLabel:]`). Same as
    // the platform 0x1d540 anchor.
    controller.set_inventory_label(value);
}

// 0x1d564 — -[HomeViewController buildersClubLabel]
#[doc(alias = "-[HomeViewController buildersClubLabel]")]
pub fn stub_1d564(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d564 (`-[HomeViewController buildersClubLabel]`). Same as
    // the platform 0x1d564 anchor.
    controller.builders_club_label()
}

// 0x1d574 — -[HomeViewController setBuildersClubLabel:]
#[doc(alias = "-[HomeViewController setBuildersClubLabel:]")]
pub fn stub_1d574(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d574 (`-[HomeViewController setBuildersClubLabel:]`). Same
    // as the platform 0x1d574 anchor.
    controller.set_builders_club_label(value);
}

// 0x1d598 — -[HomeViewController profileLabel]
#[doc(alias = "-[HomeViewController profileLabel]")]
pub fn stub_1d598(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d598 (`-[HomeViewController profileLabel]`). Same as the
    // platform 0x1d598 anchor.
    controller.profile_label()
}

// 0x1d5a8 — -[HomeViewController setProfileLabel:]
#[doc(alias = "-[HomeViewController setProfileLabel:]")]
pub fn stub_1d5a8(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d5a8 (`-[HomeViewController setProfileLabel:]`). Same as
    // the platform 0x1d5a8 anchor.
    controller.set_profile_label(value);
}

// 0x1d5cc — -[HomeViewController messagesLabel]
#[doc(alias = "-[HomeViewController messagesLabel]")]
pub fn stub_1d5cc(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d5cc (`-[HomeViewController messagesLabel]`). Same as the
    // platform 0x1d5cc anchor.
    controller.messages_label()
}

// 0x1d5dc — -[HomeViewController setMessagesLabel:]
#[doc(alias = "-[HomeViewController setMessagesLabel:]")]
pub fn stub_1d5dc(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d5dc (`-[HomeViewController setMessagesLabel:]`). Same as
    // the platform 0x1d5dc anchor.
    controller.set_messages_label(value);
}

// 0x1d600 — -[HomeViewController btnPlayDisabled]
#[doc(alias = "-[HomeViewController btnPlayDisabled]")]
pub fn stub_1d600(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d600 (`-[HomeViewController btnPlayDisabled]`). Same as the
    // platform 0x1d600 anchor.
    controller.btn_play_disabled()
}

// 0x1d610 — -[HomeViewController setBtnPlayDisabled:]
#[doc(alias = "-[HomeViewController setBtnPlayDisabled:]")]
pub fn stub_1d610(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d610 (`-[HomeViewController setBtnPlayDisabled:]`). Same as
    // the platform 0x1d610 anchor.
    controller.set_btn_play_disabled(value);
}

// 0x1d634 — -[HomeViewController communityLabel]
#[doc(alias = "-[HomeViewController communityLabel]")]
pub fn stub_1d634(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d634 (`-[HomeViewController communityLabel]`). Same as the
    // platform 0x1d634 anchor.
    controller.community_label()
}

// 0x1d644 — -[HomeViewController setCommunityLabel:]
#[doc(alias = "-[HomeViewController setCommunityLabel:]")]
pub fn stub_1d644(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d644 (`-[HomeViewController setCommunityLabel:]`). Same as
    // the platform 0x1d644 anchor.
    controller.set_community_label(value);
}

// 0x1d668 — -[HomeViewController communityButton]
#[doc(alias = "-[HomeViewController communityButton]")]
pub fn stub_1d668(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d668 (`-[HomeViewController communityButton]`). Same as the
    // platform 0x1d668 anchor.
    controller.community_button()
}

// 0x1d678 — -[HomeViewController setCommunityButton:]
#[doc(alias = "-[HomeViewController setCommunityButton:]")]
pub fn stub_1d678(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d678 (`-[HomeViewController setCommunityButton:]`). Same as
    // the platform 0x1d678 anchor.
    controller.set_community_button(value);
}

// 0x1d69c — -[HomeViewController buttonView]
#[doc(alias = "-[HomeViewController buttonView]")]
pub fn stub_1d69c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d69c (`-[HomeViewController buttonView]`). Same as the
    // platform 0x1d69c anchor.
    controller.button_view()
}

// 0x1d6ac — -[HomeViewController setButtonView:]
#[doc(alias = "-[HomeViewController setButtonView:]")]
pub fn stub_1d6ac(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d6ac (`-[HomeViewController setButtonView:]`). Same as the
    // platform 0x1d6ac anchor.
    controller.set_button_view(value);
}

// 0x1d6d0 — -[HomeViewController searchTextField]
#[doc(alias = "-[HomeViewController searchTextField]")]
pub fn stub_1d6d0(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d6d0 (`-[HomeViewController searchTextField]`). Same as the
    // platform 0x1d6d0 anchor.
    controller.search_text_field()
}

// 0x1d6e0 — -[HomeViewController setSearchTextField:]
#[doc(alias = "-[HomeViewController setSearchTextField:]")]
pub fn stub_1d6e0(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d6e0 (`-[HomeViewController setSearchTextField:]`). Same as
    // the platform 0x1d6e0 anchor.
    controller.set_search_text_field(value);
}

// 0x1d704 — -[HomeViewController loggedInView]
#[doc(alias = "-[HomeViewController loggedInView]")]
pub fn stub_1d704(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d704 (`-[HomeViewController loggedInView]`). Same as the
    // platform 0x1d704 anchor.
    controller.logged_in_view()
}

// 0x1d714 — -[HomeViewController setLoggedInView:]
#[doc(alias = "-[HomeViewController setLoggedInView:]")]
pub fn stub_1d714(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d714 (`-[HomeViewController setLoggedInView:]`). Same as the
    // platform 0x1d714 anchor.
    controller.set_logged_in_view(value);
}

// 0x1d738 — -[HomeViewController notLoggedInView]
#[doc(alias = "-[HomeViewController notLoggedInView]")]
pub fn stub_1d738(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d738 (`-[HomeViewController notLoggedInView]`). Same as the
    // platform 0x1d738 anchor.
    controller.not_logged_in_view()
}

// 0x1d748 — -[HomeViewController setNotLoggedInView:]
#[doc(alias = "-[HomeViewController setNotLoggedInView:]")]
pub fn stub_1d748(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d748 (`-[HomeViewController setNotLoggedInView:]`). Same as
    // the platform 0x1d748 anchor.
    controller.set_not_logged_in_view(value);
}

// 0x1d76c — -[HomeViewController signUpButtonLabel]
#[doc(alias = "-[HomeViewController signUpButtonLabel]")]
pub fn stub_1d76c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d76c (`-[HomeViewController signUpButtonLabel]`). Same as
    // the platform 0x1d76c anchor.
    controller.sign_up_button_label()
}

// 0x1d77c — -[HomeViewController setSignUpButtonLabel:]
#[doc(alias = "-[HomeViewController setSignUpButtonLabel:]")]
pub fn stub_1d77c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d77c (`-[HomeViewController setSignUpButtonLabel:]`). Same
    // as the platform 0x1d77c anchor.
    controller.set_sign_up_button_label(value);
}

// 0x1d7a0 — -[HomeViewController loginButtonLabel]
#[doc(alias = "-[HomeViewController loginButtonLabel]")]
pub fn stub_1d7a0(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d7a0 (`-[HomeViewController loginButtonLabel]`). Same as
    // the platform 0x1d7a0 anchor.
    controller.login_button_label()
}

// 0x1d7b0 — -[HomeViewController setLoginButtonLabel:]
#[doc(alias = "-[HomeViewController setLoginButtonLabel:]")]
pub fn stub_1d7b0(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d7b0 (`-[HomeViewController setLoginButtonLabel:]`). Same
    // as the platform 0x1d7b0 anchor.
    controller.set_login_button_label(value);
}

// 0x1d7d4 — -[HomeViewController welcomeToRobloxTextView]
#[doc(alias = "-[HomeViewController welcomeToRobloxTextView]")]
pub fn stub_1d7d4(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d7d4 (`-[HomeViewController welcomeToRobloxTextView]`).
    // Same as the platform 0x1d7d4 anchor.
    controller.welcome_to_roblox_text_view()
}

// 0x1d7e4 — -[HomeViewController setWelcomeToRobloxTextView:]
#[doc(alias = "-[HomeViewController setWelcomeToRobloxTextView:]")]
pub fn stub_1d7e4(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d7e4 (`-[HomeViewController setWelcomeToRobloxTextView:]`).
    // Same as the platform 0x1d7e4 anchor.
    controller.set_welcome_to_roblox_text_view(value);
}

// 0x1d808 — -[HomeViewController youAreCurrentlyLoggedInAsTextView]
#[doc(alias = "-[HomeViewController youAreCurrentlyLoggedInAsTextView]")]
pub fn stub_1d808(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d808 (`-[HomeViewController youAreCurrentlyLoggedInAsTextView]`).
    // Same as the platform 0x1d808 anchor.
    controller.you_are_currently_logged_in_as_text_view()
}

// 0x1d818 — -[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]
#[doc(alias = "-[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]")]
pub fn stub_1d818(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d818 (`-[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]`).
    // Same as the platform 0x1d818 anchor.
    controller.set_you_are_currently_logged_in_as_text_view(value);
}

// 0x1d83c — -[HomeViewController versionLabel]
#[doc(alias = "-[HomeViewController versionLabel]")]
pub fn stub_1d83c(controller: &crate::generated_138::AudioHomeViewController) -> u64 {
    // IDA 0x1d83c (`-[HomeViewController versionLabel]`). Same as the
    // platform 0x1d83c anchor.
    controller.version_label()
}

// 0x1d84c — -[HomeViewController setVersionLabel:]
#[doc(alias = "-[HomeViewController setVersionLabel:]")]
pub fn stub_1d84c(controller: &crate::generated_138::AudioHomeViewController, value: u64) {
    // IDA 0x1d84c (`-[HomeViewController setVersionLabel:]`). Same as
    // the platform 0x1d84c anchor.
    controller.set_version_label(value);
}

// 0x1d870 — __GLOBAL__I_a_4
#[doc(alias = "__GLOBAL__I_a_4")]
pub fn stub_1d870() {
    // IDA 0x1d870 (`__GLOBAL__I_a_4`, disasm `generic_category` x2 +
    // `system_category` stores into the merged error-category globals):
    // records the category-singleton init. `boost::system` maps to Rust
    // per AGENTS.md (no runtime init needed), so only the call is
    // recorded.
    audio_error_category_init();
}

// 0x1da08 — -[NSString stringWithPercentEscape]
#[doc(alias = "-[NSString stringWithPercentEscape]")]
pub fn stub_1da08(s: &str) -> String {
    // IDA 0x1da08 (`-[NSString stringWithPercentEscape]`):
    // `CFURLCreateStringByAddingPercentEscapes` leaving nothing
    // unescaped beyond the URL-legal set, UTF-8 encoded. The illegal set
    // (`"\u{FFFC}=,!$&'()*+;@?\n\"<>#\t :/"`) is subsumed by escaping
    // everything outside alphanumerics plus `$-_.+!*'(),`.
    audio_percent_escape(s)
}

// 0x1da5c — +[LoginViewController sharedInstance]
#[doc(alias = "+[LoginViewController sharedInstance]")]
pub fn stub_1da5c() -> &'static AudioLoginViewController {
    // IDA 0x1da5c (`+[LoginViewController sharedInstance]`): returns the
    // `dword_130C3F0` singleton.
    AudioLoginViewController::shared_instance()
}

// 0x1da6c — -[LoginViewController initWithCoder:]
#[doc(alias = "-[LoginViewController initWithCoder:]")]
pub fn stub_1da6c(controller: &AudioLoginViewController) -> bool {
    // IDA 0x1da6c (`-[LoginViewController initWithCoder:]`): super init,
    // nil `envs`, three notification observers (login-failed, login-ok,
    // signup-finished).
    controller.init_with_coder()
}

// 0x1dbd4 — -[LoginViewController dealloc]
#[doc(alias = "-[LoginViewController dealloc]")]
pub fn stub_1dbd4(controller: &AudioLoginViewController) {
    // IDA 0x1dbd4 (`-[LoginViewController dealloc]`): removes the
    // notification observer, releases the 15 retained outlets plus the
    // `envs` array when non-nil, then super `dealloc`.
    controller.dealloc();
}

// 0x1dd84 — -[LoginViewController populateEnvironmentPicker]
#[doc(alias = "-[LoginViewController populateEnvironmentPicker]")]
pub fn stub_1dd84(controller: &AudioLoginViewController, tablet: bool) {
    // IDA 0x1dd84 (`-[LoginViewController populateEnvironmentPicker]`):
    // rebuilds `envs` with the 17 environment URLs (`www.` when tablet,
    // else `m.`).
    controller.populate_environment_picker(tablet);
}

// 0x1e0d8 — -[LoginViewController pickerView:didSelectRow:inComponent:]
#[doc(alias = "-[LoginViewController pickerView:didSelectRow:inComponent:]")]
pub fn stub_1e0d8(controller: &AudioLoginViewController, row: usize) -> Option<String> {
    // IDA 0x1e0d8 (`-[LoginViewController pickerView:didSelectRow:...]`):
    // `setBaseUrl:` to `envs[row]`, then the main-queue block starts the
    // memory bouncer.
    controller.picker_did_select_row(row)
}

// 0x1e13c — ___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke
#[doc(alias = "___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke")]
pub fn stub_1e13c(controller: &AudioLoginViewController) {
    // IDA 0x1e13c (picker-select block): `startMemoryBouncer`.
    controller.picker_select_block();
}

// 0x1e170 — -[LoginViewController numberOfComponentsInPickerView:]
#[doc(alias = "-[LoginViewController numberOfComponentsInPickerView:]")]
pub fn stub_1e170(controller: &AudioLoginViewController) -> i32 {
    // IDA 0x1e170 (`numberOfComponentsInPickerView:`): returns 1.
    controller.number_of_components()
}

// 0x1e174 — -[LoginViewController pickerView:numberOfRowsInComponent:]
#[doc(alias = "-[LoginViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_1e174(controller: &AudioLoginViewController) -> usize {
    // IDA 0x1e174 (`pickerView:numberOfRowsInComponent:`): `envs.count`.
    controller.number_of_rows()
}

// 0x1e194 — -[LoginViewController pickerView:titleForRow:forComponent:]
#[doc(alias = "-[LoginViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_1e194(controller: &AudioLoginViewController, row: usize) -> Option<String> {
    // IDA 0x1e194 (`pickerView:titleForRow:...`): `envs[row]`.
    controller.title_for_row(row)
}

// 0x1e1b4 — -[LoginViewController viewWillAppear:]
#[doc(alias = "-[LoginViewController viewWillAppear:]")]
pub fn stub_1e1b4(
    controller: &AudioLoginViewController,
    animated: bool,
    remember_password: bool,
    saved_password: &str,
) {
    // IDA 0x1e1b4 (`-[LoginViewController viewWillAppear:]`): logo
    // `alpha = 1.0`, main-queue `stopShowLoggingIn`, password field from
    // the saved password when remembered, else empty.
    controller.view_will_appear(animated, remember_password, saved_password);
}

// 0x1e2c4 — ___38-[LoginViewController viewWillAppear:]_block_invoke
#[doc(alias = "___38-[LoginViewController viewWillAppear:]_block_invoke")]
pub fn stub_1e2c4(controller: &AudioLoginViewController) {
    // IDA 0x1e2c4 (`__38-[LoginViewController viewWillAppear:]_block_invoke`):
    // `stopShowLoggingIn`.
    controller.stop_show_logging_in();
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
pub fn stub_1e2d8(slot: &mut u64, src: u64) {
    // IDA 0x1e2d8 (disasm one `__Block_object_assign` at +0x14): retain
    // the capture. Same shape as 0x1bb9c in generated_138.
    *slot = src;
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
pub fn stub_1e2e4(slot: &mut u64) {
    // IDA 0x1e2e4 (disasm one `__Block_object_dispose` at +0x14):
    // release the capture. Same shape as 0x1bba8 in generated_138.
    *slot = 0;
}

/// Process-wide error-category init count behind `__GLOBAL__I_a_4`
/// (IDA 0x1d870, disasm `boost::system::generic_category` x2 +
/// `system_category` stores). `boost::system` maps to Rust per AGENTS.md
/// (no runtime init needed), so only the call is recorded.
static AUDIO_ERROR_CATEGORY_INITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Records one `__GLOBAL__I_a_4` run (IDA 0x1d870).
pub fn audio_error_category_init() {
    AUDIO_ERROR_CATEGORY_INITS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

/// Returns the process-wide error-category init count (test hook).
pub fn audio_error_category_inits() -> u32 {
    AUDIO_ERROR_CATEGORY_INITS.load(std::sync::atomic::Ordering::SeqCst)
}

/// `-[NSString stringWithPercentEscape]` (IDA 0x1da08):
/// `CFURLCreateStringByAddingPercentEscapes` with no unescaped extras,
/// UTF-8 encoded: everything outside alphanumerics plus `$-_.+!*'(),`
/// becomes `%XX` (uppercase hex).
pub fn audio_percent_escape(s: &str) -> String {
    const LEGAL: &[u8] = b"$-_.+!*'(),";
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || LEGAL.contains(&b) {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(char::from_digit((b >> 4) as u32, 16).unwrap().to_ascii_uppercase());
            out.push(char::from_digit((b & 0xF) as u32, 16).unwrap().to_ascii_uppercase());
        }
    }
    out
}

/// Audio-crate host for `LoginViewController` (IDA 0x1da5c..0x1e2c4):
/// environment list, observer/init flags, release records, base-url
/// selection, picker/login-appear state. UIKit outlets have no host
/// counterpart (`u64` ids are not needed here — nothing in this slice
/// touches them); the picker-visible `envs` strings do live here.
#[derive(Debug, Default)]
pub struct AudioLoginViewController {
    initialized: std::sync::atomic::AtomicBool,
    observers_registered: std::sync::atomic::AtomicU32,
    deallocated: std::sync::atomic::AtomicBool,
    released_ivar_count: std::sync::atomic::AtomicU32,
    envs_had_entries_on_dealloc: std::sync::atomic::AtomicBool,
    envs: parking_lot::Mutex<Vec<String>>,
    base_url: parking_lot::Mutex<Option<String>>,
    picker_selections: std::sync::atomic::AtomicU32,
    memory_bouncer_starts: std::sync::atomic::AtomicU32,
    logo_alpha_bits: std::sync::atomic::AtomicU32,
    stop_logging_in_dispatches: std::sync::atomic::AtomicU32,
    view_will_appears: std::sync::atomic::AtomicU32,
    password_text: parking_lot::Mutex<String>,
    view_loaded: std::sync::atomic::AtomicBool,
    custom_vars: parking_lot::Mutex<Vec<(String, String)>>,
    localized_login_keys: parking_lot::Mutex<Vec<&'static str>>,
    version_text: parking_lot::Mutex<String>,
    user_agent: parking_lot::Mutex<String>,
    username_text: parking_lot::Mutex<String>,
    switch_on: std::sync::atomic::AtomicBool,
    debug_views_hidden: std::sync::atomic::AtomicBool,
    unloaded_outlets: std::sync::atomic::AtomicU32,
    unloaded: std::sync::atomic::AtomicBool,
    signup_applies: std::sync::atomic::AtomicU32,
    last_failure_alert: parking_lot::Mutex<Option<String>>,
    login_failures: std::sync::atomic::AtomicU32,
    store_accesses: std::sync::atomic::AtomicU32,
    login_transitions: std::sync::atomic::AtomicU32,
    show_logging_ins: std::sync::atomic::AtomicU32,
    about_hidden: std::sync::atomic::AtomicBool,
    activity_shown: std::sync::atomic::AtomicBool,
    animation_runs: std::sync::atomic::AtomicU32,
    login_fields_alpha_steps: std::sync::atomic::AtomicU32,
}

impl AudioLoginViewController {
    /// `+[LoginViewController sharedInstance]` (IDA 0x1da5c): the
    /// `dword_130C3F0` singleton.
    pub fn shared_instance() -> &'static Self {
        static CONTROLLER: std::sync::LazyLock<AudioLoginViewController> =
            std::sync::LazyLock::new(AudioLoginViewController::default);
        &CONTROLLER
    }

    /// `-[LoginViewController initWithCoder:]` (IDA 0x1da6c): super
    /// init, nil `envs`, three notification observers (login-failed,
    /// login-ok, signup-finished).
    pub fn init_with_coder(&self) -> bool {
        use std::sync::atomic::Ordering::SeqCst;
        *self.envs.lock() = Vec::new();
        self.observers_registered.store(3, SeqCst);
        self.initialized.store(true, SeqCst);
        true
    }

    /// `-[LoginViewController dealloc]` (IDA 0x1dbd4): removes the
    /// notification observer, releases the 15 retained outlets plus the
    /// `envs` array when non-nil (Rust drops cover the stores), then
    /// super `dealloc`.
    pub fn dealloc(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.observers_registered.store(0, SeqCst);
        self.envs_had_entries_on_dealloc
            .store(!self.envs.lock().is_empty(), SeqCst);
        *self.envs.lock() = Vec::new();
        self.released_ivar_count.store(15, SeqCst);
        self.deallocated.store(true, SeqCst);
    }

    /// `-[LoginViewController populateEnvironmentPicker]` (IDA 0x1dd84):
    /// rebuilds `envs` with the 17 environment URLs. `tablet` stands in
    /// for `+[RobloxInfo thisDeviceIsATablet]` (out of slice): the host
    /// prefix is `www.` when tablet, else `m.`; the personal `sitetest3`
    /// hosts take no prefix on tablet and `m.` on phone.
    pub fn populate_environment_picker(&self, tablet: bool) {
        let host = if tablet { "www." } else { "m." };
        let personal = if tablet { "" } else { "m." };
        let mut envs = Vec::with_capacity(17);
        envs.push(format!("http://{host}roblox.com/"));
        for n in 1..=4 {
            envs.push(format!("http://{host}sitetest{n}.robloxlabs.com/"));
        }
        for name in [
            "allen", "anthony", "guru", "rosemary", "sairam", "shannon", "vlad",
        ] {
            envs.push(format!("http://{personal}{name}.sitetest3.robloxlabs.com/"));
        }
        for n in (1..=5).rev() {
            envs.push(format!("http://{host}gametest{n}.robloxlabs.com/"));
        }
        *self.envs.lock() = envs;
    }

    /// `-[LoginViewController pickerView:didSelectRow:inComponent:]`
    /// (IDA 0x1e0d8): `setBaseUrl:` to `envs[row]`, then the main-queue
    /// block (`stub_1e13c`) starts the memory bouncer. Out-of-range rows
    /// select nothing (NSArray would throw; the host returns `None`).
    pub fn picker_did_select_row(&self, row: usize) -> Option<String> {
        let url = self.envs.lock().get(row).cloned()?;
        *self.base_url.lock() = Some(url.clone());
        self.picker_selections
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.picker_select_block();
        Some(url)
    }

    /// `__59-[LoginViewController pickerView:...]_block_invoke`
    /// (IDA 0x1e13c): `startMemoryBouncer`.
    pub fn picker_select_block(&self) {
        self.memory_bouncer_starts
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[LoginViewController numberOfComponentsInPickerView:]`
    /// (IDA 0x1e170): returns 1.
    pub fn number_of_components(&self) -> i32 {
        1
    }

    /// `-[LoginViewController pickerView:numberOfRowsInComponent:]`
    /// (IDA 0x1e174): `envs.count`.
    pub fn number_of_rows(&self) -> usize {
        self.envs.lock().len()
    }

    /// `-[LoginViewController pickerView:titleForRow:forComponent:]`
    /// (IDA 0x1e194): `envs[row]` (`None` when out of range).
    pub fn title_for_row(&self, row: usize) -> Option<String> {
        self.envs.lock().get(row).cloned()
    }

    /// `-[LoginViewController viewWillAppear:]` (IDA 0x1e1b4): logo
    /// `alpha = 1.0`, main-queue `stopShowLoggingIn`, password field from
    /// the saved password when `LoginManager.rememberPassword` is set
    /// (both out of slice, threaded in), else empty. `animated` only
    /// reaches the super call.
    pub fn view_will_appear(
        &self,
        animated: bool,
        remember_password: bool,
        saved_password: &str,
    ) {
        use std::sync::atomic::Ordering::SeqCst;
        let _ = animated;
        self.logo_alpha_bits.store(0x3F80_0000, SeqCst);
        self.stop_show_logging_in();
        *self.password_text.lock() = if remember_password {
            saved_password.to_owned()
        } else {
            String::new()
        };
        self.view_will_appears.fetch_add(1, SeqCst);
    }

    /// `__38-[LoginViewController viewWillAppear:]_block_invoke`
    /// (IDA 0x1e2c4): `stopShowLoggingIn`.
    pub fn stop_show_logging_in(&self) {
        self.stop_logging_in_dispatches
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Login-screen `NSBundle` keys `viewDidLoad` stamps
    /// (IDA 0x1e40a..0x1e5b6).
    pub const LOCALIZED_LOGIN_KEYS: [&'static str; 6] = [
        "UsernameWord",
        "PasswordWord",
        "RememberPassword",
        "LoginWord",
        "SignupWord",
        "PlayNowButtonLabel",
    ];

    /// `-[LoginViewController viewDidLoad]` (IDA 0x1e2ec): super
    /// `viewDidLoad`, claims the singleton slot, records the three
    /// analytics custom vars (OS/app/device, out of slice, threaded in),
    /// stamps the six localized placeholders/labels plus the bundle
    /// version, registers the `UserAgent` default, hides the debug
    /// leaves, prefills the username when known, mirrors the remembered
    /// switch/password state, registers the two keyboard observers, and
    /// dispatches the memory-bouncer block.
    #[allow(clippy::too_many_arguments)]
    pub fn view_did_load(
        &self,
        os_version: &str,
        app_version: &str,
        device_name: &str,
        bundle_version: &str,
        user_agent: &str,
        current_username: &str,
        remember_password: bool,
        saved_password: &str,
    ) {
        use std::sync::atomic::Ordering::SeqCst;
        *self.custom_vars.lock() = vec![
            ("iOSVersion".to_owned(), os_version.to_owned()),
            ("appVersion".to_owned(), app_version.to_owned()),
            ("deviceType".to_owned(), device_name.to_owned()),
        ];
        *self.localized_login_keys.lock() = Self::LOCALIZED_LOGIN_KEYS.to_vec();
        *self.version_text.lock() = bundle_version.to_owned();
        *self.user_agent.lock() = user_agent.to_owned();
        self.debug_views_hidden.store(true, SeqCst);
        if !current_username.is_empty() {
            *self.username_text.lock() = current_username.to_owned();
        }
        self.switch_on.store(remember_password, SeqCst);
        if !saved_password.is_empty() && remember_password {
            *self.password_text.lock() = saved_password.to_owned();
        }
        self.observers_registered.fetch_add(2, SeqCst);
        self.picker_select_block();
        self.view_loaded.store(true, SeqCst);
    }

    /// `-[LoginViewController viewDidUnload]` (IDA 0x1e8cc): nils the 10
    /// outlet setters then super `viewDidUnload`, clearing the singleton
    /// slot (Rust drops cover the stores).
    pub fn view_did_unload(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.unloaded_outlets.store(10, SeqCst);
        self.unloaded.store(true, SeqCst);
    }

    /// `-[LoginViewController handleSignupNotification:]` (IDA 0x1e9d0):
    /// retains the `username`/`password` pair from the notification and,
    /// when both are present, dispatches the main-queue block stamping
    /// the two text fields.
    pub fn handle_signup_notification(
        &self,
        username: Option<&str>,
        password: Option<&str>,
    ) -> bool {
        match (username, password) {
            (Some(user), Some(pass)) => {
                self.signup_apply_block(user, pass);
                true
            }
            _ => false,
        }
    }

    /// `__48-[LoginViewController handleSignupNotification:]_block_invoke`
    /// (IDA 0x1eaa0): stamps the username/password fields, releasing the
    /// retained pair (Rust drops cover the releases).
    pub fn signup_apply_block(&self, username: &str, password: &str) {
        *self.username_text.lock() = username.to_owned();
        *self.password_text.lock() = password.to_owned();
        self.signup_applies
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[LoginViewController gotLoginFailedNotification:]` (IDA 0x1eb5c):
    /// retains the `Error` string and dispatches the main-queue block.
    pub fn got_login_failed_notification(&self, error: &str) {
        self.login_failed_block(error);
    }

    /// `__50-[LoginViewController gotLoginFailedNotification:]_block_invoke`
    /// (IDA 0x1ebdc): stops the logging-in UI, raises the error alert,
    /// and clears the password field.
    pub fn login_failed_block(&self, error: &str) {
        use std::sync::atomic::Ordering::SeqCst;
        self.stop_show_logging_in();
        *self.last_failure_alert.lock() = Some(error.to_owned());
        self.login_failures.fetch_add(1, SeqCst);
        *self.password_text.lock() = String::new();
    }

    /// `-[LoginViewController gotLoginSuccessfulNotification:]`
    /// (IDA 0x1ec84): warms the store manager, runs the login
    /// transition, and dispatches the main-queue block clearing the
    /// username field.
    pub fn got_login_successful_notification(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.store_accesses.fetch_add(1, SeqCst);
        self.login_transitions.fetch_add(1, SeqCst);
        self.login_successful_block();
    }

    /// `__54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke`
    /// (IDA 0x1ed04): clears the username field.
    pub fn login_successful_block(&self) {
        *self.username_text.lock() = String::new();
    }

    /// `-[LoginViewController showLoggingIn]` (IDA 0x1ed44): hides the
    /// about button and dispatches the main-queue animation block.
    pub fn show_logging_in(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.about_hidden.store(true, SeqCst);
        self.show_logging_ins.fetch_add(1, SeqCst);
        self.show_logging_in_block();
    }

    /// `__36-[LoginViewController showLoggingIn]_block_invoke`
    /// (IDA 0x1edbc): shows the activity indicator and runs the 0.5s
    /// fade, whose completion (`stub_1ee58`) zeroes the field alpha.
    pub fn show_logging_in_block(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.activity_shown.store(true, SeqCst);
        self.animation_runs.fetch_add(1, SeqCst);
        self.show_logging_in_fade_block();
    }

    /// `__36-[LoginViewController showLoggingIn]_block_invoke_2`
    /// (IDA 0x1ee58): `loginFieldViews.alpha = 0`.
    pub fn show_logging_in_fade_block(&self) {
        self.login_fields_alpha_steps
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x1e2ec — -[LoginViewController viewDidLoad]
#[doc(alias = "-[LoginViewController viewDidLoad]")]
#[allow(clippy::too_many_arguments)]
pub fn stub_1e2ec(
    controller: &AudioLoginViewController,
    os_version: &str,
    app_version: &str,
    device_name: &str,
    bundle_version: &str,
    user_agent: &str,
    current_username: &str,
    remember_password: bool,
    saved_password: &str,
) {
    // IDA 0x1e2ec (`-[LoginViewController viewDidLoad]`): analytics
    // vars, localized stamps, user-agent default, debug hides, field
    // prefills, keyboard observers, memory-bouncer kick.
    controller.view_did_load(
        os_version,
        app_version,
        device_name,
        bundle_version,
        user_agent,
        current_username,
        remember_password,
        saved_password,
    );
}

// 0x1e898 — ___34-[LoginViewController viewDidLoad]_block_invoke
#[doc(alias = "___34-[LoginViewController viewDidLoad]_block_invoke")]
pub fn stub_1e898(controller: &AudioLoginViewController) {
    // IDA 0x1e898 (`__34-[LoginViewController viewDidLoad]_block_invoke`):
    // `startMemoryBouncer`.
    controller.picker_select_block();
}

// 0x1e8cc — -[LoginViewController viewDidUnload]
#[doc(alias = "-[LoginViewController viewDidUnload]")]
pub fn stub_1e8cc(controller: &AudioLoginViewController) {
    // IDA 0x1e8cc (`-[LoginViewController viewDidUnload]`): nils the 10
    // outlet setters then super `viewDidUnload`, clearing the singleton
    // slot.
    controller.view_did_unload();
}

// 0x1e9d0 — -[LoginViewController handleSignupNotification:]
#[doc(alias = "-[LoginViewController handleSignupNotification:]")]
pub fn stub_1e9d0(
    controller: &AudioLoginViewController,
    username: Option<&str>,
    password: Option<&str>,
) -> bool {
    // IDA 0x1e9d0 (`-[LoginViewController handleSignupNotification:]`):
    // retains the pair; dispatches the stamp block when both present.
    controller.handle_signup_notification(username, password)
}

// 0x1eaa0 — ___48-[LoginViewController handleSignupNotification:]_block_invoke
#[doc(alias = "___48-[LoginViewController handleSignupNotification:]_block_invoke")]
pub fn stub_1eaa0(controller: &AudioLoginViewController, username: &str, password: &str) {
    // IDA 0x1eaa0 (signup stamp block): stamps the two fields, releasing
    // the retained pair.
    controller.signup_apply_block(username, password);
}

// 0x1eb08 — ___copy_helper_block_226
#[doc(alias = "___copy_helper_block_226")]
pub fn stub_1eb08(
    first_slot: &mut u64,
    second_slot: &mut u64,
    first_src: u64,
    second_src: u64,
) {
    // IDA 0x1eb08 (disasm `__Block_object_assign` x2 at +0x14/+0x18):
    // retain the two captures (self + username/password pair).
    *first_slot = first_src;
    *second_slot = second_src;
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
pub fn stub_1eb38(first_slot: &mut u64, second_slot: &mut u64) {
    // IDA 0x1eb38 (disasm `__Block_object_dispose` x2 at +0x14/+0x18):
    // release the two captures.
    *first_slot = 0;
    *second_slot = 0;
}

// 0x1eb5c — -[LoginViewController gotLoginFailedNotification:]
#[doc(alias = "-[LoginViewController gotLoginFailedNotification:]")]
pub fn stub_1eb5c(controller: &AudioLoginViewController, error: &str) {
    // IDA 0x1eb5c (`-[LoginViewController gotLoginFailedNotification:]`):
    // retains the `Error` string and dispatches the alert block.
    controller.got_login_failed_notification(error);
}

// 0x1ebdc — ___50-[LoginViewController gotLoginFailedNotification:]_block_invoke
#[doc(alias = "___50-[LoginViewController gotLoginFailedNotification:]_block_invoke")]
pub fn stub_1ebdc(controller: &AudioLoginViewController, error: &str) {
    // IDA 0x1ebdc (login-failed block): stops the logging-in UI, raises
    // the error alert, clears the password field.
    controller.login_failed_block(error);
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
pub fn stub_1ec44(
    first_slot: &mut u64,
    second_slot: &mut u64,
    first_src: u64,
    second_src: u64,
) {
    // IDA 0x1ec44 (disasm `__Block_object_assign` x2 at +0x14/+0x18):
    // retain the two captures (self + error string).
    *first_slot = first_src;
    *second_slot = second_src;
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
pub fn stub_1ec68(first_slot: &mut u64, second_slot: &mut u64) {
    // IDA 0x1ec68 (disasm `__Block_object_dispose` x2 at +0x14/+0x18):
    // release the two captures.
    *first_slot = 0;
    *second_slot = 0;
}

// 0x1ec84 — -[LoginViewController gotLoginSuccessfulNotification:]
#[doc(alias = "-[LoginViewController gotLoginSuccessfulNotification:]")]
pub fn stub_1ec84(controller: &AudioLoginViewController) {
    // IDA 0x1ec84 (`-[LoginViewController gotLoginSuccessfulNotification:]`):
    // warms the store manager, runs the login transition, dispatches the
    // field-clear block.
    controller.got_login_successful_notification();
}

// 0x1ed04 — ___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke
#[doc(alias = "___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke")]
pub fn stub_1ed04(controller: &AudioLoginViewController) {
    // IDA 0x1ed04 (login-ok block): clears the username field.
    controller.login_successful_block();
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
pub fn stub_1ed30(slot: &mut u64, src: u64) {
    // IDA 0x1ed30 (disasm one `__Block_object_assign` at +0x14): retain
    // the capture.
    *slot = src;
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
pub fn stub_1ed3c(slot: &mut u64) {
    // IDA 0x1ed3c (disasm one `__Block_object_dispose` at +0x14):
    // release the capture.
    *slot = 0;
}

// 0x1ed44 — -[LoginViewController showLoggingIn]
#[doc(alias = "-[LoginViewController showLoggingIn]")]
pub fn stub_1ed44(controller: &AudioLoginViewController) {
    // IDA 0x1ed44 (`-[LoginViewController showLoggingIn]`): hides the
    // about button and dispatches the animation block.
    controller.show_logging_in();
}

// 0x1edbc — ___36-[LoginViewController showLoggingIn]_block_invoke
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke")]
pub fn stub_1edbc(controller: &AudioLoginViewController) {
    // IDA 0x1edbc (show-logging-in block): shows the activity indicator
    // and runs the 0.5s fade.
    controller.show_logging_in_block();
}

// 0x1ee58 — ___36-[LoginViewController showLoggingIn]_block_invoke_2
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke_2")]
pub fn stub_1ee58(controller: &AudioLoginViewController) {
    // IDA 0x1ee58 (fade completion): `loginFieldViews.alpha = 0`.
    controller.show_logging_in_fade_block();
}
