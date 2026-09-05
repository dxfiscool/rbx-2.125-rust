// Auto-generated skeletons for rbx-script — Lua|Script|Yield batch (gap filler)
// Filter: Lua|Script|Yield (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x5bf78..0x62320 EA-sorted asc next 150 global not yet in script crate (script 14471 -> 14621 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_173::SignUpErr;

/// `SignupVerifier` observable state (IDA 0x5bf9c..0x5d28c): the
/// normalized base URL, endpoint/args strings, notification names, the
/// last request, the alternate username, and the post/request/notify/
/// signup tallies. The net/JSON glue folds into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SignupVerify {
    pub base: String,
    pub signup_url: String,
    pub signup_args: String,
    pub user_check_url: String,
    pub recommend_url: String,
    pub password_url: String,
    pub done_note: String,
    pub pass_note: String,
    pub user_note: String,
    pub last_url: String,
    pub last_args: String,
    pub alternate: String,
    pub posted: u32,
    pub requested: u32,
    pub notified: u32,
    pub signed_up: u32,
}

// 0x5bf78 — -[SignUpErrorViewController setMessageTextView:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setMessageTextView:]")]
pub fn stub_0x5bf78(vc: &mut SignUpErr, view: u32) {
    // IDA 0x5bf78: `setMessageTextView:` stores the text view.
    vc.text_view = Some(view);
}

// 0x5bf9c — -[SignupVerifier init]
// type: SignupVerifier *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier init]")]
pub fn stub_0x5bf9c(base: &str) -> SignupVerify {
    // IDA 0x5bf9c: `SignupVerifier init` chains to super (0x5bfb8..)
    // and normalizes the base URL (`://m.` to `://www.` at 0x5c00a,
    // `http` to `https` at 0x5c028) before deriving the endpoints; the
    // string glue folds into the host.
    SignupVerify {
        base: base.replace("://m.", "://www.").replace("http", "https"),
        ..SignupVerify::default()
    }
}

// 0x5c17c — -[SignupVerifier dealloc]
// type: void __cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier dealloc]")]
pub fn stub_0x5c17c(v: &mut SignupVerify) {
    // IDA 0x5c17c: `dealloc` releases the URL/args/notification strings
    // (0x5c1a0..) and chains to super; drop glue covers it and the
    // record resets.
    *v = SignupVerify::default();
}

// 0x5c26c — -[SignupVerifier isValidEmail:]
// type: bool __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier isValidEmail:]")]
pub fn stub_0x5c26c(email: &str) -> bool {
    // IDA 0x5c26c: `isValidEmail:` matches
    // `[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}` case-insensitively
    // (0x5c2a4) and answers whether it matches (0x5c2b8..).
    let mut parts = email.split('@');
    let (local, domain) = match (parts.next(), parts.next(), parts.next()) {
        (Some(l), Some(d), None) => (l, d),
        _ => return false,
    };
    if local.is_empty()
        || !local.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'%' | b'+' | b'-'))
    {
        return false;
    }
    let mut labels: Vec<&str> = domain.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let tld = labels.pop().unwrap();
    if !(2..=4).contains(&tld.len()) || !tld.bytes().all(|b| b.is_ascii_alphabetic()) {
        return false;
    }
    !labels.iter().any(|l| {
        l.is_empty() || !l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
    })
}

// 0x5c2e8 — -[SignupVerifier doPostResponseFromUrl:args:notificationName:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id, id)
#[doc(alias = "-[SignupVerifier doPostResponseFromUrl:args:notificationName:]")]
pub fn stub_0x5c2e8(v: &mut SignupVerify, url: &str, args: &str) {
    // IDA 0x5c2e8: `doPostResponseFromUrl:...` POSTs the args to the URL
    // (0x5c31a..0x5c360) and runs the reply block (see `stub_0x5c444`);
    // the net glue folds into the host.
    v.last_url = url.to_string();
    v.last_args = args.to_string();
    v.posted += 1;
}

// 0x5c444 — ___62-[SignupVerifier doPostResponseFromUrl:args:notificationName:]_block_invoke
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "___62-[SignupVerifier doPostResponseFromUrl:args:notificationName:]_block_invoke")]
pub fn stub_0x5c444(v: &mut SignupVerify, ok: bool) {
    // IDA 0x5c444: the POST reply block parses the JSON (0x5c4b6..) and
    // posts the notification on success (0x5c450..); the parse folds
    // into the host.
    if ok {
        v.notified += 1;
    }
}

// 0x5c534 — -[SignupVerifier doGetResponseFromUrl:notificationName:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id)
#[doc(alias = "-[SignupVerifier doGetResponseFromUrl:notificationName:]")]
pub fn stub_0x5c534(v: &mut SignupVerify, url: &str) {
    // IDA 0x5c534: `doGetResponseFromUrl:...` GETs the URL (0x5c566..)
    // and runs the reply block (see `stub_0x5c658`); the net glue folds
    // into the host.
    v.last_url = url.to_string();
    v.requested += 1;
}

// 0x5c658 — ___56-[SignupVerifier doGetResponseFromUrl:notificationName:]_block_invoke
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "___56-[SignupVerifier doGetResponseFromUrl:notificationName:]_block_invoke")]
pub fn stub_0x5c658(v: &mut SignupVerify, ok: bool) {
    // IDA 0x5c658: the GET reply block parses the JSON (0x5c688..) and
    // posts the notification on success (0x5c68c..); the parse folds
    // into the host.
    if ok {
        v.notified += 1;
    }
}

// 0x5c708 — -[SignupVerifier checkPassword:username:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id)
#[doc(alias = "-[SignupVerifier checkPassword:username:]")]
pub fn stub_0x5c708(v: &mut SignupVerify, username: &str, password: &str) {
    // IDA 0x5c708: `checkPassword:username:` formats the password-check
    // args (0x5c742) and POSTs them (0x5c772).
    v.last_args = format!("{username}:{password}");
    v.last_url = v.password_url.clone();
    v.posted += 1;
}

// 0x5c77c — -[SignupVerifier checkUsername:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier checkUsername:]")]
pub fn stub_0x5c77c(v: &mut SignupVerify, username: &str) -> bool {
    // IDA 0x5c77c: `checkUsername:` GETs the check URL for names longer
    // than 2 (0x5c79a..0x5c884), else notifies the short name locally
    // (0x5c7b4..).
    if username.len() > 2 {
        v.requested += 1;
        true
    } else {
        v.notified += 1;
        false
    }
}

