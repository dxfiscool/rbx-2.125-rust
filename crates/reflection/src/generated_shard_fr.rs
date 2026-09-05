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
pub fn stub_58334() -> ! {
    todo!("0x58334 ___copy_helper_block__17")
}

// 0x58340 — ___destroy_helper_block__17
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__17")]
pub fn stub_58340() -> ! {
    todo!("0x58340 ___destroy_helper_block__17")
}

// 0x58348 — -[UIWebViewCacheManager dealloc]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager dealloc]")]
pub fn stub_58348() -> ! {
    todo!("0x58348 -[UIWebViewCacheManager dealloc]")
}

// 0x583a8 — -[UIWebViewCacheManager baseUrlDidChange:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager baseUrlDidChange:]")]
pub fn stub_583a8() -> ! {
    todo!("0x583a8 -[UIWebViewCacheManager baseUrlDidChange:]")
}

// 0x583b8 — -[UIWebViewCacheManager gotDidLeaveGameNotification:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager gotDidLeaveGameNotification:]")]
pub fn stub_583b8() -> ! {
    todo!("0x583b8 -[UIWebViewCacheManager gotDidLeaveGameNotification:]")
}

// 0x583f0 — -[UIWebViewCacheManager setPagesToPreload]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager setPagesToPreload]")]
pub fn stub_583f0() -> ! {
    todo!("0x583f0 -[UIWebViewCacheManager setPagesToPreload]")
}

// 0x584e4 — +[UIWebViewCacheManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UIWebViewCacheManager sharedInstance]")]
pub fn stub_584e4() -> ! {
    todo!("0x584e4 +[UIWebViewCacheManager sharedInstance]")
}

// 0x58540 — ___39+[UIWebViewCacheManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39+[UIWebViewCacheManager sharedInstance]_block_invoke")]
pub fn stub_58540() -> ! {
    todo!("0x58540 ___39+[UIWebViewCacheManager sharedInstance]_block_invoke")
}

// 0x58574 — ___copy_helper_block_55
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_55")]
pub fn stub_58574() -> ! {
    todo!("0x58574 ___copy_helper_block_55")
}

// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_56")]
pub fn stub_58580() -> ! {
    todo!("0x58580 ___destroy_helper_block_56")
}

// 0x58588 — -[UIWebViewCacheManager flush]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager flush]")]
pub fn stub_58588() -> ! {
    todo!("0x58588 -[UIWebViewCacheManager flush]")
}

// 0x585dc — -[UIWebViewCacheManager preloadDesignatedWebViews]
// type: char __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager preloadDesignatedWebViews]")]
pub fn stub_585dc() -> ! {
    todo!("0x585dc -[UIWebViewCacheManager preloadDesignatedWebViews]")
}

// 0x58658 — ___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke")]
pub fn stub_58658() -> ! {
    todo!("0x58658 ___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke")
}

// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_78")]
pub fn stub_58844() -> ! {
    todo!("0x58844 ___copy_helper_block_78")
}

// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_79")]
pub fn stub_58850() -> ! {
    todo!("0x58850 ___destroy_helper_block_79")
}

// 0x58858 — -[UIWebViewCacheManager designatedWebviewsToHomePages]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager designatedWebviewsToHomePages]")]
pub fn stub_58858() -> ! {
    todo!("0x58858 -[UIWebViewCacheManager designatedWebviewsToHomePages]")
}

// 0x588b8 — ___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke")]
pub fn stub_588b8() -> ! {
    todo!("0x588b8 ___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke")
}

// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_83")]
pub fn stub_589f4() -> ! {
    todo!("0x589f4 ___copy_helper_block_83")
}

// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_84")]
pub fn stub_58a00() -> ! {
    todo!("0x58a00 ___destroy_helper_block_84")
}

// 0x58a08 — -[UIWebViewCacheManager getPreloadedWebViewForUrl:]
// type: id __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager getPreloadedWebViewForUrl:]")]
pub fn stub_58a08() -> ! {
    todo!("0x58a08 -[UIWebViewCacheManager getPreloadedWebViewForUrl:]")
}

// 0x58bb0 — __GLOBAL__I_a_30
#[doc(alias = "global constructor keyed to_a_30")]
#[doc(alias = "__GLOBAL__I_a_30")]
pub fn stub_58bb0() -> ! {
    todo!("0x58bb0 global constructor keyed to_a_30")
}

// 0x58d48 — -[RobloxPageViewController handleStartGameFailure]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameFailure]")]
pub fn stub_58d48() -> ! {
    todo!("0x58d48 -[RobloxPageViewController handleStartGameFailure]")
}

// 0x58d4c — -[RobloxPageViewController handleStartGameSuccess]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameSuccess]")]
pub fn stub_58d4c() -> ! {
    todo!("0x58d4c -[RobloxPageViewController handleStartGameSuccess]")
}

// 0x58d50 — -[RobloxPageViewController initWithCoder:]
// type: RobloxPageViewController *__cdecl(RobloxPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxPageViewController initWithCoder:]")]
pub fn stub_58d50() -> ! {
    todo!("0x58d50 -[RobloxPageViewController initWithCoder:]")
}

// 0x58d7c — -[RobloxPageViewController viewDidLoad]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController viewDidLoad]")]
pub fn stub_58d7c() -> ! {
    todo!("0x58d7c -[RobloxPageViewController viewDidLoad]")
}

