//! platform — generated_plat_ios_eagl — 100 stubs EA-sorted asc platform namespace (Cocoa/iOS bridge, Ogre EAGL2, RobloxView) | Source ida/export.json | range 0x7dd5d4..0xf53b34 | rbx_core::SharedPtr not boost
//! Source: ida/export.json (85545 funcs) platform-namespace next 100 EA-sorted asc not in global set (2 IPAddress `ipad`-substring false positives excluded)
//! Batch: 100 stubs | range 0x7dd5d4..0xf53b34 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// ---- Cocoa String_sink + Ogre EAGL2 batch (IDA 0x7dd5d4..0xe885b8) ----
//
// Boost→Rust: `boost::shared_ptr`→`rbx_core::SharedPtr` (never a boost shim);
// `boost::iostreams` output chains become the small `StringSink`-side models
// below; `boost::iostreams::cant_seek` becomes an `io::Error`. ObjC `id` is a
// plain `usize` (`0` = nil, no host runtime). Ogre `EAGL2Support`/`EAGL2Window`
// become `Eagl2Support`/`Eagl2Window` with the same option table and update
// order as the disasm.

/// Request model for `RBX::Cocoa::httpGetPostCocoa` (IDA 0x7dd5d4).
#[derive(Debug, Clone, Default)]
pub struct CocoaHttpRequest {
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub is_post: bool,
    pub post_body: String,
    pub compress_post: bool,
}

/// POST body encoding selected at IDA 0x7dd680..0x7dd862.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostEncoding {
    RawStream,
    Gzip,
}

/// Branch selector for the POST body path (IDA 0x7dd680/0x7dd688): POST +
/// compress takes the gzip `filtering_stream` chain (0x7dd6d8..0x7dd80c),
/// POST without compress uses the raw stream (0x7dd862), GET uses neither.
pub fn cocoa_post_encoding(req: &CocoaHttpRequest) -> Option<PostEncoding> {
    if !req.is_post {
        return None;
    }
    if req.compress_post {
        Some(PostEncoding::Gzip)
    } else {
        Some(PostEncoding::RawStream)
    }
}

/// HTTP GET/POST through `MacHttpController` (IDA 0x7dd5d4..0x7dd906).
/// `transport` stands in for `-[MacHttpController doGetPost:]` (0x7dd880):
/// there is no ObjC runtime on the host, so the controller boundary is a
/// closure returning the received bytes or the integer error code. Everything
/// around it is 1:1 — pool/controller setup (0x7dd606..0x7dd680), the POST
/// branch above, the `runtime_error("%s: err=0x%X (%d)")` throw
/// (0x7dda32..0x7dda8e), and the `replace` of the out string with the
/// received `bytes`/`length` (0x7dd8a0..0x7dd8de).
pub fn http_get_post_cocoa(
    req: &CocoaHttpRequest,
    transport: &dyn Fn(&CocoaHttpRequest) -> Result<Vec<u8>, i64>,
) -> Result<String, String> {
    let _encoding = cocoa_post_encoding(req);
    match transport(req) {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).into_owned()),
        Err(code) => Err(format!("{}: err=0x{:X} ({})", req.url, code as u32, code)),
    }
}

// 0x7dd5d4 — __ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs
#[doc(alias = "__ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs")]
pub fn stub_0x7dd5d4(
    req: &CocoaHttpRequest,
    transport: &dyn Fn(&CocoaHttpRequest) -> Result<Vec<u8>, i64>,
) -> Result<String, String> {
    // IDA 0x7dd5d4
    http_get_post_cocoa(req, transport)
}

/// Output-only `String_sink` devices cannot seek: `cant_seek` followed by
/// `throw_exception<ios_base::failure>` (IDA 0x7e04c0..0x7e04f4). The throw
/// becomes `Err`; the whole `concept_adapter`→`device_wrapper_impl` forward
/// chain (0x7e0490, 0x7e049a) collapses here.
pub fn string_sink_seek() -> std::io::Result<u64> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "boost::iostreams::cant_seek: String_sink is output-only",
    ))
}

// 0x7e047c — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
#[doc(alias = "__ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")]
pub fn stub_0x7e047c() -> std::io::Result<u64> {
    // IDA 0x7e047c: forwards to device_wrapper_impl::seek (0x7e0490).
    string_sink_seek()
}

// 0x7e0494 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")]
pub fn stub_0x7e0494() -> std::io::Result<u64> {
    // IDA 0x7e0494: forwards to device_wrapper_impl::seek<String_sink> (0x7e049a).
    string_sink_seek()
}

// 0x7e04a0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")]
pub fn stub_0x7e04a0() -> std::io::Result<u64> {
    // IDA 0x7e04a0
    string_sink_seek()
}

/// Host model of `indirect_streambuf<String_sink>` (IDA 0x7e0854..0x7e0956):
/// growable buffer (`basic_buffer`, +48), open flag (+40), sink ref (+36),
/// mode flags (+60) and state word (+32).
#[derive(Debug, Default)]
pub struct IndirectStreambuf {
    pub buffer: Vec<u8>,
    pub open: bool,
    pub sink: usize,
    pub mode_flags: u32,
    pub state: u32,
    pub has_locale: bool,
}