// 0x5c888 — -[SignupVerifier getAlternateUsername:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier getAlternateUsername:]")]
pub fn stub_0x5c888(v: &mut SignupVerify, username: &str) {
    // IDA 0x5c888: `getAlternateUsername:` GETs the recommend URL
    // (0x5c8be..); the net glue folds into the host and the reply books
    // via `stub_0x5c9d8`.
    v.last_url = v.recommend_url.clone();
    v.last_args = username.to_string();
    v.requested += 1;
}

// 0x5c9d8 — ___39-[SignupVerifier getAlternateUsername:]_block_invoke
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "___39-[SignupVerifier getAlternateUsername:]_block_invoke")]
pub fn stub_0x5c9d8(v: &mut SignupVerify, alternate: Option<&str>) {
    // IDA 0x5c9d8: the recommend reply block notifies a non-empty
    // alternate name (0x5ca2e..0x5ca4c); the string glue folds into the
    // host.
    if let Some(name) = alternate {
        if !name.is_empty() {
            v.alternate = name.to_string();
            v.notified += 1;
        }
    }
}

// 0x5cae8 — -[SignupVerifier passwordsMatch:verifyPassword:]
// type: bool __cdecl(SignupVerifier *self, SEL, id, id)
#[doc(alias = "-[SignupVerifier passwordsMatch:verifyPassword:]")]
pub fn stub_0x5cae8(password: &str, verify: &str) -> bool {
    // IDA 0x5cae8: `passwordsMatch:...` answers true only for two
    // non-empty equal strings (0x5cb04..0x5cb34).
    !password.is_empty() && !verify.is_empty() && password == verify
}

// 0x5cb3c — -[SignupVerifier processSignUpResponse:data:error:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id, id)
#[doc(alias = "-[SignupVerifier processSignUpResponse:data:error:]")]
pub fn stub_0x5cb3c(v: &mut SignupVerify, ok: bool) {
    // IDA 0x5cb3c: `processSignUpResponse:...` parses the JSON response
    // (0x5cb66..) and books a successful signup; the parse folds into
    // the host.
    if ok {
        v.signed_up += 1;
    }
}

// 0x5cd38 — -[SignupVerifier doSignUp:password:verifyPassword:birthString:gender:email:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id, id, id, int, id)
#[doc(alias = "-[SignupVerifier doSignUp:password:verifyPassword:birthString:gender:email:]")]
pub fn stub_0x5cd38(v: &mut SignupVerify, password: &str, verify: &str) -> bool {
    // IDA 0x5cd38: `doSignUp:...` validates the fields and POSTs the
    // signup (the field checks fold into the host); the net glue folds
    // into the host.
    if stub_0x5cae8(password, verify) {
        v.last_url = v.signup_url.clone();
        v.posted += 1;
        true
    } else {
        false
    }
}

// 0x5d184 — ___76-[SignupVerifier doSignUp:password:verifyPassword:birthString:gender:email:]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___76-[SignupVerifier doSignUp:password:verifyPassword:birthString:gender:email:]_block_invoke")]
pub fn stub_0x5d184(v: &mut SignupVerify, ok: bool) {
    // IDA 0x5d184: the signup block forwards the response (twin of
    // 0x5cb3c).
    stub_0x5cb3c(v, ok);
}

// 0x5d1bc — -[SignupVerifier signUpUrlString]
// type: NSString *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier signUpUrlString]")]
pub fn stub_0x5d1bc(v: &SignupVerify) -> String {
    // IDA 0x5d1bc: `signUpUrlString` answers the URL.
    v.signup_url.clone()
}

// 0x5d1cc — -[SignupVerifier setSignUpUrlString:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier setSignUpUrlString:]")]
pub fn stub_0x5d1cc(v: &mut SignupVerify, url: &str) {
    // IDA 0x5d1cc: `setSignUpUrlString:` stores the URL.
    v.signup_url = url.to_string();
}

// 0x5d1f0 — -[SignupVerifier signUpArgs]
// type: NSString *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier signUpArgs]")]
pub fn stub_0x5d1f0(v: &SignupVerify) -> String {
    // IDA 0x5d1f0: `signUpArgs` answers the args.
    v.signup_args.clone()
}

// 0x5d200 — -[SignupVerifier setSignUpArgs:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier setSignUpArgs:]")]
pub fn stub_0x5d200(v: &mut SignupVerify, args: &str) {
    // IDA 0x5d200: `setSignUpArgs:` stores the args.
    v.signup_args = args.to_string();
}

// 0x5d224 — -[SignupVerifier usernameCheckUrl]
// type: NSString *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier usernameCheckUrl]")]
pub fn stub_0x5d224(v: &SignupVerify) -> String {
    // IDA 0x5d224: `usernameCheckUrl` answers the URL.
    v.user_check_url.clone()
}

// 0x5d234 — -[SignupVerifier setUsernameCheckUrl:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier setUsernameCheckUrl:]")]
pub fn stub_0x5d234(v: &mut SignupVerify, url: &str) {
    // IDA 0x5d234: `setUsernameCheckUrl:` stores the URL.
    v.user_check_url = url.to_string();
}

// 0x5d258 — -[SignupVerifier recommendUsernameUrl]
// type: NSString *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier recommendUsernameUrl]")]
pub fn stub_0x5d258(v: &SignupVerify) -> String {
    // IDA 0x5d258: `recommendUsernameUrl` answers the URL.
    v.recommend_url.clone()
}

// 0x5d268 — -[SignupVerifier setRecommendUsernameUrl:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier setRecommendUsernameUrl:]")]
pub fn stub_0x5d268(v: &mut SignupVerify, url: &str) {
    // IDA 0x5d268: `setRecommendUsernameUrl:` stores the URL.
    v.recommend_url = url.to_string();
}

// 0x5d28c — -[SignupVerifier passwordCheckUrl]
// type: NSString *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier passwordCheckUrl]")]
pub fn stub_0x5d28c(v: &SignupVerify) -> String {
    // IDA 0x5d28c: `passwordCheckUrl` answers the URL.
    v.password_url.clone()
}

#[doc(alias = "-[SignupVerifier setPasswordCheckUrl:]")]
pub fn stub_0x5d29c() -> crate::slot::PortedFn {
// IDA 0x5d29c: -[SignupVerifier setPasswordCheckUrl:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d29c, "-[SignupVerifier setPasswordCheckUrl:]")
}

#[doc(alias = "-[SignupVerifier passwordCheckArgs]")]
pub fn stub_0x5d2c0() -> crate::slot::PortedFn {
// IDA 0x5d2c0: -[SignupVerifier passwordCheckArgs].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d2c0, "-[SignupVerifier passwordCheckArgs]")
}

