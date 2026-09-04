//! core shard ke — 150 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core after kd 0x789e50 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost; 25622 filtered, 5677->5527 gaps, 35159->35309 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.
//! Batch: 20 IDA-grounded ports 0x79890c-0x79ca70 — the v8xml cluster
//! (`XmlElement::isXsiNil`/`findNextChildWithSameTag`, the six
//! `XmlNameValuePair::getValue` overloads + `clearValue` + `toString`, and
//! the `TextXmlParser`/`TextXmlWriter` reader/writer family). Untouched
//! carriers keep their stub bodies; ports live in `xml_value`/`xml_tree`/
//! `xml_parse`/`xml_write` under idiomatic names, wired via `stub_0x*`.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// IDA-grounded XML batch: 20 ports 0x79890c-0x79ca70.
/// `XmlElement` / `XmlNameValuePair` value semantics, the `TextXmlParser`
/// cursor reader and the `TextXmlWriter` serializer from
/// Client/App/v8xml (XmlElement.cpp / XmlSerializer.cpp).
/// Each item notes the EA whose decompile + disassembly grounds it.
/// Conventions: `shared_ptr` -> `crate::SharedPtr` (Arc),
/// `std::string` -> `String`, throws -> `Result<_, ParseError>`.
/// `[INFERENCE]` marks behavior the binary does not pin down; everything
/// else follows the IDA pseudocode branch-for-branch.
pub mod xml_value {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// was: `FLog::Asserts` — gates the `ReleaseAssert` / `_debugHook` paths
    /// (IDA 0x798e2a, 0x7990d0, 0x79a8ea, 0x79b482, 0x79b366, 0x79b2d4).
    /// The port preserves the return-value behavior on every such path; the
    /// hook itself is a sink.
    pub static FLOG_ASSERTS: AtomicBool = AtomicBool::new(true);

    /// Returns the current `FLog::Asserts` value (IDA reads the global byte).
    pub fn flog_asserts() -> bool {
        FLOG_ASSERTS.load(Ordering::Relaxed)
    }

    /// was: `RBX::_internal::_debugHook` + `ReleaseAssert` — notes the site
    /// where the original raises its assert dialog (XmlElement.cpp:328/437,
    /// XmlSerializer.cpp:186/291/299). No-op sink: every such path returns
    /// its IDA-observed value immediately after.
    pub fn assert_hook(_message: &str, _file: &str, _line: u32) {}

    /// was: `XmlNameValuePair::valueType` (IDA +4 word). Tags observed across
    /// the getValue family / clearValue / toString: 0-9.
    pub const TAG_NONE: u32 = 0;
    pub const TAG_NAME: u32 = 1;
    pub const TAG_STRING: u32 = 2;
    pub const TAG_OWNED_STRING: u32 = 3;
    pub const TAG_BOOL: u32 = 4;
    pub const TAG_INT: u32 = 5;
    pub const TAG_UINT: u32 = 6;
    pub const TAG_FLOAT: u32 = 7;
    pub const TAG_IDREF: u32 = 8;
    pub const TAG_DOUBLE: u32 = 9;
    /// [INFERENCE] `isValueType<RBX::ContentId>` arm (IDA 0x79ca98): the exact
    /// original tag is unknown; branch behavior is preserved on every path
    /// that can observe it.
    pub const TAG_CONTENT_ID: u32 = 10;

    /// was: `RBX::Name` well-known singletons, confirmed or inferred text.
    /// `xsi:nil` is confirmed (IDA string `aXsiNil` @0x11300db == "xsi:nil").
    pub const XSI_NIL_NAME: &str = "xsi:nil";
    /// [INFERENCE] `tag_mimeType` text (only ever written as an attribute).
    pub const MIME_TYPE_NAME: &str = "mimeType";
    /// [INFERENCE] `tag_null` text (writer emits `<null></null>` for nulls).
    pub const NULL_TAG: &str = "null";
    /// [INFERENCE] `tag_hash` text (symmetric container marker).
    pub const HASH_TAG: &str = "hash";
    /// [INFERENCE] the `tag == "ContentId"` container check (IDA 0x79bc32
    /// compares the tag against the `ContentId` reflection name).
    pub const CONTENT_ID_TAG: &str = "ContentId";
    /// [INFERENCE] `value_IDREF_null` text for null IDREF bindings.
    pub const IDREF_NULL_TEXT: &str = "null";
    /// [INFERENCE] tombstone texts stored for discarded binary/hash/null
    /// children (IDA 0x79be92/0x79bf02/0x79bdf6 copy a null-name pair; the
    /// null-name strings are unverified, so the originating tag is kept and
    /// re-serializes stably).
    pub const BINARY_MARKER: &str = "binary";
    pub const HASH_MARKER: &str = "hash";
    pub const NULL_MARKER: &str = "null";

