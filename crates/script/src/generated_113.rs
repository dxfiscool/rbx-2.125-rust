// Auto-generated skeletons for rbx-script — filler EA-sorted asc (global holes)
// Filter: Lua|Script|Yield|lua (case-sensitive, lua lower) -> 5401 filtered, all stubbed (0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x9fa34..0xa7594 | filtered 5401 done, script 11785->11885 total, global 79921->80021 covered, 5525 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use parking_lot::Mutex;
use rbx_core::SharedPtr;
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};

// ---- FMOD OggVorbis + playlist codec host model (IDA 0x9fa34..0xa0c58) ----
// Target is 32-bit ARM; interior image words are plain u32/i32 slots so the
// byte offsets cited below hold on any host.
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Boost mapping: no shared_ptr in this range; nullable `this`/state pointers
// become Option<&T> (None follows the target's -28 container adjust to a null
// codec and returns FMOD_OK instead of faulting), MemPool alloc/free becomes
// Vec<u8> ownership with the same per-codec usage accounting.
// Unmodeled: the vorbis decoder itself (ov_* entry points are host shims over
// VorbisStream), the OS file layer (FmodFile is an in-memory cursor),
// FMOD::CodecWav::parseChunk (host RIFF walker below), and the +56 metadata
// callback slot (kept as an optional hook, unset by default).

/// FMOD_RESULT success (IDA FMOD_OK).
pub const FMOD_OK: i32 = 0;
/// Host end-of-file marker for FMOD::File reads (target reuses 22 here).
pub const FMOD_EOF: i32 = 22;

/// FMOD seek origins: 0 = SET, 1 = CUR, 2 = END.
pub const FMOD_SEEK_SET: i32 = 0;
pub const FMOD_SEEK_CUR: i32 = 1;
pub const FMOD_SEEK_END: i32 = 2;

/// Host model of `FMOD::File`: a byte cursor with a seekable flag.
/// IDA 0x9fd64: seekable bit is word +98 (`[R0,#0x188] & 1`).
#[derive(Debug, Default)]
pub struct FmodFile {
    pub data: Vec<u8>,
    pub pos: usize,
    pub seekable: bool,
}

impl FmodFile {
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    /// was: `FMOD::File::seek(File *,int,int)` — (offset, whence).
    /// IDA 0xa0760/0xa08b0: `seek(f,-1,1)` backs up one byte; `seek(f,0,0)`
    /// rewinds. Out-of-range moves fail (nonzero propagates at 0xa0768).
    pub fn seek(&mut self, offset: i32, whence: i32) -> i32 {
        let base: i64 = match whence {
            FMOD_SEEK_SET => 0,
            FMOD_SEEK_CUR => self.pos as i64,
            FMOD_SEEK_END => self.data.len() as i64,
            // BUG(host): unknown whence has no observed code; 20 matches the
            // setPositionInternal "unknown failure" mapping (IDA 0x9fb8c).
            _ => return 20,
        };
        let next = base + offset as i64;
        if next < 0 || next > self.data.len() as i64 {
            return 20;
        }
        self.pos = next as usize;
        FMOD_OK
    }

    /// was: `FMOD::File::tell(File *,unsigned int *)` (IDA 0xa0454).
    pub fn tell(&self) -> u32 {
        self.pos as u32
    }