#[doc(alias = "-[SignupVerifier setPasswordCheckArgs:]")]
pub fn stub_0x5d2d0() -> crate::slot::PortedFn {
// IDA 0x5d2d0: -[SignupVerifier setPasswordCheckArgs:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d2d0, "-[SignupVerifier setPasswordCheckArgs:]")
}

#[doc(alias = "-[SignupVerifier signUpDoneNotification]")]
pub fn stub_0x5d2f4() -> crate::slot::PortedFn {
// IDA 0x5d2f4: -[SignupVerifier signUpDoneNotification].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d2f4, "-[SignupVerifier signUpDoneNotification]")
}

#[doc(alias = "-[SignupVerifier setSignUpDoneNotification:]")]
pub fn stub_0x5d304() -> crate::slot::PortedFn {
// IDA 0x5d304: -[SignupVerifier setSignUpDoneNotification:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d304, "-[SignupVerifier setSignUpDoneNotification:]")
}

#[doc(alias = "-[SignupVerifier passwordVerifyNotification]")]
pub fn stub_0x5d328() -> crate::slot::PortedFn {
// IDA 0x5d328: -[SignupVerifier passwordVerifyNotification].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d328, "-[SignupVerifier passwordVerifyNotification]")
}

#[doc(alias = "-[SignupVerifier setPasswordVerifyNotification:]")]
pub fn stub_0x5d338() -> crate::slot::PortedFn {
// IDA 0x5d338: -[SignupVerifier setPasswordVerifyNotification:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d338, "-[SignupVerifier setPasswordVerifyNotification:]")
}

#[doc(alias = "-[SignupVerifier usernameVerifyNotification]")]
pub fn stub_0x5d35c() -> crate::slot::PortedFn {
// IDA 0x5d35c: -[SignupVerifier usernameVerifyNotification].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d35c, "-[SignupVerifier usernameVerifyNotification]")
}

#[doc(alias = "-[SignupVerifier setUsernameVerifyNotification:]")]
pub fn stub_0x5d36c() -> crate::slot::PortedFn {
// IDA 0x5d36c: -[SignupVerifier setUsernameVerifyNotification:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d36c, "-[SignupVerifier setUsernameVerifyNotification:]")
}

#[doc(alias = "-[SignupVerifier recommendUsernameNotification]")]
pub fn stub_0x5d390() -> crate::slot::PortedFn {
// IDA 0x5d390: -[SignupVerifier recommendUsernameNotification].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d390, "-[SignupVerifier recommendUsernameNotification]")
}

#[doc(alias = "-[SignupVerifier setRecommendUsernameNotification:]")]
pub fn stub_0x5d3a0() -> crate::slot::PortedFn {
// IDA 0x5d3a0: -[SignupVerifier setRecommendUsernameNotification:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d3a0, "-[SignupVerifier setRecommendUsernameNotification:]")
}

#[doc(alias = "-[SignupViewController initWithCoder:]")]
pub fn stub_0x5d3c8() -> crate::slot::PortedFn {
// IDA 0x5d3c8: -[SignupViewController initWithCoder:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d3c8, "-[SignupViewController initWithCoder:]")
}

#[doc(alias = "-[SignupViewController dealloc]")]
pub fn stub_0x5d824() -> crate::slot::PortedFn {
// IDA 0x5d824: -[SignupViewController dealloc].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5d824, "-[SignupViewController dealloc]")
}

#[doc(alias = "+[SignupViewController getSignupFinishedNotification]")]
pub fn stub_0x5dbc0() -> crate::slot::PortedFn {
// IDA 0x5dbc0: +[SignupViewController getSignupFinishedNotification].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5dbc0, "+[SignupViewController getSignupFinishedNotification]")
}

#[doc(alias = "-[SignupViewController keyboardWillShow:]")]
pub fn stub_0x5dbcc() -> crate::slot::PortedFn {
// IDA 0x5dbcc: -[SignupViewController keyboardWillShow:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5dbcc, "-[SignupViewController keyboardWillShow:]")
}

#[doc(alias = "-[SignupViewController setUIButtonTextColor:color:]")]
pub fn stub_0x5dc38() -> crate::slot::PortedFn {
// IDA 0x5dc38: -[SignupViewController setUIButtonTextColor:color:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5dc38, "-[SignupViewController setUIButtonTextColor:color:]")
}

#[doc(alias = "-[SignupViewController viewDidLoad]")]
pub fn stub_0x5dce0() -> crate::slot::PortedFn {
// IDA 0x5dce0: -[SignupViewController viewDidLoad].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5dce0, "-[SignupViewController viewDidLoad]")
}

#[doc(alias = "-[SignupViewController localizeStrings]")]
pub fn stub_0x5e258() -> crate::slot::PortedFn {
// IDA 0x5e258: -[SignupViewController localizeStrings].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5e258, "-[SignupViewController localizeStrings]")
}

#[doc(alias = "-[SignupViewController setGenderUI]")]
pub fn stub_0x5e93c() -> crate::slot::PortedFn {
// IDA 0x5e93c: -[SignupViewController setGenderUI].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5e93c, "-[SignupViewController setGenderUI]")
}

#[doc(alias = "-[SignupViewController genderPickerDoneClicked:]")]
pub fn stub_0x5ea70() -> crate::slot::PortedFn {
// IDA 0x5ea70: -[SignupViewController genderPickerDoneClicked:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5ea70, "-[SignupViewController genderPickerDoneClicked:]")
}

#[doc(alias = "-[SignupViewController genderTouchUp:]")]
pub fn stub_0x5eb30() -> crate::slot::PortedFn {
// IDA 0x5eb30: -[SignupViewController genderTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5eb30, "-[SignupViewController genderTouchUp:]")
}

#[doc(alias = "___38-[SignupViewController genderTouchUp:]_block_invoke")]
pub fn stub_0x5ebb8() -> crate::slot::PortedFn {
// IDA 0x5ebb8: ___38-[SignupViewController genderTouchUp:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5ebb8, "___38-[SignupViewController genderTouchUp:]_block_invoke")
}

#[doc(alias = "-[SignupViewController birthdayTouchUp:]")]
pub fn stub_0x5ed98() -> crate::slot::PortedFn {
// IDA 0x5ed98: -[SignupViewController birthdayTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5ed98, "-[SignupViewController birthdayTouchUp:]")
}

