//! platform - generated_next_w - 100 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 100 after 0x77428 not yet in crates/platform/src
//! Filter: iOS|ViewController|RobloxView|Platform ObjC (781/781 done) | 26080->26180 distinct
//! Batch: 100 stubs | range 0x77428..0x7d208 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
/// Minimal `FMOD::ChannelStream` counterpart (IDA 0x77428..): the fan-out
/// voice count plus the last latched mix state. Every method fans out
/// over the sub-voices and returns the last result; an empty stream
/// returns 0 without touching anything.
#[derive(Debug)]
pub struct StreamChannel {
    subs: std::sync::atomic::AtomicU32,
    started: std::sync::atomic::AtomicBool,
    volume: parking_lot::Mutex<f32>,
    frequency: parking_lot::Mutex<f32>,
    pan: parking_lot::Mutex<[f32; 2]>,
    clocked: std::sync::atomic::AtomicBool,
    speaker_mix: parking_lot::Mutex<[f32; 8]>,
    speaker_levels: parking_lot::Mutex<Vec<f32>>,
    lowpass: parking_lot::Mutex<f32>,
    occlusion: parking_lot::Mutex<[f32; 2]>,
    reverb: parking_lot::Mutex<crate::generated_next_k::ReverbProps>,
    group: parking_lot::Mutex<u32>,
    updates: std::sync::atomic::AtomicU32,
    fanouts: std::sync::atomic::AtomicU32,
    stopped: std::sync::atomic::AtomicBool,
    loop_count: std::sync::atomic::AtomicI32,
    loop_start: std::sync::atomic::AtomicU32,
    loop_len: std::sync::atomic::AtomicU32,
    mode: std::sync::atomic::AtomicU32,
    position: std::sync::atomic::AtomicU32,
    position_unit: std::sync::atomic::AtomicU32,
    paused: std::sync::atomic::AtomicBool,
    allocated: std::sync::atomic::AtomicBool,
}
impl Default for StreamChannel {
    fn default() -> Self {
        Self {
            subs: std::sync::atomic::AtomicU32::new(0),
            started: std::sync::atomic::AtomicBool::new(false),
            volume: parking_lot::Mutex::new(1.0),
            frequency: parking_lot::Mutex::new(44100.0),
            pan: parking_lot::Mutex::new([0.0; 2]),
            clocked: std::sync::atomic::AtomicBool::new(false),
            speaker_mix: parking_lot::Mutex::new([0.0; 8]),
            speaker_levels: parking_lot::Mutex::new(Vec::new()),
            lowpass: parking_lot::Mutex::new(1.0),
            occlusion: parking_lot::Mutex::new([0.0; 2]),
            reverb: parking_lot::Mutex::new(crate::generated_next_k::ReverbProps::default()),
            group: parking_lot::Mutex::new(0),
            updates: std::sync::atomic::AtomicU32::new(0),
            fanouts: std::sync::atomic::AtomicU32::new(0),
            stopped: std::sync::atomic::AtomicBool::new(false),
            loop_count: std::sync::atomic::AtomicI32::new(0),
            loop_start: std::sync::atomic::AtomicU32::new(0),
            loop_len: std::sync::atomic::AtomicU32::new(0),
            mode: std::sync::atomic::AtomicU32::new(0),
            position: std::sync::atomic::AtomicU32::new(0),
            position_unit: std::sync::atomic::AtomicU32::new(0),
            paused: std::sync::atomic::AtomicBool::new(false),
            allocated: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl StreamChannel {
    pub fn set_subs(&self, subs: u32) {
        self.subs.store(subs, std::sync::atomic::Ordering::SeqCst);
    }
    /// Shared fan-out guard: empty streams return 0 immediately
    /// (e.g. 0x7743c, 0x77498, 0x77504).
    fn fan_out(&self) -> bool {
        let subs = self.subs.load(std::sync::atomic::Ordering::SeqCst);
        if subs == 0 {
            return false;
        }
        self.fanouts.fetch_add(subs, std::sync::atomic::Ordering::SeqCst);
        true
    }
    pub fn fanout_count(&self) -> u32 {
        self.fanouts.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelStream::set2DFreqVolumePanFor3D` (IDA 0x77428): fans out
    /// over the sub-voices (0x7743c..0x7746c).
    pub fn compute_3d(&self) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        0
    }
    /// `ChannelStream::moveChannelGroup` (IDA 0x77474): fans the rewire
    /// out (0x77498..0x774d4).
    pub fn move_group(&self, group: u32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.group.lock() = group;
        0
    }
    /// `ChannelStream::start` (IDA 0x774e0): 36 without stream data, else
    /// starts every sub-voice (0x774e8..0x7756c).
    pub fn start(&self, has_data: bool) -> i32 {
        if !has_data {
            return 36;
        }
        if !self.fan_out() {
            return 0;
        }
        self.started.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_started(&self) -> bool {
        self.started.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelStream::update` (IDA 0x77574): fans the tick out
    /// (0x77590..0x775c4).
    pub fn update(&self) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        self.updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelStream::setVolume` (IDA 0x775d0): fans out; the single
    /// voice takes the direct path (0x775e0..0x7764c).
    pub fn set_volume(&self, volume: f32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.volume.lock() = volume;
        0
    }
    /// `ChannelStream::setFrequency` (IDA 0x77718): fans out (0x77734..
    /// 0x77768).
    pub fn set_frequency(&self, frequency: f32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.frequency.lock() = frequency;
        0
    }
    /// `ChannelStream::setPan` (IDA 0x77774): stereo pairs split hard
    /// left/right per row (0x777bc..0x7780c), then fans out.
    pub fn set_pan(&self, left: f32, right: f32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.pan.lock() = [left, right];
        0
    }
    /// `ChannelStream::setDSPClockDelay` (IDA 0x7781c): fans out
    /// (0x77830..0x77860).
    pub fn set_clock_delay(&self) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        self.clocked.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelStream::setSpeakerMix` (IDA 0x77868): fans the eight gains
    /// out (0x77890..0x778f4).
    pub fn set_speaker_mix(&self, mix: [f32; 8]) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.speaker_mix.lock() = mix;
        0
    }
    /// `ChannelStream::setSpeakerLevels` (IDA 0x77904): fans the matrix
    /// out (0x77928..0x77964).
    pub fn set_speaker_levels(&self, levels: Vec<f32>) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.speaker_levels.lock() = levels;
        0
    }
    /// `ChannelStream::set3DAttributes` (IDA 0x77970): fans out
    /// (0x77984..0x779b4).
    pub fn set_3d_attributes(&self) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        0
    }
    /// `ChannelStream::setLowPassGain` (IDA 0x779bc): fans out (0x779d8..
    /// 0x77a0c).
    pub fn set_lowpass(&self, gain: f32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.lowpass.lock() = gain;
        0
    }
    /// `ChannelStream::set3DMinMaxDistance` (IDA 0x77a18): fans out
    /// (0x77a2c..0x77a5c).
    pub fn set_3d_min_max(&self) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        0
    }
    /// `ChannelStream::set3DOcclusion` (IDA 0x77a64): fans out (0x77a84..
    /// 0x77abc).
    pub fn set_occlusion(&self, direct: f32, reverb: f32) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.occlusion.lock() = [direct, reverb];
        0
    }
    /// `ChannelStream::setReverbProperties` (IDA 0x77ac8): fans out
    /// (0x77ae4..0x77b18).
    pub fn set_reverb(&self, props: crate::generated_next_k::ReverbProps) -> i32 {
        if !self.fan_out() {
            return 0;
        }
        *self.reverb.lock() = props;
        0
    }
    /// `ChannelStream::getReverbProperties` (IDA 0x77b24): empty streams
    /// return 0, else the first sub-voice result (0x77b2c..0x77b3c).
    pub fn reverb_props(&self) -> (i32, crate::generated_next_k::ReverbProps) {
        let _ = self.fan_out();
        (0, self.reverb.lock().clone())
    }
    /// `ChannelStream::isPlaying` (IDA 0x77b48): the inverse of the
    /// stopped byte (0x77b48..0x77b58).
    pub fn stream_playing(&self) -> (i32, bool) {
        (
            0,
            !self.stopped.load(std::sync::atomic::Ordering::SeqCst),
        )
    }
    /// `ChannelStream::getSpectrum` (IDA 0x77b5c) and `getWaveData`
    /// (0x77b6c): delegate to the first sub-voice (sole calls).
    pub fn spectrum(&self, len: usize) -> (i32, Vec<f32>) {
        let _ = self.fan_out();
        (0, vec![0.0; len])
    }
    pub fn wave_data(&self, channels: usize, frames: usize) -> (i32, Vec<f32>) {
        let _ = self.fan_out();
        (0, vec![0.0; channels * frames])
    }
    /// `ChannelStream::getDSPHead` (IDA 0x77b7c): delegates to the first
    /// sub-voice (sole call).
    pub fn dsp_head(&self) -> (i32, u32) {
        let _ = self.fan_out();
        (0, 0)
    }
    /// `ChannelStream::setLoopCount` (IDA 0x77b8c): runs the real setter,
    /// then mirrors into the stream (0x77b9c..0x77bbc).
    pub fn set_loop_count(&self, count: i32) -> i32 {
        crate::generated_next_k::REAL_CHANNEL.set_loop_count(count);
        self.loop_count.store(count, std::sync::atomic::Ordering::SeqCst);
        let _ = self.fan_out();
        0
    }
    /// `ChannelStream::setLoopPoints` (IDA 0x77bc0): runs the real setter,
    /// then mirrors start only (0x77bd8..0x77c10).
    pub fn set_loop_points(&self, start: u32, len: u32) -> i32 {
        let result = crate::generated_next_k::REAL_CHANNEL.set_loop_points(start, len);
        if result == 0 {
            self.loop_start.store(start, std::sync::atomic::Ordering::SeqCst);
            self.loop_len.store(len, std::sync::atomic::Ordering::SeqCst);
        }
        let _ = self.fan_out();
        result
    }
    /// `ChannelStream::getPosition` (IDA 0x77c14): 37 without an
    /// out-param or a stream, else the unit position (0x77c30..tail).
    pub fn position(&self) -> (i32, u32) {
        (0, self.position.load(std::sync::atomic::Ordering::SeqCst))
    }
    /// `ChannelStream::stop` (IDA 0x77f74): marks the stopped byte and
    /// stops the stream (0x77f8c..tail).
    pub fn stop(&self) -> i32 {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
        self.started.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelStream::setMode` (IDA 0x78168): runs the real setter plus
    /// the stream, then fans the masked mode out (0x7817c..0x781e8).
    pub fn set_mode(&self, mode: u32) -> i32 {
        crate::generated_next_k::REAL_CHANNEL.set_mode(mode);
        self.mode.store(mode, std::sync::atomic::Ordering::SeqCst);
        let _ = self.fan_out();
        0
    }
    /// `ChannelStream::ChannelStream` (IDA 0x781f0): runs the real ctor,
    /// zeroes the voice list with one slot live (0x781fc..0x78260).
    pub fn construct(&self) {
        self.set_subs(1);
        self.stopped.store(false, std::sync::atomic::Ordering::SeqCst);
        self.started.store(false, std::sync::atomic::Ordering::SeqCst);
        self.allocated.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ChannelStream::alloc` (IDA 0x78270): latches the stream params
    /// (0x78280..tail).
    pub fn alloc(&self) -> i32 {
        self.allocated.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelStream::setPositionEx` (IDA 0x78540): seeks the stream
    /// (0x78540..tail).
    pub fn set_position_ex(&self, pos: u32, unit: u32) -> i32 {
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        self.position_unit.store(unit, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelStream::setPaused` (IDA 0x78af0): fans out under the lock
    /// unless the stream flag is set (0x78b08..0x78b78).
    pub fn set_paused(&self, paused: bool) -> i32 {
        self.paused.store(paused, std::sync::atomic::Ordering::SeqCst);
        let _ = self.fan_out();
        0
    }
    pub fn is_paused(&self) -> bool {
        self.paused.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ChannelStream::updateStream` (IDA 0x78b80): pumps the stream
    /// (0x78b80..tail).
    pub fn update_stream(&self) -> i32 {
        self.updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ChannelStream::setPosition` (IDA 0x78fb4): dispatches into the
    /// vtable `setPositionEx` (sole call, 0x78fb4).
    pub fn set_position(&self, pos: u32, unit: u32) -> i32 {
        self.set_position_ex(pos, unit)
    }
    /// `ChannelStream::~ChannelStream` D1 (IDA 0x78fe8): vtable reset
    /// only; D0 above also deletes.
    pub fn destroy(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
        self.started.store(false, std::sync::atomic::Ordering::SeqCst);
        self.set_subs(0);
    }
}
/// One `FMOD::ChannelGroupI` row behind the validate wrappers (IDA
/// 0x79000..): handle 0 is never valid.
#[derive(Debug, Clone)]
pub struct FmodGroupRow {
    pub valid: bool,
    pub volume: f32,
    pub paused: bool,
    pub muted: bool,
    pub pitch: f32,
    pub children: u32,
}
impl Default for FmodGroupRow {
    fn default() -> Self {
        Self {
            valid: true,
            volume: 1.0,
            paused: false,
            muted: false,
            pitch: 1.0,
            children: 0,
        }
    }
}
/// Minimal channel-group table behind `FMOD::Channel(GroupI)::\*` (IDA
/// 0x79000..): validate plus the recursive mute/pause walks.
#[derive(Debug, Default)]
pub struct FmodGroups {
    rows: parking_lot::Mutex<Vec<FmodGroupRow>>,
}
impl FmodGroups {
    pub const INVALID_HANDLE: i32 = 36;
    pub const NO_OUT: i32 = 37;
    /// Allocates a group row; the 1-based index below is the handle.
    pub fn alloc(&self) -> u32 {
        let mut rows = self.rows.lock();
        rows.push(FmodGroupRow::default());
        rows.len() as u32
    }
    /// `ChannelGroupI::validate` (IDA 0x79034): 36 on a null handle, 37
    /// without an out-slot, else 0 (0x79038..0x7904c).
    pub fn validate(&self, handle: u32, with_out: bool) -> i32 {
        if handle == 0 {
            return Self::INVALID_HANDLE;
        }
        if !with_out {
            return Self::NO_OUT;
        }
        if (handle as usize) > self.rows.lock().len() {
            return Self::INVALID_HANDLE;
        }
        0
    }
    pub fn with_row<T>(&self, handle: u32, f: impl FnOnce(&mut FmodGroupRow) -> T) -> Option<T> {
        self.rows.lock().get_mut(handle.checked_sub(1)? as usize).map(f)
    }
    pub fn get<T: Clone>(&self, handle: u32, f: impl FnOnce(&FmodGroupRow) -> T) -> Option<T> {
        self.rows.lock().get(handle.checked_sub(1)? as usize).map(f)
    }
    /// `ChannelGroupI::setMute` (IDA 0x791e8): latches the flag, then
    /// walks the children unless told otherwise (0x791f0..tail).
    pub fn set_mute(&self, handle: u32, muted: bool, recursive: bool) -> i32 {
        if self.validate(handle, true) != 0 {
            return Self::INVALID_HANDLE;
        }
        if recursive {
            for row in self.rows.lock().iter_mut() {
                row.muted = muted;
            }
        } else {
            let _ = self.with_row(handle, |row| row.muted = muted);
        }
        0
    }
    /// `ChannelGroupI::setPaused` (IDA 0x79280): same recursive walk for
    /// the pause flag (0x7928c..tail).
    pub fn set_paused(&self, handle: u32, paused: bool, recursive: bool) -> i32 {
        if self.validate(handle, true) != 0 {
            return Self::INVALID_HANDLE;
        }
        if recursive {
            for row in self.rows.lock().iter_mut() {
                row.paused = paused;
            }
        } else {
            let _ = self.with_row(handle, |row| row.paused = paused);
        }
        0
    }
    /// `ChannelGroupI::getMemoryUsedImpl` (IDA 0x7906c): 0x64 base plus
    /// the name, DSP and buffer legs (0x7908c..0x790f0).
    pub fn memory_used(&self, handle: u32) -> u32 {
        match self.get(handle, |row| 0x64 + 0x64 * row.children) {
            Some(bytes) => bytes,
            None => 0,
        }
    }
}
pub static FMOD_GROUPS: std::sync::LazyLock<FmodGroups> =
    std::sync::LazyLock::new(FmodGroups::default);
static STREAM_CHANNEL: std::sync::LazyLock<StreamChannel> =
    std::sync::LazyLock::new(StreamChannel::default);

// 0x77428 - __ZN4FMOD13ChannelStream23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set2DFreqVolumePanFor3D(void)")]
pub fn stub_77428() -> i32 {
    // IDA 0x77428 `ChannelStream::set2DFreqVolumePanFor3D`: fans out over
    // the sub-voices (0x7743c..0x7746c).
    STREAM_CHANNEL.compute_3d()
}

// 0x77474 - __ZN4FMOD13ChannelStream16moveChannelGroupEPNS_13ChannelGroupIES2_b
// type: int __fastcall(int, int, int, unsigned __int8)
#[doc(alias = "FMOD::ChannelStream::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_77474(group: u32) -> i32 {
    // IDA 0x77474 `ChannelStream::moveChannelGroup`: fans the rewire out
    // (0x77498..0x774d4).
    STREAM_CHANNEL.move_group(group)
}

// 0x774e0 - __ZN4FMOD13ChannelStream5startEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::start(void)")]
pub fn stub_774e0(has_data: bool) -> i32 {
    // IDA 0x774e0 `ChannelStream::start`: 36 without stream data, else
    // starts every sub-voice (0x774e8..0x7756c).
    STREAM_CHANNEL.start(has_data)
}

// 0x77574 - __ZN4FMOD13ChannelStream6updateEi
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::update(int)")]
pub fn stub_77574() -> i32 {
    // IDA 0x77574 `ChannelStream::update`: fans the tick out (0x77590..
    // 0x775c4).
    STREAM_CHANNEL.update()
}

// 0x775d0 - __ZN4FMOD13ChannelStream9setVolumeEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setVolume(float)")]
pub fn stub_775d0(volume: f32) -> i32 {
    // IDA 0x775d0 `ChannelStream::setVolume`: fans out; the single voice
    // takes the direct path (0x775e0..0x7764c).
    STREAM_CHANNEL.set_volume(volume)
}

// 0x77718 - __ZN4FMOD13ChannelStream12setFrequencyEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setFrequency(float)")]
pub fn stub_77718(frequency: f32) -> i32 {
    // IDA 0x77718 `ChannelStream::setFrequency`: fans out (0x77734..
    // 0x77768).
    STREAM_CHANNEL.set_frequency(frequency)
}

// 0x77774 - __ZN4FMOD13ChannelStream6setPanEff
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::setPan(float,float)")]
pub fn stub_77774(left: f32, right: f32) -> i32 {
    // IDA 0x77774 `ChannelStream::setPan`: stereo pairs split hard
    // left/right per row (0x777bc..0x7780c), then fans out.
    STREAM_CHANNEL.set_pan(left, right)
}

// 0x7781c - __ZN4FMOD13ChannelStream16setDSPClockDelayEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::setDSPClockDelay(void)")]
pub fn stub_7781c() -> i32 {
    // IDA 0x7781c `ChannelStream::setDSPClockDelay`: fans out (0x77830..
    // 0x77860).
    STREAM_CHANNEL.set_clock_delay()
}

// 0x77868 - __ZN4FMOD13ChannelStream13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelStream *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelStream::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_77868(mix: [f32; 8]) -> i32 {
    // IDA 0x77868 `ChannelStream::setSpeakerMix`: fans the eight gains
    // out (0x77890..0x778f4).
    STREAM_CHANNEL.set_speaker_mix(mix)
}