impl IndirectStreambuf {
    /// `open` (IDA 0x7e08f4..0x7e0956): default 4096-byte buffer (0x7e0900);
    /// `-1` keeps the default, `0` skips the resize, otherwise resizes to the
    /// request (0x7e090e..0x7e091a). Then the device-open step (0x7e0922),
    /// stale-open clear (0x7e0924..0x7e0932), sink store (0x7e0936), open set
    /// (0x7e093a), mode `1` or `3` when grown past one byte
    /// (0x7e0938..0x7e0948), and `state &= ~7` (0x7e094c..0x7e0956).
    pub fn open(&mut self, sink: usize, buf_size: i32) -> u32 {
        let mut grown: i64 = 0;
        if buf_size == -1 {
            self.buffer.resize(4096, 0);
            grown = 4096;
        } else if buf_size != 0 {
            self.buffer.resize(buf_size as usize, 0);
            grown = buf_size as i64;
        }
        self.sink = sink;
        self.open = true;
        let mut mode = 1u32;
        if grown > 1 {
            mode = 3;
        }
        self.mode_flags |= mode;
        self.state &= 0xFFFFFFF8;
        self.state
    }

    /// Destructor body shared by the scalar (0x7e0854) and deleting
    /// (0x7e08a0) variants: free the buffer (0x7e086a..0x7e0870), clear the
    /// open flag (0x7e0874..0x7e087e), restore the plain `streambuf` vtable
    /// (0x7e0894, no host vtable — noted only) and destroy the locale
    /// (0x7e0898).
    pub fn destroy(&mut self) {
        self.buffer = Vec::new();
        self.open = false;
        self.has_locale = false;
    }
}

/// Host model of `stream_buffer<String_sink>` (IDA 0x7e0524): the indirect
/// buffer plus the close-flags word (+60).
#[derive(Debug, Default)]
pub struct CocoaStreamBuffer {
    pub inner: IndirectStreambuf,
    pub close_flags: u32,
}

/// `execute_all` over close/close/reset (IDA 0x7e076c..0x7e07f8): run the two
/// member-close ops (0x7e07ca, collapses to closed) then reset the optional
/// adapter (`a5[4] = 0`, 0x7e07d0..0x7e07da).
pub fn execute_all_close_reset(buf: &mut IndirectStreambuf) {
    buf.open = false;
    buf.sink = 0;
}

/// `execute_all` over close/close/reset/clear-flags (IDA 0x7e0690..0x7e0718):
/// the 3-op chain above (0x7e06f2) then `*a6 = 0` clears the flags word
/// (0x7e06fa..0x7e0718).
pub fn execute_all_close_reset_clear(buf: &mut CocoaStreamBuffer) {
    execute_all_close_reset(&mut buf.inner);
    buf.close_flags = 0;
}

impl CocoaStreamBuffer {
    /// Destructor (IDA 0x7e0524..0x7e060e): when `(flags & 5) == 5` run the
    /// close chain (0x7e058a..0x7e05a2), then the indirect-buffer teardown
    /// above (0x7e05b6..0x7e060e).
    pub fn destroy(&mut self) {
        if self.close_flags & 5 == 5 {
            execute_all_close_reset_clear(self);
        }
        self.inner.destroy();
    }
}

// 0x7e0524 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev")]
pub fn stub_0x7e0524(buf: &mut CocoaStreamBuffer) {
    // IDA 0x7e0524
    buf.destroy();
}

// 0x7e0690 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")]
pub fn stub_0x7e0690(buf: &mut CocoaStreamBuffer) {
    // IDA 0x7e0690
    execute_all_close_reset_clear(buf);
}

// 0x7e076c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
pub fn stub_0x7e076c(buf: &mut IndirectStreambuf) {
    // IDA 0x7e076c
    execute_all_close_reset(buf);
}

// 0x7e0854 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")]
pub fn stub_0x7e0854(buf: &mut IndirectStreambuf) {
    // IDA 0x7e0854
    buf.destroy();
}

// 0x7e08a0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")]
pub fn stub_0x7e08a0(buf: Box<IndirectStreambuf>) {
    // IDA 0x7e08a0: scalar body (0x7e08b4..0x7e08e4) plus `operator delete`
    // (0x7e08e4) — the `Box` drop is the delete.
    let mut buf = buf;
    buf.destroy();
}

// 0x7e08f4 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii")]
pub fn stub_0x7e08f4(buf: &mut IndirectStreambuf, sink: usize, buf_size: i32) -> u32 {
    // IDA 0x7e08f4
    buf.open(sink, buf_size)
}

/// One `Ogre::_ConfigOption`: display name, current value and the possible
/// values vector, in disasm order (IDA 0xe8457c..0xe84d3c).
#[derive(Debug, Clone, Default)]
pub struct ConfigOption {
    pub name: String,
    pub current_value: String,
    pub possible_values: Vec<String>,
}

/// Host model of `Ogre::EAGL2Support` (IDA 0xe844ec..0xe84558): the ctor
/// installs four empty strings (0xe8451c..0xe84524), the option/config maps
/// as empty lists with self-pointing heads (0xe8452a..0xe84554) and the
/// vtable (0xe84556) — all of which is `Vec::new()` plus construction here.
#[derive(Debug, Default)]
pub struct Eagl2Support {
    pub options: Vec<ConfigOption>,
}

