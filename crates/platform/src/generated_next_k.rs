//! platform — generated_next_k — 100 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 100 after 0x71144 not yet in crates/platform/src
//! Batch: 100 stubs | range 0x71178..0x773f0 | rbx_core::SharedPtr not boost
//! Filter: iOS|ViewController|RobloxView|Platform 1276 total, 1276/1276 done, 0 remaining — global gap filler

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x71178 — __ZN4FMOD7Channel7setMuteEb
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setMute(bool)")]
pub fn stub_71178(handle: u32, muted: bool) -> i32 {
    // IDA 0x71178 `Channel::setMute`: validates, forwards (0x7118c..
    // 0x711a8).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.with_row(handle, |row| row.muted = muted) {
        Some(()) => 0,
        None => FmodChannels::INVALID,
    }
}

// 0x711ac — __ZN4FMOD7Channel12getFrequencyEPf
// type: int __fastcall(FMOD::Channel *this, float *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getFrequency(float *)")]
pub fn stub_711ac(handle: u32) -> (i32, f32) {
    // IDA 0x711ac `Channel::getFrequency`: validates, zeroes the rate on
    // failure, forwards (0x711c0..0x711dc).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.get(handle, |row| row.frequency) {
        Some(frequency) => (0, frequency),
        None => (FmodChannels::INVALID, 0.0),
    }
}

// 0x711f0 — __ZN4FMOD7Channel12setFrequencyEf
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setFrequency(float)")]
pub fn stub_711f0(handle: u32, frequency: f32) -> i32 {
    // IDA 0x711f0 `Channel::setFrequency`: validates, forwards (0x71204..
    // 0x71220).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.with_row(handle, |row| row.frequency = frequency) {
        Some(()) => 0,
        None => FmodChannels::INVALID,
    }
}

// 0x71224 — __ZN4FMOD7Channel9setVolumeEf
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setVolume(float)")]
pub fn stub_71224(handle: u32, volume: f32) -> i32 {
    // IDA 0x71224 `Channel::setVolume`: validates, forwards with the ramp
    // flag clear (0x7123c..0x7125c).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.with_row(handle, |row| row.volume = volume) {
        Some(()) => 0,
        None => FmodChannels::INVALID,
    }
}

// 0x71260 — __ZN4FMOD7Channel9getPausedEPb
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getPaused(bool *)")]
pub fn stub_71260(handle: u32) -> (i32, bool) {
    // IDA 0x71260 `Channel::getPaused`: validates, zeroes the flag on
    // failure, forwards (0x71274..0x71290).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.get(handle, |row| row.paused) {
        Some(paused) => (0, paused),
        None => (FmodChannels::INVALID, false),
    }
}

// 0x712a4 — __ZN4FMOD7Channel9setPausedEb
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setPaused(bool)")]
pub fn stub_712a4(handle: u32, paused: bool) -> i32 {
    // IDA 0x712a4 `Channel::setPaused`: validates, forwards (0x712b8..
    // 0x712d4).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.with_row(handle, |row| row.paused = paused) {
        Some(()) => 0,
        None => FmodChannels::INVALID,
    }
}

// 0x712d8 — __ZN4FMOD7Channel4stopEv
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::stop(void)")]
pub fn stub_712d8(handle: u32) -> i32 {
    // IDA 0x712d8 `Channel::stop`: validates, forwards (0x712e8..0x71300).
    use crate::generated_next_j::{FMOD_CHANNELS, FmodChannels};
    match FMOD_CHANNELS.with_row(handle, |row| {
        row.playing = false;
        row.stopped = true;
    }) {
        Some(()) => 0,
        None => FmodChannels::INVALID,
    }
}