    /// was: `FMOD::File::read(File *,void *,uint,uint,uint *)`.
    /// Returns `(code, bytes_read)`; 0 = full, 22 = short (EOF) — both are
    /// success at IDA 0x9fe64..0x9fe6c, anything else fails the caller.
    pub fn read(&mut self, out: &mut [u8], size: u32, count: u32) -> (i32, u32) {
        let want = size as usize * count as usize;
        let avail = self.data.len().saturating_sub(self.pos);
        let n = want.min(avail).min(out.len());
        out[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        if n >= want {
            (FMOD_OK, n as u32)
        } else {
            (FMOD_EOF, n as u32)
        }
    }

    /// was: `FMOD::File::getByte(File *,uchar *)` (IDA 0xa06dc).
    pub fn get_byte(&mut self) -> (i32, u8) {
        if self.pos < self.data.len() {
            let b = self.data[self.pos];
            self.pos += 1;
            (FMOD_OK, b)
        } else {
            (FMOD_EOF, 0)
        }
    }
}

/// One stored codec metadata row.
/// was: `FMOD::Codec::metaData` (numeric class/type slots noted per call).
#[derive(Debug, Clone, Default)]
pub struct CodecMeta {
    pub key: String,
    pub value: Vec<u8>,
    pub index: u8,
}

/// Host vorbis stream: the +272 area (`ov_*` state, 0x2B0 bytes on target).
#[derive(Debug, Default)]
pub struct VorbisStream {
    pub open: bool,
    pub comments: Vec<String>,
    pub comments_live: bool,
    pub pcm_pos: i64,
    pub pcm_total: i64,
    pub stream_totals: Vec<i64>,
    pub channels: u32,
    pub rate: u32,
    pub remaining: u64,
    /// One-shot injected `ov_read` error (0/None = decode normally).
    pub decode_error: Option<i32>,
    pub raw_pos: u32,
}

/// was: `ov_open_callbacks` (IDA 0xa0128).
fn vorbis_open(stream: &mut VorbisStream) {
    stream.open = true;
    stream.pcm_pos = 0;
}

/// was: `ov_read` (IDA 0x9fbf8): bytes decoded, or negative status.
/// The caller maps -131 -> 37, -139 -> 44, -3 -> 0 bytes/continue (IDA 0x9fcf0).
fn vorbis_read(stream: &mut VorbisStream, out: &mut [u8], bytes: usize) -> i32 {
    if let Some(err) = stream.decode_error.take() {
        return err;
    }
    let n = (bytes as u64).min(stream.remaining).min(out.len() as u64) as usize;
    out[..n].fill(0);
    stream.remaining -= n as u64;
    stream.pcm_pos += n as i64;
    n as i32
}

/// was: `ov_comment(vorbis,-1)` (IDA 0x9faa8/0x9fc10).
fn vorbis_comments(stream: &VorbisStream) -> Option<&[String]> {
    if stream.comments_live {
        Some(&stream.comments)
    } else {
        None
    }
}

/// was: `FMOD_vorbis_comment_clear` (IDA 0x9fcd0).
fn vorbis_comment_clear(stream: &mut VorbisStream) {
    stream.comments.clear();
    stream.comments_live = false;
}

/// was: `ov_pcm_seek` (IDA 0x9fb80).
fn vorbis_pcm_seek(stream: &mut VorbisStream, pos: u32) -> i32 {
    if !stream.open {
        return -139;
    }
    if pos as i64 > stream.pcm_total {
        return -1;
    }
    stream.pcm_pos = pos as i64;
    0
}

/// was: `ov_clear(handle, vorbis_area)` (IDA 0x9fd44).
fn vorbis_clear(stream: &mut VorbisStream) {
    stream.open = false;
    stream.remaining = 0;
    vorbis_comment_clear(stream);
}

/// was: `_FMOD_vorbis_window_init` (IDA 0xa0228).
fn vorbis_window_init() {}

/// `FMOD::CodecOggVorbis::gInitialized` (IDA 0xa00a4).
static OGGVORBIS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Wave-format words carried at codec +968 (IDA 0xa029c..0xa02cc).
#[derive(Debug, Clone, Default)]
pub struct OggWaveFormat {
    /// Word +260 off the format block (doubled into +276 at IDA 0xa02cc).
    pub words_260: u32,
    /// Total PCM accumulator at +272 (IDA 0xa039c), or 0x7FFF_FFFF when the
    /// file is not seekable (IDA 0xa0304).
    pub total_pcm: u32,
    /// Byte +268 bias, -1 until adjusted by the data start (IDA 0xa02e0).
    pub start_bias: i32,
}

impl OggWaveFormat {
    fn fresh() -> OggWaveFormat {
        OggWaveFormat { words_260: 0, total_pcm: 0, start_bias: -1 }
    }
}

/// `FMOD::CodecOggVorbis` host state. Offsets are target byte offsets.
#[derive(Debug, Default)]
pub struct OggVorbisCodec {
    /// Byte +269: getMemoryUsed dedupe flag (IDA 0x9fa50..0x9fa84).
    pub mem_reported: Mutex<bool>,
    /// Byte +272: vorbis area (IDA 0x9fd44, 0xa00fc memset 0x2B0).
    pub vorbis: Mutex<VorbisStream>,
    /// Byte +196: data start / file offset (IDA 0xa0044, 0xa0308).
    pub data_start: Mutex<u32>,
    /// Byte +200/+204: stream words (IDA 0xa024c/0xa0254).
    pub stream_words: Mutex<(u32, u32)>,
    /// Byte +260: codec state word, cleared on open (IDA 0x9ff18).
    pub state_word: Mutex<u32>,
    /// Byte +264: backing file (IDA 0x9ff10).
    pub file: Mutex<Option<FmodFile>>,
    /// Bytes +960/+964: adopted WAV parse products (IDA 0xa025c/0xa0264).
    pub wav_extra: Mutex<Option<Vec<u8>>>,
    pub wav_word_78: Mutex<u32>,
    /// Byte +968: wave-format block (IDA 0xa029c).
    pub wave_format: Mutex<Option<OggWaveFormat>>,
    /// Bytes +1224/+1228/+1232: length words (IDA 0xa02ac..0xa02bc).
    pub length_words: Mutex<(u32, u32, u32)>,
    /// Byte +1236: file length (IDA 0xa02a4).
    pub file_len: Mutex<u32>,
    /// Byte +1264: decoder memory-usage counter (IDA 0x9fe98/0xa05b4).
    pub mem_usage: Mutex<u32>,
    /// Ogg `metaData` rows, class 3 (IDA 0x9fb5c).
    pub meta: Mutex<Vec<CodecMeta>>,
    /// Function slot at +56 invoked per comment by readInternal
    /// (IDA 0x9fc8c/0x9fcb8); unset on host.
    pub metadata_notify: Mutex<Option<fn(&OggVorbisCodec, &[u8])>>,
}

/// `FMOD_CODEC_STATE` view: 28-byte header then the codec (IDA 0x9fa40: -28).
#[repr(C)]
pub struct CodecStateOgg {
    pub _header: [u8; 28],
    pub pub_codec: OggVorbisCodec,
}

impl Default for CodecStateOgg {
    fn default() -> CodecStateOgg {
        CodecStateOgg { _header: [0; 28], pub_codec: OggVorbisCodec::default() }
    }
}

/// `FMOD::MemoryTracker` host carrier for 0x9fa34.
#[derive(Debug, Default)]
pub struct MemoryTracker {
    pub total: Mutex<u32>,
}

/// was: `FMOD::CodecOggVorbis::getMemoryUsedImpl` (IDA 0x9fa10, audio crate
/// stub): account the decoder usage counter into the tracker.
fn oggvorbis_get_memory_used_impl(codec: &OggVorbisCodec, tracker: Option<&MemoryTracker>) -> i32 {
    if let Some(t) = tracker {
        *t.total.lock() += *codec.mem_usage.lock();
    }
    FMOD_OK
}

/// `FMOD::CodecPlaylist` host state: file at +66 words = +264 bytes
/// (IDA 0xa06d8: `[R4,#0x108]`), plus stored `metaData` rows (class 8).
#[derive(Debug, Default)]
pub struct CodecPlaylist {
    pub file: Mutex<Option<FmodFile>>,
    pub meta: Mutex<Vec<CodecMeta>>,
}

/// `FMOD_CODEC_STATE` view for the playlist codec (IDA 0xa0690: -28).
#[repr(C)]
pub struct CodecStatePlaylist {
    pub _header: [u8; 28],
    pub pub_codec: CodecPlaylist,
}

impl Default for CodecStatePlaylist {
    fn default() -> CodecStatePlaylist {
        CodecStatePlaylist { _header: [0; 28], pub_codec: CodecPlaylist::default() }
    }
}

/// Ogg `Codec::metaData` row (IDA 0x9fb5c: class 3, type 3).
fn ogg_meta(codec: &OggVorbisCodec, key: &str, value: &[u8]) {
    codec.meta.lock().push(CodecMeta { key: key.to_owned(), value: value.to_vec(), index: 0 });
}

/// Playlist `Codec::metaData` row (IDA 0xa0c1c: class 8, `index` = entry #).
fn playlist_meta(codec: &CodecPlaylist, key: &str, value: &[u8], index: u8) {
    codec.meta.lock().push(CodecMeta { key: key.to_owned(), value: value.to_vec(), index });
}

/// was: `FMOD_strncmp` — first-`n`-bytes compare (IDA 0xa0038/0xa0088).
fn fmod_strncmp(a: &[u8], b: &[u8], n: usize) -> i32 {
    let len = n.min(a.len()).min(b.len());
    for i in 0..len {
        if a[i] != b[i] {
            return a[i] as i32 - b[i] as i32;
        }
    }
    if a.len() >= n && b.len() >= n {
        0
    } else {
        a.len().min(n) as i32 - b.len().min(n) as i32
    }
}

/// was: `FMOD_strnicmp` — case-insensitive first-`n` compare (IDA 0xa0cac).
fn fmod_strnicmp(a: &[u8], b: &[u8], n: usize) -> i32 {
    let len = n.min(a.len()).min(b.len());
    for i in 0..len {
        let (x, y) = (a[i].to_ascii_lowercase(), b[i].to_ascii_lowercase());
        if x != y {
            return x as i32 - y as i32;
        }
    }
    if a.len() >= n && b.len() >= n {
        0
    } else {
        a.len().min(n) as i32 - b.len().min(n) as i32
    }
}

/// was: `atoi` (IDA 0xa0e50).
fn fmod_atoi(s: &[u8]) -> i32 {
    let mut neg = false;
    let mut val: i32 = 0;
    let mut it = s.iter().peekable();
    if it.peek() == Some(&&b'-') {
        neg = true;
        it.next();
    }
    for &c in it {
        if !c.is_ascii_digit() {
            break;
        }
        val = val.saturating_mul(10).saturating_add((c - b'0') as i32);
    }
    if neg { -val } else { val }
}

/// Split a raw `KEY=value` comment the way the 0x9faf0 scan does: the first
/// `=` (or string end) terminates the key.
fn split_comment(raw: &str) -> Option<(&str, &str)> {
    raw.find('=').map(|eq| (&raw[..eq], &raw[eq + 1..]))
}

/// Ogg codec description table (IDA 0x9fd80: 0x7C bytes cleared then filled;
/// slots hold callee addresses on target, symbols on host).
#[derive(Debug, Clone, Default)]
pub struct OggVorbisCodecDesc {
    pub name: &'static str,         // IDA 0x9fdac
    pub version: u32,               // IDA 0x9fdb4: 65792
    pub format_tag: u32,            // IDA 0x9fdbc: 2
    pub open: &'static str,         // IDA 0x9fdc8
    pub close: &'static str,        // IDA 0x9fdd4
    pub read: &'static str,         // IDA 0x9fde0
    pub set_position: &'static str, // IDA 0x9fdec
    pub get_memory_used: &'static str, // IDA 0x9fdf8
    pub block_count: u32,           // IDA 0x9fe00: 14
    pub block_bytes: u32,           // IDA 0x9fe08: 1268
}

static OGGVORBIS_CODEC_DESC: LazyLock<OggVorbisCodecDesc> = LazyLock::new(|| OggVorbisCodecDesc {
    name: "FMOD Ogg Vorbis Codec",
    version: 65792,
    format_tag: 2,
    open: "FMOD::CodecOggVorbis::openCallback",
    close: "FMOD::CodecOggVorbis::closeCallback",
    read: "FMOD::CodecOggVorbis::readCallback",
    set_position: "FMOD::CodecOggVorbis::setPositionCallback",
    get_memory_used: "FMOD::CodecOggVorbis::getMemoryUsedCallback",
    block_count: 14,
    block_bytes: 1268,
});

/// Global MemPool carrier (`dword_130F450`); host tracks live bytes only.
#[derive(Debug, Default)]
struct OggMemPool {
    live_bytes: usize,
}

static OGG_MEM_POOL: LazyLock<Mutex<OggMemPool>> = LazyLock::new(|| Mutex::new(OggMemPool::default()));

// 0x9fa34 — __ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
// IDA 0x9fa34: null state adjusts to a null codec (0x9fa40..0x9fa44); with a
// tracker the +269 flag short-circuits to 0, else getMemoryUsedImpl runs and a
// success sets the flag (0x9fa50..0x9fa70); without a tracker the impl runs
// with a null tracker and a success clears the flag (0x9fa7c..0x9fa84).
// BUG(host): the target faults reading +269 off a null codec; the host
// returns FMOD_OK for a null codec instead.
pub fn stub_0x9fa34(state: Option<&CodecStateOgg>, tracker: Option<&MemoryTracker>) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    let codec = &state.pub_codec;
    if tracker.is_some() {
        if *codec.mem_reported.lock() {
            return FMOD_OK;
        }
        let result = oggvorbis_get_memory_used_impl(codec, tracker);
        if result == FMOD_OK {
            *codec.mem_reported.lock() = true;
        }
        result
    } else {
        let result = oggvorbis_get_memory_used_impl(codec, None);
        if result == FMOD_OK {
            *codec.mem_reported.lock() = false;
        }
        result
    }
}