#[doc(alias = "___40-[SignupViewController birthdayTouchUp:]_block_invoke")]
pub fn stub_0x5ee20() -> crate::slot::PortedFn {
// IDA 0x5ee20: ___40-[SignupViewController birthdayTouchUp:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5ee20, "___40-[SignupViewController birthdayTouchUp:]_block_invoke")
}

#[doc(alias = "-[SignupViewController setBirthdayTextUI]")]
pub fn stub_0x5f038() -> crate::slot::PortedFn {
// IDA 0x5f038: -[SignupViewController setBirthdayTextUI].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f038, "-[SignupViewController setBirthdayTextUI]")
}

#[doc(alias = "-[SignupViewController releaseTextFieldFocus]")]
pub fn stub_0x5f1a8() -> crate::slot::PortedFn {
// IDA 0x5f1a8: -[SignupViewController releaseTextFieldFocus].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f1a8, "-[SignupViewController releaseTextFieldFocus]")
}

#[doc(alias = "-[SignupViewController hideAllPickers]")]
pub fn stub_0x5f210() -> crate::slot::PortedFn {
// IDA 0x5f210: -[SignupViewController hideAllPickers].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f210, "-[SignupViewController hideAllPickers]")
}

#[doc(alias = "-[SignupViewController hideGenderPicker]")]
pub fn stub_0x5f240() -> crate::slot::PortedFn {
// IDA 0x5f240: -[SignupViewController hideGenderPicker].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f240, "-[SignupViewController hideGenderPicker]")
}

#[doc(alias = "___40-[SignupViewController hideGenderPicker]_block_invoke")]
pub fn stub_0x5f2b8() -> crate::slot::PortedFn {
// IDA 0x5f2b8: ___40-[SignupViewController hideGenderPicker]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5f2b8, "___40-[SignupViewController hideGenderPicker]_block_invoke")
}

#[doc(alias = "-[SignupViewController hideBirthdayPicker]")]
pub fn stub_0x5f3f8() -> crate::slot::PortedFn {
// IDA 0x5f3f8: -[SignupViewController hideBirthdayPicker].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f3f8, "-[SignupViewController hideBirthdayPicker]")
}

#[doc(alias = "___42-[SignupViewController hideBirthdayPicker]_block_invoke")]
pub fn stub_0x5f470() -> crate::slot::PortedFn {
// IDA 0x5f470: ___42-[SignupViewController hideBirthdayPicker]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5f470, "___42-[SignupViewController hideBirthdayPicker]_block_invoke")
}

#[doc(alias = "-[SignupViewController birthdayDoneTouch:]")]
pub fn stub_0x5f5ec() -> crate::slot::PortedFn {
// IDA 0x5f5ec: -[SignupViewController birthdayDoneTouch:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f5ec, "-[SignupViewController birthdayDoneTouch:]")
}

#[doc(alias = "-[SignupViewController didReceiveMemoryWarning]")]
pub fn stub_0x5f7cc() -> crate::slot::PortedFn {
// IDA 0x5f7cc: -[SignupViewController didReceiveMemoryWarning].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f7cc, "-[SignupViewController didReceiveMemoryWarning]")
}

#[doc(alias = "-[SignupViewController viewWillAppear:]")]
pub fn stub_0x5f7f8() -> crate::slot::PortedFn {
// IDA 0x5f7f8: -[SignupViewController viewWillAppear:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f7f8, "-[SignupViewController viewWillAppear:]")
}

#[doc(alias = "-[SignupViewController cancelTouch:]")]
pub fn stub_0x5f87c() -> crate::slot::PortedFn {
// IDA 0x5f87c: -[SignupViewController cancelTouch:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f87c, "-[SignupViewController cancelTouch:]")
}

#[doc(alias = "-[SignupViewController usernameDoneEdit:]")]
pub fn stub_0x5f890() -> crate::slot::PortedFn {
// IDA 0x5f890: -[SignupViewController usernameDoneEdit:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5f890, "-[SignupViewController usernameDoneEdit:]")
}

#[doc(alias = "-[SignupViewController gotRecommendedUsernameResponse:]")]
pub fn stub_0x5faa0() -> crate::slot::PortedFn {
// IDA 0x5faa0: -[SignupViewController gotRecommendedUsernameResponse:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5faa0, "-[SignupViewController gotRecommendedUsernameResponse:]")
}

#[doc(alias = "-[SignupViewController gotUsernameVerifyResponse:]")]
pub fn stub_0x5fb10() -> crate::slot::PortedFn {
// IDA 0x5fb10: -[SignupViewController gotUsernameVerifyResponse:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5fb10, "-[SignupViewController gotUsernameVerifyResponse:]")
}

#[doc(alias = "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke")]
pub fn stub_0x5fc70() -> crate::slot::PortedFn {
// IDA 0x5fc70: ___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5fc70, "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke")
}

#[doc(alias = "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke327")]
pub fn stub_0x5fd50() -> crate::slot::PortedFn {
// IDA 0x5fd50: ___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke327.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5fd50, "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke327")
}

#[doc(alias = "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke333")]
pub fn stub_0x5fdd8() -> crate::slot::PortedFn {
// IDA 0x5fdd8: ___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke333.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5fdd8, "___50-[SignupViewController gotUsernameVerifyResponse:]_block_invoke333")
}

#[doc(alias = "-[SignupViewController passwordDoneEdit:]")]
pub fn stub_0x5fe10() -> crate::slot::PortedFn {
// IDA 0x5fe10: -[SignupViewController passwordDoneEdit:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5fe10, "-[SignupViewController passwordDoneEdit:]")
}

#[doc(alias = "-[SignupViewController gotPasswordVerifyResponse:]")]
pub fn stub_0x60010() -> crate::slot::PortedFn {
// IDA 0x60010: -[SignupViewController gotPasswordVerifyResponse:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60010, "-[SignupViewController gotPasswordVerifyResponse:]")
}

#[doc(alias = "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke")]
pub fn stub_0x60130() -> crate::slot::PortedFn {
// IDA 0x60130: ___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x60130, "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke")
}

#[doc(alias = "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke348")]
pub fn stub_0x601f8() -> crate::slot::PortedFn {
// IDA 0x601f8: ___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke348.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x601f8, "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke348")
}

#[doc(alias = "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke352")]
pub fn stub_0x60280() -> crate::slot::PortedFn {
// IDA 0x60280: ___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke352.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x60280, "___50-[SignupViewController gotPasswordVerifyResponse:]_block_invoke352")
}