/// Minimal `FMOD::ChannelEmulated` counterpart (IDA 0x71304..0x7183c): the
/// virtual latch, DSP head id, speaker tables plus the lifecycle flags.
#[derive(Debug, Default)]
pub struct EmulatedChannel {
    virtual_on: std::sync::atomic::AtomicBool,
    dsp_head: parking_lot::Mutex<u32>,
    speaker_levels: parking_lot::Mutex<Vec<f32>>,
    speaker_mix: parking_lot::Mutex<[f32; 8]>,
    updates: std::sync::atomic::AtomicU32,
    allocated: std::sync::atomic::AtomicBool,
    dsp_unit: std::sync::atomic::AtomicBool,
    stopped: std::sync::atomic::AtomicBool,
}
impl EmulatedChannel {
    /// `ChannelEmulated::ChannelEmulated` (IDA 0x71698): runs the real
    /// ctor plus the DSPI base, zeroes the unit id (0x716a4..0x716d4).
    pub fn construct(&self) {
        self.virtual_on.store(true, std::sync::atomic::Ordering::SeqCst);
        self.allocated.store(false, std::sync::atomic::Ordering::SeqCst);
        self.dsp_unit.store(false, std::sync::atomic::Ordering::SeqCst);
        self.stopped.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ChannelEmulated::init` (IDA 0x715e8): runs the real init, then
    /// creates the head DSP unit unless software is off (0x71600..0x7168c).
    pub fn init(&self, software_off: bool) -> i32 {
        REAL_CHANNEL.init();
        if !software_off {
            self.dsp_unit.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    /// `ChannelEmulated::alloc` (IDA 0x71580): runs the real alloc, then
    /// wires the head unit into the graph (0x71594..0x715e0).
    pub fn alloc(&self) -> i32 {
        REAL_CHANNEL.alloc();
        self.allocated.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelEmulated::update` (IDA 0x71344): runs the real update,
    /// then the virtual-voice walk (0x71358..tail).
    pub fn update(&self) -> i32 {
        REAL_CHANNEL.update();
        self.updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn update_count(&self) -> u32 {
        self.updates.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelEmulated::close` (IDA 0x71540): runs the real close, then
    /// releases the head unit (0x71550..0x71574).
    pub fn close(&self) -> i32 {
        REAL_CHANNEL.close();
        self.dsp_unit.store(false, std::sync::atomic::Ordering::SeqCst);
        self.allocated.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelEmulated::stop` (IDA 0x716e8): stops the voice, unwires
    /// the head unit plus the reverb link (0x716f4..tail).
    pub fn stop(&self) -> i32 {
        REAL_CHANNEL.stop();
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelEmulated::~ChannelEmulated` D1 (IDA 0x7183c): vtable reset
    /// only; D0 above also deletes.
    pub fn destroy(&self) {
        self.construct();
        self.virtual_on.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
static EMU_CHANNEL: std::sync::LazyLock<EmulatedChannel> =
    std::sync::LazyLock::new(EmulatedChannel::default);
/// Reverb channel properties behind `get/setReverbProperties` (IDA
/// 0x72528/0x725a0): the decay/wet pair the lookups resolve.
#[derive(Debug, Clone, Default)]
pub struct ReverbProps {
    pub decay_ms: f32,
    pub wet: f32,
}
/// Minimal `FMOD::ChannelReal` counterpart (IDA 0x71854..0x719c8): the
/// flag word, ref count plus the voice params.
#[derive(Debug)]
pub struct RealChannel {
    flags: std::sync::atomic::AtomicU32,
    refs: std::sync::atomic::AtomicU32,
    volume: parking_lot::Mutex<f32>,
    frequency: parking_lot::Mutex<f32>,
    playing: std::sync::atomic::AtomicBool,
    updates: std::sync::atomic::AtomicU32,
    pan: parking_lot::Mutex<[f32; 2]>,
    speaker_mix: parking_lot::Mutex<[f32; 8]>,
    speaker_levels: parking_lot::Mutex<Vec<f32>>,
    position_ms: std::sync::atomic::AtomicU32,
    position_unit: std::sync::atomic::AtomicU32,
    loop_start: std::sync::atomic::AtomicU32,
    loop_len: std::sync::atomic::AtomicU32,
    loop_count: std::sync::atomic::AtomicI32,
    lowpass: parking_lot::Mutex<f32>,
    occlusion: parking_lot::Mutex<[f32; 2]>,
    mode: std::sync::atomic::AtomicU32,
    reverb: parking_lot::Mutex<ReverbProps>,
    speaker_gain: parking_lot::Mutex<f32>,
}
impl Default for RealChannel {
    /// `ChannelReal::ChannelReal` (IDA 0x71854): zeroes the links, latches
    /// the idle flag plus the default freq/volume (0x71860..0x7188c).
    fn default() -> Self {
        Self {
            flags: std::sync::atomic::AtomicU32::new(0),
            refs: std::sync::atomic::AtomicU32::new(0),
            volume: parking_lot::Mutex::new(100.0),
            frequency: parking_lot::Mutex::new(44100.0),
            playing: std::sync::atomic::AtomicBool::new(false),
            updates: std::sync::atomic::AtomicU32::new(0),
            pan: parking_lot::Mutex::new([0.0; 2]),
            speaker_mix: parking_lot::Mutex::new([0.0; 8]),
            speaker_levels: parking_lot::Mutex::new(Vec::new()),
            position_ms: std::sync::atomic::AtomicU32::new(0),
            position_unit: std::sync::atomic::AtomicU32::new(0),
            loop_start: std::sync::atomic::AtomicU32::new(0),
            loop_len: std::sync::atomic::AtomicU32::new(0),
            loop_count: std::sync::atomic::AtomicI32::new(0),
            lowpass: parking_lot::Mutex::new(1.0),
            occlusion: parking_lot::Mutex::new([0.0; 2]),
            mode: std::sync::atomic::AtomicU32::new(0),
            reverb: parking_lot::Mutex::new(ReverbProps::default()),
            speaker_gain: parking_lot::Mutex::new(1.0),
        }
    }
}
impl RealChannel {
    /// `ChannelReal::init` (IDA 0x718a0): zeroes the links and latches the
    /// system/output/voice ids (0x718b0..0x718d0).
    pub fn init(&self) -> i32 {
        self.flags.store(0, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::alloc` (IDA 0x718e8/0x7190c): bumps the sound ref
    /// and clears the start latch (0x718f0..0x71928).
    pub fn alloc(&self) -> i32 {
        self.refs.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::close` (IDA 0x718dc): dispatches through the vtable
    /// (sole call).
    pub fn close(&self) -> i32 {
        self.playing.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::update` (IDA 0x71938) plus `updateStream` (0x71940):
    /// poll the voice; both return 0 here.
    pub fn update(&self) -> i32 {
        self.updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::start` (IDA 0x71948): returns 0.
    pub fn start(&self) -> i32 {
        self.playing.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_playing(&self) -> bool {
        self.playing.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelReal::stop` (IDA 0x71950): drops the sound ref and marks
    /// the stopped bits (0x71958..0x71970).
    pub fn stop(&self) -> i32 {
        self.refs.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        self.flags.fetch_or(0x80, std::sync::atomic::Ordering::SeqCst);
        self.playing.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::setPaused` (IDA 0x7197c): toggles the 0x20 flag bit
    /// (0x71980..0x71990).
    pub fn set_paused(&self, paused: bool) -> i32 {
        const PAUSED: u32 = 0x20;
        if paused {
            self.flags.fetch_or(PAUSED, std::sync::atomic::Ordering::SeqCst);
        } else {
            self.flags.fetch_and(!PAUSED, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    /// `ChannelReal::getPaused` (IDA 0x719a0): 37 without an out-param,
    /// else the 0x20 flag bit (0x719a4..0x719bc).
    pub fn paused(&self) -> bool {
        self.flags.load(std::sync::atomic::Ordering::SeqCst) & 0x20 != 0
    }
    /// `ChannelReal::setPan` (IDA 0x719d0): returns 0 (0x719d4).
    pub fn set_pan(&self, left: f32, right: f32) -> i32 {
        *self.pan.lock() = [left, right];
        0
    }
    /// `ChannelReal::setSpeakerMix` (IDA 0x719e0): routes the eight gains
    /// into the voice matrix (0x719f8..tail).
    pub fn set_speaker_mix(&self, mix: [f32; 8]) -> i32 {
        *self.speaker_mix.lock() = mix;
        0
    }
    /// `ChannelReal::setPosition` (IDA 0x71e34): units other than 0..2
    /// and 4 return 25; else latches the position (0x71e48..0x71e94).
    pub fn set_position(&self, pos: u32, unit: u32) -> i32 {
        if unit != 4 && unit > 2 {
            return 25;
        }
        self.position_ms.store(pos, std::sync::atomic::Ordering::SeqCst);
        self.position_unit.store(unit, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::getPosition` (IDA 0x72008): 37 without a voice or an
    /// out-param, else the position in the unit (0x72010..tail).
    pub fn position(&self) -> (i32, u32) {
        (
            0,
            self.position_ms.load(std::sync::atomic::Ordering::SeqCst),
        )
    }
    /// `ChannelReal::setLoopPoints` (IDA 0x722f0): 37 past the sound end,
    /// else latches start plus length (0x722f8..0x7231c).
    pub fn set_loop_points(&self, start: u32, len: u32) -> i32 {
        if len == 0 {
            return 37;
        }
        self.loop_start.store(start, std::sync::atomic::Ordering::SeqCst);
        self.loop_len.store(len, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::setLoopCount` (IDA 0x72328): latches the count
    /// (0x72328..0x72330).
    pub fn set_loop_count(&self, count: i32) -> i32 {
        self.loop_count.store(count, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelReal::getLoopCount` (IDA 0x72334): 37 without an
    /// out-param, else the count (0x72338..0x72348).
    pub fn loop_count(&self) -> i32 {
        self.loop_count.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelReal::setLowPassGain` (IDA 0x7234c): returns 0 (0x72350).
    pub fn set_lowpass(&self, gain: f32) -> i32 {
        *self.lowpass.lock() = gain;
        0
    }
    /// `ChannelReal::set3DOcclusion` (IDA 0x72364): forwards into the
    /// voice with the group value, 0 without a group (0x7236c..0x72380).
    pub fn set_occlusion(&self, direct: f32, reverb: f32) -> i32 {
        *self.occlusion.lock() = [direct, reverb];
        0
    }
    /// `ChannelReal::isPlaying` (IDA 0x72388): 37 without an out-param,
    /// else the 0x50 flag bits (0x7238c..0x723a8).
    pub fn real_playing(&self) -> bool {
        self.playing.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelReal::getSpectrum` (IDA 0x723c4) and `getWaveData`
    /// (0x723cc): 51 on a real voice (0x723c8/0x723d0).
    pub fn spectrum_unsupported(&self) -> i32 {
        51
    }
    /// `ChannelReal::getDSPHead` (IDA 0x723d4): zeroes the head and
    /// returns 51 on a real voice (0x723dc..0x723e0).
    pub fn dsp_head_unsupported(&self) -> (i32, u32) {
        (51, 0)
    }
    /// `ChannelReal::setMode` (IDA 0x723e4): merges the loop/open bits
    /// plus the 3D bits into the mode word (0x723e8..tail).
    pub fn set_mode(&self, mode: u32) -> i32 {
        self.mode.store(mode & 0x4c0007, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn mode(&self) -> u32 {
        self.mode.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelReal::getReverbProperties` (IDA 0x72528): 37 without an
    /// out-param or a voice, else the channel properties (0x7253c..0x72584).
    pub fn reverb(&self) -> ReverbProps {
        self.reverb.lock().clone()
    }
    /// `ChannelReal::setReverbProperties` (IDA 0x725a0): 37 without
    /// properties, else latches them per instance flag (0x725b8..tail).
    pub fn set_reverb(&self, props: ReverbProps) -> i32 {
        *self.reverb.lock() = props;
        0
    }
    /// `ChannelReal::updateSpeakerLevels` (IDA 0x726d8): rebuilds the
    /// level matrix at the gain (0x726e4..tail).
    pub fn update_speaker_levels(&self, gain: f32) -> i32 {
        *self.speaker_gain.lock() = gain;
        0
    }
    /// `ChannelReal::setSpeakerLevels` (IDA 0x72910): allocs the pool on
    /// demand (44 on failure), clamps plus stores the levels (0x72920..
    /// tail).
    pub fn set_speaker_levels(&self, levels: Vec<f32>) -> i32 {
        *self.speaker_levels.lock() = levels
            .into_iter()
            .map(|level| level.clamp(0.0, 1.0))
            .collect();
        0
    }
    /// `ChannelReal::~ChannelReal` D1 (IDA 0x72a28): vtable reset only;
    /// D0 above also deletes.
    pub fn destroy(&self) {
        self.init();
    }
}
/// Minimal `FMOD::ChannelRealManual3D` counterpart (IDA 0x72a40..0x72a88):
/// the manual flag plus the last computed stamp.
#[derive(Debug, Default)]
pub struct Manual3D {
    enabled: std::sync::atomic::AtomicBool,
    computed: std::sync::atomic::AtomicU32,
}
impl Manual3D {
    /// `ChannelRealManual3D::ChannelRealManual3D` (IDA 0x72a58): runs the
    /// real ctor and clears the manual latch (0x72a64..0x72a7c).
    pub fn construct(&self) {
        REAL_CHANNEL.init();
        self.enabled.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ChannelRealManual3D::alloc` (IDA 0x72a40): clears the manual latch
    /// then runs the real alloc (0x72a4c..0x72a54).
    pub fn alloc(&self) -> i32 {
        REAL_CHANNEL.alloc()
    }
    /// `ChannelRealManual3D::set2DFreqVolumePanFor3D` (IDA 0x72a88):
    /// derives the 2D params from the 3D position (0x72a88..tail).
    pub fn compute_3d(&self) -> i32 {
        self.computed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn compute_count(&self) -> u32 {
        self.computed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static MANUAL_3D: std::sync::LazyLock<Manual3D> = std::sync::LazyLock::new(Manual3D::default);
static REAL_CHANNEL: std::sync::LazyLock<RealChannel> =
    std::sync::LazyLock::new(RealChannel::default);
// 0x71304 — __ZN4FMOD15ChannelEmulated9isVirtualEPb
// type: int __fastcall(FMOD::ChannelEmulated *this, bool *, int, bool)
#[doc(alias = "FMOD::ChannelEmulated::isVirtual(bool *)")]
pub fn stub_71304(with_out: bool) -> (i32, bool) {
    // IDA 0x71304 `ChannelEmulated::isVirtual`: 37 without an out-param,
    // else true (0x71308..0x71318).
    if with_out {
        (0, true)
    } else {
        (37, false)
    }
}

// 0x7131c — __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int *)
#[doc(alias = "FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7131c(with_out: bool) -> (i32, u32) {
    // IDA 0x7131c `ChannelEmulated::getDSPHead`: 37 without an out-param,
    // else the head id (0x71320..0x71330).
    if with_out {
        (0, *EMU_CHANNEL.dsp_head.lock())
    } else {
        (37, 0)
    }
}

// 0x71334 — __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelEmulated *this, int, float *, int)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")]
pub fn stub_71334(levels: Vec<f32>) -> i32 {
    // IDA 0x71334 `ChannelEmulated::setSpeakerLevels`: returns 0
    // (0x71338).
    *EMU_CHANNEL.speaker_levels.lock() = levels;
    0
}

// 0x7133c — __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelEmulated *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_7133c(mix: [f32; 8]) -> i32 {
    // IDA 0x7133c `ChannelEmulated::setSpeakerMix`: returns 0 (0x71340).
    *EMU_CHANNEL.speaker_mix.lock() = mix;
    0
}

// 0x71344 — __ZN4FMOD15ChannelEmulated6updateEi
// type: int __fastcall(FMOD::ChannelEmulated *this, int)
#[doc(alias = "FMOD::ChannelEmulated::update(int)")]
pub fn stub_71344() -> i32 {
    // IDA 0x71344 `ChannelEmulated::update`: runs the real update, then
    // the virtual-voice walk (0x71358..tail).
    EMU_CHANNEL.update()
}

// 0x71540 — __ZN4FMOD15ChannelEmulated5closeEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::close(void)")]
pub fn stub_71540() -> i32 {
    // IDA 0x71540 `ChannelEmulated::close`: runs the real close, then
    // releases the head unit (0x71550..0x71574).
    EMU_CHANNEL.close()
}

// 0x71580 — __ZN4FMOD15ChannelEmulated5allocEv
// type: int __fastcall(FMOD::DSPI **this)
#[doc(alias = "FMOD::ChannelEmulated::alloc(void)")]
pub fn stub_71580() -> i32 {
    // IDA 0x71580 `ChannelEmulated::alloc`: runs the real alloc, then
    // wires the head unit into the graph (0x71594..0x715e0).
    EMU_CHANNEL.alloc()
}

// 0x715e8 — __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelEmulated *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_715e8(software_off: bool) -> i32 {
    // IDA 0x715e8 `ChannelEmulated::init`: runs the real init, then
    // creates the head DSP unit unless software is off (0x71600..0x7168c).
    EMU_CHANNEL.init(software_off)
}

// 0x71698 — __ZN4FMOD15ChannelEmulatedC2Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_71698() -> i32 {
    // IDA 0x71698 `ChannelEmulated::ChannelEmulated`: runs the real ctor
    // plus the DSPI base, zeroes the unit id (0x716a4..0x716d4).
    EMU_CHANNEL.construct();
    0
}

// 0x716e4 — __ZN4FMOD15ChannelEmulatedC1Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_716e4() -> i32 {
    // IDA 0x716e4 `ChannelEmulated::ChannelEmulated` thunk: tail-calls
    // the C2 ctor above.
    EMU_CHANNEL.construct();
    0
}

// 0x716e8 — __ZN4FMOD15ChannelEmulated4stopEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::stop(void)")]
pub fn stub_716e8() -> i32 {
    // IDA 0x716e8 `ChannelEmulated::stop`: stops the voice, unwires the
    // head unit plus the reverb link (0x716f4..tail).
    EMU_CHANNEL.stop()
}

// 0x71818 — __ZN4FMOD15ChannelEmulatedD0Ev
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_71818() {
    // IDA 0x71818 `ChannelEmulated::~ChannelEmulated` D0: vtable reset
    // plus operator delete (0x7182c..0x71830); the drop below is the
    // delete.
    EMU_CHANNEL.destroy();
}

// 0x7183c — __ZN4FMOD15ChannelEmulatedD1Ev
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_7183c() {
    // IDA 0x7183c `ChannelEmulated::~ChannelEmulated` D1: vtable reset
    // only (0x71848).
    EMU_CHANNEL.destroy();
}

// 0x71854 — __ZN4FMOD11ChannelRealC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelReal::ChannelReal(void)")]
pub fn stub_71854() {
    // IDA 0x71854 `ChannelReal::ChannelReal`: zeroes the links, latches
    // the idle flag plus the default freq/volume (0x71860..0x7188c).
    REAL_CHANNEL.init();
}

// 0x718a0 — __ZN4FMOD11ChannelReal4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_718a0() -> i32 {
    // IDA 0x718a0 `ChannelReal::init`: zeroes the links and latches the
    // system/output/voice ids (0x718b0..0x718d0).
    REAL_CHANNEL.init()
}

// 0x718dc — __ZN4FMOD11ChannelReal5closeEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::close(void)")]
pub fn stub_718dc() -> i32 {
    // IDA 0x718dc `ChannelReal::close`: dispatches through the vtable
    // (sole call).
    REAL_CHANNEL.close()
}

// 0x718e8 — __ZN4FMOD11ChannelReal5allocEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::alloc(void)")]
pub fn stub_718e8() -> i32 {
    // IDA 0x718e8 `ChannelReal::alloc`: bumps the sound ref (0x718f0..
    // 0x71908).
    REAL_CHANNEL.alloc()
}

// 0x7190c — __ZN4FMOD11ChannelReal5allocEPNS_4DSPIE
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelReal::alloc(FMOD::DSPI *)")]
pub fn stub_7190c() -> i32 {
    // IDA 0x7190c `ChannelReal::alloc` (DSPI): bumps the sound ref and
    // clears the start latch (0x7191c..0x71928).
    REAL_CHANNEL.alloc()
}

// 0x71930 — __ZN4FMOD11ChannelReal23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")]
pub fn stub_71930() -> i32 {
    // IDA 0x71930 `ChannelReal::set2DFreqVolumePanFor3D`: returns 0
    // (0x71934).
    0
}

// 0x71938 — __ZN4FMOD11ChannelReal6updateEi
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::update(int)")]
pub fn stub_71938() -> i32 {
    // IDA 0x71938 `ChannelReal::update`: returns 0 (0x7193c).
    REAL_CHANNEL.update()
}

// 0x71940 — __ZN4FMOD11ChannelReal12updateStreamEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::updateStream(void)")]
pub fn stub_71940() -> i32 {
    // IDA 0x71940 `ChannelReal::updateStream`: returns 0 (0x71944).
    REAL_CHANNEL.update()
}

// 0x71948 — __ZN4FMOD11ChannelReal5startEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::start(void)")]
pub fn stub_71948() -> i32 {
    // IDA 0x71948 `ChannelReal::start`: returns 0 (0x7194c).
    REAL_CHANNEL.start()
}

// 0x71950 — __ZN4FMOD11ChannelReal4stopEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::stop(void)")]
pub fn stub_71950() -> i32 {
    // IDA 0x71950 `ChannelReal::stop`: drops the sound ref and marks the
    // stopped bits (0x71958..0x71970).
    REAL_CHANNEL.stop()
}

// 0x7197c — __ZN4FMOD11ChannelReal9setPausedEb
// type: int __fastcall(FMOD::ChannelReal *this, bool)
#[doc(alias = "FMOD::ChannelReal::setPaused(bool)")]
pub fn stub_7197c(paused: bool) -> i32 {
    // IDA 0x7197c `ChannelReal::setPaused`: toggles the 0x20 flag bit
    // (0x71980..0x71990).
    REAL_CHANNEL.set_paused(paused)
}

// 0x719a0 — __ZN4FMOD11ChannelReal9getPausedEPb
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::getPaused(bool *)")]
pub fn stub_719a0(with_out: bool) -> (i32, bool) {
    // IDA 0x719a0 `ChannelReal::getPaused`: 37 without an out-param, else
    // the 0x20 flag bit (0x719a4..0x719bc).
    if with_out {
        (0, REAL_CHANNEL.paused())
    } else {
        (37, false)
    }
}

// 0x719c0 — __ZN4FMOD11ChannelReal9setVolumeEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setVolume(float)")]
pub fn stub_719c0(volume: f32) -> i32 {
    // IDA 0x719c0 `ChannelReal::setVolume`: returns 0 (0x719c4).
    *REAL_CHANNEL.volume.lock() = volume;
    0
}

// 0x719c8 — __ZN4FMOD11ChannelReal12setFrequencyEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setFrequency(float)")]
pub fn stub_719c8(frequency: f32) -> i32 {
    // IDA 0x719c8 `ChannelReal::setFrequency`: returns 0 (0x719cc).
    *REAL_CHANNEL.frequency.lock() = frequency;
    0
}

// 0x719d0 — __ZN4FMOD11ChannelReal6setPanEff
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::setPan(float,float)")]
pub fn stub_719d0(left: f32, right: f32) -> i32 {
    // IDA 0x719d0 `ChannelReal::setPan`: returns 0 (0x719d4).
    REAL_CHANNEL.set_pan(left, right)
}

// 0x719d8 — __ZN4FMOD11ChannelReal16setDSPClockDelayEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::setDSPClockDelay(void)")]
pub fn stub_719d8() -> i32 {
    // IDA 0x719d8 `ChannelReal::setDSPClockDelay`: returns 0 (0x719dc).
    0
}

// 0x719e0 — __ZN4FMOD11ChannelReal13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelReal *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t)
#[doc(alias = "FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_719e0(mix: [f32; 8]) -> i32 {
    // IDA 0x719e0 `ChannelReal::setSpeakerMix`: routes the eight gains
    // into the voice matrix (0x719f8..tail).
    REAL_CHANNEL.set_speaker_mix(mix)
}

// 0x71e34 — __ZN4FMOD11ChannelReal11setPositionEjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")]
pub fn stub_71e34(pos: u32, unit: u32) -> i32 {
    // IDA 0x71e34 `ChannelReal::setPosition`: units other than 0..2 and 4
    // return 25; else latches the position (0x71e48..0x71e94).
    REAL_CHANNEL.set_position(pos, unit)
}

// 0x72008 — __ZN4FMOD11ChannelReal11getPositionEPjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")]
pub fn stub_72008(unit: u32) -> (i32, u32) {
    // IDA 0x72008 `ChannelReal::getPosition`: 37 without a voice or an
    // out-param, else the position in the unit (0x72010..tail).
    let _ = unit;
    REAL_CHANNEL.position()
}

// 0x722f0 — __ZN4FMOD11ChannelReal13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_722f0(start: u32, len: u32) -> i32 {
    // IDA 0x722f0 `ChannelReal::setLoopPoints`: 37 past the sound end,
    // else latches start plus length (0x722f8..0x7231c).
    REAL_CHANNEL.set_loop_points(start, len)
}

// 0x72328 — __ZN4FMOD11ChannelReal12setLoopCountEi
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setLoopCount(int)")]
pub fn stub_72328(count: i32) -> i32 {
    // IDA 0x72328 `ChannelReal::setLoopCount`: latches the count
    // (0x72328..0x72330).
    REAL_CHANNEL.set_loop_count(count)
}

// 0x72334 — __ZN4FMOD11ChannelReal12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelReal *this, int *)
#[doc(alias = "FMOD::ChannelReal::getLoopCount(int *)")]
pub fn stub_72334(with_out: bool) -> (i32, i32) {
    // IDA 0x72334 `ChannelReal::getLoopCount`: 37 without an out-param,
    // else the count (0x72338..0x72348).
    if with_out {
        (0, REAL_CHANNEL.loop_count())
    } else {
        (37, 0)
    }
}

// 0x7234c — __ZN4FMOD11ChannelReal14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setLowPassGain(float)")]
pub fn stub_7234c(gain: f32) -> i32 {
    // IDA 0x7234c `ChannelReal::setLowPassGain`: returns 0 (0x72350).
    REAL_CHANNEL.set_lowpass(gain)
}

// 0x72354 — __ZN4FMOD11ChannelReal15set3DAttributesEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DAttributes(void)")]
pub fn stub_72354() -> i32 {
    // IDA 0x72354 `ChannelReal::set3DAttributes`: returns 0 (0x72358).
    0
}

// 0x7235c — __ZN4FMOD11ChannelReal19set3DMinMaxDistanceEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DMinMaxDistance(void)")]
pub fn stub_7235c() -> i32 {
    // IDA 0x7235c `ChannelReal::set3DMinMaxDistance`: returns 0 (0x72360).
    0
}

// 0x72364 — __ZN4FMOD11ChannelReal14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::set3DOcclusion(float,float)")]
pub fn stub_72364(direct: f32, reverb: f32) -> i32 {
    // IDA 0x72364 `ChannelReal::set3DOcclusion`: forwards into the voice
    // with the group value, 0 without a group (0x7236c..0x72380).
    REAL_CHANNEL.set_occlusion(direct, reverb)
}

// 0x72388 — __ZN4FMOD11ChannelReal9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelReal *this, bool *, bool)
#[doc(alias = "FMOD::ChannelReal::isPlaying(bool *,bool)")]
pub fn stub_72388(with_out: bool) -> (i32, bool) {
    // IDA 0x72388 `ChannelReal::isPlaying`: 37 without an out-param, else
    // the 0x50 flag bits (0x7238c..0x723a8).
    if with_out {
        (0, REAL_CHANNEL.real_playing())
    } else {
        (37, false)
    }
}

// 0x723b0 — __ZN4FMOD11ChannelReal9isVirtualEPb
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::isVirtual(bool *)")]
pub fn stub_723b0(with_out: bool) -> (i32, bool) {
    // IDA 0x723b0 `ChannelReal::isVirtual`: 37 without an out-param, else
    // false — a real voice is never virtual (0x723b4..0x723c0).
    if with_out {
        (0, false)
    } else {
        (37, false)
    }
}

// 0x723c4 — __ZN4FMOD11ChannelReal11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int()
#[doc(alias = "FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_723c4() -> i32 {
    // IDA 0x723c4 `ChannelReal::getSpectrum`: 51 on a real voice
    // (0x723c8).
    REAL_CHANNEL.spectrum_unsupported()
}

// 0x723cc — __ZN4FMOD11ChannelReal11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelReal *this, float *, int, int)
#[doc(alias = "FMOD::ChannelReal::getWaveData(float *,int,int)")]
pub fn stub_723cc() -> i32 {
    // IDA 0x723cc `ChannelReal::getWaveData`: 51 on a real voice
    // (0x723d0).
    REAL_CHANNEL.spectrum_unsupported()
}

// 0x723d4 — __ZN4FMOD11ChannelReal10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")]
pub fn stub_723d4() -> (i32, u32) {
    // IDA 0x723d4 `ChannelReal::getDSPHead`: zeroes the head and returns
    // 51 on a real voice (0x723dc..0x723e0).
    REAL_CHANNEL.dsp_head_unsupported()
}

// 0x723e4 — __ZN4FMOD11ChannelReal7setModeEj
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setMode(unsigned int)")]
pub fn stub_723e4(mode: u32) -> i32 {
    // IDA 0x723e4 `ChannelReal::setMode`: merges the loop/open bits plus
    // the 3D bits into the mode word (0x723e8..tail).
    REAL_CHANNEL.set_mode(mode)
}

// 0x72528 — __ZN4FMOD11ChannelReal19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_72528() -> (i32, ReverbProps) {
    // IDA 0x72528 `ChannelReal::getReverbProperties`: 37 without an
    // out-param or a voice, else the channel properties (0x7253c..0x72584).
    (0, REAL_CHANNEL.reverb())
}

// 0x725a0 — __ZN4FMOD11ChannelReal19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_725a0(props: ReverbProps) -> i32 {
    // IDA 0x725a0 `ChannelReal::setReverbProperties`: 37 without
    // properties, else latches them per instance flag (0x725b8..tail).
    REAL_CHANNEL.set_reverb(props)
}

// 0x726d8 — __ZN4FMOD11ChannelReal19updateSpeakerLevelsEf
// type: int __fastcall(FMOD::ChannelReal *this, float32_t)
#[doc(alias = "FMOD::ChannelReal::updateSpeakerLevels(float)")]
pub fn stub_726d8(gain: f32) -> i32 {
    // IDA 0x726d8 `ChannelReal::updateSpeakerLevels`: rebuilds the level
    // matrix at the gain (0x726e4..tail).
    REAL_CHANNEL.update_speaker_levels(gain)
}

// 0x72910 — __ZN4FMOD11ChannelReal16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelReal *this, int, float *, int)
#[doc(alias = "FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")]
pub fn stub_72910(levels: Vec<f32>) -> i32 {
    // IDA 0x72910 `ChannelReal::setSpeakerLevels`: allocs the pool on
    // demand (44 on failure), clamps plus stores the levels (0x72920..
    // tail).
    REAL_CHANNEL.set_speaker_levels(levels)
}

// 0x72a04 — __ZN4FMOD11ChannelRealD0Ev
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a04() {
    // IDA 0x72a04 `ChannelReal::~ChannelReal` D0: vtable reset plus
    // operator delete (0x72a18..0x72a1c); the drop below is the delete.
    REAL_CHANNEL.destroy();
}

// 0x72a28 — __ZN4FMOD11ChannelRealD1Ev
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a28() {
    // IDA 0x72a28 `ChannelReal::~ChannelReal` D1: vtable reset only
    // (0x72a34).
    REAL_CHANNEL.destroy();
}

// 0x72a40 — __ZN4FMOD19ChannelRealManual3D5allocEv
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::alloc(void)")]
pub fn stub_72a40() -> i32 {
    // IDA 0x72a40 `ChannelRealManual3D::alloc`: clears the manual latch
    // then runs the real alloc (0x72a4c..0x72a54).
    MANUAL_3D.alloc()
}

// 0x72a58 — __ZN4FMOD19ChannelRealManual3DC2Ev
// type: _DWORD *__fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")]
pub fn stub_72a58() {
    // IDA 0x72a58 `ChannelRealManual3D::ChannelRealManual3D`: runs the
    // real ctor and clears the manual latch (0x72a64..0x72a7c).
    MANUAL_3D.construct();
}

// 0x72a88 — __ZN4FMOD19ChannelRealManual3D23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")]
pub fn stub_72a88() -> i32 {
    // IDA 0x72a88 `ChannelRealManual3D::set2DFreqVolumePanFor3D`: derives
    // the 2D params from the 3D position (0x72a88..tail).
    MANUAL_3D.compute_3d()
}

// 0x73de4 — __ZN4FMOD19ChannelRealManual3DD0Ev
// type: void __fastcall(FMOD::ChannelRealManual3D *__hidden this)
#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
pub fn stub_73de4() -> ! {
    todo!("0x73de4 FMOD::ChannelRealManual3D::~ChannelRealManual3D()")
}

// 0x73e08 — __ZN4FMOD19ChannelRealManual3DD1Ev
// type: void __fastcall(FMOD::ChannelRealManual3D *__hidden this)
#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
pub fn stub_73e08() -> ! {
    todo!("0x73e08 FMOD::ChannelRealManual3D::~ChannelRealManual3D()")
}

// 0x73e20 — __ZN4FMOD15ChannelSoftware14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelSoftware *this, float)
#[doc(alias = "FMOD::ChannelSoftware::setLowPassGain(float)")]
pub fn stub_73e20() -> ! {
    todo!("0x73e20 FMOD::ChannelSoftware::setLowPassGain(float)")
}

// 0x73e34 — __ZN4FMOD15ChannelSoftware16setDSPClockDelayEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::setDSPClockDelay(void)")]
pub fn stub_73e34() -> ! {
    todo!("0x73e34 FMOD::ChannelSoftware::setDSPClockDelay(void)")
}

// 0x73f0c — __ZN4FMOD15ChannelSoftware11setPositionEjj
// type: int __fastcall(unsigned __int64 this, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::setPosition(unsigned int,unsigned int)")]
pub fn stub_73f0c() -> ! {
    todo!("0x73f0c FMOD::ChannelSoftware::setPosition(unsigned int,unsigned int)")
}

// 0x741f4 — __ZN4FMOD15ChannelSoftware11getPositionEPjj
// type: int __fastcall(int this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")]
pub fn stub_741f4() -> ! {
    todo!("0x741f4 FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")
}

// 0x74554 — __ZN4FMOD15ChannelSoftware10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")]
pub fn stub_74554() -> ! {
    todo!("0x74554 FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")
}

// 0x74564 — __ZN4FMOD15ChannelSoftware16moveChannelGroupEPNS_13ChannelGroupIES2_b
// type: FMOD::DSPI *__fastcall(FMOD::DSPI **this, FMOD::DSPI **, FMOD::DSPI **, bool)
#[doc(alias = "FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_74564() -> ! {
    todo!("0x74564 FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")
}

// 0x745d4 — __ZN4FMOD15ChannelSoftware19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_745d4() -> ! {
    todo!("0x745d4 FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x7464c — __ZN4FMOD15ChannelSoftware12addToReverbsEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")]
pub fn stub_7464c() -> ! {
    todo!("0x7464c FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")
}

// 0x748b4 — __ZN4FMOD15ChannelSoftware11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelSoftware *this, float *, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getWaveData(float *,int,int)")]
pub fn stub_748b4() -> ! {
    todo!("0x748b4 FMOD::ChannelSoftware::getWaveData(float *,int,int)")
}

// 0x749c4 — __ZN4FMOD15ChannelSoftware11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_749c4() -> ! {
    todo!("0x749c4 FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")
}

// 0x74b20 — __ZN4FMOD15ChannelSoftware9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *, bool)
#[doc(alias = "FMOD::ChannelSoftware::isPlaying(bool *,bool)")]
pub fn stub_74b20() -> ! {
    todo!("0x74b20 FMOD::ChannelSoftware::isPlaying(bool *,bool)")
}

// 0x74bd0 — __ZN4FMOD15ChannelSoftware7setModeEj
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setMode(unsigned int)")]
pub fn stub_74bd0() -> ! {
    todo!("0x74bd0 FMOD::ChannelSoftware::setMode(unsigned int)")
}

// 0x74c04 — __ZN4FMOD15ChannelSoftware12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::getLoopCount(int *)")]
pub fn stub_74c04() -> ! {
    todo!("0x74c04 FMOD::ChannelSoftware::getLoopCount(int *)")
}

// 0x74c44 — __ZN4FMOD15ChannelSoftware12setLoopCountEi
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopCount(int)")]
pub fn stub_74c44() -> ! {
    todo!("0x74c44 FMOD::ChannelSoftware::setLoopCount(int)")
}

// 0x74c90 — __ZN4FMOD15ChannelSoftware13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelSoftware *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_74c90() -> ! {
    todo!("0x74c90 FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")
}

// 0x74cd8 — __ZN4FMOD15ChannelSoftware6setPanEff
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t, float)
#[doc(alias = "FMOD::ChannelSoftware::setPan(float,float)")]
pub fn stub_74cd8() -> ! {
    todo!("0x74cd8 FMOD::ChannelSoftware::setPan(float,float)")
}