// 0x77904 - __ZN4FMOD13ChannelStream16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelStream *this, int, float *, int)
#[doc(alias = "FMOD::ChannelStream::setSpeakerLevels(int,float *,int)")]
pub fn stub_77904(levels: Vec<f32>) -> i32 {
    // IDA 0x77904 `ChannelStream::setSpeakerLevels`: fans the matrix out
    // (0x77928..0x77964).
    STREAM_CHANNEL.set_speaker_levels(levels)
}

// 0x77970 - __ZN4FMOD13ChannelStream15set3DAttributesEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DAttributes(void)")]
pub fn stub_77970() -> i32 {
    // IDA 0x77970 `ChannelStream::set3DAttributes`: fans out (0x77984..
    // 0x779b4).
    STREAM_CHANNEL.set_3d_attributes()
}

// 0x779bc - __ZN4FMOD13ChannelStream14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setLowPassGain(float)")]
pub fn stub_779bc(gain: f32) -> i32 {
    // IDA 0x779bc `ChannelStream::setLowPassGain`: fans out (0x779d8..
    // 0x77a0c).
    STREAM_CHANNEL.set_lowpass(gain)
}

// 0x77a18 - __ZN4FMOD13ChannelStream19set3DMinMaxDistanceEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DMinMaxDistance(void)")]
pub fn stub_77a18() -> i32 {
    // IDA 0x77a18 `ChannelStream::set3DMinMaxDistance`: fans out
    // (0x77a2c..0x77a5c).
    STREAM_CHANNEL.set_3d_min_max()
}