impl Eagl2Support {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }

    pub fn find_option(&self, name: &str) -> Option<&ConfigOption> {
        self.options.iter().find(|o| o.name == name)
    }

    /// `addConfig` (IDA 0xe8457c..0xe851dc): pushes the six options with
    /// their value lists and currents, then inserts each into the option
    /// map (0xe84d16..0xe851dc). Screen size comes from
    /// `+[UIScreen mainScreen].applicationFrame` (0xe84736..0xe84768).
    pub fn add_config(&mut self, screen_w: u32, screen_h: u32) {
        // "Full Screen" = {Yes, No}, current Yes (0xe84648..0xe84724).
        self.options.push(ConfigOption {
            name: "Full Screen".into(),
            current_value: "Yes".into(),
            possible_values: vec!["Yes".into(), "No".into()],
        });
        // "Video Mode": literals "320 x 480" (0xe84794), "768 x 1024"
        // (0xe847ec); current is "<w> x <h>" from the screen frame via
        // `StringConverter::toString` + `" x "` append (0xe84842..0xe848a6).
        self.options.push(ConfigOption {
            name: "Video Mode".into(),
            current_value: format!("{screen_w} x {screen_h}"),
            possible_values: vec!["320 x 480".into(), "768 x 1024".into()],
        });
        // "Display Frequency" = {"0 Hz"}, current "0 Hz" (0xe848f4..0xe84974).
        self.options.push(ConfigOption {
            name: "Display Frequency".into(),
            current_value: "0 Hz".into(),
            possible_values: vec!["0 Hz".into()],
        });
        // "Content Scaling Factor" = {1.0, 1.33, 1.5, 2.0}, current 1.0
        // (0xe84982..0xe84af8).
        self.options.push(ConfigOption {
            name: "Content Scaling Factor".into(),
            current_value: "1.0".into(),
            possible_values: vec!["1.0".into(), "1.33".into(), "1.5".into(), "2.0".into()],
        });
        // "FSAA" = {0, 2, 4}, current 0 (0xe84b06..0xe84c2a).
        self.options.push(ConfigOption {
            name: "FSAA".into(),
            current_value: "0".into(),
            possible_values: vec!["0".into(), "2".into(), "4".into()],
        });
        // "RTT Preferred Mode" = {Copy, FBO}, current FBO (0xe84c38..0xe84d02).
        self.options.push(ConfigOption {
            name: "RTT Preferred Mode".into(),
            current_value: "FBO".into(),
            possible_values: vec!["Copy".into(), "FBO".into()],
        });
    }
}

// 0xe844ec — __ZN4Ogre12EAGL2SupportC1Ev
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2SupportC1Ev")]
pub fn stub_0xe844ec() -> Eagl2Support {
    // IDA 0xe844ec
    Eagl2Support::new()
}

// 0xe8455c — __ZN4Ogre12EAGL2SupportD0Ev
// type: void __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2SupportD0Ev")]
pub fn stub_0xe8455c(_support: Box<Eagl2Support>) {
    // IDA 0xe8455c: `GLES2Support::~GLES2Support` (0xe84562) then
    // `operator delete` (0xe84568) — both happen in the `Box` drop.
}

// 0xe84570 — __ZN4Ogre12EAGL2SupportD1Ev
// type: void __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2SupportD1Ev")]
pub fn stub_0xe84570(support: &mut Eagl2Support) {
    // IDA 0xe84570: `GLES2Support::~GLES2Support` (0xe84574) only.
    support.options.clear();
}

// 0xe8457c — __ZN4Ogre12EAGL2Support9addConfigEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2Support9addConfigEv")]
pub fn stub_0xe8457c(support: &mut Eagl2Support, screen_w: u32, screen_h: u32) {
    // IDA 0xe8457c
    support.add_config(screen_w, screen_h);
}

/// `validateConfig` (IDA 0xe862b0..0xe862c4): copies `StringUtil::BLANK`
/// over the out string and returns it.
pub fn eagl2_support_validate_config(out: &mut String) -> &mut String {
    out.clear();
    out
}

// 0xe862b0 — __ZN4Ogre12EAGL2Support14validateConfigEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2Support14validateConfigEv")]
pub fn stub_0xe862b0(out: &mut String) -> &mut String {
    // IDA 0xe862b0
    eagl2_support_validate_config(out)
}

/// `getDisplayName` (IDA 0xe862c8..0xe862e0): copies the `aTodo` literal.
/// The binary genuinely contains `"todo"` as the display-name literal
/// (disasm `MOVW R1, aTodo` at 0xe862ce..0xe862d8), so this is faithful,
/// not a placeholder.
pub fn eagl2_support_display_name() -> String {
    "todo".into()
}

// 0xe862c8 — __ZN4Ogre12EAGL2Support14getDisplayNameEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2Support14getDisplayNameEv")]
pub fn stub_0xe862c8() -> String {
    // IDA 0xe862c8
    eagl2_support_display_name()
}

/// Window request built by `createWindow` (IDA 0xe862e4..0xe86684): the
/// arguments forwarded to `GLES2RenderSystem::createRenderWindow`
/// (0xe8667e, rendering crate, out of slice) after the config lookup.
#[derive(Debug, Clone, Default)]
pub struct Eagl2WindowRequest {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub params: Vec<(String, String)>,
}

/// `createWindow` (IDA 0xe862e4..0xe86684): with fullscreen off returns no
/// window (`v14 = 0`, 0xe86306..0xe866a2). Otherwise reads the screen frame
/// (0xe86380..0xe863ce, passed in as `screen_w`/`screen_h`), checks
/// `Full Screen == "Yes"` (0xe86410..0xe86434), forwards `Display
/// Frequency`→`displayFrequency` (0xe8646a..0xe86496) and
/// `Content Scaling Factor`→`contentScalingFactor` (0xe864dc..0xe86508),
/// splits `Video Mode` at `'x'` and parses both sides with
/// `StringConverter::parseUnsignedInt` (0xe86558..0xe865d6, screen size is
/// the fallback when the option is missing), forwards `FSAA`
/// (0xe8661c..0xe86648), then erases the temp map (0xe86680).
pub fn eagl2_support_create_window(
    support: &Eagl2Support,
    name: &str,
    fullscreen: bool,
    screen_w: u32,
    screen_h: u32,
) -> Option<Eagl2WindowRequest> {
    if !fullscreen {
        return None;
    }
    let value = |key: &str| {
        support
            .find_option(key)
            .map(|o| o.current_value.clone())
            .unwrap_or_default()
    };
    let is_full = value("Full Screen") == "Yes";
    let mut params = Vec::new();
    params.push(("displayFrequency".to_string(), value("Display Frequency")));
    params.push((
        "contentScalingFactor".to_string(),
        value("Content Scaling Factor"),
    ));
    let (mut width, mut height) = (screen_w, screen_h);
    let mode = value("Video Mode");
    if let Some(x) = mode.find('x') {
        if let (Ok(w), Ok(h)) = (
            mode[..x].trim().parse::<u32>(),
            mode[x + 1..].trim().parse::<u32>(),
        ) {
            width = w;
            height = h;
        }
    }
    params.push(("FSAA".to_string(), value("FSAA")));
    Some(Eagl2WindowRequest {
        name: name.into(),
        width,
        height,
        fullscreen: is_full,
        params,
    })
}