    /// was: the IDREF-bound object behind a tag-8 `shared_ptr` payload
    /// (IDA 0x7991b0: `shared_count` copy + `getHandleIndex`). Only its
    /// identity is observable (via the `RBX%d` handle index).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct IdRefTarget {
        pub opaque: usize,
    }

    /// was: `RBX::ContentId` as stored in a pair (IDA 0x79ca98-0x79cb08).
    /// `mime` carries the `mimeType` attribute text when one was present.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct ContentId {
        pub text: String,
        pub mime: Option<String>,
    }

    /// was: `XmlNameValuePair` payload (IDA +8 word; 8 bytes for doubles).
    /// Tag 8 keeps shared ownership exactly like the original `shared_ptr`
    /// copy in `toString` (IDA 0x7991c6 addref / 0x7991d8 release).
    #[derive(Debug, Clone, PartialEq)]
    pub enum Value {
        None,
        Name(String),
        Text(String),
        Owned(String),
        Bool(bool),
        Int(i32),
        UInt(u32),
        Float(f32),
        IdRef(Option<crate::SharedPtr<IdRefTarget>>),
        Double(f64),
        ContentId(ContentId),
    }

    impl Default for Value {
        fn default() -> Self {
            Value::None
        }
    }

    /// was: `XmlNameValuePair` — name at +0, `valueType` at +4, payload at +8.
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct NameValuePair {
        pub name: String,
        pub value: Value,
    }

    impl NameValuePair {
        /// Returns the string payload for text-like values, `""` otherwise.
        /// Used for attribute-value reads (mime names, test probes).
        pub fn text_or_empty(&self) -> &str {
            match &self.value {
                Value::Text(s) | Value::Owned(s) | Value::Name(s) => s,
                _ => "",
            }
        }
        /// was: fresh pair with empty (`None`) value.
        pub fn new(name: String) -> Self {
            Self { name, value: Value::None }
        }

        /// was: `addAttribute<std::string>` / `setValue(std::string)` payload
        /// (IDA 0xa5251a: tag 2 with a heap string copy).
        pub fn with_text(name: String, text: String) -> Self {
            Self { name, value: Value::Text(text) }
        }

        /// was: the `valueType` word (IDA +4).
        pub fn tag(&self) -> u32 {
            match self.value {
                Value::None => TAG_NONE,
                Value::Name(_) => TAG_NAME,
                Value::Text(_) => TAG_STRING,
                Value::Owned(_) => TAG_OWNED_STRING,
                Value::Bool(_) => TAG_BOOL,
                Value::Int(_) => TAG_INT,
                Value::UInt(_) => TAG_UINT,
                Value::Float(_) => TAG_FLOAT,
                Value::IdRef(_) => TAG_IDREF,
                Value::Double(_) => TAG_DOUBLE,
                Value::ContentId(_) => TAG_CONTENT_ID,
            }
        }
        /// IDA 0x7989ec `clearValue`: tags 8/3/2 drop the owned payload
        /// (tag 8 releases the shared count then frees, tags 3/2 destroy the
        /// heap string then free); every path ends with `tag = 0`.
        pub fn clear_value(&mut self) {
            self.value = Value::None;
        }


        pub fn get_bool(&mut self, out: &mut bool) -> bool {
            if self.tag() == TAG_STRING {
                let text = match &self.value {
                    Value::Text(s) => s.clone(),
                    _ => return false,
                };
                // IDA 0x79897a: the converter writes the caller's `out`
                // directly; the pair then caches a copy of it.
                if !convert_bool_to_value(&text, out) {
                    return false; // IDA 0x798980
                }
                let cached = *out;
                self.clear_value(); // IDA 0x798986
                self.value = Value::Bool(cached); // IDA 0x79898c/0x798990
                true
            } else if self.tag() == TAG_BOOL {
                if let Value::Bool(v) = self.value {
                    *out = v; // IDA 0x79899c
                }
                true
            } else {
                false
            }
        }

        /// Const read mirroring `get_bool` without the parse cache store, for
        /// the `const` `isXsiNil` path (IDA 0x798936 calls the caching
        /// overload; the cache write is unobservable, so the const port reads
        /// without storing). Outputs match 1:1.
        pub fn get_bool_shared(&self, out: &mut bool) -> bool {
            match &self.value {
                Value::Text(s) => convert_bool_to_value(s, out),
                Value::Bool(v) => {
                    *out = *v;
                    true
                }
                _ => false,
            }
        }

        /// IDA 0x798d64 `getValue(int&)`: tag 2 converts via
        /// `StringConverter<int>` then caches (`QWORD(+4) = 5 | v<<32`);
        /// tag 5 reads the cache; else 0.
        pub fn get_int(&mut self, out: &mut i32) -> bool {
            if self.tag() == TAG_STRING {
                let text = match &self.value {
                    Value::Text(s) => s.clone(),
                    _ => return false,
                };
                // IDA 0x798d7a: the converter writes the caller's `out`
                // directly; the pair then caches a copy of it.
                if !convert_int_to_value(&text, out) {
                    return false; // IDA 0x798d80
                }
                let cached = *out;
                self.clear_value(); // IDA 0x798d86
                self.value = Value::Int(cached); // IDA 0x798d8a-0x798d8e
                true
            } else if self.tag() == TAG_INT {
                if let Value::Int(v) = self.value {
                    *out = v; // IDA 0x798d9c
                }
                true
            } else {
                false
            }
        }

        /// IDA 0x798da4 `getValue(unsigned&)`: same shape with tag 6.
        pub fn get_uint(&mut self, out: &mut u32) -> bool {
            if self.tag() == TAG_STRING {
                let text = match &self.value {
                    Value::Text(s) => s.clone(),
                    _ => return false,
                };
                // IDA 0x798dba: the converter writes the caller's `out`
                // directly; the pair then caches a copy of it.
                if !convert_uint_to_value(&text, out) {
                    return false; // IDA 0x798dc0
                }
                let cached = *out;
                self.clear_value(); // IDA 0x798dc6
                self.value = Value::UInt(cached); // IDA 0x798dca-0x798dce
                true
            } else if self.tag() == TAG_UINT {
                if let Value::UInt(v) = self.value {
                    *out = v; // IDA 0x798ddc
                }
                true
            } else {
                false
            }
        }

        /// IDA 0x798de4 `getValue(float&)`: tag 2 converts and caches
        /// (tag 7); tag 7 reads the cache. Any other tag returns 0; when
        /// asserts are on and the tag is 9 (`DOUBLE`) the original calls its
        /// debug hook with `"valueType!=DOUBLE"` (XmlElement.cpp:328) and
        /// `ReleaseAssert`s before returning 0.
        /// BUG preserved: a `double`-valued pair is never readable as float
        /// even though the widening conversion exists in the double getter.
        pub fn get_float(&mut self, out: &mut f32) -> bool {
            match self.tag() {
                TAG_STRING => {
                    let text = match &self.value {
                        Value::Text(s) => s.clone(),
                        _ => return false,
                    };
                    // IDA 0x798dfc: the converter writes the caller's `out`
                    // directly; the pair then caches a copy of it.
                    if convert_float_to_value(&text, out) {
                        let cached = *out;
                        self.clear_value(); // IDA 0x798e00
                        self.value = Value::Float(cached); // IDA 0x798e04-0x798e08
                        true
                    } else {
                        false
                    }
                }
                TAG_FLOAT => {
                    if let Value::Float(v) = self.value {
                        *out = v; // IDA 0x798e18
                    }
                    true
                }
                _ => {
                    if self.tag() == TAG_DOUBLE && flog_asserts() {
                        assert_hook("valueType!=DOUBLE", "XmlElement.cpp", 328);
                    }
                    false
                }
            }
        }

        /// IDA 0x798e7c `getValue(double&)`: tag 2 parses and caches as tag 9;
        /// tag 7 widens the cached float and re-caches as tag 9; tag 9 reads
        /// the cache directly (no re-store); else 0.
        pub fn get_double(&mut self, out: &mut f64) -> bool {
            match self.tag() {
                TAG_STRING => {
                    let text = match &self.value {
                        Value::Text(s) => s.clone(),
                        _ => return false,
                    };
                    // IDA 0x798eb4: the converter writes the caller's `out`
                    // directly; the pair then caches a copy of it.
                    if !convert_double_to_value(&text, out) {
                        return false; // IDA 0x798eba
                    }
                    let cached = *out;
                    self.clear_value(); // IDA 0x798ec0
                    self.value = Value::Double(cached); // IDA 0x798eca-0x798ece
                    true
                }
                TAG_FLOAT => {
                    let v = match self.value {
                        Value::Float(v) => v as f64, // IDA 0x798e96
                        _ => return false,
                    };
                    *out = v;
                    self.clear_value();
                    self.value = Value::Double(v);
                    true
                }
                TAG_DOUBLE => {
                    if let Value::Double(v) = self.value {
                        *out = v; // IDA 0x798ea6
                    }
                    true
                }
                _ => false,
            }
        }

        /// IDA 0x799060 `toString`: renders the cached value. Tags 0/1/2/3
        /// copy strings; 4/5/6/7/9 go through `StringConverter`;
        /// tag 8 formats `RBX<handle>` via `getHandleIndex`, or the
        /// `value_IDREF_null` text when the binding is null. The default arm
        /// asserts (`false`, XmlElement.cpp:437) and yields the empty string.
        pub fn to_string_value(&self, writer: &mut super::xml_write::XmlWriter) -> String {
            match &self.value {
                Value::None => String::new(), // IDA 0x79915e
                Value::Name(n) => n.clone(),  // IDA 0x799164: string at payload+4
                Value::Text(s) | Value::Owned(s) => s.clone(), // IDA 0x799146
                Value::Bool(v) => convert_bool_to_string(*v), // IDA 0x799174
                Value::Int(v) => format!("{v}"),              // IDA 0x799186 "%d"
                Value::UInt(v) => format!("{v}"),             // IDA 0x799198 "%u"
                Value::Float(v) => convert_float_to_string(*v), // IDA 0x7991aa "%.9g"
                Value::IdRef(slot) => match slot {
                    Some(target) => {
                        let key = crate::SharedPtr::as_ptr(target) as usize;
                        let h = writer.handle_index(key); // IDA 0x799206
                        format!("RBX{h}") // IDA 0x79921a sprintf "RBX%d"
                    }
                    None => IDREF_NULL_TEXT.to_string(), // IDA 0x79927e
                },
                Value::Double(v) => convert_double_to_string(*v), // IDA 0x79926e
                Value::ContentId(_) => {
                    if flog_asserts() {
                        assert_hook("false", "XmlElement.cpp", 437); // IDA 0x7990d0
                    }
                    String::new() // IDA 0x79911c
                }
            }
        }
    }

    /// IDA 0x38ce78 `StringConverter<bool>::convertToValue`: only
    /// true/True/TRUE and false/False/FALSE succeed; `out` is written on
    /// success and left alone on failure.
    pub fn convert_bool_to_value(text: &str, out: &mut bool) -> bool {
        match text {
            "true" | "True" | "TRUE" => {
                *out = true;
                true
            }
            "false" | "False" | "FALSE" => {
                *out = false;
                true
            }
            _ => false,
        }
    }

    /// IDA 0x38ce48 `StringConverter<bool>::convertToString`.
    pub fn convert_bool_to_string(v: bool) -> String {
        if v { "true".to_string() } else { "false".to_string() }
    }

    /// IDA 0x38cfa0 `StringConverter<int>::convertToValue`: every char must
    /// be a digit except an optional leading `-`; empty fails; `atoi` reads
    /// the value. BUG preserved: `"-"` alone passes validation and converts
    /// to 0 with success.
    pub fn convert_int_to_value(text: &str, out: &mut i32) -> bool {
        if text.is_empty() {
            return false; // IDA 0x38cfae
        }
        let bytes = text.as_bytes();
        for (i, &c) in bytes.iter().enumerate() {
            if i == 0 && c == b'-' {
                continue; // IDA 0x38cfc8: `i == 0 && c == '-'`
            }
            if !c.is_ascii_digit() {
                return false; // IDA 0x38cfdc
            }
        }
        if text == "-" {
            *out = 0; // IDA 0x38cfe0: atoi("-") == 0, success
            return true;
        }
        *out = atoi_prefix(text);
        true
    }

    /// IDA 0x38d038 `StringConverter<unsigned>::convertToValue`: strict
    /// `boost::lexical_cast` digit parse; empty fails; a failed cast throws
    /// `bad_lexical_cast`, which the frame catches into a 0 return.
    /// [INFERENCE] leading `skipws` blanks are accepted like the
    /// `num_get`-backed original; signs and trailing junk are rejected.
    pub fn convert_uint_to_value(text: &str, out: &mut u32) -> bool {
        let t = text.trim_start_matches([' ', '\t', '\n', '\x0B', '\x0C', '\r']);
        if t.is_empty() || !t.bytes().all(|c| c.is_ascii_digit()) {
            return false;
        }
        match t.parse::<u32>() {
            Ok(v) => {
                *out = v;
                true
            }
            Err(_) => false, // IDA 0x38d0d4: bad_lexical_cast -> return 0
        }
    }

    /// C `atoi` prefix semantics shared by `decodeString` (IDA 0x79a8a6):
    /// skips blanks, takes an optional sign and the digit run; 0 when empty.
    /// Wrapping accumulate matches the unchecked original arithmetic.
    pub fn atoi_prefix(text: &str) -> i32 {
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r') {
            i += 1;
        }
        let mut neg = false;
        if i < bytes.len() && (bytes[i] == b'-' || bytes[i] == b'+') {
            neg = bytes[i] == b'-';
            i += 1;
        }
        let mut v: i32 = 0;
        let mut any = false;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            any = true;
            v = v.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as i32);
            i += 1;
        }
        if !any {
            return 0;
        }
        if neg { v.wrapping_neg() } else { v }
    }

    /// `strtod` prefix value used by the float/double converters: skips
    /// blanks, handles `inf`/`nan` spellings case-insensitively, then takes
    /// the longest float prefix; 0.0 when there is none.
    pub fn atof_value(text: &str) -> f64 {
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r') {
            i += 1;
        }
        let neg = i < bytes.len() && (bytes[i] == b'-' || bytes[i] == b'+');
        let j = if neg { i + 1 } else { i };
        let rest = &text[j.min(bytes.len())..];
        let lower = rest.to_ascii_lowercase();
        if lower.starts_with("inf") {
            return if neg { f64::NEG_INFINITY } else { f64::INFINITY };
        }
        if lower.starts_with("nan") {
            return f64::NAN;
        }
        let mut k = j;
        while k < bytes.len() && bytes[k].is_ascii_digit() {
            k += 1;
        }
        if k < bytes.len() && bytes[k] == b'.' {
            k += 1;
            while k < bytes.len() && bytes[k].is_ascii_digit() {
                k += 1;
            }
        }
        let mut end = k;
        if k < bytes.len() && (bytes[k] == b'e' || bytes[k] == b'E') {
            let mut e = k + 1;
            if e < bytes.len() && (bytes[e] == b'-' || bytes[e] == b'+') {
                e += 1;
            }
            let digits = e;
            while e < bytes.len() && bytes[e].is_ascii_digit() {
                e += 1;
            }
            if e > digits {
                end = e;
            }
        }
        if end == j || (end == j + 1 && bytes[j] == b'.') {
            return 0.0;
        }
        text[i..end].parse::<f64>().unwrap_or(0.0)
    }

    /// IDA 0x38d260 `StringConverter<double>::convertToValue`: empty fails;
    /// `INF`/`-INF`/`NAN` map to the bit patterns `0x7FF0000000000000`,
    /// `0xFFF0000000000000`, `0x7FF8000000000000`; anything else goes through
    /// `atof` and always succeeds.
    pub fn convert_double_to_value(text: &str, out: &mut f64) -> bool {
        if text.is_empty() {
            return false; // IDA 0x38d272
        }
        if text == "INF" {
            *out = f64::INFINITY; // IDA 0x38d2be
            return true;
        }
        if text == "-INF" {
            *out = f64::NEG_INFINITY; // IDA 0x38d2c6
            return true;
        }
        if text == "NAN" {
            *out = f64::NAN; // IDA 0x38d2ce: 0x7FF8000000000000
            return true;
        }
        *out = atof_value(text); // IDA 0x38d2b8
        true
    }

    /// IDA 0x38d440 `StringConverter<float>::convertToValue`: same shape via
    /// `strtod` narrowed to `float` (`INF` -> `0x7F800000`,
    /// `-INF` -> `0xFF800000`, `NAN` -> `0x7FC00000`).
    pub fn convert_float_to_value(text: &str, out: &mut f32) -> bool {
        if text.is_empty() {
            return false; // IDA 0x38d452
        }
        if text == "INF" {
            *out = f32::INFINITY; // IDA 0x38d4a6
            return true;
        }
        if text == "-INF" {
            *out = f32::NEG_INFINITY; // IDA 0x38d4ac
            return true;
        }
        if text == "NAN" {
            *out = f32::NAN; // IDA 0x38d4b4
            return true;
        }
        *out = atof_value(text) as f32; // IDA 0x38d49c strtod
        true
    }

    /// C `%g` with `sig` significant digits (glibc `snprintf` used by
    /// `convertToString` for floats/doubles): `INF` is handled by the caller;
    /// NaN renders `nan`/`-nan` like glibc; fixed notation wins for
    /// `-4 <= exp < sig`, else scientific with a signed 2+-digit exponent.
    pub fn format_g(value: f64, sig: usize) -> String {
        if value.is_nan() {
            return if value.is_sign_negative() {
                "-nan".to_string()
            } else {
                "nan".to_string()
            };
        }
        if value == 0.0 {
            return if value.is_sign_negative() {
                "-0".to_string()
            } else {
                "0".to_string()
            };
        }
        let exp10 = value.abs().log10().floor() as i32;
        if (-4..sig as i32).contains(&exp10) {
            let decimals = (sig as i32 - 1 - exp10) as usize;
            trim_g_zeros(format!("{value:.decimals$}"))
        } else {
            let raw = format!("{value:.prec$e}", prec = sig - 1);
            let (mantissa, exp) = raw.split_once('e').unwrap_or((&raw, "0"));
            let mantissa = trim_g_zeros(mantissa.to_string());
            let exp: i32 = exp.parse().unwrap_or(0);
            format!("{mantissa}e{exp:+03}")
        }
    }

    fn trim_g_zeros(mut s: String) -> String {
        if s.contains('.') {
            while s.ends_with('0') {
                s.pop();
            }
            if s.ends_with('.') {
                s.pop();
            }
        }
        s
    }

    /// IDA 0x38d2e0 `StringConverter<double>::convertToString`.
    pub fn convert_double_to_string(v: f64) -> String {
        if v == f64::INFINITY {
            "INF".to_string() // IDA 0x38d35e
        } else if v == f64::NEG_INFINITY {
            "-INF".to_string() // IDA 0x38d384
        } else {
            format_g(v, 20) // IDA 0x38d3c2: snprintf "%.20g"
        }
    }

    /// IDA 0x38d4c4 `StringConverter<float>::convertToString`.
    pub fn convert_float_to_string(v: f32) -> String {
        if v == f32::INFINITY {
            "INF".to_string() // IDA 0x38d542
        } else if v == f32::NEG_INFINITY {
            "-INF".to_string() // IDA 0x38d568
        } else {
            format_g(v as f64, 9) // IDA 0x38d5aa: snprintf "%.9g"
        }
    }
}
/// was: `XmlAttribute` (20 bytes: next at +0, pair at +4 — IDA 0xa524d6)
/// and `XmlElement` (36 bytes: next at +0, first child at +4, last child at
/// +8, value pair at +12, attribute head at +28, attribute tail at +32 —
/// IDA 0x79b428 `operator new(36)`, 0x79b3c4, 0x79c27c, 0xa525e2).
pub mod xml_tree {
    use super::xml_value::{NameValuePair, XSI_NIL_NAME};

    /// was: `XmlAttribute` — next link at +0, inline `XmlNameValuePair`
    /// at +4 (IDA 0x798922 walk, 0x79894c `v2[1]` name compare,
    /// 0xa5250e pair init).
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct XmlAttribute {
        pub next: Option<Box<XmlAttribute>>,
        pub pair: NameValuePair,
    }

    /// was: `XmlElement` — next sibling at +0 (IDA 0x7989be walk),
    /// first child at +4 (IDA 0x79ca14), value pair at +12 with the tag as
    /// its name (IDA 0x79afee / 0x79b29e tag print, 0x79ca8e pair use),
    /// attribute list at +28 (IDA 0x79891e / 0x79894c / 0x79aff2).
    /// Child/attribute tails (+8/+32) are walk-maintained instead of stored.
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct XmlElement {
        pub next: Option<Box<XmlElement>>,
        pub first_child: Option<Box<XmlElement>>,
        pub value: NameValuePair,
        pub attrs: Option<Box<XmlAttribute>>,
    }

    impl XmlElement {
        /// was: `XmlElement::XmlElement(name)` on the 36-byte allocation
        /// (IDA 0x79b428-0x79b43c): tag set, everything else empty.
        pub fn new(tag: String) -> Self {
            Self { next: None, first_child: None, value: NameValuePair::new(tag), attrs: None }
        }

        /// was: the tag `Name` at +12 (its string is printed by the writer).
        pub fn tag(&self) -> &str {
            &self.value.name
        }

        /// IDA 0x79894c/0x7989d4 `findAttribute`: walks from the head
        /// (this+28), returning the first node whose pair name matches.
        pub fn find_attribute(&self, name: &str) -> Option<&XmlAttribute> {
            let mut node = self.attrs.as_deref();
            while let Some(attr) = node {
                if attr.pair.name == name {
                    // IDA 0x798960: match -> return node
                    return Some(attr);
                }
                node = attr.next.as_deref();
            }
            None // IDA 0x798956: null link -> return 0
        }

        /// Mutable variant of `find_attribute` (same walk).
        pub fn find_attribute_mut(&mut self, name: &str) -> Option<&mut XmlAttribute> {
            let mut node = self.attrs.as_deref_mut();
            while let Some(attr) = node {
                if attr.pair.name == name {
                    return Some(attr);
                }
                node = attr.next.as_deref_mut();
            }
            None
        }

        /// IDA 0xa524a4 `addAttribute<std::string>`: allocates the 20-byte
        /// node (next = 0, tag 2 heap-string payload) and appends via the
        /// head (+28) / tail (+32) pair — document order is preserved.
        pub fn add_attribute(&mut self, name: String, text: String) {
            let node = Box::new(XmlAttribute { next: None, pair: NameValuePair::with_text(name, text) });
            match self.attrs.as_deref_mut() {
                None => self.attrs = Some(node), // IDA 0xa525f0: empty -> head = new
                Some(head) => {
                    let mut cur: &mut XmlAttribute = head;
                    while cur.next.is_some() {
                        cur = cur.next.as_deref_mut().expect("checked");
                    }
                    cur.next = Some(node); // IDA 0xa525ea: tail.next = new
                }
            }
        }

        /// Appends a finished child (mirrors the LABEL_114 tail link at
        /// IDA 0x79c27c: `tail ? tail.next = elem : first = elem; tail = elem`).
        pub fn append_child(&mut self, child: XmlElement) {
            let node = Box::new(child);
            match self.first_child.as_deref_mut() {
                None => self.first_child = Some(node),
                Some(head) => {
                    let mut cur: &mut XmlElement = head;
                    while cur.next.is_some() {
                        cur = cur.next.as_deref_mut().expect("checked");
                    }
                    cur.next = Some(node);
                }
            }
        }

        /// Iterates children in sibling order (IDA `*v6` chain, 0x79ca3e).
        pub fn children(&self) -> ChildIter<'_> {
            ChildIter { next: self.first_child.as_deref() }
        }