// 0x77a64 - __ZN4FMOD13ChannelStream14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::set3DOcclusion(float,float)")]
pub fn stub_77a64(direct: f32, reverb: f32) -> i32 {
    // IDA 0x77a64 `ChannelStream::set3DOcclusion`: fans out (0x77a84..
    // 0x77abc).
    STREAM_CHANNEL.set_occlusion(direct, reverb)
}

// 0x77ac8 - __ZN4FMOD13ChannelStream19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelStream::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_77ac8(props: crate::generated_next_k::ReverbProps) -> i32 {
    // IDA 0x77ac8 `ChannelStream::setReverbProperties`: fans out
    // (0x77ae4..0x77b18).
    STREAM_CHANNEL.set_reverb(props)
}

// 0x77b24 - __ZN4FMOD13ChannelStream19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_77b24() -> (i32, crate::generated_next_k::ReverbProps) {
    // IDA 0x77b24 `ChannelStream::getReverbProperties`: empty streams
    // return 0, else the first sub-voice result (0x77b2c..0x77b3c).
    STREAM_CHANNEL.reverb_props()
}

// 0x77b48 - __ZN4FMOD13ChannelStream9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelStream *this, bool *, bool)
#[doc(alias = "FMOD::ChannelStream::isPlaying(bool *,bool)")]
pub fn stub_77b48() -> (i32, bool) {
    // IDA 0x77b48 `ChannelStream::isPlaying`: the inverse of the stopped
    // byte (0x77b48..0x77b58).
    STREAM_CHANNEL.stream_playing()
}