// 0x9fa8c — __ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::readVorbisComments(void)")]
// IDA 0x9fa8c: walk the vorbis comment vector (0x9fac0..0x9fb10); skip empty
// entries (0x9facc); scan each comment to the first `=`/NUL (0x9faf0..0x9fafc);
// on `=` split in place and file `Codec::metaData(this,3,key,val,len+1,3,0)`
// (0x9fb2c..0x9fb5c), propagating its result (0x9fb64).
pub fn stub_0x9fa8c(codec: &OggVorbisCodec) -> i32 {
    let raws: Vec<String> = {
        let stream = codec.vorbis.lock();
        match vorbis_comments(&stream) {
            Some(c) => c.to_vec(),
            None => return FMOD_OK,
        }
    };
    if raws.is_empty() {
        return FMOD_OK;
    }
    for raw in &raws {
        if raw.is_empty() {
            continue;
        }
        if let Some((key, value)) = split_comment(raw) {
            ogg_meta(codec, key, value.as_bytes());
        }
    }
    FMOD_OK
}

// 0x9fb70 — __ZN4FMOD14CodecOggVorbis19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecOggVorbis *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionInternal(int,unsigned int,unsigned int)")]
// IDA 0x9fb70: `ov_pcm_seek` (0x9fb80); >= 0 -> 0 (0x9fb88), -139 -> 44
// (0x9fb94..0x9fb98), anything else -> 20 (0x9fb8c).
pub fn stub_0x9fb70(codec: &OggVorbisCodec, _a2: i32, pos: u32, _a4: u32) -> i32 {
    let rc = vorbis_pcm_seek(&mut codec.vorbis.lock(), pos);
    if rc >= 0 {
        FMOD_OK
    } else if rc == -139 {
        44
    } else {
        20
    }
}

// 0x9fba0 — __ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecOggVorbis *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// IDA 0x9fba0: state -28 (0x9fba4), tail-call setPositionInternal.
// BUG(host): null state returns FMOD_OK instead of faulting (see 0x9fa34).
pub fn stub_0x9fba0(state: Option<&CodecStateOgg>, a2: i32, a3: u32, a4: u32) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    stub_0x9fb70(&state.pub_codec, a2, a3, a4)
}

// 0x9fbac — __ZN4FMOD14CodecOggVorbis12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readInternal(void *,unsigned int,unsigned int *)")]
// IDA 0x9fbac: `ov_read` into the buffer (0x9fbf8), store bytes (0x9fbfc);
// -131 -> 37, -139 -> 44, other negatives except -3 -> 0 bytes + 22
// (0x9fcf0..0x9fd14); -3 and success fall into the comment pass (0x9fc10):
// each comment scanned to `=`/NUL (0x9fc40..0x9fc64), the +56 slot invoked
// with this+28 (0x9fc8c..0x9fcb8), then `vorbis_comment_clear` (0x9fcd0).
pub fn stub_0x9fbac(codec: &OggVorbisCodec, out: &mut [u8], bytes: u32, decoded: &mut u32) -> i32 {
    let n = vorbis_read(&mut codec.vorbis.lock(), out, bytes as usize);
    *decoded = n.max(0) as u32;
    if n <= 0 {
        if n == -131 {
            *decoded = 0;
            return 37;
        }
        if n == -139 {
            return 44;
        }
        if n != -3 {
            *decoded = 0;
            return 22;
        }
        *decoded = 0;
    }
    let raws: Vec<String> = match vorbis_comments(&codec.vorbis.lock()) {
        Some(c) if !c.is_empty() => c.to_vec(),
        _ => return FMOD_OK,
    };
    for raw in &raws {
        let value: &[u8] = match split_comment(raw) {
            Some((_, v)) => v.as_bytes(),
            None => raw.as_bytes(),
        };
        if let Some(notify) = *codec.metadata_notify.lock() {
            notify(codec, value);
        }
    }
    vorbis_comment_clear(&mut codec.vorbis.lock());
    FMOD_OK
}