// 0xe862e4 — __ZN4Ogre12EAGL2Support12createWindowEbPNS_17GLES2RenderSystemERKSs
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this, bool, Ogre::GLES2RenderSystem *, const std::string *)
#[doc(alias = "__ZN4Ogre12EAGL2Support12createWindowEbPNS_17GLES2RenderSystemERKSs")]
pub fn stub_0xe862e4(
    support: &Eagl2Support,
    name: &str,
    fullscreen: bool,
    screen_w: u32,
    screen_h: u32,
) -> Option<Eagl2WindowRequest> {
    // IDA 0xe862e4
    eagl2_support_create_window(support, name, fullscreen, screen_w, screen_h)
}

/// GL context state behind `EAGLES2Context` (IDA 0xe86b80, 0xe88894):
/// active flag (+36), depth (+40) and framebuffer (+44). There is no GL on
/// the host, so `framebuffer` is only ever a placeholder handle.
#[derive(Debug, Clone, Copy, Default)]
pub struct GlContextState {
    pub active: bool,
    pub depth: i32,
    pub framebuffer: u32,
}

/// `createNewContext` failure (IDA 0xe86be6..0xe86ca8):
/// `RenderingAPIException("Fail to create new context", "createNewContext",
/// OgreEAGL2Support.mm:284)`.
#[derive(Debug, Clone)]
pub struct Eagl2ContextError {
    pub message: String,
    pub function: String,
}

/// `newWindow` (IDA 0xe86aa0..0xe86b46): `NedPoolingImpl::allocBytes(0xB8)`
/// (0xe86ada), `EAGL2Window` construct (0xe86b06), then the `vtable + 248`
/// `create` dispatch (0xe86b26, `EAGL2Window::create` 0xe89488 — not yet
/// ported, so the requested geometry is stored on the window for it).
pub fn eagl2_support_new_window(
    support: *const Eagl2Support,
    name: &str,
    width: u32,
    height: u32,
    fullscreen: bool,
    os_version: f32,
) -> Eagl2Window {
    let mut window = Eagl2Window::new(support, os_version);
    window.name = name.into();
    window.width = width;
    window.height = height;
    window.fullscreen_request = fullscreen;
    window
}

// 0xe86aa0 — __ZN4Ogre12EAGL2Support9newWindowERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "__ZN4Ogre12EAGL2Support9newWindowERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
pub fn stub_0xe86aa0(
    support: *const Eagl2Support,
    name: &str,
    width: u32,
    height: u32,
    fullscreen: bool,
    os_version: f32,
) -> Eagl2Window {
    // IDA 0xe86aa0
    eagl2_support_new_window(support, name, width, height, fullscreen, os_version)
}

/// `createNewContext` (IDA 0xe86b80..0xe86c06): `operator new(0x34)` +
/// `EAGLES2Context` construct (0xe86bb0..0xe86bde). The null check
/// (0xe86be6) is dead in practice — throwing `new` never returns null —
/// but its throw is recorded on `Eagl2ContextError` for fidelity.
pub fn eagl2_support_create_context(
    layer: usize,
    sharegroup: usize,
) -> Result<GlContextState, Eagl2ContextError> {
    let _ = (layer, sharegroup);
    Ok(GlContextState {
        active: false,
        depth: 0,
        framebuffer: 0,
    })
}

// 0xe86b80 — __ZNK4Ogre12EAGL2Support16createNewContextERPK14__CFDictionaryP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this, const __CFDictionary **, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "__ZNK4Ogre12EAGL2Support16createNewContextERPK14__CFDictionaryP11CAEAGLLayerP14EAGLSharegroup")]
pub fn stub_0xe86b80(
    layer: usize,
    sharegroup: usize,
) -> Result<GlContextState, Eagl2ContextError> {
    // IDA 0xe86b80
    eagl2_support_create_context(layer, sharegroup)
}

// 0xe86d80 — __ZN4Ogre12EAGL2Support14getProcAddressERKSs
#[doc(alias = "__ZN4Ogre12EAGL2Support14getProcAddressERKSs")]
pub fn stub_0xe86d80(_name: &str) -> usize {
    // IDA 0xe86d80..0xe86d82: MOVS R0,#0; BX LR — always null.
    0
}

// 0xe86d84 — __ZN4Ogre12EAGL2Support5startEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2Support5startEv")]
pub fn stub_0xe86d84() {
    // IDA 0xe86d84: BX LR, empty.
}

// 0xe86d88 — __ZN4Ogre12EAGL2Support4stopEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "__ZN4Ogre12EAGL2Support4stopEv")]
pub fn stub_0xe86d88() {
    // IDA 0xe86d88: BX LR, empty.
}