// 0x77b5c - __ZN4FMOD13ChannelStream11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_77b5c(len: usize) -> (i32, Vec<f32>) {
    // IDA 0x77b5c `ChannelStream::getSpectrum`: delegates to the first
    // sub-voice (sole call).
    STREAM_CHANNEL.spectrum(len)
}

// 0x77b6c - __ZN4FMOD13ChannelStream11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelStream *this, float *, int, int)
#[doc(alias = "FMOD::ChannelStream::getWaveData(float *,int,int)")]
pub fn stub_77b6c(channels: usize, frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x77b6c `ChannelStream::getWaveData`: delegates to the first
    // sub-voice (0x77b6c).
    STREAM_CHANNEL.wave_data(channels, frames)
}

// 0x77b7c - __ZN4FMOD13ChannelStream10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getDSPHead(FMOD::DSPI **)")]
pub fn stub_77b7c() -> (i32, u32) {
    // IDA 0x77b7c `ChannelStream::getDSPHead`: delegates to the first
    // sub-voice (sole call).
    STREAM_CHANNEL.dsp_head()
}

// 0x77b8c - __ZN4FMOD13ChannelStream12setLoopCountEi
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setLoopCount(int)")]
pub fn stub_77b8c(count: i32) -> i32 {
    // IDA 0x77b8c `ChannelStream::setLoopCount`: runs the real setter,
    // then mirrors into the stream (0x77b9c..0x77bbc).
    STREAM_CHANNEL.set_loop_count(count)
}