// 0x9fd24 — __ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// IDA 0x9fd24: state -28 (0x9fd28), tail-call readInternal.
// BUG(host): null state returns FMOD_OK instead of faulting (see 0x9fa34).
pub fn stub_0x9fd24(
    state: Option<&CodecStateOgg>,
    out: &mut [u8],
    bytes: u32,
    decoded: &mut u32,
) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    stub_0x9fbac(&state.pub_codec, out, bytes, decoded)
}

// 0x9fd30 — __ZN4FMOD14CodecOggVorbis13closeInternalEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::closeInternal(void)")]
// IDA 0x9fd30: clear the word at +68 (0x9fd40), `ov_clear(this, this+272)`
// (0x9fd44), return 0 (0x9fd4c).
pub fn stub_0x9fd30(codec: &OggVorbisCodec) -> i32 {
    vorbis_clear(&mut codec.vorbis.lock());
    FMOD_OK
}

// 0x9fd50 — __ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecOggVorbis *)
#[doc(alias = "FMOD::CodecOggVorbis::closeCallback(FMOD_CODEC_STATE *)")]
// IDA 0x9fd50: state -28 (0x9fd54), tail-call closeInternal.
// BUG(host): null state returns FMOD_OK instead of faulting (see 0x9fa34).
pub fn stub_0x9fd50(state: Option<&CodecStateOgg>) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    stub_0x9fd30(&state.pub_codec)
}

// 0x9fd5c — __ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi
// type: int __fastcall(FMOD *this, int, __int64, int)
#[doc(alias = "FMOD::FMOD_OggVorbis_SeekCallback(void *,long long,int)")]
// IDA 0x9fd5c: if word +98 has bit 1 clear (`[R0,#0x188]`, 0x9fd64..0x9fd6c)
// return -1; else `File::seek(this, a2, SBYTE4(a3))` (0x9fd74..0x9fd78 — the
// offset slot carries the sign-extended byte of the int64 high word).
pub fn stub_0x9fd5c(file: &mut FmodFile, a2: i32, pos_hi: i64, _a4: i32) -> i32 {
    if !file.seekable {
        return -1;
    }
    let narrowed = ((pos_hi >> 32) as i8) as i32;
    file.seek(a2, narrowed)
}

// 0x9fd80 — __ZN4FMOD14CodecOggVorbis16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::getDescriptionEx(void)")]
// IDA 0x9fd80: memset the 0x7C table (0x9fd9c), name "FMOD Ogg Vorbis Codec"
// (0x9fdac), version 65792 (0x9fdb4), format 2 (0x9fdbc), open/close/read/
// setPosition/getMemoryUsed slots (0x9fdc8..0x9fdf8), 14 (0x9fe00) and 1268
// (0x9fe08); returns the table (0x9fe0c).
pub fn stub_0x9fd80() -> &'static OggVorbisCodecDesc {
    LazyLock::force(&OGGVORBIS_CODEC_DESC)
}

// 0x9fe30 — __ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_
// type: unsigned int __fastcall(FMOD *this, unsigned int, unsigned int, FMOD::File *, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_ReadCallback(void *,unsigned long,unsigned long,void *)")]
// IDA 0x9fe30: `File::read(a4, this, a2, a3, &v8)` (0x9fe60); codes 0 and 22
// return the byte count (0x9fe64..0x9fe70), anything else returns -1 (0x9fe6c).
pub fn stub_0x9fe30(dst: &mut [u8], size: u32, count: u32, file: &mut FmodFile) -> u32 {
    let (code, n) = file.read(dst, size, count);
    if code == FMOD_OK || code == FMOD_EOF {
        n
    } else {
        u32::MAX
    }
}

// 0x9fe7c — _FMOD_OggVorbis_Free
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_FMOD_OggVorbis_Free")]
// IDA 0x9fe7c: when both the codec and the block are non-null, subtract the
// header size word `*(a2-4)` from usage at +1264 (0x9fe84..0x9fe98); then
// `MemPool::free(pool, a2, "fmod_codec_oggvorbis.cpp", 81)` (0x9febc).
pub fn stub_0x9fe7c(codec: Option<&OggVorbisCodec>, block: Vec<u8>) -> i32 {
    if let Some(codec) = codec {
        let mut usage = codec.mem_usage.lock();
        *usage = usage.saturating_sub(block.len() as u32);
    }
    {
        let mut pool = OGG_MEM_POOL.lock();
        pool.live_bytes = pool.live_bytes.saturating_sub(block.len());
    }
    drop(block);
    FMOD_OK
}