#[doc(alias = "-[SignupViewController verifyDoneEdit:]")]
pub fn stub_0x602b8() -> crate::slot::PortedFn {
// IDA 0x602b8: -[SignupViewController verifyDoneEdit:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x602b8, "-[SignupViewController verifyDoneEdit:]")
}

#[doc(alias = "-[SignupViewController signupTouchUp:]")]
pub fn stub_0x604f8() -> crate::slot::PortedFn {
// IDA 0x604f8: -[SignupViewController signupTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x604f8, "-[SignupViewController signupTouchUp:]")
}

#[doc(alias = "-[SignupViewController respondToSignUp:]")]
pub fn stub_0x605e0() -> crate::slot::PortedFn {
// IDA 0x605e0: -[SignupViewController respondToSignUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x605e0, "-[SignupViewController respondToSignUp:]")
}

#[doc(alias = "___40-[SignupViewController respondToSignUp:]_block_invoke")]
pub fn stub_0x60688() -> crate::slot::PortedFn {
// IDA 0x60688: ___40-[SignupViewController respondToSignUp:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x60688, "___40-[SignupViewController respondToSignUp:]_block_invoke")
}

#[doc(alias = "___40-[SignupViewController respondToSignUp:]_block_invoke_2")]
pub fn stub_0x607d8() -> crate::slot::PortedFn {
// IDA 0x607d8: ___40-[SignupViewController respondToSignUp:]_block_invoke_2.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x607d8, "___40-[SignupViewController respondToSignUp:]_block_invoke_2")
}

#[doc(alias = "-[SignupViewController textFieldBeginEdit:]")]
pub fn stub_0x60954() -> crate::slot::PortedFn {
// IDA 0x60954: -[SignupViewController textFieldBeginEdit:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60954, "-[SignupViewController textFieldBeginEdit:]")
}

#[doc(alias = "-[SignupViewController emailDoneEdit:]")]
pub fn stub_0x60a0c() -> crate::slot::PortedFn {
// IDA 0x60a0c: -[SignupViewController emailDoneEdit:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60a0c, "-[SignupViewController emailDoneEdit:]")
}

#[doc(alias = "-[SignupViewController numberOfComponentsInPickerView:]")]
pub fn stub_0x60af4() -> crate::slot::PortedFn {
// IDA 0x60af4: -[SignupViewController numberOfComponentsInPickerView:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60af4, "-[SignupViewController numberOfComponentsInPickerView:]")
}

#[doc(alias = "-[SignupViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_0x60af8() -> crate::slot::PortedFn {
// IDA 0x60af8: -[SignupViewController pickerView:numberOfRowsInComponent:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60af8, "-[SignupViewController pickerView:numberOfRowsInComponent:]")
}

#[doc(alias = "-[SignupViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_0x60b18() -> crate::slot::PortedFn {
// IDA 0x60b18: -[SignupViewController pickerView:titleForRow:forComponent:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60b18, "-[SignupViewController pickerView:titleForRow:forComponent:]")
}

#[doc(alias = "-[SignupViewController pickerView:viewForRow:forComponent:reusingView:]")]
pub fn stub_0x60b38() -> crate::slot::PortedFn {
// IDA 0x60b38: -[SignupViewController pickerView:viewForRow:forComponent:reusingView:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60b38, "-[SignupViewController pickerView:viewForRow:forComponent:reusingView:]")
}

#[doc(alias = "-[SignupViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_0x60c80() -> crate::slot::PortedFn {
// IDA 0x60c80: -[SignupViewController disablesAutomaticKeyboardDismissal].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60c80, "-[SignupViewController disablesAutomaticKeyboardDismissal]")
}

#[doc(alias = "-[SignupViewController prepareForSegue:sender:]")]
pub fn stub_0x60c84() -> crate::slot::PortedFn {
// IDA 0x60c84: -[SignupViewController prepareForSegue:sender:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60c84, "-[SignupViewController prepareForSegue:sender:]")
}

#[doc(alias = "-[SignupViewController dismissErrorPopover]")]
pub fn stub_0x60e10() -> crate::slot::PortedFn {
// IDA 0x60e10: -[SignupViewController dismissErrorPopover].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60e10, "-[SignupViewController dismissErrorPopover]")
}

#[doc(alias = "-[SignupViewController dismissErrorPopoverWithNewUsername:]")]
pub fn stub_0x60e38() -> crate::slot::PortedFn {
// IDA 0x60e38: -[SignupViewController dismissErrorPopoverWithNewUsername:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60e38, "-[SignupViewController dismissErrorPopoverWithNewUsername:]")
}

#[doc(alias = "-[SignupViewController usernameCheckTouchUp:]")]
pub fn stub_0x60ea0() -> crate::slot::PortedFn {
// IDA 0x60ea0: -[SignupViewController usernameCheckTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x60ea0, "-[SignupViewController usernameCheckTouchUp:]")
}

#[doc(alias = "-[SignupViewController passwordCheckTouchUp:]")]
pub fn stub_0x610d4() -> crate::slot::PortedFn {
// IDA 0x610d4: -[SignupViewController passwordCheckTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x610d4, "-[SignupViewController passwordCheckTouchUp:]")
}

#[doc(alias = "-[SignupViewController verifyCheckTouchUp:]")]
pub fn stub_0x612a8() -> crate::slot::PortedFn {
// IDA 0x612a8: -[SignupViewController verifyCheckTouchUp:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x612a8, "-[SignupViewController verifyCheckTouchUp:]")
}

#[doc(alias = "-[SignupViewController popoverControllerDidDismissPopover:]")]
pub fn stub_0x614cc() -> crate::slot::PortedFn {
// IDA 0x614cc: -[SignupViewController popoverControllerDidDismissPopover:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x614cc, "-[SignupViewController popoverControllerDidDismissPopover:]")
}

#[doc(alias = "-[SignupViewController resignTextFieldResponder:]")]
pub fn stub_0x614e0() -> crate::slot::PortedFn {
// IDA 0x614e0: -[SignupViewController resignTextFieldResponder:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x614e0, "-[SignupViewController resignTextFieldResponder:]")
}

#[doc(alias = "-[SignupViewController textField:shouldChangeCharactersInRange:replacementString:]")]
pub fn stub_0x6150c() -> crate::slot::PortedFn {
// IDA 0x6150c: -[SignupViewController textField:shouldChangeCharactersInRange:replacementString:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6150c, "-[SignupViewController textField:shouldChangeCharactersInRange:replacementString:]")
}