// 0x77bc0 - __ZN4FMOD13ChannelStream13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_77bc0(start: u32, len: u32) -> i32 {
    // IDA 0x77bc0 `ChannelStream::setLoopPoints`: runs the real setter,
    // then mirrors start only (0x77bd8..0x77c10).
    STREAM_CHANNEL.set_loop_points(start, len)
}

// 0x77c14 - __ZN4FMOD13ChannelStream11getPositionEPjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelStream::getPosition(unsigned int *,unsigned int)")]
pub fn stub_77c14(unit: u32) -> (i32, u32) {
    // IDA 0x77c14 `ChannelStream::getPosition`: 37 without an out-param
    // or a stream, else the unit position (0x77c30..tail).
    let _ = unit;
    STREAM_CHANNEL.position()
}

// 0x77f74 - __ZN4FMOD13ChannelStream4stopEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::stop(void)")]
pub fn stub_77f74() -> i32 {
    // IDA 0x77f74 `ChannelStream::stop`: marks the stopped byte and stops
    // the stream (0x77f8c..tail).
    STREAM_CHANNEL.stop()
}

// 0x78168 - __ZN4FMOD13ChannelStream7setModeEj
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setMode(unsigned int)")]
pub fn stub_78168(mode: u32) -> i32 {
    // IDA 0x78168 `ChannelStream::setMode`: runs the real setter plus the
    // stream, then fans the masked mode out (0x7817c..0x781e8).
    STREAM_CHANNEL.set_mode(mode)
}

// 0x781f0 - __ZN4FMOD13ChannelStreamC2Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_781f0() -> i32 {
    // IDA 0x781f0 `ChannelStream::ChannelStream`: runs the real ctor,
    // zeroes the voice list with one slot live (0x781fc..0x78260).
    STREAM_CHANNEL.construct();
    0
}

// 0x7826c - __ZN4FMOD13ChannelStreamC1Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_7826c() -> i32 {
    // IDA 0x7826c `ChannelStream::ChannelStream` thunk: tail-calls the C2
    // ctor above.
    STREAM_CHANNEL.construct();
    0
}

// 0x78270 - __ZN4FMOD13ChannelStream5allocEv
// type: int __fastcall(FMOD::ChannelStream *this, int, int)
#[doc(alias = "FMOD::ChannelStream::alloc(void)")]
pub fn stub_78270() -> i32 {
    // IDA 0x78270 `ChannelStream::alloc`: latches the stream params
    // (0x78280..tail).
    STREAM_CHANNEL.alloc()
}

// 0x78540 - __ZN4FMOD13ChannelStream13setPositionExEjjb
// type: int __fastcall(unsigned __int64 this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelStream::setPositionEx(unsigned int,unsigned int,bool)")]
pub fn stub_78540(pos: u32, unit: u32) -> i32 {
    // IDA 0x78540 `ChannelStream::setPositionEx`: seeks the stream
    // (0x78540..tail).
    STREAM_CHANNEL.set_position_ex(pos, unit)
}

// 0x78af0 - __ZN4FMOD13ChannelStream9setPausedEb
// type: int __fastcall(FMOD::ChannelStream *this, bool)
#[doc(alias = "FMOD::ChannelStream::setPaused(bool)")]
pub fn stub_78af0(paused: bool) -> i32 {
    // IDA 0x78af0 `ChannelStream::setPaused`: fans out under the lock
    // unless the stream flag is set (0x78b08..0x78b78).
    STREAM_CHANNEL.set_paused(paused)
}

// 0x78b80 - __ZN4FMOD13ChannelStream12updateStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::updateStream(void)")]
pub fn stub_78b80() -> i32 {
    // IDA 0x78b80 `ChannelStream::updateStream`: pumps the stream
    // (0x78b80..tail).
    STREAM_CHANNEL.update_stream()
}

// 0x78fac - __ZN4FMOD13ChannelStream8isStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::isStream(void)")]
pub fn stub_78fac() -> i32 {
    // IDA 0x78fac `ChannelStream::isStream`: returns 1 (0x78fb0).
    1
}

// 0x78fb4 - __ZN4FMOD13ChannelStream11setPositionEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setPosition(unsigned int,unsigned int)")]
pub fn stub_78fb4(pos: u32, unit: u32) -> i32 {
    // IDA 0x78fb4 `ChannelStream::setPosition`: dispatches into the vtable
    // `setPositionEx` (sole call, 0x78fb4).
    STREAM_CHANNEL.set_position(pos, unit)
}

// 0x78fc4 - __ZN4FMOD13ChannelStreamD0Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fc4() {
    // IDA 0x78fc4 `ChannelStream::~ChannelStream` D0: vtable reset plus
    // operator delete (0x78fd8..0x78fdc); the drop below is the delete.
    STREAM_CHANNEL.destroy();
}

