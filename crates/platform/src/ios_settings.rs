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
    constructed: AtomicBool,
    freed: AtomicBool,
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

    // 0x23d9c — iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)
    // mangled: __ZN18iOSSettingsService35ReadValueiOSGoogleAnalyticsAccount2EPKc
    // IDA 0x23d9c
    #[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)")]
    pub fn read_value_ios_google_analytics_account2(text: &str) {
        // `std::string` copy-assign into `_thisPtr + 88`
        // (IDA 0x23dd2..0x23e0a). Verified via IDA decompile.
        *Self::shared().ios_google_analytics_account2.lock() = text.to_owned();
    }

    // 0x23ed4 — iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)
    // mangled: __ZN18iOSSettingsService36ReadValueiOSGoogleAnalyticsSampleRateEPKc
    // IDA 0x23ed4
    #[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)")]
    pub fn read_value_ios_google_analytics_sample_rate(text: &str) -> i32 {
        // `atoi` into `_thisPtr + 92` (IDA 0x23ed8..0x23ee8).
        // Verified via IDA decompile.
        let value = parse_int(text);
        Self::shared().ios_google_analytics_sample_rate.store(value, Ordering::SeqCst);
        value
    }

    // 0x23eec — iOSSettingsService::ReadValueSearchEndpointIPad(char const*)
    // mangled: __ZN18iOSSettingsService25ReadValueSearchEndpointIPadEPKc
    // IDA 0x23eec
    #[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPad(char const*)")]
    pub fn read_value_search_endpoint_ipad(text: &str) {
        // `std::string` copy-assign into `_thisPtr + 132`
        // (IDA 0x23f22..0x23f5a). Verified via IDA decompile.
        *Self::shared().search_endpoint_ipad.lock() = text.to_owned();
    }

    // 0x24024 — iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)
    // mangled: __ZN18iOSSettingsService27ReadValueSearchEndpointIPhoneEPKc
    // IDA 0x24024
    #[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)")]
    pub fn read_value_search_endpoint_iphone(text: &str) {
        // `std::string` copy-assign into the singleton slot
        // (same shape as IDA 0x23eec).
        *Self::shared().search_endpoint_iphone.lock() = text.to_owned();
    }

    // 0x2415c — iOSSettingsService::ReadValueCacheUIWebViews(char const*)
    // mangled: __ZN18iOSSettingsService23ReadValueCacheUIWebViewsEPKc
    // IDA 0x2415c
    #[doc(alias = "iOSSettingsService::ReadValueCacheUIWebViews(char const*)")]
    pub fn read_value_cache_ui_web_views(text: &str) -> bool {
        // `SimpleJSON::ParseBool` into `_thisPtr + 140`
        // (IDA 0x24160..0x24170). Verified via IDA decompile.
        let value = parse_bool(text);
        Self::shared().cache_ui_web_views.store(value, Ordering::SeqCst);
        value
    }

    // 0x24178 — iOSSettingsService::ReadValueThumbstickControlStyle(char const*)
    // mangled: __ZN18iOSSettingsService29ReadValueThumbstickControlStyleEPKc
    // IDA 0x24178
    #[doc(alias = "iOSSettingsService::ReadValueThumbstickControlStyle(char const*)")]
    pub fn read_value_thumbstick_control_style(text: &str) -> i32 {
        // `atoi` into `_thisPtr + 144` (IDA 0x2417c..0x2418c).
        // Verified via IDA decompile.
        let value = parse_int(text);
        Self::shared().thumbstick_control_style.store(value, Ordering::SeqCst);
        value
    }

    // 0x24194 — iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)
    // mangled: __ZN18iOSSettingsService30ReadValueFreeMemoryCheckerActiveEPKc
    // IDA 0x24194
    #[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)")]
    pub fn read_value_free_memory_checker_active(text: &str) -> bool {
        // `SimpleJSON::ParseBool` into `_thisPtr + 148`
        // (IDA 0x24198..0x241a8). Verified via IDA decompile.
        let value = parse_bool(text);
        Self::shared().free_memory_checker_active.store(value, Ordering::SeqCst);
        value
    }

    // 0x241b0 — iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)
    // mangled: __ZN18iOSSettingsService43ReadValueFreeMemoryCheckerRateMilliSecondsEPKc
    // IDA 0x241b0
    #[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)")]
    pub fn read_value_free_memory_checker_rate_milli_seconds(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().free_memory_checker_rate_milli_seconds.store(value, Ordering::SeqCst);
        value
    }

    // 0x241cc — iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)
    // mangled: __ZN18iOSSettingsService46ReadValueFreeMemoryCheckerThresholdKiloBytesEPKc
    // IDA 0x241cc
    #[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)")]
    pub fn read_value_free_memory_checker_threshold_kilo_bytes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().free_memory_checker_threshold_kilo_bytes.store(value, Ordering::SeqCst);
        value
    }

    // 0x241e8 — iOSSettingsService::ReadValueMemoryBouncerActive(char const*)
    // mangled: __ZN18iOSSettingsService26ReadValueMemoryBouncerActiveEPKc
    // IDA 0x241e8
    #[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerActive(char const*)")]
    pub fn read_value_memory_bouncer_active(text: &str) -> bool {
        // `SimpleJSON::ParseBool` into the singleton slot
        // (same shape as IDA 0x24194).
        let value = parse_bool(text);
        Self::shared().memory_bouncer_active.store(value, Ordering::SeqCst);
        value
    }

    // 0x24204 — iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)
    // mangled: __ZN18iOSSettingsService45ReadValueMemoryBouncerEnforceRateMilliSecondsEPKc
    // IDA 0x24204
    #[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)")]
    pub fn read_value_memory_bouncer_enforce_rate_milli_seconds(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().memory_bouncer_enforce_rate_milli_seconds.store(value, Ordering::SeqCst);
        value
    }

    // 0x24220 — iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)
    // mangled: __ZN18iOSSettingsService41ReadValueMemoryBouncerThresholdKiloBytesEPKc
    // IDA 0x24220
    #[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)")]
    pub fn read_value_memory_bouncer_threshold_kilo_bytes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().memory_bouncer_threshold_kilo_bytes.store(value, Ordering::SeqCst);
        value
    }

    // 0x2423c — iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)
    // mangled: __ZN18iOSSettingsService37ReadValueMemoryBouncerLimitMegaBytesEPKc
    // IDA 0x2423c
    #[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)")]
    pub fn read_value_memory_bouncer_limit_mega_bytes(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().memory_bouncer_limit_mega_bytes.store(value, Ordering::SeqCst);
        value
    }

    // 0x24258 — iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)
    // mangled: __ZN18iOSSettingsService52ReadValueMemoryBouncerLimitMegaBytesForLowMemDevicesEPKc
    // IDA 0x24258
    #[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)")]
    pub fn read_value_memory_bouncer_limit_mega_bytes_for_low_mem_devices(text: &str) -> i32 {
        // `atoi` into the singleton slot (same shape as IDA 0x24178).
        let value = parse_int(text);
        Self::shared().memory_bouncer_limit_mega_bytes_for_low_mem_devices.store(value, Ordering::SeqCst);
        value
    }

    // 0x43180 — iOSSettingsService::iOSSettingsService(void)
    // mangled: __ZN18iOSSettingsServiceC2Ev
    // IDA 0x43180
    #[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
    pub fn new() {
        // Empty map + empty strings, `_thisPtr = this` (IDA 0x431a8..0x4320e),
        // then `Init` (IDA 0x43236). Verified via IDA decompile.
        let this = Self::shared();
        this.readers.lock().clear();
        this.ios_google_analytics_account2.lock().clear();
        this.search_endpoint_ipad.lock().clear();
        this.search_endpoint_iphone.lock().clear();
        this.constructed.store(true, Ordering::SeqCst);
        this.freed.store(false, Ordering::SeqCst);
        Self::init();
    }

    pub fn is_constructed() -> bool {
        Self::shared().constructed.load(Ordering::SeqCst)
    }

    // 0x432c8 — iOSSettingsService::~iOSSettingsService()
    // mangled: __ZN18iOSSettingsServiceD2Ev
    // IDA 0x432c8
    #[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
    pub fn destroy_d2() {
        // Vtable reset, string teardown, map erase
        // (IDA 0x432dc..0x4330a). Verified via IDA decompile.
        let this = Self::shared();
        this.readers.lock().clear();
        this.ios_google_analytics_account2.lock().clear();
        this.search_endpoint_ipad.lock().clear();
        this.search_endpoint_iphone.lock().clear();
        this.constructed.store(false, Ordering::SeqCst);
    }

    // 0x432b0 — iOSSettingsService::~iOSSettingsService()
    // mangled: __ZN18iOSSettingsServiceD1Ev
    // IDA 0x432b0
    #[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
    pub fn destroy_d1() {
        // Thunk to D2 (IDA 0x432b0). Verified via IDA decompile.
        Self::destroy_d2();
    }

    // 0x432b4 — iOSSettingsService::~iOSSettingsService()
    // mangled: __ZN18iOSSettingsServiceD0Ev
    // IDA 0x432b4
    #[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
    pub fn delete_d0() {
        // D2 then `operator delete` (IDA 0x432ba..0x432c4).
        // Verified via IDA decompile.
        Self::destroy_d2();
        Self::shared().freed.store(true, Ordering::SeqCst);
    }

    pub fn is_freed() -> bool {
        Self::shared().freed.load(Ordering::SeqCst)
    }

    // 0xf27354 — iOSSettingsService::iOSSettingsService(void)
    // mangled: __ZN18iOSSettingsServiceC2Ev$shim
    // IDA 0xf27354
    #[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
    pub fn new_shim() {
        // Thunk to C2 (IDA 0xf2735c). Verified via IDA decompile.
        Self::new();
    }

    // 0xf27364 — iOSSettingsService::~iOSSettingsService()
    // mangled: __ZN18iOSSettingsServiceD2Ev$shim
    // IDA 0xf27364
    #[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
    pub fn destroy_d2_shim() {
        // Thunk to D2 (IDA 0xf2736c). Verified via IDA decompile.
        Self::destroy_d2();
    }
}