// 0x9fec8 — __ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// IDA 0x9fec8: rewind the file (0x9ff2c), sniff RIFF/WAVE wrapping
// (0x9ff54..0xa01ac) handled by the host chunk walker, then require the
// "OggS" capture (0xa0070..0xa0090); one-shot `_FMOD_vorbis_window_init`
// (0xa0228..0xa0230); file length from the WAV branch or `File::getLength`
// (0xa00b0..0xa00d4); `ov_open_callbacks` (0xa0128, -139 -> 44 else 25);
// `ov_info` + readVorbisComments (0xa0280..0xa0290); wave-format/length words
// (0xa029c..0xa02cc, 0xa02d0..0xa02e8 bias adjust); per-stream PCM totals when
// seekable (0xa02f8..0xa03e0, empty total -> 25) else 0x7FFF_FFFF (0xa0304);
// adopt `ov_raw_tell` as the data start when unset (0xa0308..0xa03fc).
pub fn stub_0x9fec8(codec: &OggVorbisCodec) -> i32 {
    let mut file_guard = codec.file.lock();
    let Some(file) = file_guard.as_mut() else { return 20 };
    *codec.state_word.lock() = 0;
    let mut vorbis = VorbisStream::default();
    let mut data_start: u32 = 0;
    let mut file_len: u32 = 0;
    let mut from_wav = false;

    let mut rc = file.seek(0, FMOD_SEEK_SET);
    if rc != FMOD_OK {
        return rc;
    }
    let mut magic = [0u8; 8];
    let (code, _) = file.read(&mut magic, 1, 8);
    if code != FMOD_OK && code != FMOD_EOF {
        return code;
    }
    if fmod_strncmp(&magic[..4], b"RIFF", 4) == 0 {
        let mut wave = [0u8; 4];
        let (code, _) = file.read(&mut wave, 1, 4);
        if code != FMOD_OK && code != FMOD_EOF {
            return code;
        }
        if fmod_strncmp(&wave, b"WAVE", 4) == 0 {
            match wav_parse_riff(file) {
                // IDA 0xa0194: parse failure frees the temp data buffer and
                // continues at LABEL_25 (temp frees are host no-ops).
                None => {}
                Some(parsed) => {
                    if parsed.fmt_tag != 0x6750 {
                        // IDA 0xa0334..0xa0388: not vorbis-in-wave; free temps,
                        // adopt nothing, report 25.
                        return 25;
                    }
                    data_start = parsed.data_off;
                    file_len = parsed.data_len;
                    *codec.stream_words.lock() = (parsed.fmt_word_50, parsed.fmt_word_51);
                    *codec.wav_word_78.lock() = parsed.chunk_78;
                    *codec.wav_extra.lock() = parsed.chunk_79;
                    from_wav = true;
                }
            }
        }
    }
    // IDA LABEL_5 (0xa0044).
    rc = file.seek(data_start as i32, FMOD_SEEK_SET);
    if rc != FMOD_OK {
        return rc;
    }
    let mut oggs = [0u8; 4];
    let (code, _) = file.read(&mut oggs, 1, 4);
    if code != FMOD_OK && code != FMOD_EOF {
        return code;
    }
    if fmod_strncmp(&oggs, b"OggS", 4) != 0 {
        return 25;
    }
    if !OGGVORBIS_INITIALIZED.swap(true, Ordering::SeqCst) {
        vorbis_window_init();
    }
    if !from_wav {
        file_len = file.len();
    }
    // IDA 0xa00fc: clear the vorbis area, then open on the file.
    vorbis.remaining = file.len().saturating_sub(data_start) as u64;
    vorbis.pcm_total = vorbis.remaining as i64;
    vorbis.open = true;
    vorbis.comments_live = true;
    drop(file_guard);

    *codec.data_start.lock() = data_start;
    *codec.file_len.lock() = file_len;
    let comments_rc = stub_0x9fa8c(codec);
    if comments_rc != FMOD_OK {
        return comments_rc;
    }
    // IDA 0xa029c..0xa02cc: wave-format + length words.
    let (channels, rate) = {
        let stream = codec.vorbis.lock();
        (stream.channels, stream.rate)
    };
    *codec.length_words.lock() = (2, rate, channels);
    let mut wave = OggWaveFormat::fresh();
    wave.words_260 = 0;
    wave.total_pcm = 0;
    if !from_wav && wave.start_bias != -1 {
        wave.start_bias -= data_start as i32;
    }
    if file_len == 0 {
        // IDA 0xa02f8: seekable files accumulate per-stream PCM totals.
        let seekable = codec.file.lock().as_ref().map(|f| f.seekable).unwrap_or(false);
        if seekable {
            let totals: Vec<i64> = codec.vorbis.lock().stream_totals.clone();
            let mut acc: i64 = 0;
            for t in &totals {
                acc += *t;
            }
            if acc == 0 {
                return 25;
            }
            wave.total_pcm = acc as u32;
        } else {
            wave.total_pcm = 0x7FFF_FFFF;
        }
    }
    if data_start == 0 {
        // IDA 0xa0308..0xa03fc: adopt ov_raw_tell when no start is known.
        data_start = codec.vorbis.lock().raw_pos;
        *codec.data_start.lock() = data_start;
    }
    *codec.wave_format.lock() = Some(wave);
    *codec.vorbis.lock() = vorbis;
    FMOD_OK
}

/// Host model of `FMOD::CodecWav::parseChunk` (IDA 0xa0194): walk the RIFF
/// chunks for the `fmt `/`data` pair used by the vorbis-in-wave path.
#[derive(Debug, Default)]
struct WavParsed {
    fmt_tag: u32,
    fmt_word_50: u32,
    fmt_word_51: u32,
    chunk_78: u32,
    chunk_79: Option<Vec<u8>>,
    data_off: u32,
    data_len: u32,
}

fn wav_parse_riff(file: &FmodFile) -> Option<WavParsed> {
    let data = &file.data;
    if data.len() < 12 {
        return None;
    }
    let mut parsed = WavParsed::default();
    let mut off = 12usize;
    let mut have_fmt = false;
    while off + 8 <= data.len() {
        let id = &data[off..off + 4];
        let len = u32::from_le_bytes([data[off + 4], data[off + 5], data[off + 6], data[off + 7]]) as usize;
        let body = off + 8;
        if id == b"fmt " {
            if body + 2 > data.len() {
                return None;
            }
            parsed.fmt_tag = u16::from_le_bytes([data[body], data[body + 1]]) as u32;
            if body + 6 <= data.len() {
                parsed.fmt_word_50 =
                    u16::from_le_bytes([data[body + 2], data[body + 3]]) as u32;
                parsed.fmt_word_51 =
                    u16::from_le_bytes([data[body + 4], data[body + 5]]) as u32;
            }
            have_fmt = true;
        } else if id == b"data" {
            parsed.data_off = body as u32;
            parsed.data_len = len as u32;
        }
        off = body + len;
    }
    if have_fmt && parsed.data_len > 0 {
        Some(parsed)
    } else {
        None
    }
}

// 0xa0448 — __ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// IDA 0xa0448: state -28 when non-null (0xa044c), tail-call openInternal.
// BUG(host): null state returns FMOD_OK instead of faulting (see 0x9fa34).
pub fn stub_0xa0448(state: Option<&CodecStateOgg>) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    stub_0x9fec8(&state.pub_codec)
}

// 0xa0454 — __ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv
// type: unsigned int __fastcall(FMOD *this, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_TellCallback(void *)")]
// IDA 0xa0454: `File::tell(this, &v3)` (0xa0464), return it (0xa0470).
pub fn stub_0xa0454(file: &FmodFile) -> u32 {
    file.tell()
}

// 0xa0474 — _FMOD_OggVorbis_ReAlloc
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "_FMOD_OggVorbis_ReAlloc")]
// IDA 0xa0474: with a non-null block and codec, subtract the header size word
// and the old usage from +1264 (0xa0488..0xa04a4); `MemPool::realloc` (0xa04d0,
// "fmod_codec_oggvorbis.cpp", 59); on success with a non-null codec add the
// new size to +1264 (0xa04d8..0xa04ec).
pub fn stub_0xa0474(
    codec: Option<&OggVorbisCodec>,
    block: Option<Vec<u8>>,
    new_size: usize,
) -> Option<Vec<u8>> {
    let old_len = block.as_ref().map(|b| b.len()).unwrap_or(0);
    if let Some(codec) = codec {
        let mut usage = codec.mem_usage.lock();
        *usage = usage.saturating_sub(old_len as u32);
    }
    {
        let mut pool = OGG_MEM_POOL.lock();
        pool.live_bytes = pool.live_bytes.saturating_sub(old_len);
    }
    let mut out = block.unwrap_or_default();
    out.resize(new_size, 0);
    {
        let mut pool = OGG_MEM_POOL.lock();
        pool.live_bytes = pool.live_bytes.saturating_add(out.len());
    }
    if let Some(codec) = codec {
        let mut usage = codec.mem_usage.lock();
        *usage = usage.saturating_add(new_size as u32);
    }
    Some(out)
}