// 0x58e20 — -[RobloxPageViewController viewWillAppear:]
// type: void __cdecl(RobloxPageViewController *self, SEL, char)
#[doc(alias = "-[RobloxPageViewController viewWillAppear:]")]
pub fn stub_58e20() -> ! {
    todo!("0x58e20 -[RobloxPageViewController viewWillAppear:]")
}

// 0x58e4c — -[RobloxPageViewController shouldAutorotate]
// type: char __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController shouldAutorotate]")]
pub fn stub_58e4c() -> ! {
    todo!("0x58e4c -[RobloxPageViewController shouldAutorotate]")
}

// 0x58e50 — -[RobloxPageViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController supportedInterfaceOrientations]")]
pub fn stub_58e50() -> ! {
    todo!("0x58e50 -[RobloxPageViewController supportedInterfaceOrientations]")
}

// 0x58eb8 — -[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(RobloxPageViewController *self, SEL, int)
#[doc(alias = "-[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_58eb8() -> ! {
    todo!("0x58eb8 -[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x58f40 — -[NSString(Escaping) stringWithPercentEscape]_0
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString(Escaping) stringWithPercentEscape]_0")]
pub fn stub_58f40() -> ! {
    todo!("0x58f40 -[NSString(Escaping) stringWithPercentEscape]_0")
}

// 0x58f94 — +[LoginManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginManager sharedInstance]")]
pub fn stub_58f94() -> ! {
    todo!("0x58f94 +[LoginManager sharedInstance]")
}

// 0x58ff0 — ___30+[LoginManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___30+[LoginManager sharedInstance]_block_invoke")]
pub fn stub_58ff0() -> ! {
    todo!("0x58ff0 ___30+[LoginManager sharedInstance]_block_invoke")
}

// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__18")]
pub fn stub_59024() -> ! {
    todo!("0x59024 ___copy_helper_block__18")
}

// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__18")]
pub fn stub_59030() -> ! {
    todo!("0x59030 ___destroy_helper_block__18")
}

// 0x59038 — -[LoginManager init]
// type: LoginManager *__cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager init]")]
pub fn stub_59038() -> ! {
    todo!("0x59038 -[LoginManager init]")
}

// 0x5913c — -[LoginManager dealloc]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager dealloc]")]
pub fn stub_5913c() -> ! {
    todo!("0x5913c -[LoginManager dealloc]")
}

// 0x591a0 — -[LoginManager applicationWillTerminate]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager applicationWillTerminate]")]
pub fn stub_591a0() -> ! {
    todo!("0x591a0 -[LoginManager applicationWillTerminate]")
}

// 0x592a0 — -[LoginManager getRememberPassword]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getRememberPassword]")]
pub fn stub_592a0() -> ! {
    todo!("0x592a0 -[LoginManager getRememberPassword]")
}

// 0x592b0 — -[LoginManager setRememberPassword:]
// type: void __cdecl(LoginManager *self, SEL, char)
#[doc(alias = "-[LoginManager setRememberPassword:]")]
pub fn stub_592b0() -> ! {
    todo!("0x592b0 -[LoginManager setRememberPassword:]")
}

// 0x594e4 — -[LoginManager getLoginFailedNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginFailedNotification]")]
pub fn stub_594e4() -> ! {
    todo!("0x594e4 -[LoginManager getLoginFailedNotification]")
}

// 0x594f4 — -[LoginManager getLoginSuccessfulNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginSuccessfulNotification]")]
pub fn stub_594f4() -> ! {
    todo!("0x594f4 -[LoginManager getLoginSuccessfulNotification]")
}

// 0x59504 — -[LoginManager updateUserInfo:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager updateUserInfo:password:]")]
pub fn stub_59504() -> ! {
    todo!("0x59504 -[LoginManager updateUserInfo:password:]")
}

// 0x59690 — -[LoginManager isConnectedToInternet]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager isConnectedToInternet]")]
pub fn stub_59690() -> ! {
    todo!("0x59690 -[LoginManager isConnectedToInternet]")
}

// 0x598e4 — -[LoginManager doLogout]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager doLogout]")]
pub fn stub_598e4() -> ! {
    todo!("0x598e4 -[LoginManager doLogout]")
}

// 0x59a6c — ___24-[LoginManager doLogout]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___24-[LoginManager doLogout]_block_invoke")]
pub fn stub_59a6c() -> ! {
    todo!("0x59a6c ___24-[LoginManager doLogout]_block_invoke")
}

// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_149")]
pub fn stub_59aa8() -> ! {
    todo!("0x59aa8 ___copy_helper_block_149")
}

// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_150")]
pub fn stub_59acc() -> ! {
    todo!("0x59acc ___destroy_helper_block_150")
}

// 0x59ae8 — -[LoginManager doLoginWithUsername:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager doLoginWithUsername:password:]")]
pub fn stub_59ae8() -> ! {
    todo!("0x59ae8 -[LoginManager doLoginWithUsername:password:]")
}

// 0x59ecc — ___45-[LoginManager doLoginWithUsername:password:]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___45-[LoginManager doLoginWithUsername:password:]_block_invoke")]
pub fn stub_59ecc() -> ! {
    todo!("0x59ecc ___45-[LoginManager doLoginWithUsername:password:]_block_invoke")
}

// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_192")]
pub fn stub_5a068() -> ! {
    todo!("0x5a068 ___copy_helper_block_192")
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