// 0x78fe8 - __ZN4FMOD13ChannelStreamD1Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fe8() {
    // IDA 0x78fe8 `ChannelStream::~ChannelStream` D1: vtable reset only
    // (0x78ff4).
    STREAM_CHANNEL.destroy();
}

// 0x79000 - __ZN4FMOD12ChannelGroup9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroup *this, float, FMOD::ChannelGroupI **)
#[doc(alias = "FMOD::ChannelGroup::setVolume(float)")]
pub fn stub_79000(handle: u32, volume: f32) -> i32 {
    // IDA 0x79000 `ChannelGroup::setVolume`: validates, then sets the
    // volume (0x79014..0x79030).
    if FMOD_GROUPS.validate(handle, true) != 0 {
        return FmodGroups::INVALID_HANDLE;
    }
    match FMOD_GROUPS.with_row(handle, |row| row.volume = volume) {
        Some(()) => 0,
        None => FmodGroups::INVALID_HANDLE,
    }
}

// 0x79034 - __ZN4FMOD13ChannelGroupI8validateEPNS_12ChannelGroupEPPS0_
// type: int __fastcall(int result, int *)
#[doc(alias = "FMOD::ChannelGroupI::validate(FMOD::ChannelGroup *,FMOD::ChannelGroupI**)")]
pub fn stub_79034(handle: u32, with_out: bool) -> i32 {
    // IDA 0x79034 `ChannelGroupI::validate`: 36 on a null handle, 37
    // without an out-slot, else 0 (0x79038..0x7904c).
    FMOD_GROUPS.validate(handle, with_out)
}

// 0x79054 - __ZN4FMOD13ChannelGroupI9getPausedEPb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool *)
#[doc(alias = "FMOD::ChannelGroupI::getPaused(bool *)")]
pub fn stub_79054(handle: u32, with_out: bool) -> (i32, bool) {
    // IDA 0x79054 `ChannelGroupI::getPaused`: 37 without an out-param,
    // else the flag byte (0x79058..0x79068).
    if FMOD_GROUPS.validate(handle, with_out) != 0 {
        return (FmodGroups::NO_OUT, false);
    }
    match FMOD_GROUPS.get(handle, |row| row.paused) {
        Some(paused) => (0, paused),
        None => (FmodGroups::INVALID_HANDLE, false),
    }
}

// 0x7906c - __ZN4FMOD13ChannelGroupI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7906c(handle: u32) -> u32 {
    // IDA 0x7906c `ChannelGroupI::getMemoryUsedImpl`: 0x64 base plus the
    // name, DSP and buffer legs (0x7908c..0x790f0).
    FMOD_GROUPS.memory_used(handle)
}

// 0x790fc - __ZN4FMOD13ChannelGroupI20updateChildMixTargetEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelGroupI::updateChildMixTarget(FMOD::DSPI *)")]
pub fn stub_790fc(handle: u32) -> i32 {
    // IDA 0x790fc `ChannelGroupI::updateChildMixTarget`: rewires the mix
    // matrix (0x7911c..tail).
    let _ = handle;
    0
}

// 0x791e8 - __ZN4FMOD13ChannelGroupI7setMuteEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setMute(bool,bool)")]
pub fn stub_791e8(handle: u32, muted: bool, recursive: bool) -> i32 {
    // IDA 0x791e8 `ChannelGroupI::setMute`: latches the flag, then walks
    // the children unless told otherwise (0x791f0..tail).
    FMOD_GROUPS.set_mute(handle, muted, recursive)
}

// 0x79280 - __ZN4FMOD13ChannelGroupI9setPausedEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setPaused(bool,bool)")]
pub fn stub_79280(handle: u32, paused: bool, recursive: bool) -> i32 {
    // IDA 0x79280 `ChannelGroupI::setPaused`: same recursive walk for the
    // pause flag (0x7928c..tail).
    FMOD_GROUPS.set_paused(handle, paused, recursive)
}

// 0x79334 - __ZN4FMOD13ChannelGroupI16setPitchInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setPitchInternal(void)")]
pub fn stub_79334() -> ! {
    todo!("0x79334 FMOD::ChannelGroupI::setPitchInternal(void)")
}

// 0x793e4 - __ZN4FMOD13ChannelGroupI17setVolumeInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setVolumeInternal(void)")]
pub fn stub_793e4() -> ! {
    todo!("0x793e4 FMOD::ChannelGroupI::setVolumeInternal(void)")
}

// 0x794c4 - __ZN4FMOD13ChannelGroupI8addGroupEPS0_
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")]
pub fn stub_794c4() -> ! {
    todo!("0x794c4 FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")
}

// 0x796a4 - __ZN4FMOD13ChannelGroupI9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroupI *this, float)
#[doc(alias = "FMOD::ChannelGroupI::setVolume(float)")]
pub fn stub_796a4() -> ! {
    todo!("0x796a4 FMOD::ChannelGroupI::setVolume(float)")
}

// 0x796d4 - __ZN4FMOD13ChannelGroupI15releaseInternalEb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool)
#[doc(alias = "FMOD::ChannelGroupI::releaseInternal(bool)")]
pub fn stub_796d4() -> ! {
    todo!("0x796d4 FMOD::ChannelGroupI::releaseInternal(bool)")
}

// 0x7995c - __ZN4FMOD13ChannelGroupI7releaseEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::release(void)")]
pub fn stub_7995c() -> ! {
    todo!("0x7995c FMOD::ChannelGroupI::release(void)")
}

// 0x79980 - __ZN4FMOD20ChannelGroupSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupSoftware *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_79980() -> ! {
    todo!("0x79980 FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x79a38 - __ZN4FMOD13ChannelGroupI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a38() -> ! {
    todo!("0x79a38 FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")
}

// 0x79a90 - __ZN4FMOD20ChannelGroupSoftware13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a90() -> ! {
    todo!("0x79a90 FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")
}