// 0xa0500 — _FMOD_OggVorbis_Calloc
// type: int __fastcall(int, int, int)
#[doc(alias = "_FMOD_OggVorbis_Calloc")]
// IDA 0xa0500: `v3 = a2*a3` (0xa0510); `MemPool::calloc(pool, a2*a3,
// "fmod_codec_oggvorbis.cpp", 33, 0)` (0xa053c); on success with a non-null
// codec add v3 to +1264 (0xa0540..0xa0550). Zeroed on host by construction.
pub fn stub_0xa0500(codec: Option<&OggVorbisCodec>, count: usize, size: usize) -> Option<Vec<u8>> {
    let total = count.saturating_mul(size);
    let out = vec![0u8; total];
    {
        let mut pool = OGG_MEM_POOL.lock();
        pool.live_bytes = pool.live_bytes.saturating_add(out.len());
    }
    if let Some(codec) = codec {
        let mut usage = codec.mem_usage.lock();
        *usage = usage.saturating_add(total as u32);
    }
    Some(out)
}

// 0xa0564 — _FMOD_OggVorbis_Malloc
// type: int __fastcall(int, int)
#[doc(alias = "_FMOD_OggVorbis_Malloc")]
// IDA 0xa0564: `MemPool::alloc(pool, a2, "fmod_codec_oggvorbis.cpp", 20, 0, 0)`
// (0xa05a0); on success with a non-null codec add a2 to +1264
// (0xa05a4..0xa05b4).
pub fn stub_0xa0564(codec: Option<&OggVorbisCodec>, size: usize) -> Option<Vec<u8>> {
    let out = vec![0u8; size];
    {
        let mut pool = OGG_MEM_POOL.lock();
        pool.live_bytes = pool.live_bytes.saturating_add(out.len());
    }
    if let Some(codec) = codec {
        let mut usage = codec.mem_usage.lock();
        *usage = usage.saturating_add(size as u32);
    }
    Some(out)
}

// 0xa0614 — __GLOBAL__I_FMOD_OggVorbis_Malloc
// type: int()
// was: global constructor keyed to_FMOD_OggVorbis_Malloc
#[doc(alias = "global constructor keyed to_FMOD_OggVorbis_Malloc")]
// IDA 0xa0614: `__static_initialization_and_destruction_0(1, 0xFFFF)` — force
// the translation-unit statics (codec table, pool, init flag) live.
pub fn stub_0xa0614() {
    LazyLock::force(&OGGVORBIS_CODEC_DESC);
    LazyLock::force(&OGG_MEM_POOL);
    OGGVORBIS_INITIALIZED.store(false, Ordering::SeqCst);
}

// 0xa0620 — __ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi
// type: int __fastcall(FMOD::CodecPlaylist *this, const char *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getQuoteData(char const*,char *,int *)")]
// IDA 0xa0620: scan to the opening `"` (0xa062c..0xa0638); then copy to the
// closing `"` (0xa0648..0xa0670), NUL-terminate, store the length (0xa0678..).
// BUG: no opening quote within 512 bytes spins forever (0xa0640..0xa0644);
// preserved below.
pub fn stub_0xa0620(data: &[u8], out: &mut [u8], len: &mut usize) -> i32 {
    let mut pos = 0usize;
    loop {
        let c = *data.get(pos).unwrap_or(&0);
        pos += 1;
        if c == b'"' {
            break;
        }
        if pos == 512 {
            // BUG: original at 0xa0644 spins forever; preserved.
            loop {
                core::hint::spin_loop();
            }
        }
    }
    let mut count = 0usize;
    loop {
        let c = *data.get(pos + count).unwrap_or(&0);
        if c == b'"' {
            break;
        }
        if count < out.len() {
            out[count] = c;
        }
        count += 1;
        if count + pos > 510 {
            break;
        }
    }
    if count < out.len() {
        out[count] = 0;
    }
    *len = count;
    FMOD_OK
}

// 0xa0684 — __ZN4FMOD13CodecPlaylist13closeInternalEv
// type: int __fastcall(FMOD::CodecPlaylist *this)
#[doc(alias = "FMOD::CodecPlaylist::closeInternal(void)")]
// IDA 0xa0684: returns 0 (0xa0688); no state touched.
pub fn stub_0xa0684(_codec: &CodecPlaylist) -> i32 {
    FMOD_OK
}

// 0xa068c — __ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecPlaylist *)
#[doc(alias = "FMOD::CodecPlaylist::closeCallback(FMOD_CODEC_STATE *)")]
// IDA 0xa068c: state -28 when non-null (0xa0690), tail-call closeInternal.
// BUG(host): null state returns FMOD_OK instead of faulting (see 0x9fa34).
pub fn stub_0xa068c(state: Option<&CodecStatePlaylist>) -> i32 {
    let Some(state) = state else { return FMOD_OK };
    stub_0xa0684(&state.pub_codec)
}

// 0xa0698 — __ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// IDA 0xa0698: returns 0 (0xa069c); playlist data flows via metaData.
pub fn stub_0xa0698() -> i32 {
    FMOD_OK
}

// 0xa06a0 — __ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// IDA 0xa06a0: returns 0 (0xa06a4); playlists are not seekable by position.
pub fn stub_0xa06a0() -> i32 {
    FMOD_OK
}

// 0xa06a8 — __ZN4FMOD13CodecPlaylist9isNewLineEc
// type: bool __fastcall(FMOD::File **this, char)
#[doc(alias = "FMOD::CodecPlaylist::isNewLine(char)")]
// IDA 0xa06a8: `\n` (SXTB, 0xa06b4..0xa06c0) -> true; anything but `\r` ->
// false (0xa06c8..0xa06cc); on `\r` peek the next byte from the file at +264
// (0xa06d8..0xa06dc, `[R4,#0x108]`), back up one (0xa06ec), true unless the
// peeked byte is `\n` (0xa0700).
pub fn stub_0xa06a8(codec: &CodecPlaylist, c: u8) -> bool {
    let c = c as i8 as i32;
    if c == 10 {
        return true;
    }
    if c != 13 {
        return false;
    }
    let mut guard = codec.file.lock();
    let Some(file) = guard.as_mut() else { return true };
    let (code, next) = file.get_byte();
    if code == FMOD_OK {
        let _ = file.seek(-1, FMOD_SEEK_CUR);
        next != 10
    } else {
        true
    }
}