/// Viewport dimensions refreshed by `resize`/`windowMovedOrResized`
/// (`Viewport::_updateDimensions`, IDA 0xe887dc/0xe88880).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ViewportDims {
    pub w: u32,
    pub h: u32,
}

/// Host model of `Ogre::EAGL2Window` (IDA 0xe88388..0xe884b6): base
/// `RenderWindow` construct (0xe883aa), vtable install (0xe883c4), flag
/// words (0xe883ce..0xe883e0), support link (+41, 0xe883e6),
/// `contentScaleFactor` 1.0 (+39, 0xe883da), `os_version` from
/// `UIDevice.systemVersion.floatValue` (0xe8844c..0xe8848a) with
/// multisample enabled at 4.0+ (0xe8848e..0xe88494).
#[derive(Debug, Clone)]
pub struct Eagl2Window {
    pub name: String,
    pub support: *const Eagl2Support,
    pub gl_context: Option<GlContextState>,
    pub layer: usize,
    pub render_layer: usize,
    pub view_controller: usize,
    pub content_scale: f32,
    pub os_version: f32,
    pub supports_multisample: bool,
    pub width: u32,
    pub height: u32,
    pub view_x: i32,
    pub view_bottom: i32,
    /// `interfaceOrientation` cache: 1/2 = portrait (IDA 0xe88750).
    pub orientation: i32,
    /// Destroyed flag (+148, IDA 0xe88686..0xe88690).
    pub closed: bool,
    /// Active flag (+80, IDA 0xe883f6/0xe88694).
    pub active: bool,
    /// External-ownership flags (+150/+151/+152, IDA 0xe88698..0xe886d6).
    pub external_window: bool,
    pub external_context: bool,
    pub external_view: bool,
    /// Requested fullscreen, stored for `create` (0xe89488, next batch).
    pub fullscreen_request: bool,
    /// Last framebuffer bound by `_beginUpdate` (0x8D40 target).
    pub bound_framebuffer: u32,
    pub viewports: Vec<ViewportDims>,
}

impl Eagl2Window {
    pub fn new(support: *const Eagl2Support, os_version: f32) -> Self {
        Self {
            name: String::new(),
            support,
            gl_context: None,
            layer: 0,
            render_layer: 0,
            view_controller: 0,
            content_scale: 1.0,
            os_version,
            supports_multisample: os_version >= 4.0,
            width: 0,
            height: 0,
            view_x: 0,
            view_bottom: 0,
            orientation: 1,
            closed: false,
            active: true,
            external_window: false,
            external_context: false,
            external_view: false,
            fullscreen_request: false,
            bound_framebuffer: 0,
            viewports: Vec::new(),
        }
    }

    /// `destroy` (IDA 0xe88680..0xe886f0): no-op returning set once closed
    /// (0xe88686..0xe8868a); else mark closed/inactive (0xe88690..0xe88694),
    /// drop the render window and release the layer unless externally owned
    /// (0xe88698..0xe886b8), release the GL layer unless externally owned
    /// (0xe886bc..0xe886d2), then release the view controller unless
    /// externally owned (0xe886d6..0xe886f0). (stub_0xe88680 wires here
    /// next batch.)
    pub fn destroy(&mut self) -> usize {
        if self.closed {
            return 1;
        }
        self.closed = true;
        self.active = false;
        if !self.external_window {
            self.layer = 0;
        }
        if !self.external_context {
            self.render_layer = 0;
        }
        if self.external_view {
            return 1;
        }
        let released = self.view_controller;
        self.view_controller = 0;
        released
    }

    /// `resize` (IDA 0xe88700..0xe887fc): no layer → no-op (0xe8871c..0xe88722).
    /// Portrait (orientation 1/2, 0xe88750) requests min-first, landscape
    /// max-first (0xe88754..0xe88772); both are scaled (`vmul`, 0xe8878c..0xe88790)
    /// and only a real change recreates the framebuffer (0xe887ae..0xe887d0)
    /// and refreshes every viewport (0xe887d4..0xe887ea). (stub_0xe88700
    /// wires here next batch.)
    pub fn resize(&mut self, width: u32, height: u32) {
        if self.layer == 0 {
            return;
        }
        let (req_w, req_h) = if self.orientation == 1 || self.orientation == 2 {
            (width.min(height), width.max(height))
        } else {
            (width.max(height), width.min(height))
        };
        let new_w = (self.content_scale * req_w as f32) as u32;
        let new_h = (self.content_scale * req_h as f32) as u32;
        if new_w != self.width || new_h != self.height {
            if let Some(ctx) = self.gl_context.as_mut() {
                ctx.framebuffer = 0;
            }
            self.width = new_w;
            self.height = new_h;
            if let Some(ctx) = self.gl_context.as_mut() {
                ctx.framebuffer = 1;
            }
            for vp in &mut self.viewports {
                vp.w = new_w;
                vp.h = new_h;
            }
        }
    }

    /// `_beginUpdate` (IDA 0xe88894..0xe888b6): base
    /// `RenderTarget::_beginUpdate` (0xe8889a, no host state) then, when the
    /// context is active (0xe888a2) with depth ≥ 1 (0xe888ac),
    /// `glBindFramebuffer(0x8D40, fbo)` (0xe888b6). (stub_0xe88894 wires
    /// here next batch.)
    pub fn begin_update(&mut self) {
        let (active, depth, fbo) = match self.gl_context {
            Some(ctx) => (ctx.active, ctx.depth, ctx.framebuffer),
            None => return,
        };
        if active && depth >= 1 {
            self.bound_framebuffer = fbo;
        }
    }
}

