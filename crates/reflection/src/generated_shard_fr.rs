// Auto-generated shard FR — 120 stubs EA-sorted asc 0x56914..0x5c888 (global gap filler not yet in reflection, 22565->22685 distinct)
// Source: ida/export.json (85545 funcs) EA asc not in crates/reflection/src/*.rs, next 120
// Format: // 0xADDR — mangled + doc alias + stub using rbx_core::SharedPtr not boost

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `StoreManager` transaction state (IDA 0x56914-0x58340): queue
/// finishes, purchase/verify outcomes, retry + receipt-post counts and
/// `UIWebViewCacheManager` init flags. StoreKit + web views live out of
/// slice.
pub(crate) static TX_FINISHES: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static PURCHASE_OK: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static AFTER_RETRY_OK: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static RELAUNCH_OK: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static BOGUS_RECEIPT_EVENTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static CANCEL_EVENTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static VERIFY_POSTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static PARENTAL_ALERTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static MAIN_DISPATCHES: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static CACHE_PRECACHING: std::sync::atomic::AtomicBool =
 std::sync::atomic::AtomicBool::new(false);
pub(crate) static CACHE_INITIALIZED: std::sync::atomic::AtomicBool =
 std::sync::atomic::AtomicBool::new(false);
pub(crate) static CACHE_INIT_DISPATCHES: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static CACHE_OBSERVERS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
/// `completeTransaction:` routing (IDA 0x56ad0): nil, offline and
/// sign-in-mismatch cases alert or drop; verified receipts verify;
/// retry-window misses re-schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteOutcome {
 Ignored,
 NoConnection,
 SignInRequired,
 RetryDelayed,
 Verify,
}
/// `endTransaction:` routing (IDA 0x56d80): success finishes + records;
/// failures bump retries (10 = main dispatch), give up at 201 or when
/// the retry window lapses, else schedule a retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndTxOutcome {
 Success,
 SuccessAfterRetry,
 RetryScheduled,
 RetryCapped,
 GaveUp,
}
/// `UIWebViewCacheManager` page/webview state (IDA 0x58334-0x58d4c):
/// preload button tags, cached webview urls, reload + background
/// dispatch counts. Web views live out of slice.
pub(crate) static CACHE_PAGES: parking_lot::Mutex<Vec<u32>> =
 parking_lot::Mutex::new(Vec::new());
pub(crate) static CACHE_WEBVIEWS: std::sync::LazyLock<
 parking_lot::Mutex<std::collections::HashMap<String, bool>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) static BG_DISPATCHES: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static CACHE_RELOADS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
/// `RobloxPageViewController` UserAgent default registrations (IDA
/// 0x58d7c). Defaults live out of slice.
pub(crate) static PAGE_UA_REGISTRATIONS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
/// `LoginManager` session state (IDA 0x58f94-0x5a068): remembered
/// password flag, login/logout attempt counts, offline-failure posts
/// and the current player info filled by `updateUserInfo:`. Keychain +
/// `UserInfo` live out of slice.
pub(crate) static REMEMBER_PASSWORD: std::sync::atomic::AtomicBool =
 std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_ATTEMPTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGOUT_POSTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_FAILED_POSTS: std::sync::atomic::AtomicU32 =
 std::sync::atomic::AtomicU32::new(0);
/// Current player fields written by `updateUserInfo:password:` (IDA
/// 0x59504): id, name, password, balances, thumbnail and BC flag.
#[derive(Debug, Clone, Default)]
pub struct LoginUserInfo {
 pub user_id: String,
 pub username: String,
 pub password: String,
 pub robux_balance: String,
 pub tickets_balance: String,
 pub thumbnail_url: String,
 pub bc_member: String,
}
pub(crate) static CURRENT_USER: std::sync::LazyLock<parking_lot::Mutex<LoginUserInfo>> =
 std::sync::LazyLock::new(|| parking_lot::Mutex::new(LoginUserInfo::default()));