// 0xa0704 — __ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi
// type: int __fastcall(FMOD::File **this, int *)
#[doc(alias = "FMOD::CodecPlaylist::skipWhiteSpace(int *)")]
// IDA 0xa0704: consume bytes while they are tab/space/LF/CR (0xa0724..0xa0750);
// on the first other byte back up one, report consumed-1 through a2 when
// non-null, and return the backup seek code (0xa0760..0xa077c); a short read
// returns its code (0xa072c..0xa0770).
pub fn stub_0xa0704(codec: &CodecPlaylist, skipped: Option<&mut usize>) -> i32 {
    let mut count = 0usize;
    loop {
        let (code, b) = match codec.file.lock().as_mut() {
            Some(file) => file.get_byte(),
            None => return 20,
        };
        if code != FMOD_OK {
            return code;
        }
        count += 1;
        if b != 9 && b != 32 && b != 10 && b != 13 {
            let mut guard = codec.file.lock();
            let file = guard.as_mut().expect("file present above");
            let rc = file.seek(-1, FMOD_SEEK_CUR);
            if rc == FMOD_OK {
                if let Some(skipped) = skipped {
                    *skipped = count - 1;
                }
            }
            return rc;
        }
    }
}

// 0xa0784 — __ZN4FMOD13CodecPlaylist8readLineEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::readLine(char *,int,int *)")]
// IDA 0xa0784: skipWhiteSpace first (0xa07ac, failure returns 0xa07b0..0xa07c0);
// accumulate non-newline bytes up to a3 (0xa07d4..0xa07f0); on isNewLine
// store the length through a4 when non-null, NUL-terminate, return the read
// code (0xa0800..0xa081c).
pub fn stub_0xa0784(
    codec: &CodecPlaylist,
    buf: &mut [u8],
    cap: usize,
    len: Option<&mut usize>,
) -> i32 {
    let rc = stub_0xa0704(codec, None);
    if rc != FMOD_OK {
        return rc;
    }
    let mut count = 0usize;
    loop {
        let (code, b) = match codec.file.lock().as_mut() {
            Some(file) => file.get_byte(),
            None => return 20,
        };
        if code != FMOD_OK {
            return code;
        }
        if b != 10 && b != 13 && count < cap && count < buf.len() {
            buf[count] = b;
            count += 1;
        }
        if stub_0xa06a8(codec, b) {
            if let Some(len) = len {
                *len = count;
            }
            if count < buf.len() {
                buf[count] = 0;
            }
            return code;
        }
    }
}

// 0xa0820 — __ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::skipSimpleComments(void)")]
// IDA 0xa0820: skipWhiteSpace (0xa0838); on `[`/`#` discard to end of line and
// loop (0xa0864..0xa0890, LABEL_2 at 0xa0884); else back up one and return
// that seek code (0xa08b0); short reads return their code (0xa0898/0xa08a0).
pub fn stub_0xa0820(codec: &CodecPlaylist) -> i32 {
    let mut skipped = 0usize;
    loop {
        let rc = stub_0xa0704(codec, Some(&mut skipped));
        if rc != FMOD_OK {
            return rc;
        }
        let (code, b) = match codec.file.lock().as_mut() {
            Some(file) => file.get_byte(),
            None => return 20,
        };
        if code != FMOD_OK {
            return code;
        }
        if b == b'[' || b == b'#' {
            loop {
                let (code, b) = match codec.file.lock().as_mut() {
                    Some(file) => file.get_byte(),
                    None => return 20,
                };
                if code != FMOD_OK {
                    return code;
                }
                if stub_0xa06a8(codec, b) {
                    break;
                }
            }
        } else {
            let mut guard = codec.file.lock();
            return guard.as_mut().expect("file present above").seek(-1, FMOD_SEEK_CUR);
        }
    }
}

// 0xa08b8 — __ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::getPLSToken(char *,int,int *)")]
// IDA 0xa08b8: skipWhiteSpace (0xa08e8); accumulate to newline, `=`, or `]`
// (0xa0900..0xa0958) — `=` re-syncs past the value (0xa09ec..0xa0a3c), `]`
// checks for a following `[` section header (0xa0968..0xa09c0); on newline
// NUL-terminate and store the length (LABEL_18, 0xa09cc..0xa09d4).
pub fn stub_0xa08b8(
    codec: &CodecPlaylist,
    buf: &mut [u8],
    cap: usize,
    len: Option<&mut usize>,
) -> i32 {
    let mut skipped = 0usize;
    let rc = stub_0xa0704(codec, Some(&mut skipped));
    if rc != FMOD_OK {
        return rc;
    }
    let mut count = 0usize;
    loop {
        let (code, b) = match codec.file.lock().as_mut() {
            Some(file) => file.get_byte(),
            None => return 20,
        };
        if code != FMOD_OK {
            return code;
        }
        if b != 10 && b != 13 && count < cap && count < buf.len() {
            buf[count] = b;
            count += 1;
        }
        if b == b'=' {
            // IDA 0xa09ec..0xa0a3c: seek back over `skipped + count`, read the
            // value's first byte, seek forward again, then newline-check it.
            let back = -((skipped + count) as i32) - 1;
            let rc = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .seek(back, FMOD_SEEK_CUR);
            if rc != FMOD_OK {
                return rc;
            }
            let (code, vb) = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .get_byte();
            if code != FMOD_OK {
                return code;
            }
            let rc = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .seek((count + skipped) as i32, FMOD_SEEK_CUR);
            if rc != FMOD_OK {
                return rc;
            }
            if stub_0xa06a8(codec, vb) {
                count = count.saturating_sub(1);
                break;
            }
            continue;
        }
        if b == b']' {
            // IDA 0xa0968..0xa09c0: possible `[section]` header follows.
            let rc = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .seek(-(count as i32), FMOD_SEEK_CUR);
            if rc != FMOD_OK {
                return rc;
            }
            let (code, hb) = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .get_byte();
            if code != FMOD_OK {
                return code;
            }
            let rc = codec
                .file
                .lock()
                .as_mut()
                .expect("file present above")
                .seek(count as i32 - 1, FMOD_SEEK_CUR);
            if rc != FMOD_OK {
                return rc;
            }
            if hb == b'[' {
                let rc = codec
                    .file
                    .lock()
                    .as_mut()
                    .expect("file present above")
                    .seek(2, FMOD_SEEK_CUR);
                if rc != FMOD_OK {
                    return rc;
                }
                break;
            }
        }
        if stub_0xa06a8(codec, b) {
            break;
        }
    }
    if let Some(len) = len {
        *len = count;
    }
    if count < buf.len() {
        buf[count] = 0;
    }
    FMOD_OK
}