/// Destructor body shared by the deleting (0xe884e4) and plain (0xe885b8)
/// variants: vtable install (0xe8851c/0xe885f0), `destroy` (0xe88544/0xe88618),
/// context release + null (0xe8854a..0xe8855a), base
/// `RenderTarget::~RenderTarget` (0xe88560/0xe88634).
pub fn eagl2_window_drop(window: &mut Eagl2Window) {
    window.destroy();
    window.gl_context = None;
    window.viewports.clear();
}

// 0xe88388 — __ZN4Ogre11EAGL2WindowC1EPNS_12EAGL2SupportE
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, Ogre::EAGL2Support *)
#[doc(alias = "__ZN4Ogre11EAGL2WindowC1EPNS_12EAGL2SupportE")]
pub fn stub_0xe88388(support: *const Eagl2Support, os_version: f32) -> Eagl2Window {
    // IDA 0xe88388
    Eagl2Window::new(support, os_version)
}

// 0xe884e4 — __ZN4Ogre11EAGL2WindowD0Ev
// type: void __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZN4Ogre11EAGL2WindowD0Ev")]
pub fn stub_0xe884e4(window: Box<Eagl2Window>) {
    // IDA 0xe884e4: D1 body above plus `deallocBytes` (0xe8856a) — the `Box`
    // drop is the deallocation.
    let mut window = window;
    eagl2_window_drop(&mut window);
}

// 0xe885b8 — __ZN4Ogre11EAGL2WindowD1Ev
// type: void __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZN4Ogre11EAGL2WindowD1Ev")]
pub fn stub_0xe885b8(window: &mut Eagl2Window) {
    // IDA 0xe885b8
    eagl2_window_drop(window);
}

// 0xe88680 — __ZN4Ogre11EAGL2Window7destroyEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZN4Ogre11EAGL2Window7destroyEv")]
pub fn stub_0xe88680() -> ! {
    todo!("0xe88680 __ZN4Ogre11EAGL2Window7destroyEv")
}

// 0xe886f8 — __ZN4Ogre11EAGL2Window13setFullscreenEbjj
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool, unsigned int, unsigned int)
#[doc(alias = "__ZN4Ogre11EAGL2Window13setFullscreenEbjj")]
pub fn stub_0xe886f8() -> ! {
    todo!("0xe886f8 __ZN4Ogre11EAGL2Window13setFullscreenEbjj")
}

// 0xe886fc — __ZN4Ogre11EAGL2Window10repositionEii
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, int, int)
#[doc(alias = "__ZN4Ogre11EAGL2Window10repositionEii")]
pub fn stub_0xe886fc() -> ! {
    todo!("0xe886fc __ZN4Ogre11EAGL2Window10repositionEii")
}

// 0xe88700 — __ZN4Ogre11EAGL2Window6resizeEjj
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, unsigned int, unsigned int)
#[doc(alias = "__ZN4Ogre11EAGL2Window6resizeEjj")]
pub fn stub_0xe88700() -> ! {
    todo!("0xe88700 __ZN4Ogre11EAGL2Window6resizeEjj")
}

// 0xe88800 — __ZN4Ogre11EAGL2Window20windowMovedOrResizedEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZN4Ogre11EAGL2Window20windowMovedOrResizedEv")]
pub fn stub_0xe88800() -> ! {
    todo!("0xe88800 __ZN4Ogre11EAGL2Window20windowMovedOrResizedEv")
}

// 0xe88894 — __ZN4Ogre11EAGL2Window12_beginUpdateEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZN4Ogre11EAGL2Window12_beginUpdateEv")]
pub fn stub_0xe88894() -> ! {
    todo!("0xe88894 __ZN4Ogre11EAGL2Window12_beginUpdateEv")
}

// 0xe888bc — __ZN4Ogre11EAGL2Window23initNativeCreatedWindowEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "__ZN4Ogre11EAGL2Window23initNativeCreatedWindowEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
pub fn stub_0xe888bc() -> ! {
    todo!("0xe888bc __ZN4Ogre11EAGL2Window23initNativeCreatedWindowEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")
}

// 0xe89488 — __ZN4Ogre11EAGL2Window6createERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "__ZN4Ogre11EAGL2Window6createERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
pub fn stub_0xe89488() -> ! {
    todo!("0xe89488 __ZN4Ogre11EAGL2Window6createERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")
}

// 0xe89c80 — __ZN4Ogre11EAGL2Window11swapBuffersEb
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool)
#[doc(alias = "__ZN4Ogre11EAGL2Window11swapBuffersEb")]
pub fn stub_0xe89c80() -> ! {
    todo!("0xe89c80 __ZN4Ogre11EAGL2Window11swapBuffersEb")
}

// 0xe89f88 — __ZN4Ogre11EAGL2Window18getCustomAttributeERKSsPv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, const std::string *, void *)
#[doc(alias = "__ZN4Ogre11EAGL2Window18getCustomAttributeERKSsPv")]
pub fn stub_0xe89f88() -> ! {
    todo!("0xe89f88 __ZN4Ogre11EAGL2Window18getCustomAttributeERKSsPv")
}

// 0xe8a038 — __ZN4Ogre11EAGL2Window20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE
#[doc(alias = "__ZN4Ogre11EAGL2Window20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE")]
pub fn stub_0xe8a038() -> ! {
    todo!("0xe8a038 __ZN4Ogre11EAGL2Window20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE")
}

// 0xe8a554 — __ZNK4Ogre11EAGL2Window23requiresTextureFlippingEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZNK4Ogre11EAGL2Window23requiresTextureFlippingEv")]
pub fn stub_0xe8a554() -> ! {
    todo!("0xe8a554 __ZNK4Ogre11EAGL2Window23requiresTextureFlippingEv")
}