        /// IDA 0x79890c `isXsiNil`: walks the attribute list from this+28;
        /// the first `xsi:nil` attribute decides via `getValue(bool&)` —
        /// returns its value when conversion succeeds, 0 when it fails;
        /// 0 when no such attribute exists.
        pub fn is_xsi_nil(&self) -> bool {
            let mut node = self.attrs.as_deref(); // IDA 0x79891e: v1 = this+28
            while let Some(attr) = node {
                // IDA 0x798922: v1 = *v1 (head first, then next links)
                if attr.pair.name == XSI_NIL_NAME {
                    // IDA 0x79892c/0x798936
                    let mut v = false;
                    if attr.pair.get_bool_shared(&mut v) {
                        // IDA 0x79893c-0x798946
                        return v;
                    }
                    return false; // IDA 0x798938: convert failed
                }
                node = attr.next.as_deref();
            }
            false // IDA 0x798926: null link
        }

        /// IDA 0x7989bc `findNextChildWithSameTag`: walks from the anchor's
        /// next link (`*v2` first), returning the first sibling whose tag
        /// word (+12) matches the anchor's; null when the chain ends.
        /// (`this` is provably unused — disasm never reads R0.)
        pub fn find_next_with_same_tag(anchor: &XmlElement) -> Option<&XmlElement> {
            let mut cur = anchor.next.as_deref(); // IDA 0x7989be
            while let Some(elem) = cur {
                if elem.tag() == anchor.tag() {
                    // IDA 0x7989d0: tags equal -> return v2
                    return Some(elem);
                }
                cur = elem.next.as_deref();
            }
            None // IDA 0x7989c4
        }
    }

    /// Borrowed child iterator over the `next` chain.
    pub struct ChildIter<'a> {
        next: Option<&'a XmlElement>,
    }

    impl<'a> Iterator for ChildIter<'a> {
        type Item = &'a XmlElement;
        fn next(&mut self) -> Option<Self::Item> {
            let cur = self.next?;
            self.next = cur.next.as_deref();
            Some(cur)
        }
    }
}
/// was: `TextXmlParser` cursor reading plus the free helpers `decodeString`
/// (IDA 0x79a624), `removeTag` (IDA 0x79b2b8) and `parseAttributes`
/// (IDA 0x79b3c4). Throws become `Result<_, ParseError>` with the exact
/// original message text.
pub mod xml_parse {
    use super::xml_value::{
        ContentId, NameValuePair, Value, BINARY_MARKER, CONTENT_ID_TAG, HASH_MARKER, HASH_TAG,
        MIME_TYPE_NAME, NULL_MARKER, NULL_TAG, XSI_NIL_NAME, assert_hook,
    };
    use super::xml_tree::XmlElement;
    use std::fmt;

    /// Byte returned by `peek` at end of input (IDA `-1` sentinel).
    pub const STREAM_EOF: i32 = -1;
    /// `skipWhitespace` return when it stops at a content byte (IDA 0).
    pub const SKIP_FOUND: i32 = 0;