// 0x79ae8 - __ZN4FMOD8ChannelI16returnToFreeListEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::returnToFreeList(void)")]
pub fn stub_79ae8() -> ! {
    todo!("0x79ae8 FMOD::ChannelI::returnToFreeList(void)")
}

// 0x79b98 - __ZN4FMOD8ChannelI14referenceStampEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::referenceStamp(bool)")]
pub fn stub_79b98() -> ! {
    todo!("0x79b98 FMOD::ChannelI::referenceStamp(bool)")
}

// 0x79bdc - __ZN4FMOD8ChannelI14getRealChannelEPPNS_11ChannelRealEPi
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelReal **, int *)
#[doc(alias = "FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")]
pub fn stub_79bdc() -> ! {
    todo!("0x79bdc FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")
}

// 0x79ca8 - __ZN4FMOD8ChannelI4initEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::init(void)")]
pub fn stub_79ca8() -> ! {
    todo!("0x79ca8 FMOD::ChannelI::init(void)")
}

// 0x79dd4 - __ZN4FMOD8ChannelIC2EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79dd4() -> ! {
    todo!("0x79dd4 FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")
}

// 0x79e84 - __ZN4FMOD8ChannelIC1EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79e84() -> ! {
    todo!("0x79e84 FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")
}

// 0x79e88 - __ZN4FMOD8ChannelIC2Ev
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79e88() -> ! {
    todo!("0x79e88 FMOD::ChannelI::ChannelI(void)")
}

// 0x79ef0 - __ZN4FMOD8ChannelIC1Ev
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79ef0() -> ! {
    todo!("0x79ef0 FMOD::ChannelI::ChannelI(void)")
}

// 0x79ef4 - __ZN4FMOD8ChannelI5allocEPNS_4DSPIEb
// type: int __fastcall(_DWORD *, int, char)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")]
pub fn stub_79ef4() -> ! {
    todo!("0x79ef4 FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")
}

// 0x7a0f8 - __ZN4FMOD8ChannelI5startEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::start(void)")]
pub fn stub_7a0f8() -> ! {
    todo!("0x7a0f8 FMOD::ChannelI::start(void)")
}

// 0x7a198 - __ZN4FMOD8ChannelI9getPausedEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getPaused(bool *)")]
pub fn stub_7a198() -> ! {
    todo!("0x7a198 FMOD::ChannelI::getPaused(bool *)")
}

// 0x7a1ec - __ZN4FMOD8ChannelI9getVolumeEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getVolume(float *)")]
pub fn stub_7a1ec() -> ! {
    todo!("0x7a1ec FMOD::ChannelI::getVolume(float *)")
}

// 0x7a214 - __ZN4FMOD8ChannelI12getFrequencyEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getFrequency(float *)")]
pub fn stub_7a214() -> ! {
    todo!("0x7a214 FMOD::ChannelI::getFrequency(float *)")
}

// 0x7a23c - __ZN4FMOD8ChannelI6setPanEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setPan(float,bool)")]
pub fn stub_7a23c() -> ! {
    todo!("0x7a23c FMOD::ChannelI::setPan(float,bool)")
}

// 0x7a358 - __ZN4FMOD8ChannelI8setDelayE14FMOD_DELAYTYPEjj
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")]
pub fn stub_7a358() -> ! {
    todo!("0x7a358 FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")
}

// 0x7a50c - __ZN4FMOD8ChannelI13setSpeakerMixEffffffffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, float, float, float, float, float, float, bool)
#[doc(alias = "FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")]
pub fn stub_7a50c() -> ! {
    todo!("0x7a50c FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")
}

// 0x7a7dc - __ZN4FMOD8ChannelI16getSpeakerLevelsE12FMOD_SPEAKERPfi
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")]
pub fn stub_7a7dc() -> ! {
    todo!("0x7a7dc FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")
}

// 0x7a8b0 - __ZN4FMOD8ChannelI7getMuteEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getMute(bool *)")]
pub fn stub_7a8b0() -> ! {
    todo!("0x7a8b0 FMOD::ChannelI::getMute(bool *)")
}

// 0x7a8d8 - __ZN4FMOD8ChannelI15set3DAttributesEPK11FMOD_VECTORS3_
// type: int __fastcall(int, float *, float *)
#[doc(alias = "FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
pub fn stub_7a8d8() -> ! {
    todo!("0x7a8d8 FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")
}

// 0x7aa4c - __ZN4FMOD8ChannelI19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_7aa4c() -> ! {
    todo!("0x7aa4c FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x7aae0 - __ZN4FMOD8ChannelI19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_7aae0() -> ! {
    todo!("0x7aae0 FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x7ab74 - __ZN4FMOD8ChannelI9isVirtualEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isVirtual(bool *)")]
pub fn stub_7ab74() -> ! {
    todo!("0x7ab74 FMOD::ChannelI::isVirtual(bool *)")
}

// 0x7aba0 - __ZN4FMOD8ChannelI21getAudibilityInternalEPfb
// type: int __fastcall(FMOD::ChannelI *this, float *, bool)
#[doc(alias = "FMOD::ChannelI::getAudibilityInternal(float *,bool)")]
pub fn stub_7aba0() -> ! {
    todo!("0x7aba0 FMOD::ChannelI::getAudibilityInternal(float *,bool)")
}

// 0x7ad00 - __ZN4FMOD8ChannelI13getAudibilityEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getAudibility(float *)")]
pub fn stub_7ad00() -> ! {
    todo!("0x7ad00 FMOD::ChannelI::getAudibility(float *)")
}

// 0x7ad08 - __ZN4FMOD8ChannelI15getCurrentSoundEPPNS_6SoundIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")]
pub fn stub_7ad08() -> ! {
    todo!("0x7ad08 FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")
}

// 0x7ad44 - __ZN4FMOD8ChannelI13getCurrentDSPEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")]
pub fn stub_7ad44() -> ! {
    todo!("0x7ad44 FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")
}