// 0xe8a568 — __ZNK4Ogre11EAGL2Window9isVisibleEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZNK4Ogre11EAGL2Window9isVisibleEv")]
pub fn stub_0xe8a568() -> ! {
    todo!("0xe8a568 __ZNK4Ogre11EAGL2Window9isVisibleEv")
}

// 0xe8a570 — __ZN4Ogre11EAGL2Window10setVisibleEb
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool)
#[doc(alias = "__ZN4Ogre11EAGL2Window10setVisibleEb")]
pub fn stub_0xe8a570() -> ! {
    todo!("0xe8a570 __ZN4Ogre11EAGL2Window10setVisibleEb")
}

// 0xe8a590 — __ZNK4Ogre11EAGL2Window8isClosedEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "__ZNK4Ogre11EAGL2Window8isClosedEv")]
pub fn stub_0xe8a590() -> ! {
    todo!("0xe8a590 __ZNK4Ogre11EAGL2Window8isClosedEv")
}

// 0xe8a698 — __ZN4Ogre14EAGLES2ContextC1EP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "__ZN4Ogre14EAGLES2ContextC1EP11CAEAGLLayerP14EAGLSharegroup")]
pub fn stub_0xe8a698() -> ! {
    todo!("0xe8a698 __ZN4Ogre14EAGLES2ContextC1EP11CAEAGLLayerP14EAGLSharegroup")
}

// 0xe8a6a4 — __ZN4Ogre14EAGLES2ContextC2EP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "__ZN4Ogre14EAGLES2ContextC2EP11CAEAGLLayerP14EAGLSharegroup")]
pub fn stub_0xe8a6a4() -> ! {
    todo!("0xe8a6a4 __ZN4Ogre14EAGLES2ContextC2EP11CAEAGLLayerP14EAGLSharegroup")
}

// 0xe8a970 — __ZN4Ogre14EAGLES2ContextD0Ev
// type: void __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2ContextD0Ev")]
pub fn stub_0xe8a970() -> ! {
    todo!("0xe8a970 __ZN4Ogre14EAGLES2ContextD0Ev")
}

// 0xe8aab4 — __ZN4Ogre14EAGLES2ContextD1Ev
// type: void __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2ContextD1Ev")]
pub fn stub_0xe8aab4() -> ! {
    todo!("0xe8aab4 __ZN4Ogre14EAGLES2ContextD1Ev")
}

// 0xe8abf8 — __ZN4Ogre14EAGLES2Context18destroyFramebufferEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2Context18destroyFramebufferEv")]
pub fn stub_0xe8abf8() -> ! {
    todo!("0xe8abf8 __ZN4Ogre14EAGLES2Context18destroyFramebufferEv")
}

// 0xe8ac58 — __ZN4Ogre14EAGLES2Context17createFramebufferEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2Context17createFramebufferEv")]
pub fn stub_0xe8ac58() -> ! {
    todo!("0xe8ac58 __ZN4Ogre14EAGLES2Context17createFramebufferEv")
}

// 0xe8b298 — __ZN4Ogre14EAGLES2Context10setCurrentEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2Context10setCurrentEv")]
pub fn stub_0xe8b298() -> ! {
    todo!("0xe8b298 __ZN4Ogre14EAGLES2Context10setCurrentEv")
}

// 0xe8b488 — __ZN4Ogre14EAGLES2Context10endCurrentEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZN4Ogre14EAGLES2Context10endCurrentEv")]
pub fn stub_0xe8b488() -> ! {
    todo!("0xe8b488 __ZN4Ogre14EAGLES2Context10endCurrentEv")
}

// 0xe8b48c — __ZNK4Ogre14EAGLES2Context5cloneEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZNK4Ogre14EAGLES2Context5cloneEv")]
pub fn stub_0xe8b48c() -> ! {
    todo!("0xe8b48c __ZNK4Ogre14EAGLES2Context5cloneEv")
}

// 0xe8b490 — __ZNK4Ogre14EAGLES2Context10getContextEv
// type: _DWORD __fastcall(Ogre::EAGLES2Context *__hidden this)
#[doc(alias = "__ZNK4Ogre14EAGLES2Context10getContextEv")]
pub fn stub_0xe8b490() -> ! {
    todo!("0xe8b490 __ZNK4Ogre14EAGLES2Context10getContextEv")
}

// 0xf1f1c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_0xf1f1c8() -> ! {
    todo!("0xf1f1c8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf1f270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_0xf1f270() -> ! {
    todo!("0xf1f270 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf1f2f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")]
pub fn stub_0xf1f2f4() -> ! {
    todo!("0xf1f2f4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")
}

// 0xf1f360 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")]
pub fn stub_0xf1f360() -> ! {
    todo!("0xf1f360 __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")
}

// 0xf267d4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")]
pub fn stub_0xf267d4() -> ! {
    todo!("0xf267d4 j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")
}