#[doc(alias = "-[SignupViewController textFieldShouldBeginEditing:]")]
pub fn stub_0x616c8() -> crate::slot::PortedFn {
// IDA 0x616c8: -[SignupViewController textFieldShouldBeginEditing:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x616c8, "-[SignupViewController textFieldShouldBeginEditing:]")
}

#[doc(alias = "___52-[SignupViewController textFieldShouldBeginEditing:]_block_invoke")]
pub fn stub_0x617d0() -> crate::slot::PortedFn {
// IDA 0x617d0: ___52-[SignupViewController textFieldShouldBeginEditing:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x617d0, "___52-[SignupViewController textFieldShouldBeginEditing:]_block_invoke")
}

#[doc(alias = "-[SignupViewController textFieldShouldEndEditing:]")]
pub fn stub_0x61ab0() -> crate::slot::PortedFn {
// IDA 0x61ab0: -[SignupViewController textFieldShouldEndEditing:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61ab0, "-[SignupViewController textFieldShouldEndEditing:]")
}

#[doc(alias = "___50-[SignupViewController textFieldShouldEndEditing:]_block_invoke")]
pub fn stub_0x61ba0() -> crate::slot::PortedFn {
// IDA 0x61ba0: ___50-[SignupViewController textFieldShouldEndEditing:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61ba0, "___50-[SignupViewController textFieldShouldEndEditing:]_block_invoke")
}

#[doc(alias = "___destroy_helper_block_491")]
pub fn stub_0x61c70() -> crate::slot::PortedFn {
// IDA 0x61c70: ___destroy_helper_block_491.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61c70, "___destroy_helper_block_491")
}

#[doc(alias = "-[SignupViewController textFieldShouldReturn:]")]
pub fn stub_0x61c8c() -> crate::slot::PortedFn {
// IDA 0x61c8c: -[SignupViewController textFieldShouldReturn:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61c8c, "-[SignupViewController textFieldShouldReturn:]")
}

#[doc(alias = "-[SignupViewController webViewDidFinishLoad:]")]
pub fn stub_0x61d40() -> crate::slot::PortedFn {
// IDA 0x61d40: -[SignupViewController webViewDidFinishLoad:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61d40, "-[SignupViewController webViewDidFinishLoad:]")
}

#[doc(alias = "___45-[SignupViewController webViewDidFinishLoad:]_block_invoke")]
pub fn stub_0x61dc0() -> crate::slot::PortedFn {
// IDA 0x61dc0: ___45-[SignupViewController webViewDidFinishLoad:]_block_invoke.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61dc0, "___45-[SignupViewController webViewDidFinishLoad:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_501")]
pub fn stub_0x61de4() -> crate::slot::PortedFn {
// IDA 0x61de4: ___copy_helper_block_501.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61de4, "___copy_helper_block_501")
}

#[doc(alias = "___destroy_helper_block_502")]
pub fn stub_0x61df0() -> crate::slot::PortedFn {
// IDA 0x61df0: ___destroy_helper_block_502.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61df0, "___destroy_helper_block_502")
}

#[doc(alias = "-[SignupViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x61df8() -> crate::slot::PortedFn {
// IDA 0x61df8: -[SignupViewController webView:shouldStartLoadWithRequest:navigationType:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61df8, "-[SignupViewController webView:shouldStartLoadWithRequest:navigationType:]")
}

#[doc(alias = "-[SignupViewController alertView:clickedButtonAtIndex:]")]
pub fn stub_0x61e80() -> crate::slot::PortedFn {
// IDA 0x61e80: -[SignupViewController alertView:clickedButtonAtIndex:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61e80, "-[SignupViewController alertView:clickedButtonAtIndex:]")
}

#[doc(alias = "-[SignupViewController birthString]")]
pub fn stub_0x61eac() -> crate::slot::PortedFn {
// IDA 0x61eac: -[SignupViewController birthString].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61eac, "-[SignupViewController birthString]")
}

#[doc(alias = "-[SignupViewController setBirthString:]")]
pub fn stub_0x61ebc() -> crate::slot::PortedFn {
// IDA 0x61ebc: -[SignupViewController setBirthString:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61ebc, "-[SignupViewController setBirthString:]")
}

#[doc(alias = "-[SignupViewController birthDate]")]
pub fn stub_0x61ee0() -> crate::slot::PortedFn {
// IDA 0x61ee0: -[SignupViewController birthDate].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61ee0, "-[SignupViewController birthDate]")
}

#[doc(alias = "-[SignupViewController setBirthDate:]")]
pub fn stub_0x61ef0() -> crate::slot::PortedFn {
// IDA 0x61ef0: -[SignupViewController setBirthDate:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61ef0, "-[SignupViewController setBirthDate:]")
}

#[doc(alias = "-[SignupViewController username]")]
pub fn stub_0x61f14() -> crate::slot::PortedFn {
// IDA 0x61f14: -[SignupViewController username].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f14, "-[SignupViewController username]")
}

#[doc(alias = "-[SignupViewController setUsername:]")]
pub fn stub_0x61f24() -> crate::slot::PortedFn {
// IDA 0x61f24: -[SignupViewController setUsername:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f24, "-[SignupViewController setUsername:]")
}

#[doc(alias = "-[SignupViewController password]")]
pub fn stub_0x61f48() -> crate::slot::PortedFn {
// IDA 0x61f48: -[SignupViewController password].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f48, "-[SignupViewController password]")
}

#[doc(alias = "-[SignupViewController setPassword:]")]
pub fn stub_0x61f58() -> crate::slot::PortedFn {
// IDA 0x61f58: -[SignupViewController setPassword:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f58, "-[SignupViewController setPassword:]")
}

#[doc(alias = "-[SignupViewController passwordVerify]")]
pub fn stub_0x61f7c() -> crate::slot::PortedFn {
// IDA 0x61f7c: -[SignupViewController passwordVerify].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f7c, "-[SignupViewController passwordVerify]")
}

#[doc(alias = "-[SignupViewController setPasswordVerify:]")]
pub fn stub_0x61f8c() -> crate::slot::PortedFn {
// IDA 0x61f8c: -[SignupViewController setPasswordVerify:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61f8c, "-[SignupViewController setPasswordVerify:]")
}

#[doc(alias = "-[SignupViewController gender]")]
pub fn stub_0x61fb0() -> crate::slot::PortedFn {
// IDA 0x61fb0: -[SignupViewController gender].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61fb0, "-[SignupViewController gender]")
}