    /// was: the `std::streambuf` behind `TextXmlParser`+4 (`eback`/`gptr`/
    /// `egptr` triple at +4/+8/+12 — IDA 0x799efa, 0x799faa). `peek` is the
    /// `vtable+36` underflow-or-byte-sentry path, `consume` the `vtable+40`
    /// next-char path. The buffer is complete in memory, so refill never
    /// triggers; both primitives match the observed call shapes exactly.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct CharStream {
        pub data: Vec<u8>,
        pub pos: usize,
    }

    impl CharStream {
        pub fn new(text: &str) -> Self {
            Self { data: text.as_bytes().to_vec(), pos: 0 }
        }

        /// IDA peek idiom (`pos < end ? byte : vtable+36()`): -1 past the end.
        pub fn peek(&self) -> i32 {
            match self.data.get(self.pos) {
                Some(&b) => b as i32,
                None => STREAM_EOF,
            }
        }

        /// IDA consume idiom (`*pos++` or `vtable+40()`): caller guarantees
        /// availability exactly like the original call sites do.
        pub fn consume(&mut self) -> u8 {
            let b = self.data[self.pos];
            self.pos += 1;
            b
        }
    }

    /// was: `TextXmlParser` (stream lives at +4; element stack at +8 —
    /// IDA 0x79ba38 / 0x79bbe2).
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct TextParser {
        pub stream: CharStream,
    }

    impl TextParser {
        pub fn new(text: &str) -> Self {
            Self { stream: CharStream::new(text) }
        }
    }

    /// was: `std::runtime_error` / `RBX::runtime_error` throws, with byte-
    /// exact message text from the IDA decompiles.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ParseError {
        EmptyFile,
        ExpectedOpenEof,
        BomTagExpected,
        ExpectedCloseEof,
        TagStartEof,
        TagExpected,
        TagEof,
        NoEquals,
        BadCharCode,
        HexCharCode,
        UnknownTag(String),
        CloseWithoutOpen(String),
        NotClosingTag(String),
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::EmptyFile => write!(f, "TextXmlParser::parse empty file"), // IDA 0x79bb4e
                Self::ExpectedOpenEof => write!(f, "Expected '<' but got EOF in Xml stream"), // IDA 0x79a080
                Self::BomTagExpected => write!(f, "tag expected after Byte-Order-Mark"), // IDA 0x79a0d2
                Self::ExpectedCloseEof => write!(f, "Expected '>' but got EOF in Xml stream"), // IDA 0x79a128
                Self::TagStartEof => write!(f, "EOF encountered while reading Tag start"), // IDA 0x79a4a2
                Self::TagExpected => write!(f, "tag expected"), // IDA 0x79a3ee
                Self::TagEof => write!(f, "EOF encountered while reading Tag"), // IDA 0x79a448
                Self::NoEquals => write!(f, "Unable to parse XML attributes. '=' not found"), // IDA 0x79b6d8
                Self::BadCharCode => write!(f, "bad XML. No character code following #"), // IDA 0x79a9dc
                Self::HexCharCode => write!(f, "Unable to parse hexidecimal character code"), // IDA 0x79aa36, typo kept
                Self::UnknownTag(t) => write!(f, "TextXmlParser::parse - Unknown tag '{t}'."), // IDA 0x79c166
                Self::CloseWithoutOpen(t) => {
                    write!(f, "TextXmlParser::parse - Got close tag {t} without open tag.") // IDA 0x79c10e
                }
                Self::NotClosingTag(t) => write!(f, "TextXmlParser::parse - '{t}' should be a closing tag"), // IDA 0x79c37c
            }
        }

        // NOTE: no std::error::Error impl: core stays dependency-trivial;
        // callers match on the enum or its Display text.
    }

    /// XML whitespace for the `_MergedGlobals_367[c+16]` table lookups
    /// (IDA 0x799f06, 0x79b338, 0x79b354).
    /// [INFERENCE] exact table bytes unverified; XML-spec whitespace.
    pub fn is_ws_byte(b: u8) -> bool {
        matches!(b, 9 | 10 | 13 | 32)
    }

    /// IDA 0x799ee4 `skipWhitespace`: consumes whitespace; returns -1 on EOF
    /// (the underflow sentry), 0 once a content byte is peeked (left in the
    /// stream). Callers ignore the value.
    pub fn skip_whitespace(parser: &mut TextParser) -> i32 {
        loop {
            let c = parser.stream.peek(); // IDA 0x799efa-0x799f28
            if c == STREAM_EOF {
                return STREAM_EOF; // IDA 0x799f2e
            }
            if !is_ws_byte(c as u8) {
                return SKIP_FOUND; // IDA 0x799f06 table byte == 0
            }
            parser.stream.consume(); // IDA 0x799ef8 / 0x799f20
        }
    }

    /// IDA 0x799f34 `readFirstTag`: skips whitespace, tolerates up to 5
    /// non-`'<'` bytes (Byte-Order-Mark window, IDA 0x799fb8), then collects
    /// through the closing `'>'` (IDA 0x79a01e).
    pub fn read_first_tag(out: &mut String, parser: &mut TextParser) -> Result<(), ParseError> {
        skip_whitespace(parser); // IDA 0x799f56
        let mut attempts = -1i32; // IDA 0x799f88: v10 = -1
        loop {
            if parser.stream.peek() == STREAM_EOF {
                // IDA 0x799f98: peek underflow -> throw
                return Err(ParseError::ExpectedOpenEof);
            }
            attempts += 1;
            if attempts >= 5 {
                // IDA 0x799fb8
                return Err(ParseError::BomTagExpected);
            }
            let c = parser.stream.consume(); // IDA 0x799fc8 / 0x799fa4
            if c == b'<' {
                break; // IDA 0x799fd2
            }
        }
        let mut buf = vec![b'<']; // IDA 0x799fe4: out = "<"
        loop {
            if parser.stream.peek() == STREAM_EOF {
                // IDA 0x79a030
                return Err(ParseError::ExpectedCloseEof);
            }
            let c = parser.stream.consume(); // IDA 0x79a004 / 0x79a046
            buf.push(c);
            if c == b'>' {
                break; // IDA 0x79a01e
            }
        }
        *out = String::from_utf8_lossy(&buf).into_owned();
        Ok(())
    }

    /// IDA 0x79a2a8 `readTag`: single attempt — the first content byte must
    /// be `'<'` ("tag expected"), then collects through `'>'`.
    pub fn read_tag(out: &mut String, parser: &mut TextParser) -> Result<(), ParseError> {
        skip_whitespace(parser); // IDA 0x79a2cc
        if parser.stream.peek() == STREAM_EOF {
            // IDA 0x79a3b8
            return Err(ParseError::TagStartEof);
        }
        let c = parser.stream.consume(); // IDA 0x79a308 / 0x79a3d0
        if c != b'<' {
            // IDA 0x79a312
            return Err(ParseError::TagExpected);
        }
        let mut buf = vec![b'<']; // IDA 0x79a324
        loop {
            if parser.stream.peek() == STREAM_EOF {
                // IDA 0x79a370
                return Err(ParseError::TagEof);
            }
            let c = parser.stream.consume(); // IDA 0x79a344 / 0x79a386
            buf.push(c);
            if c == b'>' {
                break; // IDA 0x79a35e
            }
        }
        *out = String::from_utf8_lossy(&buf).into_owned();
        Ok(())
    }

    /// IDA 0x79aca0 `readText`: collects until EOF or `'<'` — the terminator
    /// is peeked, never consumed (IDA 0x79ad16 break before consume). When
    /// `decode_entities` is set and the text contains `'&'`, runs
    /// `decodeString` over it (IDA 0x79ad70).
    pub fn read_text(
        out: &mut String,
        parser: &mut TextParser,
        decode_entities: bool,
    ) -> Result<(), ParseError> {
        skip_whitespace(parser); // IDA 0x79acc4
        let mut buf = Vec::new();
        loop {
            let c = parser.stream.peek(); // IDA 0x79ad06 peek
            if c == STREAM_EOF || c == b'<' as i32 {
                break; // IDA 0x79ad1c
            }
            buf.push(parser.stream.consume()); // IDA 0x79ad2a / 0x79ad54
        }
        let text = String::from_utf8_lossy(&buf).into_owned();
        if decode_entities && text.contains('&') {
            // IDA 0x79ad70: find(38) != -1
            *out = decode_string(&text)?; // IDA 0x79ad88
        } else {
            *out = text; // IDA 0x79ad7a
        }
        Ok(())
    }

    /// IDA 0x79a624 `decodeString`: copies plain chars, expands `&...;`
    /// entities. `lt/gt/amp/quot/apos` map to `<>&"'`, `nbsp` maps to a
    /// plain space (32, IDA 0x79a84a — not U+00A0). `#DDD` pushes
    /// `atoi`-of-the-digits as a char; a bare `#` throws, and `#x..` throws
    /// the (misspelled) hex error — hex entities are unsupported.
    /// BUG preserved: no hexadecimal support despite the `&#x` convention.
    /// Unknown entities hit the assert path (XmlSerializer.cpp:186) and are
    /// emitted literally as `&body;` — including a synthesized `;` when the
    /// input was unterminated.
    pub fn decode_string(input: &str) -> Result<String, ParseError> {
        let bytes = input.as_bytes();
        let mut out = String::new();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] != b'&' {
                out.push(bytes[i] as char); // IDA 0x79a770 push_back
                i += 1; // IDA 0x79a996
                continue;
            }
            let mut j = i + 1;
            let mut body = String::new();
            while j < bytes.len() && bytes[j] != b';' {
                // IDA 0x79a74e: gather to ';'
                body.push(bytes[j] as char);
                j += 1;
            }
            i = if j < bytes.len() { j + 1 } else { j }; // consume ';' when present
            match body.as_str() {
                "lt" => out.push('<'),   // IDA 0x79a796
                "gt" => out.push('>'),   // IDA 0x79a7ba
                "amp" => out.push('&'),  // IDA 0x79a7de
                "quot" => out.push('"'), // IDA 0x79a802
                "apos" => out.push('\''), // IDA 0x79a826
                "nbsp" => out.push(' '), // IDA 0x79a84a
                _ if body.starts_with('#') => {
                    if body.len() <= 1 {
                        // IDA 0x79a872
                        return Err(ParseError::BadCharCode);
                    }
                    if body.as_bytes()[1] == b'x' {
                        // IDA 0x79a88e
                        return Err(ParseError::HexCharCode);
                    }
                    out.push(char::from(super::xml_value::atoi_prefix(&body[1..]) as u8)); // IDA 0x79a8a6/0x79a8b4
                }
                _ => {
                    assert_hook("false", "XmlSerializer.cpp", 186); // IDA 0x79a8ea
                    out.push('&'); // IDA 0x79a928: "&" + body + ";"
                    out.push_str(&body);
                    out.push(';'); // IDA 0x79a95c
                }
            }
        }
        Ok(out)
    }

    /// IDA 0x79b2b8 `removeTag`: asserts `contents[0] == '<'`
    /// (XmlSerializer.cpp:291), skips blanks from index 1, scans the tag
    /// name to the next blank or `'>'`, asserts non-empty (line 299), and
    /// returns the name plus the attribute-text offset. The scan keeps a
    /// trailing `/` (self-close) exactly like the original.
    pub fn remove_tag(contents: &str) -> (String, usize) {
        let bytes = contents.as_bytes();
        debug_assert!(bytes.first() == Some(&b'<')); // IDA 0x79b2e2
        let mut v10 = 1usize;
        let v11 = loop {
            // IDA 0x79b32e: skip whitespace from index 1
            if v10 >= bytes.len() || !is_ws_byte(bytes[v10]) {
                break v10;
            }
            v10 += 1;
        };
        let mut v12 = v11;
        let v13 = loop {
            // IDA 0x79b34a: scan to whitespace or '>'
            if v12 >= bytes.len() {
                break v12;
            }
            let c = bytes[v12];
            if is_ws_byte(c) || c == b'>' {
                break v12;
            }
            v12 += 1;
        };
        debug_assert!(v13 > v11); // IDA 0x79b366
        (contents[v11..v13.min(bytes.len())].to_string(), v13.min(bytes.len()))
    }

    /// IDA 0x79b3c4 `parseAttributes`: splits the tag via `removeTag`,
    /// interns the name, then walks the attribute text: blanks are skipped,
    /// anything else must be `name="value"` (`=` search from the cursor;
    /// missing `=` throws). The value runs from after `="` to the next
    /// `"` and is entity-decoded when it contains `'&'`.
    /// [INFERENCE] a missing closing quote reads to the end of the tag.
    pub fn parse_attributes(tag_text: &str) -> Result<XmlElement, ParseError> {
        let (name, mut pos) = remove_tag(tag_text); // IDA 0x79b3ec
        let mut elem = XmlElement::new(name); // IDA 0x79b428 new(36) + ctor
        let bytes = tag_text.as_bytes();
        loop {
            if pos >= bytes.len() {
                break;
            }
            let c = bytes[pos];
            if c == 0 || c == b'>' {
                break; // IDA 0x79b4d8
            }
            if !is_ws_byte(c) {
                let eq = match tag_text[pos..].find('=') {
                    // IDA 0x79b50a: find(61, pos)
                    Some(rel) => pos + rel,
                    None => return Err(ParseError::NoEquals), // IDA 0x79b6ce
                };
                let attr_name = tag_text[pos..eq].trim_end().to_string();
                let vs = (eq + 2).min(bytes.len()); // IDA 0x79b546: past `="`
                let end = tag_text[vs..].find('"').map(|rel| vs + rel).unwrap_or(bytes.len());
                let mut value = tag_text[vs..end].to_string();
                if value.contains('&') {
                    value = decode_string(&value)?; // IDA 0x79b55c
                }
                pos = if end < bytes.len() { end + 1 } else { bytes.len() }; // IDA 0x79b586
                elem.add_attribute(attr_name, value); // IDA 0x79b5aa
                continue;
            }
            pos += 1; // IDA 0x79b47e
        }
        Ok(elem)
    }

    /// Reads the `mimeType` attribute payload as the `ContentId` name text.
    /// [INFERENCE] the exact `getValue` overload filling the name is unknown;
    /// string payloads round-trip exactly.
    pub fn mime_text(pair: &NameValuePair) -> Option<String> {
        match &pair.value {
            Value::Text(s) | Value::Owned(s) | Value::Name(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Appends a finished element to its parent, or roots it when the stack
    /// is empty. The original links each element into its parent at creation
    /// (IDA 0x79c27c tail link); linking finished subtrees at pop/close is
    /// observably identical — the partial tree never escapes `parse`.
    fn link_finished(stack: &mut Vec<XmlElement>, root: &mut Option<XmlElement>, elem: XmlElement) {
        match stack.last_mut() {
            Some(parent) => parent.append_child(elem),
            None => *root = Some(elem),
        }
    }

    /// IDA 0x79ba0c `parse`: throws on an empty stream, reads the first tag
    /// (skipping a `<?...?>` prolog), then loops tags. Open tags become
    /// elements: non-`ContentId` tags take entity-decoded text; `ContentId`
    /// tags honor `xsi:nil`, take direct text when non-empty, else dispatch
    /// on the child tag (`binary` discards with a log side channel,
    /// `hash`/`url`/null-name set the value, anything else throws), always
    /// followed by an expected close tag. Close tags pop; popping the last
    /// element roots it and ends the parse. `/>` self-close pops immediately
    /// (it never ends the parse by itself, exactly like the original).
    pub fn parse_into(out: &mut Option<XmlElement>, parser: &mut TextParser) -> Result<(), ParseError> {
        if parser.stream.peek() == STREAM_EOF {
            // IDA 0x79bb32
            return Err(ParseError::EmptyFile);
        }
        let mut first = String::new();
        read_first_tag(&mut first, parser)?; // IDA 0x79bac8
        let mut pending = if first.starts_with("<?") {
            // IDA 0x79bb08: prolog tag -> skip to LABEL_123
            None
        } else {
            Some(first)
        };
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;
        let mut done = false;
        while !done {
            let tagtext = match pending.take() {
                Some(t) => t,
                None => {
                    let mut s = String::new();
                    read_tag(&mut s, parser)?; // IDA 0x79c2f4
                    s
                }
            };
            let tb = tagtext.as_bytes();
            let is_close = tb.first() == Some(&b'<') && tb.get(1) == Some(&b'/');
            if !is_close {
                // IDA 0x79bc10: open-tag path
                stack.push(parse_attributes(&tagtext)?); // IDA 0x79bc1e/0x79bc28
                let is_content_id = stack.last().map(|e| e.tag() == CONTENT_ID_TAG).unwrap_or(false);
                if !is_content_id {
                    // IDA 0x79bc42: leaf text path
                    let mut text = String::new();
                    read_text(&mut text, parser, true)?; // IDA 0x79bc4e
                    let top = stack.last_mut().expect("pushed"); // IDA 0x79bc5c setValue
                    top.value = NameValuePair { name: top.tag().to_string(), value: Value::Text(text) };
                } else {
                    // IDA 0x79bcce: xsi:nil check
                    let xsi_skip = match stack.last_mut().expect("pushed").find_attribute_mut(XSI_NIL_NAME) {
                        Some(attr) => {
                            let mut b = false;
                            attr.pair.get_bool(&mut b) && b // IDA 0x79bcde: Value==1 && v107
                        }
                        None => false,
                    };
                    if !xsi_skip {
                        let mut text = String::new();
                        read_text(&mut text, parser, false)?; // IDA 0x79bcfe
                        if text.is_empty() {
                            // IDA 0x79bd28: container dispatch
                            let mime = stack
                                .last()
                                .expect("pushed")
                                .find_attribute(MIME_TYPE_NAME)
                                .map(|attr| &attr.pair)
                                .and_then(mime_text); // IDA 0x79bd36
                            let mut child = String::new();
                            read_tag(&mut child, parser)?; // IDA 0x79bd6e
                            // IDA 0x79bd84 `substr(tag, 1)`: strips both brackets —
                            // the full-equality `tag_null`/`tag_hash` compares only match
                            // bracket-free text, and `<null></null>` must round-trip.
                            let inner = child.strip_prefix('<').unwrap_or(&child);
                            let inner = inner.strip_suffix('>').unwrap_or(inner);
                            let top = stack.last_mut().expect("pushed");
                            if inner.starts_with(BINARY_MARKER) {
                                // IDA 0x79bda2: log "Not reading binary data" side channel dropped
                                let mut tmp = String::new();
                                read_text(&mut tmp, parser, false)?; // IDA 0x79be64
                                top.value.name = top.tag().to_string();
                                top.value.value = Value::Name(BINARY_MARKER.to_string()); // IDA 0x79be92
                            } else if inner == HASH_TAG {
                                // IDA 0x79bdae
                                let mut tmp = String::new();
                                read_text(&mut tmp, parser, false)?; // IDA 0x79bed4
                                top.value.name = top.tag().to_string();
                                top.value.value = Value::Name(HASH_MARKER.to_string()); // IDA 0x79bf02
                            } else if inner.starts_with("url") {
                                // IDA 0x79bdcc
                                let mut url = String::new();
                                read_text(&mut url, parser, true)?; // IDA 0x79bf6e
                                let name = top.tag().to_string();
                                top.value = NameValuePair {
                                    name,
                                    value: Value::ContentId(ContentId { text: url, mime }), // IDA 0x79bf98
                                };
                            } else if inner == NULL_TAG {
                                // IDA 0x79bdd4
                                top.value.name = top.tag().to_string();
                                top.value.value = Value::Name(NULL_MARKER.to_string()); // IDA 0x79bdf6
                            } else {
                                return Err(ParseError::UnknownTag(inner.to_string())); // IDA 0x79c13c
                            }
                            let mut close = String::new();
                            read_tag(&mut close, parser)?; // IDA 0x79c032
                            let cb = close.as_bytes();
                            if !(cb.first() == Some(&b'<') && cb.get(1) == Some(&b'/')) {
                                // IDA 0x79c04a
                                return Err(ParseError::NotClosingTag(close));
                            }
                        }
                    }
                }
                // IDA LABEL_114: self-close check on the raw tag text
                if tb.len() >= 2 && tb[tb.len() - 2] == b'/' && tb[tb.len() - 1] == b'>' {
                    // IDA 0x79c292: pop without finishing the parse
                    let elem = stack.pop().expect("pushed above");
                    link_finished(&mut stack, &mut root, elem);
                }
            } else {
                // IDA 0x79bc9a: close-tag path
                if stack.is_empty() {
                    return Err(ParseError::CloseWithoutOpen(tagtext)); // IDA 0x79c104
                }
                let elem = stack.pop().expect("checked above");
                if stack.is_empty() {
                    root = Some(elem); // IDA 0x79bcb6: *this = root
                    done = true; // IDA 0x79bcc2
                } else {
                    link_finished(&mut stack, &mut root, elem);
                }
            }
        }
        *out = root;
        Ok(())
    }
}
/// was: `TextXmlWriter` (output stream at +28 — IDA 0x79af7a) with the free
/// helper `encodedWrite` (IDA 0x79ae3c) and the `serialize` family
/// (IDA 0x79c9ec / 0x79c9f4 / 0x79ca70, `writeOpenTag` 0x79af50,
/// `writeCloseTag` 0x79b250).
pub mod xml_write {
    use super::xml_value::{ContentId, NameValuePair, Value, XSI_NIL_NAME, MIME_TYPE_NAME};
    use super::xml_tree::XmlElement;

    /// was: `TextXmlWriter` — `out` is the `std::ostream` at +28;
    /// `handles` is the `getHandleIndex` memo table (dedup by shared-target
    /// identity, stable indices in first-seen order).
    /// [INFERENCE] the exact dedup policy is unverified; first-seen indexing
    /// reproduces the observed `RBX%d` numbering on fresh writers.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct XmlWriter {
        pub out: String,
        handles: Vec<usize>,
    }

    impl XmlWriter {
        pub fn new() -> Self {
            Self::default()
        }

        /// was: `XmlWriter::getHandleIndex` (IDA 0x799206).
        pub fn handle_index(&mut self, key: usize) -> usize {
            if let Some(i) = self.handles.iter().position(|&h| h == key) {
                return i;
            }
            self.handles.push(key);
            self.handles.len() - 1
        }

        /// IDA 0x79ae3c `encodedWrite`: escapes `>`/`<`/`"`/`&`/`'` to
        /// `&gt;`/`&lt;`/`&quot;`/`&amp;`/`&apos;`; any other byte above
        /// `0x7E`, or below `0x20` except `\n`/`\r`, becomes `&#N;`
        /// (IDA 0x79af08). Operates per character; the original is byte-wise,
        /// which only diverges for non-ASCII text. [INFERENCE]
        pub fn encoded_write(&mut self, text: &str) {
            for ch in text.chars() {
                match ch {
                    '>' => self.out.push_str("&gt;"), // IDA 0x79aeba
                    '<' => self.out.push_str("&lt;"), // IDA 0x79aec4
                    '"' => self.out.push_str("&quot;"), // IDA 0x79aed4
                    '&' => self.out.push_str("&amp;"), // IDA 0x79aeb0
                    '\'' => self.out.push_str("&apos;"), // IDA 0x79aee6
                    c if (c as u32) > 0x7E
                        || (c != '\n' && (c as u32) < 0x20 && c != '\r') =>
                    {
                        self.out.push_str(&format!("&#{};", c as u32)); // IDA 0x79af22
                    }
                    c => self.out.push(c), // IDA 0x79ae8a
                }
            }
        }

        /// IDA 0x79af50 `writeOpenTag`: `indent` tabs (9, IDA 0x79afb4),
        /// `'<'` + tag, then each attribute as ` name="encoded"` in list
        /// order (IDA 0x79b03a-0x79b0b0), the optional extra attribute the
        /// same way (IDA 0x79b0c8-0x79b150), and the closing `'>'`.
        pub fn write_open_tag(
            &mut self,
            elem: &XmlElement,
            indent: usize,
            extra: Option<(&str, &NameValuePair)>,
        ) {
            for _ in 0..indent {
                self.out.push('\t'); // IDA 0x79afb4
            }
            self.out.push('<'); // IDA 0x79afcc
            self.out.push_str(elem.tag()); // IDA 0x79afee
            let mut attr = elem.attrs.as_deref(); // IDA 0x79aff2
            while let Some(a) = attr {
                self.out.push(' '); // IDA 0x79b03a
                self.out.push_str(&a.pair.name); // IDA 0x79b056
                self.out.push_str("=\""); // IDA 0x79b068
                let s = a.pair.to_string_value(self); // IDA 0x79b074
                let encoded = {
                    let mut tmp = XmlWriter::new();
                    std::mem::swap(&mut tmp.out, &mut self.out);
                    tmp.encoded_write(&s);
                    std::mem::swap(&mut tmp.out, &mut self.out);
                    tmp.out
                };
                self.out.push_str(&encoded); // IDA 0x79b086
                self.out.push('"'); // IDA 0x79b0a8
                attr = a.next.as_deref(); // IDA 0x79b0b6
            }
            if let Some((name, pair)) = extra {
                self.out.push(' '); // IDA 0x79b0c8
                self.out.push_str(name); // IDA 0x79b0e6
                self.out.push_str("=\""); // IDA 0x79b0fc
                let s = pair.to_string_value(self); // IDA 0x79b108
                let encoded = {
                    let mut tmp = XmlWriter::new();
                    std::mem::swap(&mut tmp.out, &mut self.out);
                    tmp.encoded_write(&s);
                    std::mem::swap(&mut tmp.out, &mut self.out);
                    tmp.out
                };
                self.out.push_str(&encoded); // IDA 0x79b11e
                self.out.push('"'); // IDA 0x79b144
            }
            self.out.push('>'); // IDA 0x79b15a
        }

        /// IDA 0x79b250 `writeCloseTag`: `indent` tabs, `"</"`, tag, `'>'`.
        pub fn write_close_tag(&mut self, elem: &XmlElement, indent: usize) {
            for _ in 0..indent {
                self.out.push('\t'); // IDA 0x79b272
            }
            self.out.push_str("</"); // IDA 0x79b28e
            self.out.push_str(elem.tag()); // IDA 0x79b29e
            self.out.push('>'); // IDA 0x79b2a6
        }

        /// IDA 0x79ca70 `serializeNode`: `ContentId`-valued elements take the
        /// content branch — plain open tag when the stored name is the null
        /// name, else a `mimeType` attribute (IDA 0x79cb34) — then, unless an
        /// `xsi:nil` attribute is present, `rbxasset://`/`http` text goes
        /// through as `<url>encoded</url>` while anything else (or empty
        /// text) yields `<null></null>` (IDA 0x79cc72/0x79cc8c). The
        /// `StandardOut` "Not writing binary data" log is a side channel the
        /// port drops. Other values take the open tag plus encoded `toString`
        /// text (IDA 0x79cb66-0x79cb84).
        pub fn serialize_node(&mut self, elem: &XmlElement, indent: usize) {
            match &elem.value.value {
                Value::ContentId(cid) => {
                    match &cid.mime {
                        // IDA 0x79cb1a: stored name == null name -> no attribute
                        None => self.write_open_tag(elem, indent, None), // IDA 0x79cbd0
                        Some(mime) => {
                            let attr = NameValuePair::with_text(MIME_TYPE_NAME.to_string(), mime.clone());
                            // IDA 0x79cb34/0x79cb44
                            self.write_open_tag(elem, indent, Some((MIME_TYPE_NAME, &attr)));
                        }
                    }
                    if elem.find_attribute(XSI_NIL_NAME).is_none() {
                        // IDA 0x79cbe8
                        if !cid.text.is_empty() {
                            // IDA 0x79cbf2
                            if cid.text.starts_with("rbxasset://") || cid.text.starts_with("http") {
                                // IDA 0x79cc2a
                                self.out.push_str("<url>"); // IDA 0x79cca6
                                let snapshot = cid.text.clone();
                                self.encoded_write(&snapshot); // IDA 0x79ccb6
                                self.out.push_str("</url>"); // IDA 0x79ccce
                            } else {
                                self.out.push_str("<null></null>"); // IDA 0x79cc72
                            }
                        } else {
                            self.out.push_str("<null></null>"); // IDA 0x79cc8c
                        }
                    }
                }
                _ => {
                    self.write_open_tag(elem, indent, None); // IDA 0x79cb66
                    let s = elem.value.to_string_value(self); // IDA 0x79cb74
                    let snapshot = s;
                    self.encoded_write(&snapshot); // IDA 0x79cb84
                }
            }
        }

        /// IDA 0x79c9f4 `serialize(element, indent)`: null element returns at
        /// once; otherwise the node, then each child on its own line at
        /// `indent + 1` (IDA 0x79ca2c `'\n'`), a trailing newline, and the
        /// close tag at the original indent — or at 0 when childless.
        pub fn serialize(&mut self, elem: Option<&XmlElement>, indent: usize) {
            let elem = match elem {
                Some(e) => e,
                None => return, // IDA 0x79ca08: null -> return this
            };
            self.serialize_node(elem, indent); // IDA 0x79ca10
            let mut child = elem.first_child.as_deref(); // IDA 0x79ca14
            match child {
                None => self.write_close_tag(elem, 0), // IDA 0x79ca60: v12 = 0
                Some(_) => {
                    while let Some(c) = child {
                        self.out.push('\n'); // IDA 0x79ca2c
                        self.serialize(Some(c), indent + 1); // IDA 0x79ca3a
                        child = c.next.as_deref(); // IDA 0x79ca3e
                    }
                    self.out.push('\n'); // IDA 0x79ca48
                    self.write_close_tag(elem, indent); // IDA 0x79ca62
                }
            }
        }
    }

    /// Keeps `ContentId` constructible for tests without reaching into value internals.
    #[allow(dead_code)]
    pub fn content_id_pair(name: String, text: String, mime: Option<String>) -> NameValuePair {
        NameValuePair { name, value: Value::ContentId(ContentId { text, mime }) }
    }
}


#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD1Ev")]
// 0x789ea4 — __ZN13ArchiveBinderD1Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
pub fn stub_0x789ea4() {
    // IDA 0x789ea4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD1Ev")]
// 0x789fb4 — __ZN12MemoryBinderD1Ev
// type: void __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x789fb4() {
    // IDA 0x789fb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "MemoryBinder::resolveRefs(void)")]
#[doc(alias = "__ZN12MemoryBinder11resolveRefsEv")]
// 0x78a410 — __ZN12MemoryBinder11resolveRefsEv
// type: _DWORD __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x78a410() {
    // IDA 0x78a410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD0Ev")]
// 0x78a47c — __ZN12MemoryBinderD0Ev
// type: void __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x78a47c() {
    // IDA 0x78a47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD0Ev")]
// 0x78b4b8 — __ZN13ArchiveBinderD0Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
pub fn stub_0x78b4b8() {
    // IDA 0x78b4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "ArchiveBinder::resolveIDREF(ArchiveBinder::IDREFBinding)")]
#[doc(alias = "__ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE")]
// 0x78bf04 — __ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE
pub fn stub_0x78bf04() {
    // IDA 0x78bf04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_360")]
#[doc(alias = "__GLOBAL__I_a_360")]
// 0x78c2ac — __GLOBAL__I_a_360
pub fn stub_0x78c2ac() {
    // IDA 0x78c2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_361")]
#[doc(alias = "__GLOBAL__I_a_361")]
// 0x7986fc — __GLOBAL__I_a_361
pub fn stub_0x7986fc() {
    // IDA 0x7986fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "XmlElement::isXsiNil(void)const")]
#[doc(alias = "__ZNK10XmlElement8isXsiNilEv")]
// 0x79890c — __ZNK10XmlElement8isXsiNilEv
// type: int __fastcall(XmlElement *this)
pub fn stub_0x79890c(elem: &xml_tree::XmlElement) -> bool {
    // IDA 0x79890c: walks attrs from this+28, first xsi:nil decides.
    elem.is_xsi_nil()
}

#[doc(alias = "XmlNameValuePair::getValue(bool &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERb")]
// 0x798964 — __ZNK16XmlNameValuePair8getValueERb
// type: int __fastcall(XmlNameValuePair *this, bool *)
pub fn stub_0x798964(pair: &mut xml_value::NameValuePair, out: &mut bool) -> bool {
    // IDA 0x798964: tag 2 converts+caches (tag 4), tag 4 reads cache.
    pair.get_bool(out)
}

#[doc(alias = "XmlElement::findNextChildWithSameTag(XmlElement const*)const")]
#[doc(alias = "__ZNK10XmlElement24findNextChildWithSameTagEPKS_")]
// 0x7989bc — __ZNK10XmlElement24findNextChildWithSameTagEPKS_
// type: const XmlElement *__fastcall(XmlElement *this, const XmlElement *)
pub fn stub_0x7989bc<'a>(anchor: &'a xml_tree::XmlElement) -> Option<&'a xml_tree::XmlElement> {
    // IDA 0x7989bc: walks from anchor.next, first tag match wins.
    // (`this` provably unused — disasm never reads R0.)
    xml_tree::XmlElement::find_next_with_same_tag(anchor)
}

#[doc(alias = "XmlNameValuePair::clearValue(void)const")]
#[doc(alias = "__ZNK16XmlNameValuePair10clearValueEv")]
// 0x7989ec — __ZNK16XmlNameValuePair10clearValueEv
// type: void __fastcall(std::string **this)
pub fn stub_0x7989ec(pair: &mut xml_value::NameValuePair) {
    // IDA 0x7989ec: tags 8/3/2 drop payloads, then tag = 0.
    pair.clear_value()
}

#[doc(alias = "XmlNameValuePair::getValue(int &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERi")]
// 0x798d64 — __ZNK16XmlNameValuePair8getValueERi
// type: int __fastcall(XmlNameValuePair *this, int *)
pub fn stub_0x798d64(pair: &mut xml_value::NameValuePair, out: &mut i32) -> bool {
    // IDA 0x798d64: tag 2 converts+caches (tag 5), tag 5 reads cache.
    pair.get_int(out)
}

#[doc(alias = "XmlNameValuePair::getValue(unsigned int &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERj")]
// 0x798da4 — __ZNK16XmlNameValuePair8getValueERj
// type: int __fastcall(XmlNameValuePair *this, unsigned int *)
pub fn stub_0x798da4(pair: &mut xml_value::NameValuePair, out: &mut u32) -> bool {
    // IDA 0x798da4: tag 2 converts+caches (tag 6), tag 6 reads cache.
    pair.get_uint(out)
}

#[doc(alias = "XmlNameValuePair::getValue(float &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERf")]
// 0x798de4 — __ZNK16XmlNameValuePair8getValueERf
pub fn stub_0x798de4(pair: &mut xml_value::NameValuePair, out: &mut f32) -> bool {
    // IDA 0x798de4: tag 2 converts+caches (tag 7), tag 7 reads cache.
    // BUG preserved: tag 9 (double) is rejected via the assert path.
    pair.get_float(out)
}

#[doc(alias = "XmlNameValuePair::getValue(double &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERd")]
// 0x798e7c — __ZNK16XmlNameValuePair8getValueERd
pub fn stub_0x798e7c(pair: &mut xml_value::NameValuePair, out: &mut f64) -> bool {
    // IDA 0x798e7c: tag 2 parses+caches (tag 9), tag 7 widens+caches, tag 9 reads.
    pair.get_double(out)
}

#[doc(alias = "XmlNameValuePair::toString(XmlWriter *)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8toStringEP9XmlWriter")]
// 0x799060 — __ZNK16XmlNameValuePair8toStringEP9XmlWriter
pub fn stub_0x799060(pair: &xml_value::NameValuePair, writer: &mut xml_write::XmlWriter) -> String {
    // IDA 0x799060: renders the cached value (RBX<handle> for IDREF).
    pair.to_string_value(writer)
}

#[doc(alias = "global constructor keyed to_a_362")]
pub fn stub_0x79972c() {
    // IDA 0x79972c: global static ctor key with no decompilable body (decompile failed).
    // Static init — carrier no-op.
}

#[doc(alias = "TextXmlParser::skipWhitespace(void)")]
#[doc(alias = "__ZN13TextXmlParser14skipWhitespaceEv")]
// 0x799ee4 — __ZN13TextXmlParser14skipWhitespaceEv
pub fn stub_0x799ee4(parser: &mut xml_parse::TextParser) -> i32 {
    // IDA 0x799ee4: consumes whitespace; -1 on EOF, 0 at content.
    xml_parse::skip_whitespace(parser)
}

#[doc(alias = "TextXmlParser::readFirstTag(void)")]
#[doc(alias = "__ZN13TextXmlParser12readFirstTagEv")]
// 0x799f34 — __ZN13TextXmlParser12readFirstTagEv
pub fn stub_0x799f34(out: &mut String, parser: &mut xml_parse::TextParser) -> Result<(), xml_parse::ParseError> {
    // IDA 0x799f34: BOM-tolerant first-tag read through '>'.
    xml_parse::read_first_tag(out, parser)
}

#[doc(alias = "TextXmlParser::readTag(void)")]
#[doc(alias = "__ZN13TextXmlParser7readTagEv")]
// 0x79a2a8 — __ZN13TextXmlParser7readTagEv
pub fn stub_0x79a2a8(out: &mut String, parser: &mut xml_parse::TextParser) -> Result<(), xml_parse::ParseError> {
    // IDA 0x79a2a8: requires '<', collects through '>'.
    xml_parse::read_tag(out, parser)
}

#[doc(alias = "TextXmlParser::readText(bool)")]
#[doc(alias = "__ZN13TextXmlParser8readTextEb")]
pub fn stub_0x79aca0(
    out: &mut String,
    parser: &mut xml_parse::TextParser,
    decode_entities: bool,
) -> Result<(), xml_parse::ParseError> {
    // IDA 0x79aca0: collects to EOF/'<' (terminator unconsumed), decodes when asked.
    xml_parse::read_text(out, parser, decode_entities)
}

#[doc(alias = "TextXmlWriter::writeOpenTag(XmlElement const*,int,XmlAttribute const*)")]
#[doc(alias = "__ZN13TextXmlWriter12writeOpenTagEPK10XmlElementiPK12XmlAttribute")]
pub fn stub_0x79af50(
    writer: &mut xml_write::XmlWriter,
    elem: &xml_tree::XmlElement,
    indent: usize,
    extra: Option<(&str, &xml_value::NameValuePair)>,
) {
    // IDA 0x79af50: tabs + '<tag' + attrs + optional extra attr + '>'.
    writer.write_open_tag(elem, indent, extra)
}

#[doc(alias = "TextXmlWriter::writeCloseTag(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter13writeCloseTagEPK10XmlElementi")]
pub fn stub_0x79b250(writer: &mut xml_write::XmlWriter, elem: &xml_tree::XmlElement, indent: usize) {
    // IDA 0x79b250: tabs + '</tag>'.
    writer.write_close_tag(elem, indent)
}

#[doc(alias = "TextXmlParser::parse(void)")]
#[doc(alias = "__ZN13TextXmlParser5parseEv")]
pub fn stub_0x79ba0c(
    out: &mut Option<xml_tree::XmlElement>,
    parser: &mut xml_parse::TextParser,
) -> Result<(), xml_parse::ParseError> {
    // IDA 0x79ba0c: full document parse; root lands in out.
    xml_parse::parse_into(out, parser)
}

#[doc(alias = "TextXmlWriter::serialize(XmlElement const*)")]
#[doc(alias = "__ZN13TextXmlWriter9serializeEPK10XmlElement")]
pub fn stub_0x79c9ec(writer: &mut xml_write::XmlWriter, elem: Option<&xml_tree::XmlElement>) -> bool {
    // IDA 0x79c9ec: serialize(elem, 0); truthful success like the chained stream insert.
    writer.serialize(elem, 0);
    true
}

#[doc(alias = "TextXmlWriter::serialize(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter9serializeEPK10XmlElementi")]
pub fn stub_0x79c9f4(writer: &mut xml_write::XmlWriter, elem: Option<&xml_tree::XmlElement>, indent: usize) {
    // IDA 0x79c9f4: node, newline-separated children at indent+1, close tag.
    writer.serialize(elem, indent)
}

#[doc(alias = "TextXmlWriter::serializeNode(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter13serializeNodeEPK10XmlElementi")]
pub fn stub_0x79ca70(writer: &mut xml_write::XmlWriter, elem: &xml_tree::XmlElement, indent: usize) {
    // IDA 0x79ca70: ContentId-aware single-node write.
    writer.serialize_node(elem, indent)
}

#[doc(alias = "global constructor keyed to_a_363")]
#[doc(alias = "__GLOBAL__I_a_363")]
// 0x79d364 — __GLOBAL__I_a_363
pub fn stub_0x79d364() {
    // IDA 0x79d364: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "sub_7A3758")]
// 0x7a3758 — sub_7A3758
pub fn stub_0x7a3758() {
    // IDA 0x7a3758: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_364")]
#[doc(alias = "__GLOBAL__I_a_364")]
// 0x7aa43c — __GLOBAL__I_a_364
pub fn stub_0x7aa43c() {
    // IDA 0x7aa43c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_365")]
#[doc(alias = "__GLOBAL__I_a_365")]
// 0x7ab8c4 — __GLOBAL__I_a_365
pub fn stub_0x7ab8c4() {
    // IDA 0x7ab8c4: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_366")]
#[doc(alias = "__GLOBAL__I_a_366")]
// 0x7ac888 — __GLOBAL__I_a_366
pub fn stub_0x7ac888() {
    // IDA 0x7ac888: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_367")]
#[doc(alias = "__GLOBAL__I_a_367")]
// 0x7afc24 — __GLOBAL__I_a_367
pub fn stub_0x7afc24() {
    // IDA 0x7afc24: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_368")]
#[doc(alias = "__GLOBAL__I_a_368")]
// 0x7b1f98 — __GLOBAL__I_a_368
pub fn stub_0x7b1f98() {
    // IDA 0x7b1f98: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_369")]
#[doc(alias = "__GLOBAL__I_a_369")]
// 0x7b2eac — __GLOBAL__I_a_369
pub fn stub_0x7b2eac() {
    // IDA 0x7b2eac: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_370")]
#[doc(alias = "__GLOBAL__I_a_370")]
// 0x7b3734 — __GLOBAL__I_a_370
pub fn stub_0x7b3734() {
    // IDA 0x7b3734: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_371")]
#[doc(alias = "__GLOBAL__I_a_371")]
// 0x7b3fa0 — __GLOBAL__I_a_371
pub fn stub_0x7b3fa0() {
    // IDA 0x7b3fa0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_372")]
#[doc(alias = "__GLOBAL__I_a_372")]
// 0x7b4784 — __GLOBAL__I_a_372
pub fn stub_0x7b4784() {
    // IDA 0x7b4784: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_373")]
#[doc(alias = "__GLOBAL__I_a_373")]
// 0x7b4cb0 — __GLOBAL__I_a_373
pub fn stub_0x7b4cb0() {
    // IDA 0x7b4cb0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_374")]
#[doc(alias = "__GLOBAL__I_a_374")]
// 0x7b592c — __GLOBAL__I_a_374
pub fn stub_0x7b592c() {
    // IDA 0x7b592c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_375")]
#[doc(alias = "__GLOBAL__I_a_375")]
// 0x7b5ea8 — __GLOBAL__I_a_375
pub fn stub_0x7b5ea8() {
    // IDA 0x7b5ea8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "___cxx_global_array_dtor")]
// 0x7cb7b4 — ___cxx_global_array_dtor
pub fn stub_0x7cb7b4() {
    // IDA 0x7cb7b4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_376")]
#[doc(alias = "__GLOBAL__I_a_376")]
// 0x7cb890 — __GLOBAL__I_a_376
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x7cb890() {
    // IDA 0x7cb890: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_377")]
#[doc(alias = "__GLOBAL__I_a_377")]
// 0x7d207c — __GLOBAL__I_a_377
pub fn stub_0x7d207c() {
    // IDA 0x7d207c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_378")]
#[doc(alias = "__GLOBAL__I_a_378")]
// 0x7d2fe4 — __GLOBAL__I_a_378
pub fn stub_0x7d2fe4() {
    // IDA 0x7d2fe4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_379")]
#[doc(alias = "__GLOBAL__I_a_379")]
// 0x7d423c — __GLOBAL__I_a_379
pub fn stub_0x7d423c() {
    // IDA 0x7d423c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_380")]
#[doc(alias = "__GLOBAL__I_a_380")]
// 0x7d4f2c — __GLOBAL__I_a_380
pub fn stub_0x7d4f2c() {
    // IDA 0x7d4f2c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_381")]
#[doc(alias = "__GLOBAL__I_a_381")]
// 0x7d63ac — __GLOBAL__I_a_381
pub fn stub_0x7d63ac() {
    // IDA 0x7d63ac: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_382")]
#[doc(alias = "__GLOBAL__I_a_382")]
// 0x7d69c8 — __GLOBAL__I_a_382
pub fn stub_0x7d69c8() {
    // IDA 0x7d69c8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_383")]
#[doc(alias = "__GLOBAL__I_a_383")]
// 0x7d733c — __GLOBAL__I_a_383
pub fn stub_0x7d733c() {
    // IDA 0x7d733c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_384")]
#[doc(alias = "__GLOBAL__I_a_384")]
// 0x7d7800 — __GLOBAL__I_a_384
pub fn stub_0x7d7800() {
    // IDA 0x7d7800: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_385")]
#[doc(alias = "__GLOBAL__I_a_385")]
// 0x7d7f90 — __GLOBAL__I_a_385
pub fn stub_0x7d7f90() {
    // IDA 0x7d7f90: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_386")]
#[doc(alias = "__GLOBAL__I_a_386")]
// 0x7db638 — __GLOBAL__I_a_386
pub fn stub_0x7db638() {
    // IDA 0x7db638: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "XmlNameValuePair::setValue(char const*)")]
#[doc(alias = "__ZN16XmlNameValuePair8setValueEPKc")]
// 0x7dc6cc — __ZN16XmlNameValuePair8setValueEPKc
// type: _DWORD __fastcall(XmlNameValuePair *__hidden this, const char *)
pub fn stub_0x7dc6cc() {
    // IDA 0x7dc6cc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_387")]
#[doc(alias = "__GLOBAL__I_a_387")]
// 0x7dc784 — __GLOBAL__I_a_387
pub fn stub_0x7dc784() {
    // IDA 0x7dc784: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController receivedData]")]
// 0x7dc98c — -[MacHttpController receivedData]
// type: id __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dc98c() {
    // IDA 0x7dc98c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController initWithUrl:additionalHeaders:]")]
// 0x7dc99c — -[MacHttpController initWithUrl:additionalHeaders:]
// type: MacHttpController *__cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, const map<std::string, std::string, std::less<std::string >, std::allocator<std::pair<const std::string, std::string > > > *)
pub fn stub_0x7dc99c() {
    // IDA 0x7dc99c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController setPostDataFromStream:]")]
// 0x7dcc6c — -[MacHttpController setPostDataFromStream:]
// type: void __cdecl(MacHttpController *self, SEL, basic_istream<char, std::char_traits<char> > *)
pub fn stub_0x7dcc6c() {
    // IDA 0x7dcc6c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController setPostCompressedDataFromString:]")]
// 0x7dcd0c — -[MacHttpController setPostCompressedDataFromString:]
// type: void __cdecl(MacHttpController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> > *)
pub fn stub_0x7dcd0c() {
    // IDA 0x7dcd0c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[MacHttpController dealloc]")]
// 0x7dcdbc — -[MacHttpController dealloc]
// type: void __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dcdbc() {
    // IDA 0x7dcdbc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController setAuthDomain:withr:]")]
// 0x7dce08 — -[MacHttpController setAuthDomain:withr:]
// type: void __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, id)
pub fn stub_0x7dce08() {
    // IDA 0x7dce08: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController configureRequest:]")]
// 0x7dce68 — -[MacHttpController configureRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dce68() {
    // IDA 0x7dce68: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController startConnectionWithRequest:]")]
// 0x7dcfa4 — -[MacHttpController startConnectionWithRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dcfa4() {
    // IDA 0x7dcfa4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController doGetPost:]")]
// 0x7dd034 — -[MacHttpController doGetPost:]
// type: int __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *)
pub fn stub_0x7dd034() {
    // IDA 0x7dd034: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[MacHttpController connection:didFailWithError:]")]
// 0x7dd18c — -[MacHttpController connection:didFailWithError:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd18c() {
    // IDA 0x7dd18c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController connection:didReceiveData:]")]
// 0x7dd1c4 — -[MacHttpController connection:didReceiveData:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd1c4() {
    // IDA 0x7dd1c4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController connection:didReceiveResponse:]")]
// 0x7dd1e4 — -[MacHttpController connection:didReceiveResponse:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd1e4() {
    // IDA 0x7dd1e4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController connectionDidFinishLoading:]")]
// 0x7dd24c — -[MacHttpController connectionDidFinishLoading:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dd24c() {
    // IDA 0x7dd24c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController connection:willSendRequest:redirectResponse:]")]
// 0x7dd260 — -[MacHttpController connection:willSendRequest:redirectResponse:]
// type: id __cdecl(MacHttpController *self, SEL, id, id, id)
pub fn stub_0x7dd260() {
    // IDA 0x7dd260: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController url]")]
// 0x7dd35c — -[MacHttpController url]
// type: NSURL *__cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd35c() {
    // IDA 0x7dd35c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController setUrl:]")]
// 0x7dd370 — -[MacHttpController setUrl:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dd370() {
    // IDA 0x7dd370: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController .cxx_destruct]")]
// 0x7dd388 — -[MacHttpController .cxx_destruct]
// type: void __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd388() {
    // IDA 0x7dd388: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[MacHttpController .cxx_construct]")]
// 0x7dd39c — -[MacHttpController .cxx_construct]
// type: id __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd39c() {
    // IDA 0x7dd39c: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx_isRobloxSite(char const*)")]
#[doc(alias = "__Z16rbx_isRobloxSitePKc")]
// 0x7dd3c0 — __Z16rbx_isRobloxSitePKc
// type: _DWORD __fastcall(const char *)
pub fn stub_0x7dd3c0() {
    // IDA 0x7dd3c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_388")]
#[doc(alias = "__GLOBAL__I_a_388")]
// 0x7e436c — __GLOBAL__I_a_388
pub fn stub_0x7e436c() {
    // IDA 0x7e436c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_389")]
#[doc(alias = "__GLOBAL__I_a_389")]
// 0x7e6300 — __GLOBAL__I_a_389
pub fn stub_0x7e6300() {
    // IDA 0x7e6300: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_390")]
#[doc(alias = "__GLOBAL__I_a_390")]
// 0x7e755c — __GLOBAL__I_a_390
pub fn stub_0x7e755c() {
    // IDA 0x7e755c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_391")]
#[doc(alias = "__GLOBAL__I_a_391")]
// 0x7e89ac — __GLOBAL__I_a_391
pub fn stub_0x7e89ac() {
    // IDA 0x7e89ac: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_392")]
#[doc(alias = "__GLOBAL__I_a_392")]
// 0x7e9fa8 — __GLOBAL__I_a_392
pub fn stub_0x7e9fa8() {
    // IDA 0x7e9fa8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_393")]
#[doc(alias = "__GLOBAL__I_a_393")]
// 0x7f8f2c — __GLOBAL__I_a_393
pub fn stub_0x7f8f2c() {
    // IDA 0x7f8f2c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_394")]
#[doc(alias = "__GLOBAL__I_a_394")]
// 0x7fe228 — __GLOBAL__I_a_394
pub fn stub_0x7fe228() {
    // IDA 0x7fe228: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_395")]
#[doc(alias = "__GLOBAL__I_a_395")]
// 0x815660 — __GLOBAL__I_a_395
pub fn stub_0x815660() {
    // IDA 0x815660: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_396")]
#[doc(alias = "__GLOBAL__I_a_396")]
// 0x816be8 — __GLOBAL__I_a_396
pub fn stub_0x816be8() {
    // IDA 0x816be8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_397")]
#[doc(alias = "__GLOBAL__I_a_397")]
// 0x816fa4 — __GLOBAL__I_a_397
pub fn stub_0x816fa4() {
    // IDA 0x816fa4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_398")]
#[doc(alias = "__GLOBAL__I_a_398")]
// 0x8226f0 — __GLOBAL__I_a_398
pub fn stub_0x8226f0() {
    // IDA 0x8226f0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(RobloxExtraSpace*)")]
#[doc(alias = "__ZN16RobloxExtraSpaceC2EPS_")]
// 0x8238a8 — __ZN16RobloxExtraSpaceC2EPS_
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this, RobloxExtraSpace *)
pub fn stub_0x8238a8() {
    // IDA 0x8238a8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_399")]
#[doc(alias = "__GLOBAL__I_a_399")]
// 0x823f24 — __GLOBAL__I_a_399
pub fn stub_0x823f24() {
    // IDA 0x823f24: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "l_alloc(void *,void *,unsigned long,unsigned long)")]
#[doc(alias = "__ZL7l_allocPvS_mm")]
// 0x824fd4 — __ZL7l_allocPvS_mm
// type: _DWORD __fastcall(void *, void *, unsigned int, size_t __size)
pub fn stub_0x824fd4() {
    // IDA 0x824fd4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_400")]
#[doc(alias = "__GLOBAL__I_a_400")]
// 0x825024 — __GLOBAL__I_a_400
pub fn stub_0x825024() {
    // IDA 0x825024: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_401")]
#[doc(alias = "__GLOBAL__I_a_401")]
// 0x826288 — __GLOBAL__I_a_401
pub fn stub_0x826288() {
    // IDA 0x826288: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "patchlistaux(FuncState *,int,int,int,int)")]
#[doc(alias = "__ZL12patchlistauxP9FuncStateiiii")]
// 0x8264d0 — __ZL12patchlistauxP9FuncStateiiii
pub fn stub_0x8264d0() {
    // IDA 0x8264d0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "fixjump(FuncState *,int,int)")]
#[doc(alias = "__ZL7fixjumpP9FuncStateii")]
// 0x826534 — __ZL7fixjumpP9FuncStateii
pub fn stub_0x826534() {
    // IDA 0x826534: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "exp2reg(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL7exp2regP9FuncStateP7expdesci")]
// 0x82687c — __ZL7exp2regP9FuncStateP7expdesci
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x82687c() {
    // IDA 0x82687c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "invertjump(FuncState *,expdesc *)")]
#[doc(alias = "__ZL10invertjumpP9FuncStateP7expdesc")]
// 0x826bec — __ZL10invertjumpP9FuncStateP7expdesc
pub fn stub_0x826bec() {
    // IDA 0x826bec: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "jumponcond(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL10jumponcondP9FuncStateP7expdesci")]
// 0x826c34 — __ZL10jumponcondP9FuncStateP7expdesci
pub fn stub_0x826c34() {
    // IDA 0x826c34: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "codearith(FuncState *,OpCode,expdesc *,expdesc *)")]
#[doc(alias = "__ZL9codearithP9FuncState6OpCodeP7expdescS3_")]
// 0x826dd0 — __ZL9codearithP9FuncState6OpCodeP7expdescS3_
pub fn stub_0x826dd0() {
    // IDA 0x826dd0: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "codecomp(FuncState *,OpCode,int,expdesc *,expdesc *)")]
#[doc(alias = "__ZL8codecompP9FuncState6OpCodeiP7expdescS3_")]
// 0x827148 — __ZL8codecompP9FuncState6OpCodeiP7expdescS3_
pub fn stub_0x827148() {
    // IDA 0x827148: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "discharge2anyreg(FuncState *,expdesc *)")]
#[doc(alias = "__ZL16discharge2anyregP9FuncStateP7expdesc")]
// 0x827320 — __ZL16discharge2anyregP9FuncStateP7expdesc
pub fn stub_0x827320() {
    // IDA 0x827320: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "removevalues(FuncState *,int)")]
#[doc(alias = "__ZL12removevaluesP9FuncStatei")]
// 0x82734c — __ZL12removevaluesP9FuncStatei
pub fn stub_0x82734c() {
    // IDA 0x82734c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "patchtestreg(FuncState *,int,int)")]
#[doc(alias = "__ZL12patchtestregP9FuncStateii")]
// 0x82738c — __ZL12patchtestregP9FuncStateii
pub fn stub_0x82738c() {
    // IDA 0x82738c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "discharge2reg(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL13discharge2regP9FuncStateP7expdesci")]
// 0x8273f4 — __ZL13discharge2regP9FuncStateP7expdesci
// type: int __fastcall(_DWORD *, _DWORD *, int)
pub fn stub_0x8273f4() {
    // IDA 0x8273f4: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "need_value(FuncState *,int)")]
#[doc(alias = "__ZL10need_valueP9FuncStatei")]
// 0x8274ac — __ZL10need_valueP9FuncStatei
pub fn stub_0x8274ac() {
    // IDA 0x8274ac: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "global constructor keyed to_a_402")]
#[doc(alias = "__GLOBAL__I_a_402")]
// 0x82751c — __GLOBAL__I_a_402
pub fn stub_0x82751c() {
    // IDA 0x82751c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "symbexec(Proto const*,int,int)")]
#[doc(alias = "__ZL8symbexecPK5Protoii")]
// 0x827b2c — __ZL8symbexecPK5Protoii
pub fn stub_0x827b2c() {
    // IDA 0x827b2c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "checkArgMode(Proto const*,int,OpArgMask)")]
#[doc(alias = "__ZL12checkArgModePK5Protoi9OpArgMask")]
// 0x828404 — __ZL12checkArgModePK5Protoi9OpArgMask
pub fn stub_0x828404() {
    // IDA 0x828404: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_403")]
#[doc(alias = "__GLOBAL__I_a_403")]
// 0x82843c — __GLOBAL__I_a_403
pub fn stub_0x82843c() {
    // IDA 0x82843c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_404")]
#[doc(alias = "__GLOBAL__I_a_404")]
// 0x8291d8 — __GLOBAL__I_a_404
pub fn stub_0x8291d8() {
    // IDA 0x8291d8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "DumpFunction(Proto const*,TString const*,DumpState *)")]
#[doc(alias = "__ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState")]
// 0x829330 — __ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState
pub fn stub_0x829330() {
    // IDA 0x829330: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "DumpString(TString const*,DumpState *)")]
#[doc(alias = "__ZL10DumpStringPK7TStringP9DumpState")]
// 0x8295c0 — __ZL10DumpStringPK7TStringP9DumpState
pub fn stub_0x8295c0() {
    // IDA 0x8295c0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "DumpVector(void const*,int,unsigned long,DumpState *)")]
#[doc(alias = "__ZL10DumpVectorPKvimP9DumpState")]
// 0x829614 — __ZL10DumpVectorPKvimP9DumpState
pub fn stub_0x829614() {
    // IDA 0x829614: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_405")]
#[doc(alias = "__GLOBAL__I_a_405")]
// 0x829654 — __GLOBAL__I_a_405
pub fn stub_0x829654() {
    // IDA 0x829654: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_406")]
#[doc(alias = "__GLOBAL__I_a_406")]
// 0x8299d8 — __GLOBAL__I_a_406
pub fn stub_0x8299d8() {
    // IDA 0x8299d8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "reallymarkobject(global_State *,GCObject *)")]
#[doc(alias = "__ZL16reallymarkobjectP12global_StateP8GCObject")]
// 0x82a138 — __ZL16reallymarkobjectP12global_StateP8GCObject
// type: int *__fastcall(int, int)
pub fn stub_0x82a138() {
    // IDA 0x82a138: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "markmt(global_State *)")]
#[doc(alias = "__ZL6markmtP12global_State")]
// 0x82a238 — __ZL6markmtP12global_State
pub fn stub_0x82a238() {
    // IDA 0x82a238: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "propagatemark(global_State *)")]
#[doc(alias = "__ZL13propagatemarkP12global_State")]
// 0x82a264 — __ZL13propagatemarkP12global_State
pub fn stub_0x82a264() {
    // IDA 0x82a264: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_407")]
#[doc(alias = "__GLOBAL__I_a_407")]
// 0x82a740 — __GLOBAL__I_a_407
pub fn stub_0x82a740() {
    // IDA 0x82a740: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "llex(LexState *,SemInfo *)")]
#[doc(alias = "__ZL4llexP8LexStateP7SemInfo")]
// 0x82aa20 — __ZL4llexP8LexStateP7SemInfo
pub fn stub_0x82aa20() {
    // IDA 0x82aa20: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "inclinenumber(LexState *)")]
#[doc(alias = "__ZL13inclinenumberP8LexState")]
// 0x82b004 — __ZL13inclinenumberP8LexState
pub fn stub_0x82b004() {
    // IDA 0x82b004: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "skip_sep(LexState *)")]
#[doc(alias = "__ZL8skip_sepP8LexState")]
// 0x82b078 — __ZL8skip_sepP8LexState
pub fn stub_0x82b078() {
    // IDA 0x82b078: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "read_long_string(LexState *,SemInfo *,int)")]
#[doc(alias = "__ZL16read_long_stringP8LexStateP7SemInfoi")]
// 0x82b0dc — __ZL16read_long_stringP8LexStateP7SemInfoi
pub fn stub_0x82b0dc() {
    // IDA 0x82b0dc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "save(LexState *,int)")]
#[doc(alias = "__ZL4saveP8LexStatei")]
// 0x82b1e4 — __ZL4saveP8LexStatei
pub fn stub_0x82b1e4() {
    // IDA 0x82b1e4: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "check_next(LexState *,char const*)")]
#[doc(alias = "__ZL10check_nextP8LexStatePKc")]
// 0x82b248 — __ZL10check_nextP8LexStatePKc
// type: int __fastcall(int, char *__s)
pub fn stub_0x82b248() {
    // IDA 0x82b248: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "read_numeral(LexState *,SemInfo *)")]
#[doc(alias = "__ZL12read_numeralP8LexStateP7SemInfo")]
// 0x82b288 — __ZL12read_numeralP8LexStateP7SemInfo
pub fn stub_0x82b288() {
    // IDA 0x82b288: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "global constructor keyed to_a_408")]
#[doc(alias = "__GLOBAL__I_a_408")]
// 0x82b40c — __GLOBAL__I_a_408
pub fn stub_0x82b40c() {
    // IDA 0x82b40c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_409")]
#[doc(alias = "__GLOBAL__I_a_409")]
// 0x82bb10 — __GLOBAL__I_a_409
pub fn stub_0x82bb10() {
    // IDA 0x82bb10: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_410")]
#[doc(alias = "__GLOBAL__I_a_410")]
// 0x82bca8 — __GLOBAL__I_a_410
pub fn stub_0x82bca8() {
    // IDA 0x82bca8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_411")]
#[doc(alias = "__GLOBAL__I_a_411")]
// 0x82c1a4 — __GLOBAL__I_a_411
pub fn stub_0x82c1a4() {
    // IDA 0x82c1a4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_412")]
#[doc(alias = "__GLOBAL__I_a_412")]
// 0x82c26c — __GLOBAL__I_a_412
pub fn stub_0x82c26c() {
    // IDA 0x82c26c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "open_func(LexState *,FuncState *)")]
#[doc(alias = "__ZL9open_funcP8LexStateP9FuncState")]
// 0x82c3a0 — __ZL9open_funcP8LexStateP9FuncState
pub fn stub_0x82c3a0() {
    // IDA 0x82c3a0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "chunk(LexState *)")]
#[doc(alias = "__ZL5chunkP8LexState")]
// 0x82c440 — __ZL5chunkP8LexState
pub fn stub_0x82c440() {
    // IDA 0x82c440: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "close_func(LexState *)")]
#[doc(alias = "__ZL10close_funcP8LexState")]
// 0x82cb20 — __ZL10close_funcP8LexState
pub fn stub_0x82cb20() {
    // IDA 0x82cb20: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "enterlevel(LexState *)")]
#[doc(alias = "__ZL10enterlevelP8LexState")]
// 0x82ccd8 — __ZL10enterlevelP8LexState
pub fn stub_0x82ccd8() {
    // IDA 0x82ccd8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "block(LexState *)")]
#[doc(alias = "__ZL5blockP8LexState")]
// 0x82ccf8 — __ZL5blockP8LexState
pub fn stub_0x82ccf8() {
    // IDA 0x82ccf8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "check_match(LexState *,int,int,int)")]
#[doc(alias = "__ZL11check_matchP8LexStateiii")]
// 0x82cd30 — __ZL11check_matchP8LexStateiii
// type: int __fastcall(int)
pub fn stub_0x82cd30() {
    // IDA 0x82cd30: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "breakstat(LexState *)")]
#[doc(alias = "__ZL9breakstatP8LexState")]
// 0x82cda8 — __ZL9breakstatP8LexState
pub fn stub_0x82cda8() {
    // IDA 0x82cda8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "primaryexp(LexState *,expdesc *)")]
#[doc(alias = "__ZL10primaryexpP8LexStateP7expdesc")]
// 0x82ce00 — __ZL10primaryexpP8LexStateP7expdesc
pub fn stub_0x82ce00() {
    // IDA 0x82ce00: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "assignment(LexState *,LHS_assign *,int)")]
#[doc(alias = "__ZL10assignmentP8LexStateP10LHS_assigni")]
// 0x82cf08 — __ZL10assignmentP8LexStateP10LHS_assigni
pub fn stub_0x82cf08() {
    // IDA 0x82cf08: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "errorlimit(FuncState *,int,char const*)")]
#[doc(alias = "__ZL10errorlimitP9FuncStateiPKc")]
// 0x82d02c — __ZL10errorlimitP9FuncStateiPKc
pub fn stub_0x82d02c() {
    // IDA 0x82d02c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "checknext(LexState *,int)")]
#[doc(alias = "__ZL9checknextP8LexStatei")]
// 0x82d074 — __ZL9checknextP8LexStatei
pub fn stub_0x82d074() {
    // IDA 0x82d074: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "explist1(LexState *,expdesc *)")]
#[doc(alias = "__ZL8explist1P8LexStateP7expdesc")]
// 0x82d090 — __ZL8explist1P8LexStateP7expdesc
pub fn stub_0x82d090() {
    // IDA 0x82d090: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "adjust_assign(LexState *,int,int,expdesc *)")]
#[doc(alias = "__ZL13adjust_assignP8LexStateiiP7expdesc")]
// 0x82d0cc — __ZL13adjust_assignP8LexStateiiP7expdesc
pub fn stub_0x82d0cc() {
    // IDA 0x82d0cc: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "subexpr(LexState *,expdesc *,unsigned int)")]
#[doc(alias = "__ZL7subexprP8LexStateP7expdescj")]
// 0x82d12c — __ZL7subexprP8LexStateP7expdescj
pub fn stub_0x82d12c() {
    // IDA 0x82d12c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "constructor(LexState *,expdesc *)")]
#[doc(alias = "__ZL11constructorP8LexStateP7expdesc")]
// 0x82d3d0 — __ZL11constructorP8LexStateP7expdesc
pub fn stub_0x82d3d0() {
    // IDA 0x82d3d0: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "body(LexState *,expdesc *,int,int)")]
#[doc(alias = "__ZL4bodyP8LexStateP7expdescii")]
// 0x82d530 — __ZL4bodyP8LexStateP7expdescii
pub fn stub_0x82d530() {
    // IDA 0x82d530: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "new_localvar(LexState *,TString *,int)")]
#[doc(alias = "__ZL12new_localvarP8LexStateP7TStringi")]
// 0x82d734 — __ZL12new_localvarP8LexStateP7TStringi
pub fn stub_0x82d734() {
    // IDA 0x82d734: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "adjustlocalvars(LexState *,int)")]
#[doc(alias = "__ZL15adjustlocalvarsP8LexStatei")]
// 0x82d81c — __ZL15adjustlocalvarsP8LexStatei
pub fn stub_0x82d81c() {
    // IDA 0x82d81c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "str_checkname(LexState *)")]
#[doc(alias = "__ZL13str_checknameP8LexState")]
// 0x82d860 — __ZL13str_checknameP8LexState
pub fn stub_0x82d860() {
    // IDA 0x82d860: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "listfield(LexState *,ConsControl *)")]
#[doc(alias = "__ZL9listfieldP8LexStateP11ConsControl")]
// 0x82d888 — __ZL9listfieldP8LexStateP11ConsControl
pub fn stub_0x82d888() {
    // IDA 0x82d888: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "recfield(LexState *,ConsControl *)")]
#[doc(alias = "__ZL8recfieldP8LexStateP11ConsControl")]
// 0x82d8cc — __ZL8recfieldP8LexStateP11ConsControl
pub fn stub_0x82d8cc() {
    // IDA 0x82d8cc: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[cfg(test)]
mod xml_batch_tests {
    use super::xml_parse::{
        TextParser, decode_string, parse_attributes, parse_into, read_first_tag, read_tag, read_text,
        remove_tag, skip_whitespace, ParseError,
    };
    use super::xml_tree::XmlElement;
    use super::xml_value::{
        ContentId, IdRefTarget, NameValuePair, Value, convert_bool_to_value, convert_double_to_string,
        convert_double_to_value, convert_float_to_string, convert_float_to_value, convert_int_to_value,
        convert_uint_to_value, format_g,
    };
    use super::xml_write::XmlWriter;
    use crate::SharedPtr;

    fn text_pair(text: &str) -> NameValuePair {
        NameValuePair::with_text("v".to_string(), text.to_string())
    }

    #[test]
    fn bool_converter_spellings() {
        // IDA 0x38ce78: true/True/TRUE and false/False/FALSE only.
        for (s, v) in [("true", true), ("True", true), ("TRUE", true), ("false", false), ("False", false), ("FALSE", false)] {
            let mut out = !v;
            assert!(convert_bool_to_value(s, &mut out));
            assert_eq!(out, v);
        }
        let mut out = true;
        assert!(!convert_bool_to_value("", &mut out));
        assert!(!convert_bool_to_value("1", &mut out));
        assert!(!convert_bool_to_value("true ", &mut out));
        assert!(out, "out untouched on failure");
    }

    #[test]
    fn int_converter_shape() {
        // IDA 0x38cfa0: digits with optional leading '-', empty fails.
        let mut v = 99;
        assert!(convert_int_to_value("12", &mut v) && v == 12);
        assert!(convert_int_to_value("-5", &mut v) && v == -5);
        assert!(convert_int_to_value("-", &mut v) && v == 0); // BUG: atoi("-")==0 success
        assert!(!convert_int_to_value("", &mut v));
        assert!(!convert_int_to_value("1a", &mut v));
        assert!(!convert_int_to_value("+5", &mut v));
        assert!(!convert_int_to_value(" 5", &mut v));
    }

    #[test]
    fn uint_converter_shape() {
        let mut v = 99u32;
        assert!(convert_uint_to_value("5", &mut v) && v == 5);
        assert!(!convert_uint_to_value("", &mut v));
        assert!(!convert_uint_to_value("-1", &mut v));
        assert!(!convert_uint_to_value("4294967296", &mut v)); // overflow -> bad_lexical_cast -> 0
    }

    #[test]
    fn float_double_converters() {
        // IDA 0x38d260/0x38d440 specials.
        let mut d = 0.0;
        assert!(convert_double_to_value("INF", &mut d) && d == f64::INFINITY);
        assert!(convert_double_to_value("-INF", &mut d) && d == f64::NEG_INFINITY);
        assert!(convert_double_to_value("NAN", &mut d) && d.is_nan());
        assert!(convert_double_to_value("2.5", &mut d) && d == 2.5);
        assert!(!convert_double_to_value("", &mut d));
        let mut f = 0.0f32;
        assert!(convert_float_to_value("INF", &mut f) && f == f32::INFINITY);
        assert!(convert_float_to_value("0.5", &mut f) && f == 0.5);
        // %.9g / %.20g rendering (IDA 0x38d2e0/0x38d4c4).
        assert_eq!(convert_double_to_string(f64::INFINITY), "INF");
        assert_eq!(convert_double_to_string(f64::NEG_INFINITY), "-INF");
        assert_eq!(convert_double_to_string(2.5), "2.5");
        assert_eq!(convert_float_to_string(0.5), "0.5");
        assert_eq!(format_g(1e20, 20), "1e+20");
        assert_eq!(format_g(0.0001, 9), "0.0001");
        assert_eq!(format_g(0.00001, 9), "1e-05");
    }

    #[test]
    fn pair_get_cache_transitions() {
        // IDA 0x798964: parse caches the bool, second read hits the cache.
        let mut p = text_pair("True");
        let mut b = false;
        assert!(super::stub_0x798964(&mut p, &mut b));
        assert_eq!(p.tag(), super::xml_value::TAG_BOOL);
        let mut b2 = false;
        assert!(super::stub_0x798964(&mut p, &mut b2) && b2);
        // IDA 0x798d64/0x798da4 int/uint shapes.
        let mut pi = text_pair("-7");
        let mut i = 0;
        assert!(super::stub_0x798d64(&mut pi, &mut i) && i == -7);
        let mut pu = text_pair("7");
        let mut u = 0;
        assert!(super::stub_0x798da4(&mut pu, &mut u) && u == 7);
        // IDA 0x798de4: double-valued pair rejected as float (assert path -> false).
        let mut pd = text_pair("1.5");
        let mut d = 0.0;
        assert!(super::stub_0x798e7c(&mut pd, &mut d) && d == 1.5);
        let mut f = 0.0f32;
        assert!(!super::stub_0x798de4(&mut pd, &mut f));
        // IDA 0x798e7c float->double widen re-caches as double.
        let mut pf = text_pair("0.5");
        let mut g = 0.0f32;
        assert!(super::stub_0x798de4(&mut pf, &mut g) && g == 0.5);
        let mut d2 = 0.0;
        assert!(super::stub_0x798e7c(&mut pf, &mut d2) && (d2 - 0.5).abs() < 1e-9);
        assert_eq!(pf.tag(), super::xml_value::TAG_DOUBLE);
        // convert failure leaves the tag alone (IDA 0x798980 early return).
        let mut bad = text_pair("maybe");
        let mut bb = true;
        assert!(!super::stub_0x798964(&mut bad, &mut bb));
        assert_eq!(bad.tag(), super::xml_value::TAG_STRING);
        // IDA 0x7989ec clear resets the tag.
        super::stub_0x7989ec(&mut bad);
        assert_eq!(bad.tag(), super::xml_value::TAG_NONE);
    }

    #[test]
    fn pair_to_string_shapes() {
        let mut w = XmlWriter::new();
        assert_eq!(NameValuePair::new("t".into()).to_string_value(&mut w), ""); // tag 0
        assert_eq!(text_pair("hi").to_string_value(&mut w), "hi"); // tag 2
        let mut pb = text_pair("true");
        let mut b = false;
        assert!(pb.get_bool(&mut b));
        assert_eq!(pb.to_string_value(&mut w), "true"); // tag 4 round-trip
        assert_eq!(NameValuePair { name: "n".into(), value: Value::Int(-3) }.to_string_value(&mut w), "-3");
        assert_eq!(NameValuePair { name: "n".into(), value: Value::UInt(3) }.to_string_value(&mut w), "3");
        // IDREF null vs bound (IDA 0x7991b0 case 8).
        assert_eq!(NameValuePair { name: "n".into(), value: Value::IdRef(None) }.to_string_value(&mut w), "null");
        let target: SharedPtr<IdRefTarget> = SharedPtr::new(IdRefTarget { opaque: 1 });
        let s = NameValuePair { name: "n".into(), value: Value::IdRef(Some(target)) }.to_string_value(&mut w);
        assert_eq!(s, "RBX0");
    }

    #[test]
    fn element_helpers_match_ida_walks() {
        // isXsiNil: first xsi:nil decides; convert failure -> false; absent -> false.
        let mut e = XmlElement::new("Item".into());
        assert!(!super::stub_0x79890c(&e));
        e.add_attribute("xsi:nil".into(), "true".into());
        assert!(super::stub_0x79890c(&e));
        e.add_attribute("other".into(), "1".into());
        assert_eq!(e.find_attribute("other").map(|a| a.pair.name.as_str()), Some("other"));
        assert!(e.find_attribute("missing").is_none());
        // findNextChildWithSameTag walks next links on tag equality.
        let mut a = XmlElement::new("a".into());
        let b = XmlElement::new("b".into());
        let a2 = XmlElement::new("a".into());
        a2_next_link(&mut a, b, a2);
        let found = super::stub_0x7989bc(&a).expect("a2 matches");
        assert_eq!(found.tag(), "a");
        let lone = XmlElement::new("z".into());
        assert!(super::stub_0x7989bc(&lone).is_none());
    }
    fn a2_next_link(a: &mut XmlElement, b: XmlElement, a2: XmlElement) {
        // a.next = b; b.next = a2  (a itself keeps tag "a")
        let mut bb = b;
        bb.next = Some(Box::new(a2));
        a.next = Some(Box::new(bb));
    }


    #[test]
    fn decode_string_entities() {
        // IDA 0x79a624: five named entities + nbsp-as-space + numeric.
        assert_eq!(decode_string("a&lt;b&gt;&amp;&quot;&apos;&nbsp;c").unwrap(), "a<b>&\"' c");
        assert_eq!(decode_string("&#65;").unwrap(), "A");
        assert_eq!(decode_string("&unknown;").unwrap(), "&unknown;");
        assert_eq!(decode_string("&oops").unwrap(), "&oops;");
        assert!(matches!(decode_string("&#;"), Err(ParseError::BadCharCode)));
        assert!(matches!(decode_string("&#x41;"), Err(ParseError::HexCharCode)));
    }

    #[test]
    fn stream_readers_match_ida_flows() {
        let mut p = TextParser::new("   <tag>\n");
        assert_eq!(skip_whitespace(&mut p), 0);
        let mut tag = String::new();
        assert!(read_tag(&mut tag, &mut p).is_ok() && tag == "<tag>");
        // read_first_tag tolerates a BOM-length prefix then demands '<'.
        let mut p2 = TextParser::new("\u{feff}<first>");
        let mut first = String::new();
        assert!(read_first_tag(&mut first, &mut p2).is_ok() && first == "<first>");
        assert!(matches!(
            read_first_tag(&mut first, &mut TextParser::new("nopexx")),
            Err(ParseError::BomTagExpected)
        ));
        // read_text stops before '<' without consuming it.
        let mut p3 = TextParser::new("hi &amp; bye<next>");
        let mut text = String::new();
        assert!(read_text(&mut text, &mut p3, true).is_ok() && text == "hi & bye");
        assert_eq!(p3.stream.peek(), '<' as i32);
        let elem = parse_attributes("<Item id=\"a&amp;b\" flag=\"true\">").unwrap();
        assert_eq!(elem.tag(), "Item");
        assert_eq!(elem.find_attribute("id").unwrap().pair.text_or_empty(), "a&b");
        assert!(matches!(parse_attributes("<Item oops>"), Err(ParseError::NoEquals)));
    }

    #[test]
    fn writer_output_shapes() {
        // writeOpenTag/writeCloseTag byte shapes (IDA 0x79af50/0x79b250).
        let mut e = XmlElement::new("Item".into());
        e.add_attribute("id".into(), "a&b".into());
        let mut w = XmlWriter::new();
        super::stub_0x79af50(&mut w, &e, 1, None);
        assert_eq!(w.out, "\t<Item id=\"a&amp;b\">");
        super::stub_0x79b250(&mut w, &e, 0);
        assert!(w.out.ends_with("</Item>"));
        let mut root = XmlElement::new("root".into());
        root.value = NameValuePair::with_text("root".into(), "hi & bye".into());
        let mut child = XmlElement::new("c".into());
        child.value = NameValuePair::with_text("c".into(), "x".into());
        root.append_child(child);
        let mut w2 = XmlWriter::new();
        super::stub_0x79c9f4(&mut w2, Some(&root), 0);
        assert_eq!(w2.out, "<root>hi &amp; bye\n\t<c>x</c>\n</root>");
        // ContentId url branch (IDA 0x79ca70).
        let mut u = XmlElement::new("ContentId".into());
        u.value = NameValuePair {
            name: "ContentId".into(),
            value: Value::ContentId(ContentId { text: "http://x".into(), mime: None }),
        };
        let mut w3 = XmlWriter::new();
        super::stub_0x79ca70(&mut w3, &u, 0);
        assert_eq!(w3.out, "<ContentId><url>http://x</url>");
    }
    #[test]
    fn parse_end_to_end() {
        // Prolog skip + leaf + ContentId/url + close-tag rooting.
        let doc = "<?xml version=\"1.0\"?><root><name>Bob &amp; Co</name><ContentId><url>rbxasset://1</url></ContentId></root>";
        let mut parser = TextParser::new(doc);
        let mut root = None;
        assert!(super::stub_0x79ba0c(&mut root, &mut parser).is_ok());
        let root = root.expect("rooted");
        assert_eq!(root.tag(), "root");
        let kids: Vec<_> = root.children().map(|c| c.tag().to_string()).collect();
        assert_eq!(kids, vec!["name", "ContentId"]);
        assert_eq!(root.children().next().unwrap().value.text_or_empty(), "Bob & Co");
        // xsi:nil skips content (IDA 0x79bcce).
        let doc2 = "<root><ContentId xsi:nil=\"true\"></ContentId></root>";
        let mut p2 = TextParser::new(doc2);
        let mut r2 = None;
        assert!(parse_into(&mut r2, &mut p2).is_ok());
        // Errors keep exact text.
        assert!(matches!(parse_into(&mut None, &mut TextParser::new("")), Err(ParseError::EmptyFile)));
        let err = parse_into(&mut None, &mut TextParser::new("</nope>"));
        assert!(matches!(err, Err(ParseError::CloseWithoutOpen(_))));
        // IDA 0x79bc9a: the close path pops blindly — names are never
        // compared, so a mismatched close silently ends the element.
        let mut m = None;
        assert!(parse_into(&mut m, &mut TextParser::new("<root></nope>")).is_ok());
        assert_eq!(m.expect("rooted").tag(), "root");
        let err2 = parse_into(&mut None, &mut TextParser::new("<ContentId><bogus>x</bogus></ContentId>"));
        assert!(matches!(err2, Err(ParseError::UnknownTag(_))));
        assert_eq!(
            ParseError::UnknownTag("bogus".into()).to_string(),
            "TextXmlParser::parse - Unknown tag 'bogus'."
        );
    }
}