// 0xf267e4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf267e4() -> ! {
    todo!("0xf267e4 j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

// 0xf26834 — j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")]
pub fn stub_0xf26834() -> ! {
    todo!("0xf26834 j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")
}

// 0xf268d4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")]
pub fn stub_0xf268d4() -> ! {
    todo!("0xf268d4 j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")
}

// 0xf268e4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf268e4() -> ! {
    todo!("0xf268e4 j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

// 0xf26904 — j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")]
pub fn stub_0xf26904() -> ! {
    todo!("0xf26904 j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")
}

// 0xf26954 — j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_")]
pub fn stub_0xf26954() -> ! {
    todo!("0xf26954 j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_")
}

// 0xf26964 — j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")]
pub fn stub_0xf26964() -> ! {
    todo!("0xf26964 j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")
}

// 0xf26974 — j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")]
pub fn stub_0xf26974() -> ! {
    todo!("0xf26974 j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")
}

// 0xf26984 — j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
pub fn stub_0xf26984() -> ! {
    todo!("0xf26984 j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")
}

// 0xf269b4 — j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
#[doc(alias = "j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")]
pub fn stub_0xf269b4() -> ! {
    todo!("0xf269b4 j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")
}

// 0xf269f4 — j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_")]
pub fn stub_0xf269f4() -> ! {
    todo!("0xf269f4 j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_")
}

// 0xf26a34 — j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_")]
pub fn stub_0xf26a34() -> ! {
    todo!("0xf26a34 j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_")
}

// 0xf26a64 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf26a64() -> ! {
    todo!("0xf26a64 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0xf26a74 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf26a74() -> ! {
    todo!("0xf26a74 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0xf26ad4 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
// type: int __fastcall(int, int)
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_")]
pub fn stub_0xf26ad4() -> ! {
    todo!("0xf26ad4 j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_")
}

// 0xf26b24 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_")]
pub fn stub_0xf26b24() -> ! {
    todo!("0xf26b24 j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_")
}

// 0xf26ca4 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_")]
pub fn stub_0xf26ca4() -> ! {
    todo!("0xf26ca4 j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_")
}

// 0xf26cc4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf26cc4() -> ! {
    todo!("0xf26cc4 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")
}

// 0xf26cd4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf26cd4() -> ! {
    todo!("0xf26cd4 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0xf26da4 — j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: int __fastcall(RobloxView::ViewUpdateJob *this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE")]
pub fn stub_0xf26da4() -> ! {
    todo!("0xf26da4 j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE")
}

// 0xf26f14 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf26f14() -> ! {
    todo!("0xf26f14 j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")
}

// 0xf26f54 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv")]
pub fn stub_0xf26f54() -> ! {
    todo!("0xf26f54 j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv")
}

// 0xf26f64 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_")]
pub fn stub_0xf26f64() -> ! {
    todo!("0xf26f64 j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_")
}

// 0xf26f74 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_")]
pub fn stub_0xf26f74() -> ! {
    todo!("0xf26f74 j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_")
}

// 0xf26f84 — j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv")]
pub fn stub_0xf26f84() -> ! {
    todo!("0xf26f84 j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv")
}

// 0xf26f94 — j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_")]
pub fn stub_0xf26f94() -> ! {
    todo!("0xf26f94 j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_")
}

// 0xf26fa4 — j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_")]
pub fn stub_0xf26fa4() -> ! {
    todo!("0xf26fa4 j___ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_")
}

// 0xf27164 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int()
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf27164() -> ! {
    todo!("0xf27164 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

// 0xf27174 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
// type: int()
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv")]
pub fn stub_0xf27174() -> ! {
    todo!("0xf27174 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv")
}

// 0xf271b4 — j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")]
pub fn stub_0xf271b4() -> ! {
    todo!("0xf271b4 j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")
}

// 0xf271c4 — j___ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_")]
pub fn stub_0xf271c4() -> ! {
    todo!("0xf271c4 j___ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_")
}

// 0xf27264 — j___ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: int __fastcall(RobloxView::RenderJob *this, const std::string *)
#[doc(alias = "j___ZNK10RobloxView9RenderJob14getMetricValueERKSs")]
pub fn stub_0xf27264() -> ! {
    todo!("0xf27264 j___ZNK10RobloxView9RenderJob14getMetricValueERKSs")
}

// 0xf27274 — j___ZNK10RobloxView9RenderJob9getMetricERKSs
// type: int __fastcall(RobloxView::RenderJob *this, const std::string *)
#[doc(alias = "j___ZNK10RobloxView9RenderJob9getMetricERKSs")]
pub fn stub_0xf27274() -> ! {
    todo!("0xf27274 j___ZNK10RobloxView9RenderJob9getMetricERKSs")
}

// 0xf27304 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf27304() -> ! {
    todo!("0xf27304 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xf27314 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf27314() -> ! {
    todo!("0xf27314 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xf53834 — j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii
#[doc(alias = "j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii")]
pub fn stub_0xf53834() -> ! {
    todo!("0xf53834 j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii")
}

// 0xf53844 — j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii
// type: int __fastcall(int, int, int, int, int, std::locale *, int, int, int)
#[doc(alias = "j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii")]
pub fn stub_0xf53844() -> ! {
    todo!("0xf53844 j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii")
}

// 0xf53854 — j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev")]
pub fn stub_0xf53854() -> ! {
    todo!("0xf53854 j___ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev")
}

// 0xf539f4 — j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii
// type: int __fastcall(int)
#[doc(alias = "j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii")]
pub fn stub_0xf539f4() -> ! {
    todo!("0xf539f4 j___ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii")
}

// 0xf53a34 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
pub fn stub_0xf53a34() -> ! {
    todo!("0xf53a34 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

// 0xf53a44 — j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")]
pub fn stub_0xf53a44() -> ! {
    todo!("0xf53a44 j___ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

// 0xf53aa4 — j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
#[doc(alias = "j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")]
pub fn stub_0xf53aa4() -> ! {
    todo!("0xf53aa4 j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")
}

// 0xf53ab4 — j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
#[doc(alias = "j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")]
pub fn stub_0xf53ab4() -> ! {
    todo!("0xf53ab4 j___ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

// 0xf53b24 — j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii
#[doc(alias = "j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii")]
pub fn stub_0xf53b24() -> ! {
    todo!("0xf53b24 j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii")
}

// 0xf53b34 — j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")]
pub fn stub_0xf53b34() -> ! {
    todo!("0xf53b34 j___ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}