#[doc(alias = "-[SignupViewController setGender:]")]
pub fn stub_0x61fc0() -> crate::slot::PortedFn {
// IDA 0x61fc0: -[SignupViewController setGender:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61fc0, "-[SignupViewController setGender:]")
}

#[doc(alias = "-[SignupViewController cancelButton]")]
pub fn stub_0x61fd0() -> crate::slot::PortedFn {
// IDA 0x61fd0: -[SignupViewController cancelButton].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61fd0, "-[SignupViewController cancelButton]")
}

#[doc(alias = "-[SignupViewController setCancelButton:]")]
pub fn stub_0x61fe0() -> crate::slot::PortedFn {
// IDA 0x61fe0: -[SignupViewController setCancelButton:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x61fe0, "-[SignupViewController setCancelButton:]")
}

#[doc(alias = "-[SignupViewController signupBar]")]
pub fn stub_0x62004() -> crate::slot::PortedFn {
// IDA 0x62004: -[SignupViewController signupBar].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62004, "-[SignupViewController signupBar]")
}

#[doc(alias = "-[SignupViewController setSignupBar:]")]
pub fn stub_0x62014() -> crate::slot::PortedFn {
// IDA 0x62014: -[SignupViewController setSignupBar:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62014, "-[SignupViewController setSignupBar:]")
}

#[doc(alias = "-[SignupViewController selectGenderItem]")]
pub fn stub_0x62038() -> crate::slot::PortedFn {
// IDA 0x62038: -[SignupViewController selectGenderItem].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62038, "-[SignupViewController selectGenderItem]")
}

#[doc(alias = "-[SignupViewController setSelectGenderItem:]")]
pub fn stub_0x62048() -> crate::slot::PortedFn {
// IDA 0x62048: -[SignupViewController setSelectGenderItem:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62048, "-[SignupViewController setSelectGenderItem:]")
}

#[doc(alias = "-[SignupViewController selectBirthdayItem]")]
pub fn stub_0x6206c() -> crate::slot::PortedFn {
// IDA 0x6206c: -[SignupViewController selectBirthdayItem].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6206c, "-[SignupViewController selectBirthdayItem]")
}

#[doc(alias = "-[SignupViewController setSelectBirthdayItem:]")]
pub fn stub_0x6207c() -> crate::slot::PortedFn {
// IDA 0x6207c: -[SignupViewController setSelectBirthdayItem:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6207c, "-[SignupViewController setSelectBirthdayItem:]")
}

#[doc(alias = "-[SignupViewController birthdayToolbar]")]
pub fn stub_0x620a0() -> crate::slot::PortedFn {
// IDA 0x620a0: -[SignupViewController birthdayToolbar].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x620a0, "-[SignupViewController birthdayToolbar]")
}

#[doc(alias = "-[SignupViewController setBirthdayToolbar:]")]
pub fn stub_0x620b0() -> crate::slot::PortedFn {
// IDA 0x620b0: -[SignupViewController setBirthdayToolbar:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x620b0, "-[SignupViewController setBirthdayToolbar:]")
}

#[doc(alias = "-[SignupViewController genderToolbar]")]
pub fn stub_0x620d4() -> crate::slot::PortedFn {
// IDA 0x620d4: -[SignupViewController genderToolbar].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x620d4, "-[SignupViewController genderToolbar]")
}

#[doc(alias = "-[SignupViewController setGenderToolbar:]")]
pub fn stub_0x620e4() -> crate::slot::PortedFn {
// IDA 0x620e4: -[SignupViewController setGenderToolbar:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x620e4, "-[SignupViewController setGenderToolbar:]")
}

#[doc(alias = "-[SignupViewController inputAccView]")]
pub fn stub_0x62108() -> crate::slot::PortedFn {
// IDA 0x62108: -[SignupViewController inputAccView].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62108, "-[SignupViewController inputAccView]")
}

#[doc(alias = "-[SignupViewController setInputAccView:]")]
pub fn stub_0x62118() -> crate::slot::PortedFn {
// IDA 0x62118: -[SignupViewController setInputAccView:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62118, "-[SignupViewController setInputAccView:]")
}

#[doc(alias = "-[SignupViewController nextButtonGender]")]
pub fn stub_0x6213c() -> crate::slot::PortedFn {
// IDA 0x6213c: -[SignupViewController nextButtonGender].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6213c, "-[SignupViewController nextButtonGender]")
}

#[doc(alias = "-[SignupViewController setNextButtonGender:]")]
pub fn stub_0x6214c() -> crate::slot::PortedFn {
// IDA 0x6214c: -[SignupViewController setNextButtonGender:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6214c, "-[SignupViewController setNextButtonGender:]")
}

#[doc(alias = "-[SignupViewController nextButtonBirthday]")]
pub fn stub_0x62170() -> crate::slot::PortedFn {
// IDA 0x62170: -[SignupViewController nextButtonBirthday].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62170, "-[SignupViewController nextButtonBirthday]")
}

#[doc(alias = "-[SignupViewController setNextButtonBirthday:]")]
pub fn stub_0x62180() -> crate::slot::PortedFn {
// IDA 0x62180: -[SignupViewController setNextButtonBirthday:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62180, "-[SignupViewController setNextButtonBirthday:]")
}

#[doc(alias = "-[SignupViewController doneButton]")]
pub fn stub_0x621a4() -> crate::slot::PortedFn {
// IDA 0x621a4: -[SignupViewController doneButton].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621a4, "-[SignupViewController doneButton]")
}

#[doc(alias = "-[SignupViewController setDoneButton:]")]
pub fn stub_0x621b4() -> crate::slot::PortedFn {
// IDA 0x621b4: -[SignupViewController setDoneButton:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621b4, "-[SignupViewController setDoneButton:]")
}

#[doc(alias = "-[SignupViewController usernameLabel]")]
pub fn stub_0x621d8() -> crate::slot::PortedFn {
// IDA 0x621d8: -[SignupViewController usernameLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621d8, "-[SignupViewController usernameLabel]")
}

#[doc(alias = "-[SignupViewController setUsernameLabel:]")]
pub fn stub_0x621e8() -> crate::slot::PortedFn {
// IDA 0x621e8: -[SignupViewController setUsernameLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621e8, "-[SignupViewController setUsernameLabel:]")
}

#[doc(alias = "-[SignupViewController passwordLabel]")]
pub fn stub_0x6220c() -> crate::slot::PortedFn {
// IDA 0x6220c: -[SignupViewController passwordLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6220c, "-[SignupViewController passwordLabel]")
}