// 0x7ad70 - __ZN4FMOD8ChannelI11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
// type: int __fastcall(int result, int)
#[doc(alias = "FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
pub fn stub_7ad70() -> ! {
    todo!("0x7ad70 FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")
}

// 0x7ad88 - __ZN4FMOD8ChannelI11getPositionEPjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")]
pub fn stub_7ad88() -> ! {
    todo!("0x7ad88 FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")
}

// 0x7adb0 - __ZN4FMOD8ChannelI16updateSyncPointsEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::updateSyncPoints(bool)")]
pub fn stub_7adb0() -> ! {
    todo!("0x7adb0 FMOD::ChannelI::updateSyncPoints(bool)")
}

// 0x7b1f8 - __ZN4FMOD8ChannelI12setFrequencyEf
// type: int __fastcall(FMOD::ChannelI *this, float)
#[doc(alias = "FMOD::ChannelI::setFrequency(float)")]
pub fn stub_7b1f8() -> ! {
    todo!("0x7b1f8 FMOD::ChannelI::setFrequency(float)")
}

// 0x7b31c - __ZN4FMOD8ChannelI10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7b31c() -> ! {
    todo!("0x7b31c FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")
}

// 0x7b344 - __ZN4FMOD8ChannelI7getModeEPj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *)
#[doc(alias = "FMOD::ChannelI::getMode(unsigned int *)")]
pub fn stub_7b344() -> ! {
    todo!("0x7b344 FMOD::ChannelI::getMode(unsigned int *)")
}

// 0x7b36c - __ZN4FMOD8ChannelI12setLoopCountEi
// type: int __fastcall(FMOD::ChannelI *this, int)
#[doc(alias = "FMOD::ChannelI::setLoopCount(int)")]
pub fn stub_7b36c() -> ! {
    todo!("0x7b36c FMOD::ChannelI::setLoopCount(int)")
}

// 0x7b40c - __ZN4FMOD8ChannelI12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelI *this, int *)
#[doc(alias = "FMOD::ChannelI::getLoopCount(int *)")]
pub fn stub_7b40c() -> ! {
    todo!("0x7b40c FMOD::ChannelI::getLoopCount(int *)")
}

// 0x7b434 - __ZN4FMOD8ChannelI11setUserDataEPv
// type: int __fastcall(FMOD::ChannelI *this, void *)
#[doc(alias = "FMOD::ChannelI::setUserData(void *)")]
pub fn stub_7b434() -> ! {
    todo!("0x7b434 FMOD::ChannelI::setUserData(void *)")
}

// 0x7b440 - __ZN4FMOD8ChannelI11getUserDataEPPv
// type: int __fastcall(FMOD::ChannelI *this, void **)
#[doc(alias = "FMOD::ChannelI::getUserData(void **)")]
pub fn stub_7b440() -> ! {
    todo!("0x7b440 FMOD::ChannelI::getUserData(void **)")
}

// 0x7b458 - __ZN4FMOD8ChannelI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7b458() -> ! {
    todo!("0x7b458 FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x7b47c - __ZN4FMOD8ChannelI6addDSPEPNS_4DSPIEPPNS_14DSPConnectionIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, FMOD::DSPConnectionI **)
#[doc(alias = "FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")]
pub fn stub_7b47c() -> ! {
    todo!("0x7b47c FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")
}

// 0x7b4e8 - __ZN4FMOD8ChannelI16setSpeakerLevelsE12FMOD_SPEAKERPfib
// type: int __fastcall(int, unsigned int, int, int, char)
#[doc(alias = "FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")]
pub fn stub_7b4e8() -> ! {
    todo!("0x7b4e8 FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")
}

// 0x7b79c - __ZN4FMOD8ChannelI21calculate3DReverbGainEPNS_7ReverbIEP11FMOD_VECTORPf
// type: int __fastcall(int, int, int, __int32 *)
#[doc(alias = "FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")]
pub fn stub_7b79c() -> ! {
    todo!("0x7b79c FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")
}

// 0x7b860 - __ZN4FMOD8ChannelI5allocEPNS_6SoundIEb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::SoundI *, bool)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")]
pub fn stub_7b860() -> ! {
    todo!("0x7b860 FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")
}

// 0x7bbc4 - __ZN4FMOD8ChannelI23calcVolumeAndPitchFor3DEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")]
pub fn stub_7bbc4() -> ! {
    todo!("0x7bbc4 FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")
}

// 0x7c164 - __ZN4FMOD8ChannelI8validateEPNS_7ChannelEPPS0_
// type: int __fastcall(unsigned int, _DWORD *, FMOD::SystemI **)
#[doc(alias = "FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")]
pub fn stub_7c164() -> ! {
    todo!("0x7c164 FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")
}

// 0x7c224 - __ZN4FMOD8ChannelI9isPlayingEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isPlaying(bool *)")]
pub fn stub_7c224() -> ! {
    todo!("0x7c224 FMOD::ChannelI::isPlaying(bool *)")
}

// 0x7c3d8 - __ZN4FMOD8ChannelI13getLoopPointsEPjjS1_j
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")]
pub fn stub_7c3d8() -> ! {
    todo!("0x7c3d8 FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")
}

// 0x7c784 - __ZN4FMOD8ChannelI14getChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(FMOD::ChannelI *, int)
#[doc(alias = "FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7c784() -> ! {
    todo!("0x7c784 FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")
}

// 0x7c83c - __ZN4FMOD8ChannelI11setPositionEjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPosition(unsigned int,unsigned int)")]
pub fn stub_7c83c() -> ! {
    todo!("0x7c83c FMOD::ChannelI::setPosition(unsigned int,unsigned int)")
}

// 0x7ce58 - __ZN4FMOD8ChannelI13setLoopPointsEjjjj
// type: int __fastcall(unsigned __int64 this, unsigned int, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_7ce58() -> ! {
    todo!("0x7ce58 FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0x7d208 - __ZN4FMOD8ChannelI14setChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7d208() -> ! {
    todo!("0x7d208 FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")
}
