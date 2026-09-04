//! `iOSSettingsService` — key→reader settings map (IDA 0x21ce0..0x24258).
//! Mirrors the C++ service: `Init` registers each key with its reader and
//! default; each `ReadValue*` parses the value string (`atoi` for ints,
//! `SimpleJSON::ParseBool` for bools, assign for strings) into the
//! singleton. `std::map` becomes `HashMap`; `std::string` becomes `String`.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

/// Parses like `atoi`: optional sign plus a leading run of ASCII digits;
/// anything unparseable is 0.
fn parse_int(text: &str) -> i32 {
    let t = text.trim_start();
    let (sign, rest) = match t.strip_prefix('-') {
        Some(r) => (-1i64, r),
        None => (1i64, t.strip_prefix('+').unwrap_or(t)),
    };
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return 0;
    }
    sign.saturating_mul(digits.parse::<i64>().unwrap_or(0)) as i32
}

/// Host reading of `SimpleJSON::ParseBool`.
fn parse_bool(text: &str) -> bool {
    matches!(text.trim().to_ascii_lowercase().as_str(), "true" | "1" | "yes")
}

/// `iOSSettingsService` singleton state (IDA 0x21ce0..0x24258).
#[derive(Debug, Default)]
pub struct IosSettingsService {
    init_calls: std::sync::atomic::AtomicU32,
    readers: parking_lot::Mutex<HashMap<&'static str, &'static str>>,
    pub ipad_minimum_version: AtomicI32,
    pub ipad_maximum_version: AtomicI32,
    pub iphone_minimum_version: AtomicI32,
    pub iphone_maximum_version: AtomicI32,
    pub ipod_minimum_version: AtomicI32,
    pub ipod_maximum_version: AtomicI32,
    pub disable_play_button_for_all: AtomicBool,
    pub disable_play_button_for_non_bc: AtomicBool,
    pub ipad1_maximum_ideal_parts: AtomicI32,
    pub ipad2_maximum_ideal_parts: AtomicI32,
    pub ipad3_maximum_ideal_parts: AtomicI32,
    pub ipad4_maximum_ideal_parts: AtomicI32,
    pub ipod4_maximum_ideal_parts: AtomicI32,
    pub ipod5_maximum_ideal_parts: AtomicI32,
    pub iphone4s_maximum_ideal_parts: AtomicI32,
    pub iphone5_maximum_ideal_parts: AtomicI32,
    pub time_interval_between_robux_purchase_in_minutes: AtomicI32,
    pub time_interval_between_bc_purchase_in_minutes: AtomicI32,
    pub time_interval_between_catalog_purchase_in_minutes: AtomicI32,
    pub time_limit_for_billing_service_retries_before_giving_up: AtomicI32,
    pub test_flight_logging_level: AtomicI32,
    pub test_flight_percentage: AtomicI32,
    pub bug_sense_percentage: AtomicI32,
    pub bug_sense_log_lines: AtomicI32,
    pub bug_sense_log_level: AtomicI32,
    pub ios_google_analytics_account2: parking_lot::Mutex<String>,
    pub ios_google_analytics_sample_rate: AtomicI32,
    pub search_endpoint_ipad: parking_lot::Mutex<String>,
    pub search_endpoint_iphone: parking_lot::Mutex<String>,
    pub cache_ui_web_views: AtomicBool,
    pub thumbstick_control_style: AtomicI32,
    pub free_memory_checker_active: AtomicBool,
    pub free_memory_checker_rate_milli_seconds: AtomicI32,
    pub free_memory_checker_threshold_kilo_bytes: AtomicI32,
    pub memory_bouncer_active: AtomicBool,
    pub memory_bouncer_enforce_rate_milli_seconds: AtomicI32,
    pub memory_bouncer_threshold_kilo_bytes: AtomicI32,
    pub memory_bouncer_limit_mega_bytes: AtomicI32,
    pub memory_bouncer_limit_mega_bytes_for_low_mem_devices: AtomicI32,
}