#[doc(alias = "-[SignupViewController setPasswordLabel:]")]
pub fn stub_0x6221c() -> crate::slot::PortedFn {
// IDA 0x6221c: -[SignupViewController setPasswordLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6221c, "-[SignupViewController setPasswordLabel:]")
}

#[doc(alias = "-[SignupViewController verifyLabel]")]
pub fn stub_0x62240() -> crate::slot::PortedFn {
// IDA 0x62240: -[SignupViewController verifyLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62240, "-[SignupViewController verifyLabel]")
}

#[doc(alias = "-[SignupViewController setVerifyLabel:]")]
pub fn stub_0x62250() -> crate::slot::PortedFn {
// IDA 0x62250: -[SignupViewController setVerifyLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62250, "-[SignupViewController setVerifyLabel:]")
}

#[doc(alias = "-[SignupViewController genderLabel]")]
pub fn stub_0x62274() -> crate::slot::PortedFn {
// IDA 0x62274: -[SignupViewController genderLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62274, "-[SignupViewController genderLabel]")
}

#[doc(alias = "-[SignupViewController setGenderLabel:]")]
pub fn stub_0x62284() -> crate::slot::PortedFn {
// IDA 0x62284: -[SignupViewController setGenderLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62284, "-[SignupViewController setGenderLabel:]")
}

#[doc(alias = "-[SignupViewController birthdayLabel]")]
pub fn stub_0x622a8() -> crate::slot::PortedFn {
// IDA 0x622a8: -[SignupViewController birthdayLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x622a8, "-[SignupViewController birthdayLabel]")
}

#[doc(alias = "-[SignupViewController setBirthdayLabel:]")]
pub fn stub_0x622b8() -> crate::slot::PortedFn {
// IDA 0x622b8: -[SignupViewController setBirthdayLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x622b8, "-[SignupViewController setBirthdayLabel:]")
}

#[doc(alias = "-[SignupViewController emailLabel]")]
pub fn stub_0x622dc() -> crate::slot::PortedFn {
// IDA 0x622dc: -[SignupViewController emailLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x622dc, "-[SignupViewController emailLabel]")
}

#[doc(alias = "-[SignupViewController setEmailLabel:]")]
pub fn stub_0x622ec() -> crate::slot::PortedFn {
// IDA 0x622ec: -[SignupViewController setEmailLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x622ec, "-[SignupViewController setEmailLabel:]")
}

#[doc(alias = "-[SignupViewController selectLabel]")]
pub fn stub_0x62310() -> crate::slot::PortedFn {
// IDA 0x62310: -[SignupViewController selectLabel].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62310, "-[SignupViewController selectLabel]")
}

#[doc(alias = "-[SignupViewController setSelectLabel:]")]
pub fn stub_0x62320() -> crate::slot::PortedFn {
// IDA 0x62320: -[SignupViewController setSelectLabel:].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62320, "-[SignupViewController setSelectLabel:]")
}

#[cfg(test)]
mod signup_batch_tests {
    use super::*;
    use crate::generated_173::SignUpErr;

    #[test]
    fn text_view_slot() {
        let mut vc = SignUpErr::default();
        stub_0x5bf78(&mut vc, 11);
        assert_eq!(vc.text_view, Some(11));
    }

    #[test]
    fn verifier_init() {
        let v = stub_0x5bf9c("http://m.roblox.com/");
        assert_eq!(v.base, "https://www.roblox.com/");
        let v2 = stub_0x5bf9c("https://www.roblox.com/");
        assert_eq!(v2.base, "https://www.roblox.com/");
        let mut w = v.clone();
        stub_0x5c17c(&mut w);
        assert_eq!(w, SignupVerify::default());
    }

    #[test]
    fn email_and_passwords() {
        assert!(stub_0x5c26c("a@b.co"));
        assert!(stub_0x5c26c("User.Name+1@Sub.Domain.Info"));
        assert!(!stub_0x5c26c("nope"));
        assert!(!stub_0x5c26c("a@b.c"));
        assert!(!stub_0x5c26c("a@b.toolong"));
        assert!(!stub_0x5c26c("@b.co"));
        assert!(!stub_0x5c26c("a@@b.co"));
        assert!(stub_0x5cae8("secret", "secret"));
        assert!(!stub_0x5cae8("", "secret"));
        assert!(!stub_0x5cae8("secret", ""));
        assert!(!stub_0x5cae8("a", "b"));
    }

    #[test]
    fn verify_flow() {
        let mut v = stub_0x5bf9c("https://www.roblox.com/");
        stub_0x5d1cc(&mut v, "https://signup/");
        assert_eq!(stub_0x5d1bc(&v), "https://signup/");
        stub_0x5d200(&mut v, "args");
        assert_eq!(stub_0x5d1f0(&v), "args");
        stub_0x5d234(&mut v, "https://check/");
        assert_eq!(stub_0x5d224(&v), "https://check/");
        stub_0x5d268(&mut v, "https://rec/");
        assert_eq!(stub_0x5d258(&v), "https://rec/");
        assert_eq!(stub_0x5d28c(&v), "");
        stub_0x5c2e8(&mut v, "https://x/", "a=1");
        assert_eq!(v.posted, 1);
        assert_eq!(v.last_url, "https://x/");
        stub_0x5c444(&mut v, false);
        assert_eq!(v.notified, 0);
        stub_0x5c444(&mut v, true);
        assert_eq!(v.notified, 1);
        stub_0x5c534(&mut v, "https://y/");
        assert_eq!(v.requested, 1);
        stub_0x5c658(&mut v, true);
        assert_eq!(v.notified, 2);
        assert!(!stub_0x5c77c(&mut v, "ab"));
        assert_eq!(v.notified, 3);
        assert!(stub_0x5c77c(&mut v, "abc"));
        assert_eq!(v.requested, 2);
        stub_0x5c708(&mut v, "u", "p");
        assert_eq!(v.posted, 2);
        stub_0x5c888(&mut v, "u");
        assert_eq!(v.requested, 3);
        stub_0x5c9d8(&mut v, Some(""));
        assert_eq!(v.notified, 3);
        stub_0x5c9d8(&mut v, Some("alt1"));
        assert_eq!(v.alternate, "alt1");
        assert_eq!(v.notified, 4);
        assert!(!stub_0x5cd38(&mut v, "a", "b"));
        assert!(stub_0x5cd38(&mut v, "a", "a"));
        assert_eq!(v.posted, 3);
        stub_0x5cb3c(&mut v, true);
        assert_eq!(v.signed_up, 1);
        stub_0x5d184(&mut v, true);
        assert_eq!(v.signed_up, 2);
    }
}