// 0xa0a54 — __ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_
// type: int __fastcall(FMOD::File **this, char *, int *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getNextXMLTag(char *,int *,char *,int *)")]
// IDA 0xa0a54: skipWhiteSpace (0xa0a7c); scan to `<`, capture the tag to `>`
// (0xa0a9c..0xa0ae8, `*a3 = len-1` at 0xa0af4); capture text to the next `<`
// with capacity `*a5` (0xa0b0c..0xa0b40, `*a5 = len-1` at 0xa0b5c..0xa0b60);
// a `</` closer is skipped to `>` (0xa0b7c..0xa0bb0), anything else backs up
// two (0xa0b8c).
pub fn stub_0xa0a54(
    codec: &CodecPlaylist,
    tag: &mut [u8],
    tag_len: &mut usize,
    text: &mut [u8],
    text_len: Option<&mut usize>,
) -> i32 {
    let rc = stub_0xa0704(codec, None);
    if rc != FMOD_OK {
        return rc;
    }
    loop {
        let (code, b) = match codec.file.lock().as_mut() {
            Some(file) => file.get_byte(),
            None => return 20,
        };
        if code != FMOD_OK {
            return code;
        }
        if b != b'<' {
            continue;
        }
        let mut tcount = 0usize;
        loop {
            let (code, b) = match codec.file.lock().as_mut() {
                Some(file) => file.get_byte(),
                None => return 20,
            };
            if code != FMOD_OK {
                return code;
            }
            if tcount < *tag_len && tcount < tag.len() {
                tag[tcount] = b;
                tcount += 1;
            }
            if b == b'>' {
                *tag_len = tcount.saturating_sub(1);
                let rc = stub_0xa0704(codec, None);
                if rc != FMOD_OK {
                    return rc;
                }
                let text_cap = text_len.as_deref().copied().unwrap_or(0);
                let mut xcount = 0usize;
                loop {
                    let (code, b) = match codec.file.lock().as_mut() {
                        Some(file) => file.get_byte(),
                        None => return 20,
                    };
                    if code != FMOD_OK {
                        return code;
                    }
                    if xcount < text_cap && xcount < text.len() {
                        text[xcount] = b;
                        xcount += 1;
                    }
                    if b == b'<' {
                        if let Some(text_len) = text_len {
                            *text_len = xcount.saturating_sub(1);
                        }
                        let (code, b) = codec
                            .file
                            .lock()
                            .as_mut()
                            .expect("file present above")
                            .get_byte();
                        if code != FMOD_OK {
                            return code;
                        }
                        if b == b'/' {
                            loop {
                                let (code, b) = codec
                                    .file
                                    .lock()
                                    .as_mut()
                                    .expect("file present above")
                                    .get_byte();
                                if code != FMOD_OK || b == b'>' {
                                    return code;
                                }
                            }
                        } else {
                            return codec
                                .file
                                .lock()
                                .as_mut()
                                .expect("file present above")
                                .seek(-2, FMOD_SEEK_CUR);
                        }
                    }
                }
            }
        }
    }
}

// 0xa0bb8 — __ZN4FMOD13CodecPlaylist10readSimpleEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readSimple(void)")]
// IDA 0xa0bb8: rewind (0xa0bdc); while skipSimpleComments finds content
// (0xa0c2c), readLine into a 512 buffer (0xa0c40) and file
// `metaData(this,8,"FILE",line,len+1,3,0)` per line (0xa0c1c); a line-read
// failure returns 0 early (0xa0c50), else the rewind code (0xa0bec).
pub fn stub_0xa0bb8(codec: &CodecPlaylist) -> i32 {
    let rc = match codec.file.lock().as_mut() {
        Some(file) => file.seek(0, FMOD_SEEK_SET),
        None => return 20,
    };
    if rc != FMOD_OK {
        return rc;
    }
    loop {
        if stub_0xa0820(codec) != FMOD_OK {
            break;
        }
        let mut line = [0u8; 512];
        let mut len = 0usize;
        if stub_0xa0784(codec, &mut line, 512, Some(&mut len)) != FMOD_OK {
            return FMOD_OK;
        }
        playlist_meta(codec, "FILE", &line[..len.min(512)], 0);
    }
    rc
}

// 0xa0c58 — __ZN4FMOD13CodecPlaylist7readPLSEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readPLS(void)")]
// IDA 0xa0c58: rewind (0xa0c78); require the `[playlist]` header (0xa0cac,
// else 25 at 0xa0cb8); per entry file `FileN` (metaData "FILE"), `TitleN`
// ("TITLE") and `LengthN` ("LENGTH", atoi value, type 1) rows
// (0xa0d44..0xa0e58); `NumberOfEntries`/`Version` values are consumed and
// skipped (0xa0ea8); token failures propagate, clean exhaustion returns 0
// (0xa0dd0). The `&var4[-129]` store at 0xa0e24 lands in the token buffer and
// NUL-terminates the Length digits for atoi — modeled directly.
pub fn stub_0xa0c58(codec: &CodecPlaylist) -> i32 {
    let rc = match codec.file.lock().as_mut() {
        Some(file) => file.seek(0, FMOD_SEEK_SET),
        None => return 20,
    };
    if rc != FMOD_OK {
        return rc;
    }
    let mut tok = [0u8; 512];
    if stub_0xa08b8(codec, &mut tok, 512, None) != FMOD_OK
        || fmod_strnicmp(&tok, b"[playlist]", 10) != 0
    {
        return 25;
    }
    loop {
        // IDA 0xa0d44: fetch the entry token; exhaustion breaks (0xa0d48).
        if stub_0xa08b8(codec, &mut tok, 512, None) != FMOD_OK {
            break;
        }
        // IDA 0xa0d64..0xa0dcc: while the token opens a `FileN` row, fetch
        // the value (0xa0d78, failure propagates), file it, then fetch the
        // next token (0xa0dc8, failure ends cleanly with 0); the surviving
        // token falls through to the Title/Length checks below.
        while fmod_strnicmp(&tok, b"File", 4) == 0 {
            let mut tlen = 0usize;
            let rc = stub_0xa08b8(codec, &mut tok, 512, Some(&mut tlen));
            if rc != FMOD_OK {
                return rc;
            }
            playlist_meta(codec, "FILE", &tok[..tlen.min(512)], 0);
            if stub_0xa08b8(codec, &mut tok, 512, None) != FMOD_OK {
                return FMOD_OK;
            }
        }
        if fmod_strnicmp(&tok, &b"Title"[..], 5) == 0 {
            let mut tlen = 0usize;
            let rc = stub_0xa08b8(codec, &mut tok, 512, Some(&mut tlen));
            if rc != FMOD_OK {
                return rc;
            }
            playlist_meta(codec, "TITLE", &tok[..tlen.min(512)], 0);
        } else if fmod_strnicmp(&tok, &b"Length"[..], 6) == 0 {
            let mut tlen = 0usize;
            let rc = stub_0xa08b8(codec, &mut tok, 512, Some(&mut tlen));
            if rc != FMOD_OK {
                return rc;
            }
            // IDA 0xa0e24: out-of-bounds `&var4[-129]` store NUL-terminates
            // the digit run inside the token buffer before atoi.
            if tlen < tok.len() {
                tok[tlen] = 0;
            }
            let value = fmod_atoi(&tok[..tlen.min(512)]);
            playlist_meta(codec, "LENGTH", &value.to_le_bytes(), 0);
        } else if fmod_strnicmp(&tok, &b"NumberOfEntries"[..], 15) == 0
            || fmod_strnicmp(&tok, &b"Version"[..], 7) == 0
        {
            // IDA 0xa0ea8..0xa0eb0: consume the value; failure propagates.
            let rc = stub_0xa08b8(codec, &mut tok, 512, None);
            if rc != FMOD_OK {
                return rc;
            }
        }
    }
    FMOD_OK
}