// 0x74de8 — __ZN4FMOD15ChannelSoftware12setFrequencyEf
// type: FMOD::DSPWaveTable *__fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setFrequency(float)")]
pub fn stub_74de8() -> ! {
    todo!("0x74de8 FMOD::ChannelSoftware::setFrequency(float)")
}

// 0x74edc — __ZN4FMOD15ChannelSoftware15updateReverbMixEPNS_7ReverbIEf
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::ReverbI *, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")]
pub fn stub_74edc() -> ! {
    todo!("0x74edc FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")
}

// 0x751dc — __ZN4FMOD15ChannelSoftware15updateDirectMixEf
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateDirectMix(float)")]
pub fn stub_751dc() -> ! {
    todo!("0x751dc FMOD::ChannelSoftware::updateDirectMix(float)")
}

// 0x75408 — __ZN4FMOD15ChannelSoftware13setupDSPCodecEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")]
pub fn stub_75408() -> ! {
    todo!("0x75408 FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")
}

// 0x75738 — __ZN4FMOD15ChannelSoftware5closeEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::close(void)")]
pub fn stub_75738() -> ! {
    todo!("0x75738 FMOD::ChannelSoftware::close(void)")
}

// 0x757fc — __ZN4FMOD15ChannelSoftware4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_757fc() -> ! {
    todo!("0x757fc FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x759c0 — __ZN4FMOD15ChannelSoftwareC2Ev
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_759c0() -> ! {
    todo!("0x759c0 FMOD::ChannelSoftware::ChannelSoftware(void)")
}

// 0x75a44 — __ZN4FMOD15ChannelSoftwareC1Ev
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_75a44() -> ! {
    todo!("0x75a44 FMOD::ChannelSoftware::ChannelSoftware(void)")
}

// 0x75a48 — __ZN4FMOD15ChannelSoftware9setPausedEb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool)
#[doc(alias = "FMOD::ChannelSoftware::setPaused(bool)")]
pub fn stub_75a48() -> ! {
    todo!("0x75a48 FMOD::ChannelSoftware::setPaused(bool)")
}

// 0x75b50 — __ZN4FMOD15ChannelSoftware5startEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::start(void)")]
pub fn stub_75b50() -> ! {
    todo!("0x75b50 FMOD::ChannelSoftware::start(void)")
}

// 0x75be0 — __ZN4FMOD15ChannelSoftware5allocEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::alloc(void)")]
pub fn stub_75be0() -> ! {
    todo!("0x75be0 FMOD::ChannelSoftware::alloc(void)")
}

// 0x75f8c — __ZN4FMOD15ChannelSoftware4stopEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::stop(void)")]
pub fn stub_75f8c() -> ! {
    todo!("0x75f8c FMOD::ChannelSoftware::stop(void)")
}

// 0x762c4 — __ZN4FMOD15ChannelSoftware16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelSoftware *this, int, float *, int)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")]
pub fn stub_762c4() -> ! {
    todo!("0x762c4 FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")
}

// 0x76584 — __ZN4FMOD15ChannelSoftware13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelSoftware *this, int, int, int, int, float, float, float, float)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_76584() -> ! {
    todo!("0x76584 FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x76988 — __ZN4FMOD15ChannelSoftware9setVolumeEf
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setVolume(float)")]
pub fn stub_76988() -> ! {
    todo!("0x76988 FMOD::ChannelSoftware::setVolume(float)")
}

// 0x76a80 — __ZN4FMOD15ChannelSoftware14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelSoftware *this, float, float)
#[doc(alias = "FMOD::ChannelSoftware::set3DOcclusion(float,float)")]
pub fn stub_76a80() -> ! {
    todo!("0x76a80 FMOD::ChannelSoftware::set3DOcclusion(float,float)")
}

// 0x76b3c — __ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_76b3c() -> ! {
    todo!("0x76b3c FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x7709c — __ZN4FMOD15ChannelSoftware9getPausedEPb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *)
#[doc(alias = "FMOD::ChannelSoftware::getPaused(bool *)")]
pub fn stub_7709c() -> ! {
    todo!("0x7709c FMOD::ChannelSoftware::getPaused(bool *)")
}

// 0x77138 — __ZN4FMOD15ChannelSoftware5allocEPNS_4DSPIE
// type: int __fastcall(FMOD::DSPI **this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")]
pub fn stub_77138() -> ! {
    todo!("0x77138 FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")
}

// 0x773c4 — __ZN4FMOD15ChannelSoftwareD1Ev
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773c4() -> ! {
    todo!("0x773c4 FMOD::ChannelSoftware::~ChannelSoftware()")
}

// 0x773f0 — __ZN4FMOD15ChannelSoftwareD0Ev
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773f0() -> ! {
    todo!("0x773f0 FMOD::ChannelSoftware::~ChannelSoftware()")
}