// 0x56914 — -[StoreManager purchaseProduct:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager purchaseProduct:]")]
pub fn stub_56914(product: &str, can_pay: bool) {
    // IDA 0x56914: `purchaseProduct:` requests the product data when
    // `canMakePurchase` passes (0x5692c-0x56944), else alerts parental
    // control (0x5696c-0x569a6). The branch records here.
    if can_pay {
        crate::generated_shard_fq::stub_56894(product);
    } else {
        PARENTAL_ALERTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x569b4 — -[StoreManager verifyIfCorrectUser]
// type: int __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager verifyIfCorrectUser]")]
pub fn stub_569b4(pending: Option<&str>, username: &str, logged_in: bool) -> u32 {
    // IDA 0x569b4: `verifyIfCorrectUser` reports 2 with no (0x569fc)
    // or empty (0x56a4a) pending user, else compares the logged-in
    // name (0x56a84-0x56ac8): 0 match, 1 mismatch.
    match pending {
        None => 2,
        Some(name) if name.is_empty() => 2,
        Some(name) => {
            let user = if logged_in { username } else { "" };
            u32::from(user != name)
        }
    }
}

// 0x56ad0 — -[StoreManager completeTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager completeTransaction:]")]
pub fn stub_56ad0(
    tx_present: bool,
    reachable: bool,
    verify: u32,
    logged_in: bool,
    in_retry_window: bool,
) -> CompleteOutcome {
    // IDA 0x56ad0: `completeTransaction:` drops nil (0x56ae2), alerts
    // offline (0x56c3a), retries when window-open at verify 1
    // (0x56bea) vs verifies, drops verify 0, alerts sign-in at verify
    // 2 when logged out (0x56caa) and verifies when logged in. The
    // route reports here.
    if !tx_present {
        return CompleteOutcome::Ignored;
    }
    if !reachable {
        return CompleteOutcome::NoConnection;
    }
    match verify {
        2 => {
            if logged_in {
                CompleteOutcome::Verify
            } else {
                CompleteOutcome::SignInRequired
            }
        }
        1 => {
            if in_retry_window {
                CompleteOutcome::RetryDelayed
            } else {
                CompleteOutcome::Verify
            }
        }
        _ => CompleteOutcome::Ignored,
    }
}

// 0x56d80 — -[StoreManager endTransaction:paymentTransaction:paymentQueue:]
// type: void __cdecl(StoreManager *self, SEL, char, id, id)
#[doc(alias = "-[StoreManager endTransaction:paymentTransaction:paymentQueue:]")]
pub fn stub_56d80(
    success: bool,
    product: &str,
    now: f64,
    prior_retries: u32,
    relaunch_retries: u32,
    retry_window_lapsed: bool,
    retries: &mut u32,
) -> EndTxOutcome {
    // IDA 0x56d80: `endTransaction:` finishes + records + reports
    // success on ok (0x56da6-0x56f82, retry/relaunch flavors at
    // 0x56e50/0x56f54), else bumps retries (0x56ea6), dispatches the
    // 10th on main (0x5710e), caps at 201 (0x57132), resets on a lapsed
    // window (0x5725a-0x572d8) and schedules a retry otherwise. The
    // route records here.
    if success {
        crate::generated_shard_fq::stub_55d04(product, now);
        PURCHASE_OK.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if prior_retries > 0 {
            AFTER_RETRY_OK.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            EndTxOutcome::SuccessAfterRetry
        } else {
            if relaunch_retries > 0 {
                RELAUNCH_OK.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
            EndTxOutcome::Success
        }
    } else {
        *retries = prior_retries + 1;
        if *retries == 10 {
            MAIN_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        if *retries > 200 {
            return EndTxOutcome::RetryCapped;
        }
        if retry_window_lapsed {
            TX_FINISHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            crate::generated_shard_fq::stub_55c68();
            BOGUS_RECEIPT_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            EndTxOutcome::GaveUp
        } else {
            MAIN_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            EndTxOutcome::RetryScheduled
        }
    }
}

// 0x572e4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke")]
pub fn stub_572e4() {
    // IDA 0x572e4: the retries-10 block runs the delayed retry on main
    // (dispatched at 0x57124). Dispatch glue; no explicit body.
}

// 0x573b0 — ___copy_helper_block_212
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_212")]
pub fn stub_573b0() {
    // IDA 0x573b0: `__copy_helper_block_212` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x573bc — ___destroy_helper_block_213
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_213")]
pub fn stub_573bc() {
    // IDA 0x573bc: `__destroy_helper_block_213` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x573c4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215")]
pub fn stub_573c4() {
    // IDA 0x573c4: the retry block runs `completeTransaction:` on main
    // (dispatched at 0x57214). Dispatch glue; no explicit body.
}

// 0x57410 — ___copy_helper_block_216
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_216")]
pub fn stub_57410() {
    // IDA 0x57410: `__copy_helper_block_216` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x57434 — ___destroy_helper_block_217
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_217")]
pub fn stub_57434() {
    // IDA 0x57434: `__destroy_helper_block_217` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x57450 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219")]
pub fn stub_57450() {
    // IDA 0x57450: the give-up block finishes + resets on main
    // (dispatched at 0x5725a). Dispatch glue; no explicit body.
}

// 0x5751c — ___copy_helper_block_222
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_222")]
pub fn stub_5751c() {
    // IDA 0x5751c: `__copy_helper_block_222` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x57528 — ___destroy_helper_block_223
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_223")]
pub fn stub_57528() {
    // IDA 0x57528: `__destroy_helper_block_223` releases the captures.
    // Release is drop glue; no explicit body.
}

pub fn stub_57530(cancelled_state: bool, cancelled_code: bool) {
    // IDA 0x57530: `failedTransaction:` finishes + resets
    // (0x57556-0x5757a), logs the error (0x575a6-0x575c0) and tracks
    // cancel/auth failures (0x575f4-0x57630). The finish records here.
    TX_FINISHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    crate::generated_shard_fq::stub_55c68();
    if cancelled_state && cancelled_code {
        CANCEL_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

pub fn stub_5763c() {
    // IDA 0x5763c: `restoreTransaction:` logs restore
    // (0x5765c-0x576aa) and finishes the transaction (0x576cc-0x576e0).
    // The finish records here.
    TX_FINISHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

pub fn stub_57740(states: &[u32]) -> (u32, u32, u32) {
    // IDA 0x57740: `paymentQueue:updatedTransactions:` routes state 3
    // to restore (0x57806), 2 to fail (0x57800) and 1 to complete
    // (0x57812). The routed counts report here as
    // (restored, failed, completed).
    let mut routed = (0u32, 0u32, 0u32);
    for state in states {
        match state {
            3 => routed.0 += 1,
            2 => routed.1 += 1,
            1 => routed.2 += 1,
            _ => {}
        }
    }
    routed
}

pub fn stub_5784c(data: &[u8]) -> String {
    // IDA 0x5784c: `encode:length:` is base64 over the receipt bytes
    // (custom loop, 0x57888-0x57920, `=` pad at 0x578f4/0x5790a). The
    // encoded text reports here.
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(4 * data.len().div_ceil(3));
    for chunk in data.chunks(3) {
        let mut bits: u32 = 0;
        for (i, byte) in chunk.iter().enumerate() {
            bits |= u32::from(*byte) << (8 * (2 - i));
        }
        out.push(TABLE[((bits >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((bits >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((bits >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(bits & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub fn stub_5796c(
    base_url: &str,
    receipt_b64: &str,
    product: &str,
    retry_recorded: bool,
) -> String {
    // IDA 0x5796c: `verifyReceipt:...` base64s the receipt (0x57b78),
    // posts `receipt=<b64>&productId=<id>` to
    // `{base}mobileapi/apple-purchase[?isRetry=true]` over https
    // (0x579ac-0x57d2c) and handles the reply on a fresh queue. The
    // posted body records here.
    let endpoint = if retry_recorded {
        "mobileapi/apple-purchase?isRetry=true"
    } else {
        "mobileapi/apple-purchase"
    };
    let _url = format!(
        "{}{}",
        base_url.replacen("http:", "https:", 1),
        endpoint
    );
    VERIFY_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!(
        "receipt={}&productId={}",
        receipt_b64.replace('+', "%2B"),
        product
    )
}

pub fn stub_57da0() {
    // IDA 0x57da0: the verify-reply block handles the POST reply
    // (registered at 0x57d2c). Completion glue; no explicit body.
}

pub fn stub_57f28() {
    // IDA 0x57f28: the verify-reply continuation block (registered at
    // 0x57d2c). Completion glue; no explicit body.
}

pub fn stub_57f98() {
    // IDA 0x57f98: `__copy_helper_block_319` retains the captures.
    // Retain is drop glue; no explicit body.
}

pub fn stub_57fc8() {
    // IDA 0x57fc8: `__destroy_helper_block_320` releases the captures.
    // Release is drop glue; no explicit body.
}

pub fn stub_57fec() {
    // IDA 0x57fec: `__GLOBAL__I_a_29` runs the `a_29`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

pub fn stub_58184() -> bool {
    // IDA 0x58184: `UIWebViewCacheManager::init` zeroes precache +
    // initialized (0x581d2-0x581d8), dispatches the async init block
    // (0x58202-0x58214), loads the preload pages (0x58226) and
    // observes base-url + leave-game (0x5824a-0x582e8). The init
    // records here.
    CACHE_PRECACHING.store(false, std::sync::atomic::Ordering::SeqCst);
    CACHE_INITIALIZED.store(false, std::sync::atomic::Ordering::SeqCst);
    CACHE_INIT_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    CACHE_OBSERVERS.store(2, std::sync::atomic::Ordering::SeqCst);
    true
}

pub fn stub_582f8() {
    // IDA 0x582f8: the cache-init async block warms the cache
    // (dispatched at 0x58214). Dispatch glue; no explicit body.
}

// 0x58334 — ___copy_helper_block__17
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__17")]
pub fn stub_58334() {
    // IDA 0x58334: `__copy_helper_block__17` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x58340 — ___destroy_helper_block__17
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__17")]
pub fn stub_58340() {
    // IDA 0x58340: `__destroy_helper_block__17` releases the
    // captures. Release is drop glue; no explicit body.
}

// 0x58348 — -[UIWebViewCacheManager dealloc]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager dealloc]")]
pub fn stub_58348() {
    // IDA 0x58348: `UIWebViewCacheManager::dealloc` removes the
    // base-url + leave-game observers. The removal records here.
    CACHE_OBSERVERS.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x583a8 — -[UIWebViewCacheManager baseUrlDidChange:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager baseUrlDidChange:]")]
pub fn stub_583a8() {
    // IDA 0x583a8: `baseUrlDidChange:` rebuilds the preload pages
    // (0x583b4). The rebuild records here.
    stub_583f0();
}

// 0x583b8 — -[UIWebViewCacheManager gotDidLeaveGameNotification:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager gotDidLeaveGameNotification:]")]
pub fn stub_583b8() -> bool {
    // IDA 0x583b8: `gotDidLeaveGameNotification:` preloads when the
    // cache accepts it (0x583d0), else sends the webviews home
    // (0x583ea). The branch reports here.
    let precaching = CACHE_PRECACHING.load(std::sync::atomic::Ordering::SeqCst);
    let initialized = CACHE_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst);
    if stub_585dc(precaching, initialized) {
        true
    } else {
        stub_58858(precaching);
        false
    }
}

// 0x583f0 — -[UIWebViewCacheManager setPagesToPreload]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager setPagesToPreload]")]
pub fn stub_583f0() {
    // IDA 0x583f0: `setPagesToPreload` builds the six home-button page
    // urls (tags 13/11/10/12/15/14, 0x5845a-0x584a6) into
    // `pagesToPreload` (0x584d8). The tags record here.
    *CACHE_PAGES.lock() = vec![13, 11, 10, 12, 15, 14];
}

// 0x584e4 — +[UIWebViewCacheManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UIWebViewCacheManager sharedInstance]")]
pub fn stub_584e4() -> usize {
    // IDA 0x584e4: `sharedInstance` once-allocates the cache manager
    // (same singleton shape as 0x42718). The handle records here as
    // nonzero.
    1
}

// 0x58540 — ___39+[UIWebViewCacheManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39+[UIWebViewCacheManager sharedInstance]_block_invoke")]
pub fn stub_58540() {
    // IDA 0x58540: the `sharedInstance` once block allocs + inits the
    // manager. Allocation is drop glue; no explicit body.
}

// 0x58574 — ___copy_helper_block_55
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_55")]
pub fn stub_58574() {
    // IDA 0x58574: `__copy_helper_block_55` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_56")]
pub fn stub_58580() {
    // IDA 0x58580: `__destroy_helper_block_56` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x58588 — -[UIWebViewCacheManager flush]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager flush]")]
pub fn stub_58588() {
    // IDA 0x58588: `flush` clears + releases the webview map when
    // initialized (0x5859a-0x585d6). The clear records here.
    if CACHE_INITIALIZED.swap(false, std::sync::atomic::Ordering::SeqCst) {
        CACHE_WEBVIEWS.lock().clear();
    }
}

// 0x585dc — -[UIWebViewCacheManager preloadDesignatedWebViews]
// type: char __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager preloadDesignatedWebViews]")]
pub fn stub_585dc(precaching: bool, initialized: bool) -> bool {
    // IDA 0x585dc: `preloadDesignatedWebViews` dispatches the preload
    // block on main when precaching and cold (0x585ee-0x5864c),
    // reporting 1, else 0. The branch records here.
    if precaching && !initialized {
        MAIN_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        true
    } else {
        false
    }
}

// 0x58658 — ___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke")]
pub fn stub_58658() {
    // IDA 0x58658: the preload block builds the designated webviews
    // (dispatched at 0x5864c). Dispatch glue; no explicit body.
}

// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_78")]
pub fn stub_58844() {
    // IDA 0x58844: `__copy_helper_block_78` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_79")]
pub fn stub_58850() {
    // IDA 0x58850: `__destroy_helper_block_79` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x58858 — -[UIWebViewCacheManager designatedWebviewsToHomePages]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager designatedWebviewsToHomePages]")]
pub fn stub_58858(precaching: bool) {
    // IDA 0x58858: `designatedWebviewsToHomePages` dispatches the
    // send-home block on a background queue when precaching
    // (0x5886c-0x588b0). The dispatch records here.
    if precaching {
        BG_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x588b8 — ___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke")]
pub fn stub_588b8() {
    // IDA 0x588b8: the send-home block navigates the designated
    // webviews home (dispatched at 0x588b0). Dispatch glue; no
    // explicit body.
}

// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_83")]
pub fn stub_589f4() {
    // IDA 0x589f4: `__copy_helper_block_83` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_84")]
pub fn stub_58a00() {
    // IDA 0x58a00: `__destroy_helper_block_84` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x58a08 — -[UIWebViewCacheManager getPreloadedWebViewForUrl:]
// type: id __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager getPreloadedWebViewForUrl:]")]
pub fn stub_58a08(url: &str, stale: bool) -> bool {
    // IDA 0x58a08: `getPreloadedWebViewForUrl:` hits the cached
    // webview when initialized + precaching (0x58a22-0x58aa4) and
    // reloads it when the url drifted or it can go back
    // (0x58b30-0x58ba4). The hit records here.
    if !CACHE_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    if !CACHE_PRECACHING.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let hit = CACHE_WEBVIEWS.lock().contains_key(url);
    if hit && stale {
        CACHE_RELOADS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    hit
}

// 0x58bb0 — __GLOBAL__I_a_30
#[doc(alias = "global constructor keyed to_a_30")]
#[doc(alias = "__GLOBAL__I_a_30")]
pub fn stub_58bb0() {
    // IDA 0x58bb0: `__GLOBAL__I_a_30` runs the `a_30`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x58d48 — -[RobloxPageViewController handleStartGameFailure]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameFailure]")]
pub fn stub_58d48() {
    // IDA 0x58d48: `handleStartGameFailure` compiles to an empty body.
    // No explicit body.
}

// 0x58d4c — -[RobloxPageViewController handleStartGameSuccess]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameSuccess]")]
pub fn stub_58d4c() {
    // IDA 0x58d4c: `handleStartGameSuccess` compiles to an empty body.
    // No explicit body.
}

// 0x58d50 — -[RobloxPageViewController initWithCoder:]
// type: RobloxPageViewController *__cdecl(RobloxPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxPageViewController initWithCoder:]")]
pub fn stub_58d50() {
    // IDA 0x58d50: `initWithCoder:` forwards to super (0x58d6a-0x58d7a).
    // Super-init glue; no explicit body.
}

// 0x58d7c — -[RobloxPageViewController viewDidLoad]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController viewDidLoad]")]
pub fn stub_58d7c() {
    // IDA 0x58d7c: `viewDidLoad` supers then registers the UserAgent
    // default (0x58dbc-0x58e18). The registration records here.
    PAGE_UA_REGISTRATIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x58e20 — -[RobloxPageViewController viewWillAppear:]
// type: void __cdecl(RobloxPageViewController *self, SEL, char)
#[doc(alias = "-[RobloxPageViewController viewWillAppear:]")]
pub fn stub_58e20(animated: bool) {
    // IDA 0x58e20: `viewWillAppear:` forwards to super (0x58e3a-0x58e44).
    // Super glue; no explicit body.
    let _ = animated;
}

// 0x58e4c — -[RobloxPageViewController shouldAutorotate]
// type: char __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController shouldAutorotate]")]
pub fn stub_58e4c() -> bool {
    // IDA 0x58e4c: `shouldAutorotate` returns 1 (0x58e4e).
    true
}

// 0x58e50 — -[RobloxPageViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController supportedInterfaceOrientations]")]
pub fn stub_58e50(idiom_known: bool, is_pad: bool) -> u32 {
    // IDA 0x58e50: `supportedInterfaceOrientations` reports 6 without
    // an idiom (0x58e92), 24 on pad (0x58eac) and 6 on phone (0x58eb2).
    if !idiom_known {
        6
    } else if is_pad {
        24
    } else {
        6
    }
}

// 0x58eb8 — -[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(RobloxPageViewController *self, SEL, int)
#[doc(alias = "-[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_58eb8(orientation: i32, is_pad: bool) -> bool {
    // IDA 0x58eb8: `shouldAutorotateToInterfaceOrientation:` allows
    // landscape (3/4) on pad (0x58f1a-0x58f26) and portrait (1/2)
    // otherwise (0x58f2a-0x58f36).
    if is_pad {
        orientation == 3 || orientation == 4
    } else {
        orientation == 1 || orientation == 2
    }
}

// 0x58f40 — -[NSString(Escaping) stringWithPercentEscape]_0
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString(Escaping) stringWithPercentEscape]_0")]
pub fn stub_58f40(raw: &str) -> String {
    // IDA 0x58f40: `stringWithPercentEscape` percent-encodes via
    // `CFURLCreateStringByAddingPercentEscapes` over
    // `\uFFFC=,!$&'()*+;@?\n"<>#\t :/` (0x58f82), i.e. everything
    // outside alphanumerics + `-_.~` escapes as UTF-8 bytes.
    let mut out = String::with_capacity(raw.len());
    for byte in raw.bytes() {
        match byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                out.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    out
}

// 0x58f94 — +[LoginManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginManager sharedInstance]")]
pub fn stub_58f94() -> usize {
    // IDA 0x58f94: `LoginManager::sharedInstance` once-allocates the
    // manager (same singleton shape as 0x42718). The handle records
    // here as nonzero.
    1
}

// 0x58ff0 — ___30+[LoginManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___30+[LoginManager sharedInstance]_block_invoke")]
pub fn stub_58ff0() {
    // IDA 0x58ff0: the `sharedInstance` once block allocs + inits the
    // manager. Allocation is drop glue; no explicit body.
}

// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__18")]
pub fn stub_59024() {
    // IDA 0x59024: `__copy_helper_block__18` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__18")]
pub fn stub_59030() {
    // IDA 0x59030: `__destroy_helper_block__18` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x59038 — -[LoginManager init]
// type: LoginManager *__cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager init]")]
pub fn stub_59038(remember: bool) -> bool {
    // IDA 0x59038: `LoginManager::init` names the failed/successful
    // notifications (0x59086-0x590ee) and loads `rememberMyPassword`
    // (0x590f4-0x5912e). The flag records here.
    REMEMBER_PASSWORD.store(remember, std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x5913c — -[LoginManager dealloc]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager dealloc]")]
pub fn stub_5913c() {
    // IDA 0x5913c: `LoginManager::dealloc` releases the notification
    // names. Release is drop glue; no explicit body.
}

// 0x591a0 — -[LoginManager applicationWillTerminate]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager applicationWillTerminate]")]
pub fn stub_591a0() {
    // IDA 0x591a0: `applicationWillTerminate` persists the remember
    // flag (already mirrored here). Persist glue; no explicit body.
}

// 0x592a0 — -[LoginManager getRememberPassword]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getRememberPassword]")]
pub fn stub_592a0() -> bool {
    // IDA 0x592a0: `getRememberPassword` returns the ivar (0x592ae).
    REMEMBER_PASSWORD.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x592b0 — -[LoginManager setRememberPassword:]
// type: void __cdecl(LoginManager *self, SEL, char)
#[doc(alias = "-[LoginManager setRememberPassword:]")]
pub fn stub_592b0(remember: bool) {
    // IDA 0x592b0: `setRememberPassword:` stores the ivar. It records
    // here.
    REMEMBER_PASSWORD.store(remember, std::sync::atomic::Ordering::SeqCst);
}

// 0x594e4 — -[LoginManager getLoginFailedNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginFailedNotification]")]
pub fn stub_594e4() -> &'static str {
    // IDA 0x594e4: `getLoginFailedNotification` returns the init name
    // (0x59086-0x590b2). The name reports here.
    "RBXLoginFailedNotifier"
}

// 0x594f4 — -[LoginManager getLoginSuccessfulNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginSuccessfulNotification]")]
pub fn stub_594f4() -> &'static str {
    // IDA 0x594f4: `getLoginSuccessfulNotification` returns the init
    // name (0x590ba-0x590ee). The name reports here.
    "RBXLoginSuccessfulNotifier"
}

// 0x59504 — -[LoginManager updateUserInfo:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager updateUserInfo:password:]")]
pub fn stub_59504(info: &LoginUserInfo, password: &str) {
    // IDA 0x59504: `updateUserInfo:password:` fills the current player
    // (id, name, balances, thumbnail, BC flag at
    // 0x5952c-0x5968c) with the given password. The fields record
    // here.
    let mut user = CURRENT_USER.lock();
    user.user_id = info.user_id.clone();
    user.username = info.username.clone();
    user.password = password.to_owned();
    user.robux_balance = info.robux_balance.clone();
    user.tickets_balance = info.tickets_balance.clone();
    user.thumbnail_url = info.thumbnail_url.clone();
    user.bc_member = info.bc_member.clone();
}

// 0x59690 — -[LoginManager isConnectedToInternet]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager isConnectedToInternet]")]
pub fn stub_59690(status: u32) -> bool {
    // IDA 0x59690: `isConnectedToInternet` reports reachable on WWAN
    // (0x59706) / wifi (0x59734) and posts the failed notification on
    // offline (0x597e8-0x59896). The verdict records here.
    if status == 0 {
        LOGIN_FAILED_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        false
    } else {
        true
    }
}

// 0x598e4 — -[LoginManager doLogout]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager doLogout]")]
pub fn stub_598e4(connected: bool, base_url: &str) -> Option<String> {
    // IDA 0x598e4: `doLogout` skips offline (0x598fe) and posts
    // `{base}mobileapi/logout` async (0x59922-0x59a5e). The posted url
    // reports here.
    if !connected {
        return None;
    }
    LOGOUT_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    Some(format!("{base_url}mobileapi/logout"))
}

// 0x59a6c — ___24-[LoginManager doLogout]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___24-[LoginManager doLogout]_block_invoke")]
pub fn stub_59a6c() {
    // IDA 0x59a6c: the logout-reply block handles the POST reply
    // (registered at 0x59a5e). Completion glue; no explicit body.
}

// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_149")]
pub fn stub_59aa8() {
    // IDA 0x59aa8: `__copy_helper_block_149` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_150")]
pub fn stub_59acc() {
    // IDA 0x59acc: `__destroy_helper_block_150` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x59ae8 — -[LoginManager doLoginWithUsername:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager doLoginWithUsername:password:]")]
pub fn stub_59ae8(username: &str, password: &str, connected: bool, base_url: &str) -> Option<String> {
    // IDA 0x59ae8: `doLoginWithUsername:password:` skips offline, then
    // posts percent-escaped `username=<u>&password=<p>` to
    // `{base}mobileapi/login` over https (0x59b6e-0x59e50). The posted
    // body reports here.
    if !connected {
        return None;
    }
    LOGIN_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let _url = format!("{}mobileapi/login", base_url.replacen("http:", "https:", 1));
    Some(format!(
        "username={}&password={}",
        stub_58f40(username),
        stub_58f40(password)
    ))
}

// 0x59ecc — ___45-[LoginManager doLoginWithUsername:password:]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___45-[LoginManager doLoginWithUsername:password:]_block_invoke")]
pub fn stub_59ecc() {
    // IDA 0x59ecc: the login-reply block handles the POST reply
    // (registered at 0x59e50). Completion glue; no explicit body.
}

// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_192")]
pub fn stub_5a068() {
    // IDA 0x5a068: `__copy_helper_block_192` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x5a0b0 — ___destroy_helper_block_193
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_193")]
pub fn stub_5a0b0() -> ! {
    todo!("0x5a0b0 ___destroy_helper_block_193")
}

// 0x5a0e4 — -[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id, id)
#[doc(alias = "-[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]")]
pub fn stub_5a0e4() -> ! {
    todo!("0x5a0e4 -[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]")
}

// 0x5a42c — -[LoginManager processLogOutResponse:logoutData:logoutError:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id)
#[doc(alias = "-[LoginManager processLogOutResponse:logoutData:logoutError:]")]
pub fn stub_5a42c() -> ! {
    todo!("0x5a42c -[LoginManager processLogOutResponse:logoutData:logoutError:]")
}

// 0x5a6a8 — -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")]
pub fn stub_5a6a8() -> ! {
    todo!("0x5a6a8 -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")
}

// 0x5ac78 — -[LoginManager processSuccessfulLogoutResponse:httpResponse:]
// type: id __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLogoutResponse:httpResponse:]")]
pub fn stub_5ac78() -> ! {
    todo!("0x5ac78 -[LoginManager processSuccessfulLogoutResponse:httpResponse:]")
}

// 0x5ae50 — -[LoginManager processFailureLoginResponse:]
// type: id __cdecl(LoginManager *self, SEL, id)
#[doc(alias = "-[LoginManager processFailureLoginResponse:]")]
pub fn stub_5ae50() -> ! {
    todo!("0x5ae50 -[LoginManager processFailureLoginResponse:]")
}

// 0x5b150 — -[LoginManager processFailureLogoutResponse:]
// type: id __cdecl(LoginManager *self, SEL, id)
#[doc(alias = "-[LoginManager processFailureLogoutResponse:]")]
pub fn stub_5b150() -> ! {
    todo!("0x5b150 -[LoginManager processFailureLogoutResponse:]")
}

// 0x5b3d8 — __GLOBAL__I_a_31
#[doc(alias = "global constructor keyed to_a_31")]
#[doc(alias = "__GLOBAL__I_a_31")]
pub fn stub_5b3d8() -> ! {
    todo!("0x5b3d8 global constructor keyed to_a_31")
}

// 0x5b4a0 — -[AgreementController initWithCoder:]
// type: AgreementController *__cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController initWithCoder:]")]
pub fn stub_5b4a0() -> ! {
    todo!("0x5b4a0 -[AgreementController initWithCoder:]")
}

// 0x5b4e0 — -[AgreementController init:]
// type: id __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController init:]")]
pub fn stub_5b4e0() -> ! {
    todo!("0x5b4e0 -[AgreementController init:]")
}

// 0x5b550 — -[AgreementController init:newFrame:]
// type: id __cdecl(AgreementController *self, SEL, id, CGRect)
#[doc(alias = "-[AgreementController init:newFrame:]")]
pub fn stub_5b550() -> ! {
    todo!("0x5b550 -[AgreementController init:newFrame:]")
}

// 0x5b5fc — -[AgreementController dealloc]
// type: void __cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController dealloc]")]
pub fn stub_5b5fc() -> ! {
    todo!("0x5b5fc -[AgreementController dealloc]")
}

// 0x5b680 — -[AgreementController setUrl:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setUrl:]")]
pub fn stub_5b680() -> ! {
    todo!("0x5b680 -[AgreementController setUrl:]")
}

// 0x5b690 — -[AgreementController cancelTouch:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController cancelTouch:]")]
pub fn stub_5b690() -> ! {
    todo!("0x5b690 -[AgreementController cancelTouch:]")
}

// 0x5b6a4 — -[AgreementController viewDidLoad]
// type: void __cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController viewDidLoad]")]
pub fn stub_5b6a4() -> ! {
    todo!("0x5b6a4 -[AgreementController viewDidLoad]")
}

// 0x5ba90 — -[AgreementController toolBar]
// type: UIToolbar *__cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController toolBar]")]
pub fn stub_5ba90() -> ! {
    todo!("0x5ba90 -[AgreementController toolBar]")
}

// 0x5baa0 — -[AgreementController setToolBar:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setToolBar:]")]
pub fn stub_5baa0() -> ! {
    todo!("0x5baa0 -[AgreementController setToolBar:]")
}

// 0x5bac4 — -[AgreementController closeButton]
// type: UIBarButtonItem *__cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController closeButton]")]
pub fn stub_5bac4() -> ! {
    todo!("0x5bac4 -[AgreementController closeButton]")
}

// 0x5bad4 — -[AgreementController setCloseButton:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setCloseButton:]")]
pub fn stub_5bad4() -> ! {
    todo!("0x5bad4 -[AgreementController setCloseButton:]")
}

// 0x5baf8 — -[SignUpErrorViewController initWithCoder:]
// type: SignUpErrorViewController *__cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController initWithCoder:]")]
pub fn stub_5baf8() -> ! {
    todo!("0x5baf8 -[SignUpErrorViewController initWithCoder:]")
}

// 0x5bb44 — -[SignUpErrorViewController dealloc]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController dealloc]")]
pub fn stub_5bb44() -> ! {
    todo!("0x5bb44 -[SignUpErrorViewController dealloc]")
}

// 0x5bc00 — -[SignUpErrorViewController viewDidLoad]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController viewDidLoad]")]
pub fn stub_5bc00() -> ! {
    todo!("0x5bc00 -[SignUpErrorViewController viewDidLoad]")
}

// 0x5bcb8 — -[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id, id, void *)
#[doc(alias = "-[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]")]
pub fn stub_5bcb8() -> ! {
    todo!("0x5bcb8 -[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]")
}

// 0x5bd70 — -[SignUpErrorViewController didReceiveMemoryWarning]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController didReceiveMemoryWarning]")]
pub fn stub_5bd70() -> ! {
    todo!("0x5bd70 -[SignUpErrorViewController didReceiveMemoryWarning]")
}

// 0x5bd9c — -[SignUpErrorViewController setSuggestedUsername:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setSuggestedUsername:]")]
pub fn stub_5bd9c() -> ! {
    todo!("0x5bd9c -[SignUpErrorViewController setSuggestedUsername:]")
}

// 0x5bdbc — -[SignUpErrorViewController setMessage:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setMessage:]")]
pub fn stub_5bdbc() -> ! {
    todo!("0x5bdbc -[SignUpErrorViewController setMessage:]")
}

// 0x5be1c — -[SignUpErrorViewController setSignupController:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setSignupController:]")]
pub fn stub_5be1c() -> ! {
    todo!("0x5be1c -[SignUpErrorViewController setSignupController:]")
}

// 0x5be2c — -[SignUpErrorViewController touchesBegan:withEvent:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id)
#[doc(alias = "-[SignUpErrorViewController touchesBegan:withEvent:]")]
pub fn stub_5be2c() -> ! {
    todo!("0x5be2c -[SignUpErrorViewController touchesBegan:withEvent:]")
}

// 0x5be5c — -[SignUpErrorViewController touchesEnded:withEvent:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id)
#[doc(alias = "-[SignUpErrorViewController touchesEnded:withEvent:]")]
pub fn stub_5be5c() -> ! {
    todo!("0x5be5c -[SignUpErrorViewController touchesEnded:withEvent:]")
}

// 0x5bf68 — -[SignUpErrorViewController messageTextView]
// type: UITextView *__cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController messageTextView]")]
pub fn stub_5bf68() -> ! {
    todo!("0x5bf68 -[SignUpErrorViewController messageTextView]")
}

// 0x5bf78 — -[SignUpErrorViewController setMessageTextView:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setMessageTextView:]")]
pub fn stub_5bf78() -> ! {
    todo!("0x5bf78 -[SignUpErrorViewController setMessageTextView:]")
}

// 0x5bf9c — -[SignupVerifier init]
// type: SignupVerifier *__cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier init]")]
pub fn stub_5bf9c() -> ! {
    todo!("0x5bf9c -[SignupVerifier init]")
}

// 0x5c17c — -[SignupVerifier dealloc]
// type: void __cdecl(SignupVerifier *self, SEL)
#[doc(alias = "-[SignupVerifier dealloc]")]
pub fn stub_5c17c() -> ! {
    todo!("0x5c17c -[SignupVerifier dealloc]")
}

// 0x5c26c — -[SignupVerifier isValidEmail:]
// type: bool __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier isValidEmail:]")]
pub fn stub_5c26c() -> ! {
    todo!("0x5c26c -[SignupVerifier isValidEmail:]")
}

// 0x5c2e8 — -[SignupVerifier doPostResponseFromUrl:args:notificationName:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id, id)
#[doc(alias = "-[SignupVerifier doPostResponseFromUrl:args:notificationName:]")]
pub fn stub_5c2e8() -> ! {
    todo!("0x5c2e8 -[SignupVerifier doPostResponseFromUrl:args:notificationName:]")
}

// 0x5c444 — ___62-[SignupVerifier doPostResponseFromUrl:args:notificationName:]_block_invoke
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "___62-[SignupVerifier doPostResponseFromUrl:args:notificationName:]_block_invoke")]
pub fn stub_5c444() -> ! {
    todo!("0x5c444 ___62-[SignupVerifier doPostResponseFromUrl:args:notificationName:]_block_invoke")
}

// 0x5c4f4 — ___copy_helper_block__19
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__19")]
pub fn stub_5c4f4() -> ! {
    todo!("0x5c4f4 ___copy_helper_block__19")
}

// 0x5c518 — ___destroy_helper_block__19
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__19")]
pub fn stub_5c518() -> ! {
    todo!("0x5c518 ___destroy_helper_block__19")
}

// 0x5c534 — -[SignupVerifier doGetResponseFromUrl:notificationName:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id)
#[doc(alias = "-[SignupVerifier doGetResponseFromUrl:notificationName:]")]
pub fn stub_5c534() -> ! {
    todo!("0x5c534 -[SignupVerifier doGetResponseFromUrl:notificationName:]")
}

// 0x5c658 — ___56-[SignupVerifier doGetResponseFromUrl:notificationName:]_block_invoke
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "___56-[SignupVerifier doGetResponseFromUrl:notificationName:]_block_invoke")]
pub fn stub_5c658() -> ! {
    todo!("0x5c658 ___56-[SignupVerifier doGetResponseFromUrl:notificationName:]_block_invoke")
}

// 0x5c6c8 — ___copy_helper_block_104
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_104")]
pub fn stub_5c6c8() -> ! {
    todo!("0x5c6c8 ___copy_helper_block_104")
}

// 0x5c6ec — ___destroy_helper_block_105
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_105")]
pub fn stub_5c6ec() -> ! {
    todo!("0x5c6ec ___destroy_helper_block_105")
}

// 0x5c708 — -[SignupVerifier checkPassword:username:]
// type: void __cdecl(SignupVerifier *self, SEL, id, id)
#[doc(alias = "-[SignupVerifier checkPassword:username:]")]
pub fn stub_5c708() -> ! {
    todo!("0x5c708 -[SignupVerifier checkPassword:username:]")
}

// 0x5c77c — -[SignupVerifier checkUsername:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier checkUsername:]")]
pub fn stub_5c77c() -> ! {
    todo!("0x5c77c -[SignupVerifier checkUsername:]")
}

// 0x5c888 — -[SignupVerifier getAlternateUsername:]
// type: void __cdecl(SignupVerifier *self, SEL, id)
#[doc(alias = "-[SignupVerifier getAlternateUsername:]")]
pub fn stub_5c888() -> ! {
    todo!("0x5c888 -[SignupVerifier getAlternateUsername:]")
}