impl IosSettingsService {
    fn shared() -> &'static Self {
        static SERVICE: std::sync::LazyLock<IosSettingsService> =
            std::sync::LazyLock::new(IosSettingsService::default);
        &SERVICE
    }

    // 0x21ce0 — iOSSettingsService::Init(void)
    // mangled: __ZN18iOSSettingsService4InitEv
    // IDA 0x21ce0
    #[doc(alias = "iOSSettingsService::Init(void)")]
    pub fn init() {
        // Registers all 40 `key -> ReadValue*` map entries and their
        // defaults (IDA 0x21d0c..0x227f0); `std::string` temporaries have no
        // host counterpart. Verified via IDA decompile.
        let this = Self::shared();
        this.init_calls.fetch_add(1, Ordering::SeqCst);
        let mut readers = this.readers.lock();
        readers.clear();
        // (key, reader, int_default): `var3/5/7 = 1` minimums
        // (IDA 0x21d0c..0x21e9c), `var21 = 10`, `var22 = 1440`, `var23 = 10`
        // (IDA 0x2218c..0x22216), `var24 = 48` (IDA 0x2225a),
        // `var25 = 4`, `var26 = 100`, `var27 = 0`, `var28 = 20`
        // (IDA 0x2229e..0x2236a), `var29 = 8` (IDA 0x223ae),
        // `var20 = 100` (IDA 0x2244c), `var33 = 1` (IDA 0x22586),
        // `var35 = 10000`, `var36 = 20480` (IDA 0x22614..0x2265c),
        // `var38 = 100`, `var39 = 5120`, `var40 = 250`
        // (IDA 0x226e8..0x22776), low-mem limit 0 (IDA 0x227bc).
        for (key, reader) in [
            ("iPadMinimumVersion", "ReadValueiPadMinimumVersion"),
            ("iPadMaximumVersion", "ReadValueiPadMaximumVersion"),
            ("iPhoneMinimumVersion", "ReadValueiPhoneMinimumVersion"),
            ("iPhoneMaximumVersion", "ReadValueiPhoneMaximumVersion"),
            ("iPodMinimumVersion", "ReadValueiPodMinimumVersion"),
            ("iPodMaximumVersion", "ReadValueiPodMaximumVersion"),
            ("DisablePlayButtonForAll", "ReadValueDisablePlayButtonForAll"),
            ("DisablePlayButtonForNonBC", "ReadValueDisablePlayButtonForNonBC"),
            ("iPad1_MaximumIdealParts", "ReadValueiPad1_MaximumIdealParts"),
            ("iPad2_MaximumIdealParts", "ReadValueiPad2_MaximumIdealParts"),
            ("iPad3_MaximumIdealParts", "ReadValueiPad3_MaximumIdealParts"),
            ("iPad4_MaximumIdealParts", "ReadValueiPad4_MaximumIdealParts"),
            ("iPod4_MaximumIdealParts", "ReadValueiPod4_MaximumIdealParts"),
            ("iPod5_MaximumIdealParts", "ReadValueiPod5_MaximumIdealParts"),
            ("iPhone4s_MaximumIdealParts", "ReadValueiPhone4s_MaximumIdealParts"),
            ("iPhone5_MaximumIdealParts", "ReadValueiPhone5_MaximumIdealParts"),
            ("TimeIntervalBetweenRobuxPurchaseInMinutes", "ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes"),
            ("TimeIntervalBetweenBCPurchaseInMinutes", "ReadValueTimeIntervalBetweenBCPurchaseInMinutes"),
            ("TimeIntervalBetweenCatalogPurchaseInMinutes", "ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes"),
            ("TimeLimitForBillingServiceRetriesBeforeGivingUp", "ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp"),
            ("TestFlightLoggingLevel", "ReadValueTestFlightLoggingLevel"),
            ("TestFlightPercentage", "ReadValueTestFlightPercentage"),
            ("BugSensePercentage", "ReadValueBugSensePercentage"),
            ("BugSenseLogLines", "ReadValueBugSenseLogLines"),
            ("BugSenseLogLevel", "ReadValueBugSenseLogLevel"),
            ("iOSGoogleAnalyticsAccount2", "ReadValueiOSGoogleAnalyticsAccount2"),
            ("iOSGoogleAnalyticsSampleRate", "ReadValueiOSGoogleAnalyticsSampleRate"),
            ("SearchEndpointIPad", "ReadValueSearchEndpointIPad"),
            ("SearchEndpointIPhone", "ReadValueSearchEndpointIPhone"),
            ("CacheUIWebViews", "ReadValueCacheUIWebViews"),
            ("ThumbstickControlStyle", "ReadValueThumbstickControlStyle"),
            ("FreeMemoryCheckerActive", "ReadValueFreeMemoryCheckerActive"),
            ("FreeMemoryCheckerRateMilliSeconds", "ReadValueFreeMemoryCheckerRateMilliSeconds"),
            ("FreeMemoryCheckerThresholdKiloBytes", "ReadValueFreeMemoryCheckerThresholdKiloBytes"),
            ("MemoryBouncerActive", "ReadValueMemoryBouncerActive"),
            ("MemoryBouncerEnforceRateMilliSeconds", "ReadValueMemoryBouncerEnforceRateMilliSeconds"),
            ("MemoryBouncerThresholdKiloBytes", "ReadValueMemoryBouncerThresholdKiloBytes"),
            ("MemoryBouncerLimitMegaBytes", "ReadValueMemoryBouncerLimitMegaBytes"),
            ("MemoryBouncerLimitMegaBytesForLowMemDevices", "ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices"),
        ] {
            readers.insert(key, reader);
        }
        drop(readers);
        this.ipad_minimum_version.store(1, Ordering::SeqCst);
        this.iphone_minimum_version.store(1, Ordering::SeqCst);
        this.ipod_minimum_version.store(1, Ordering::SeqCst);
        this.time_interval_between_robux_purchase_in_minutes.store(10, Ordering::SeqCst);
        this.time_interval_between_bc_purchase_in_minutes.store(1440, Ordering::SeqCst);
        this.time_interval_between_catalog_purchase_in_minutes.store(10, Ordering::SeqCst);
        this.time_limit_for_billing_service_retries_before_giving_up.store(48, Ordering::SeqCst);
        this.test_flight_logging_level.store(4, Ordering::SeqCst);
        this.test_flight_percentage.store(100, Ordering::SeqCst);
        this.bug_sense_log_lines.store(20, Ordering::SeqCst);
        this.bug_sense_log_level.store(8, Ordering::SeqCst);
        this.ios_google_analytics_sample_rate.store(100, Ordering::SeqCst);
        this.thumbstick_control_style.store(1, Ordering::SeqCst);
        this.free_memory_checker_rate_milli_seconds.store(10000, Ordering::SeqCst);
        this.free_memory_checker_threshold_kilo_bytes.store(20480, Ordering::SeqCst);
        this.memory_bouncer_enforce_rate_milli_seconds.store(100, Ordering::SeqCst);
        this.memory_bouncer_threshold_kilo_bytes.store(5120, Ordering::SeqCst);
        this.memory_bouncer_limit_mega_bytes.store(250, Ordering::SeqCst);
    }

    pub fn init_call_count() -> u32 {
        Self::shared().init_calls.load(Ordering::SeqCst)
    }

    pub fn reader_for(key: &str) -> Option<&'static str> {
        Self::shared().readers.lock().get(key).copied()
    }

    // 0x239ec — iOSSettingsService::ReadValueiPadMinimumVersion(char const*)
    // mangled: __ZN18iOSSettingsService27ReadValueiPadMinimumVersionEPKc
    // IDA 0x239ec
    #[doc(alias = "iOSSettingsService::ReadValueiPadMinimumVersion(char const*)")]
    pub fn read_value_ipad_minimum_version(text: &str) -> i32 {
        // `atoi` into `_thisPtr + 28` (IDA 0x239f0..0x23a00).
        // Verified via IDA decompile.
        let value = parse_int(text);
        Self::shared().ipad_minimum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23b50 — iOSSettingsService::ReadValueiPadMaximumVersion(char const*)
    // mangled: __ZN18iOSSettingsService27ReadValueiPadMaximumVersionEPKc
    // IDA 0x23b50
    #[doc(alias = "iOSSettingsService::ReadValueiPadMaximumVersion(char const*)")]
    pub fn read_value_ipad_maximum_version(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x239ec).
        let value = parse_int(text);
        Self::shared().ipad_maximum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23b68 — iOSSettingsService::ReadValueiPhoneMinimumVersion(char const*)
    // mangled: __ZN18iOSSettingsService29ReadValueiPhoneMinimumVersionEPKc
    // IDA 0x23b68
    #[doc(alias = "iOSSettingsService::ReadValueiPhoneMinimumVersion(char const*)")]
    pub fn read_value_iphone_minimum_version(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x239ec).
        let value = parse_int(text);
        Self::shared().iphone_minimum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23b80 — iOSSettingsService::ReadValueiPhoneMaximumVersion(char const*)
    // mangled: __ZN18iOSSettingsService29ReadValueiPhoneMaximumVersionEPKc
    // IDA 0x23b80
    #[doc(alias = "iOSSettingsService::ReadValueiPhoneMaximumVersion(char const*)")]
    pub fn read_value_iphone_maximum_version(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x239ec).
        let value = parse_int(text);
        Self::shared().iphone_maximum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23b98 — iOSSettingsService::ReadValueiPodMinimumVersion(char const*)
    // mangled: __ZN18iOSSettingsService27ReadValueiPodMinimumVersionEPKc
    // IDA 0x23b98
    #[doc(alias = "iOSSettingsService::ReadValueiPodMinimumVersion(char const*)")]
    pub fn read_value_ipod_minimum_version(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x239ec).
        let value = parse_int(text);
        Self::shared().ipod_minimum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23bb0 — iOSSettingsService::ReadValueiPodMaximumVersion(char const*)
    // mangled: __ZN18iOSSettingsService27ReadValueiPodMaximumVersionEPKc
    // IDA 0x23bb0
    #[doc(alias = "iOSSettingsService::ReadValueiPodMaximumVersion(char const*)")]
    pub fn read_value_ipod_maximum_version(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x239ec).
        let value = parse_int(text);
        Self::shared().ipod_maximum_version.store(value, Ordering::SeqCst);
        value
    }

    // 0x23bc8 — iOSSettingsService::ReadValueDisablePlayButtonForAll(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueDisablePlayButtonForAllEPKc
    // IDA 0x23bc8
    #[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForAll(char const*)")]
    pub fn read_value_disable_play_button_for_all(text: &str) -> bool {
        // `SimpleJSON::ParseBool` into `_thisPtr + 52`
        // (IDA 0x23bcc..0x23bdc). Verified via IDA decompile.
        let value = parse_bool(text);
        Self::shared().disable_play_button_for_all.store(value, Ordering::SeqCst);
        value
    }

    // 0x23be4 — iOSSettingsService::ReadValueDisablePlayButtonForNonBC(char const*)
    // mangled: __ZN18iOSSettingsService34ReadValueDisablePlayButtonForNonBCEPKc
    // IDA 0x23be4
    #[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForNonBC(char const*)")]
    pub fn read_value_disable_play_button_for_non_bc(text: &str) -> bool {
        // `SimpleJSON::ParseBool` into the singleton slot
        // (same shape as IDA 0x23bc8).
        let value = parse_bool(text);
        Self::shared().disable_play_button_for_non_bc.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c00 — iOSSettingsService::ReadValueiPad1_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPad1_MaximumIdealPartsEPKc
    // IDA 0x23c00
    #[doc(alias = "iOSSettingsService::ReadValueiPad1_MaximumIdealParts(char const*)")]
    pub fn read_value_ipad1_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into `_thisPtr + 56` (IDA 0x23c04..0x23c14).
        // Verified via IDA decompile.
        let value = parse_int(text);
        Self::shared().ipad1_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c18 — iOSSettingsService::ReadValueiPad2_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPad2_MaximumIdealPartsEPKc
    // IDA 0x23c18
    #[doc(alias = "iOSSettingsService::ReadValueiPad2_MaximumIdealParts(char const*)")]
    pub fn read_value_ipad2_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().ipad2_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c30 — iOSSettingsService::ReadValueiPad3_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPad3_MaximumIdealPartsEPKc
    // IDA 0x23c30
    #[doc(alias = "iOSSettingsService::ReadValueiPad3_MaximumIdealParts(char const*)")]
    pub fn read_value_ipad3_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().ipad3_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c48 — iOSSettingsService::ReadValueiPad4_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPad4_MaximumIdealPartsEPKc
    // IDA 0x23c48
    #[doc(alias = "iOSSettingsService::ReadValueiPad4_MaximumIdealParts(char const*)")]
    pub fn read_value_ipad4_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().ipad4_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c60 — iOSSettingsService::ReadValueiPod4_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPod4_MaximumIdealPartsEPKc
    // IDA 0x23c60
    #[doc(alias = "iOSSettingsService::ReadValueiPod4_MaximumIdealParts(char const*)")]
    pub fn read_value_ipod4_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().ipod4_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c78 — iOSSettingsService::ReadValueiPod5_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService32ReadValueiPod5_MaximumIdealPartsEPKc
    // IDA 0x23c78
    #[doc(alias = "iOSSettingsService::ReadValueiPod5_MaximumIdealParts(char const*)")]
    pub fn read_value_ipod5_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().ipod5_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23c90 — iOSSettingsService::ReadValueiPhone4s_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService34ReadValueiPhone4s_MaximumIdealPartsEPKc
    // IDA 0x23c90
    #[doc(alias = "iOSSettingsService::ReadValueiPhone4s_MaximumIdealParts(char const*)")]
    pub fn read_value_iphone4s_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().iphone4s_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23ca8 — iOSSettingsService::ReadValueiPhone5_MaximumIdealParts(char const*)
    // mangled: __ZN18iOSSettingsService33ReadValueiPhone5_MaximumIdealPartsEPKc
    // IDA 0x23ca8
    #[doc(alias = "iOSSettingsService::ReadValueiPhone5_MaximumIdealParts(char const*)")]
    pub fn read_value_iphone5_maximum_ideal_parts(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().iphone5_maximum_ideal_parts.store(value, Ordering::SeqCst);
        value
    }

    // 0x23cc0 — iOSSettingsService::ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes(char const*)
    // mangled: __ZN18iOSSettingsService51ReadValueTimeIntervalBetweenRobuxPurchaseInMinutesEPKc
    // IDA 0x23cc0
    #[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes(char const*)")]
    pub fn read_value_time_interval_between_robux_purchase_in_minutes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().time_interval_between_robux_purchase_in_minutes.store(value, Ordering::SeqCst);
        value
    }

    // 0x23cd8 — iOSSettingsService::ReadValueTimeIntervalBetweenBCPurchaseInMinutes(char const*)
    // mangled: __ZN18iOSSettingsService48ReadValueTimeIntervalBetweenBCPurchaseInMinutesEPKc
    // IDA 0x23cd8
    #[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenBCPurchaseInMinutes(char const*)")]
    pub fn read_value_time_interval_between_bc_purchase_in_minutes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().time_interval_between_bc_purchase_in_minutes.store(value, Ordering::SeqCst);
        value
    }

    // 0x23cf0 — iOSSettingsService::ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes(char const*)
    // mangled: __ZN18iOSSettingsService52ReadValueTimeIntervalBetweenCatalogPurchaseInMinutesEPKc
    // IDA 0x23cf0
    #[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes(char const*)")]
    pub fn read_value_time_interval_between_catalog_purchase_in_minutes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().time_interval_between_catalog_purchase_in_minutes.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d08 — iOSSettingsService::ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp(char const*)
    // mangled: __ZN18iOSSettingsService59ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUpEPKc
    // IDA 0x23d08
    #[doc(alias = "iOSSettingsService::ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp(char const*)")]
    pub fn read_value_time_limit_for_billing_service_retries_before_giving_up(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().time_limit_for_billing_service_retries_before_giving_up.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d20 — iOSSettingsService::ReadValueTestFlightLoggingLevel(char const*)
    // mangled: __ZN18iOSSettingsService31ReadValueTestFlightLoggingLevelEPKc
    // IDA 0x23d20
    #[doc(alias = "iOSSettingsService::ReadValueTestFlightLoggingLevel(char const*)")]
    pub fn read_value_test_flight_logging_level(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().test_flight_logging_level.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d38 — iOSSettingsService::ReadValueTestFlightPercentage(char const*)
    // mangled: __ZN18iOSSettingsService28ReadValueTestFlightPercentageEPKc
    // IDA 0x23d38
    #[doc(alias = "iOSSettingsService::ReadValueTestFlightPercentage(char const*)")]
    pub fn read_value_test_flight_percentage(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().test_flight_percentage.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d50 — iOSSettingsService::ReadValueBugSensePercentage(char const*)
    // mangled: __ZN18iOSSettingsService26ReadValueBugSensePercentageEPKc
    // IDA 0x23d50
    #[doc(alias = "iOSSettingsService::ReadValueBugSensePercentage(char const*)")]
    pub fn read_value_bug_sense_percentage(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().bug_sense_percentage.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d68 — iOSSettingsService::ReadValueBugSenseLogLines(char const*)
    // mangled: __ZN18iOSSettingsService24ReadValueBugSenseLogLinesEPKc
    // IDA 0x23d68
    #[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLines(char const*)")]
    pub fn read_value_bug_sense_log_lines(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().bug_sense_log_lines.store(value, Ordering::SeqCst);
        value
    }

    // 0x23d80 — iOSSettingsService::ReadValueBugSenseLogLevel(char const*)
    // mangled: __ZN18iOSSettingsService24ReadValueBugSenseLogLevelEPKc
    // IDA 0x23d80
    #[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLevel(char const*)")]
    pub fn read_value_bug_sense_log_level(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x23c00).
        let value = parse_int(text);
        Self::shared().bug_sense_log_level.store(value, Ordering::SeqCst);
        value
    }
}
