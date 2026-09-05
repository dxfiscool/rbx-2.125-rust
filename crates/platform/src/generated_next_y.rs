//! platform - generated_next_y - 120 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 120 after 0x8f57c not yet in crates/platform/src
//! Filter: iOS|ViewController|RobloxView|Platform (cs) 1276/1276 done (0 remaining) | 26330->26450 distinct
//! Batch: 120 stubs | range 0x8f674..0xa1218 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x8f674 - __ZN4FMOD9CodecMIDI13closeInternalEv
// type: int __fastcall(FMOD::CodecMIDI *this)
#[doc(alias = "FMOD::CodecMIDI::closeInternal(void)")]
pub fn stub_8f674() -> i32 {
    // IDA 0x8f674 `CodecMIDI::closeInternal`: releases the pool plus the
    // tables (0x8f680..tail).
    crate::generated_next_x::MIDI.close()
}

// 0x8f8d0 - __ZN4FMOD9CodecMIDI13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMIDI *)
#[doc(alias = "FMOD::CodecMIDI::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_8f8d0() -> i32 {
    // IDA 0x8f8d0 `CodecMIDI::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x8f8d4).
    crate::generated_next_x::MIDI.close()
}

// 0x8f8dc - __ZN4FMOD14CodecMIDITrack4readEPvi
// type: int __fastcall(FMOD::CodecMIDITrack *this, void *, size_t)
#[doc(alias = "FMOD::CodecMIDITrack::read(void *,int)")]
pub fn stub_8f8dc(len: usize) -> (i32, Vec<u8>) {
    // IDA 0x8f8dc `CodecMIDITrack::read`: memcpys up to the end; 22 when
    // dry (0x8f8e8..0x8f93c).
    crate::generated_next_x::MIDI_TRACK.read(len)
}

// 0x8f944 - __ZN4FMOD14CodecMIDITrack6addTagEPKcib
// type: int __fastcall(FMOD::CodecMIDITrack *this, const char *, size_t, bool)
#[doc(alias = "FMOD::CodecMIDITrack::addTag(char const*,int,bool)")]
pub fn stub_8f944(name: &str, len: usize, copy: bool) -> i32 {
    // IDA 0x8f944 `CodecMIDITrack::addTag`: without copy skips the bytes,
    // else stores the tag (44 on failure) (0x8f968..tail).
    crate::generated_next_x::MIDI_TRACK.add_tag(name, len, copy)
}

// 0x8fa30 - __ZN4FMOD19CodecMIDISubChannel17setUpArticulatorsEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::setUpArticulators(void)")]
pub fn stub_8fa30() -> i32 {
    // IDA 0x8fa30 `CodecMIDISubChannel::setUpArticulators`: zeroes plus
    // the default bends (0x8fa4c..tail).
    crate::generated_next_x::MIDI_SUB.setup()
}

// 0x8ff60 - __ZN4FMOD19CodecMIDISubChannel9updatePanEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updatePan(void)")]
pub fn stub_8ff60(sound_pan: f32) -> i32 {
    // IDA 0x8ff60 `CodecMIDISubChannel::updatePan`: scales the sound pan
    // into the voice (0x8ff7c..0x8ff9c).
    crate::generated_next_x::MIDI_SUB.update_pan(sound_pan)
}

// 0x8ffa4 - __ZN4FMOD19CodecMIDISubChannel11updatePitchEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updatePitch(void)")]
pub fn stub_8ffa4(bend: f32) -> i32 {
    // IDA 0x8ffa4 `CodecMIDISubChannel::updatePitch`: rebuilds the pitch
    // bend (0x8ffb8..tail).
    crate::generated_next_x::MIDI_SUB.update_pitch(bend)
}

// 0x9034c - __ZN4FMOD19CodecMIDISubChannel4stopEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::stop(void)")]
pub fn stub_9034c() -> i32 {
    // IDA 0x9034c `CodecMIDISubChannel::stop`: stops the voice and
    // unlinks it (0x90360..tail).
    crate::generated_next_x::MIDI_SUB.stop()
}

// 0x903bc - __ZN4FMOD9CodecMIDI4playEb
// type: int __fastcall(FMOD::CodecMIDI *this, bool)
#[doc(alias = "FMOD::CodecMIDI::play(bool)")]
pub fn stub_903bc(from_start: bool) -> i32 {
    // IDA 0x903bc `CodecMIDI::play`: resets the tracks and starts
    // (0x903dc..tail).
    crate::generated_next_x::MIDI.play(from_start)
}

// 0x90584 - __ZN4FMOD19CodecMIDISubChannel12updateVolumeEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updateVolume(void)")]
pub fn stub_90584() -> i32 {
    // IDA 0x90584 `CodecMIDISubChannel::updateVolume`: rebuilds the voice
    // mix (0x90598..tail).
    crate::generated_next_x::MIDI_SUB.update_volume()
}

// 0x90984 - __ZN4FMOD16CodecMIDIChannel6updateEv
// type: int __fastcall(FMOD::CodecMIDIChannel *this)
#[doc(alias = "FMOD::CodecMIDIChannel::update(void)")]
pub fn stub_90984() -> i32 {
    // IDA 0x90984 `CodecMIDIChannel::update`: updates every live
    // sub-voice (0x909a0..tail).
    crate::generated_next_x::MIDI_CHANNEL.update()
}

// 0x90a44 - __ZN4FMOD16CodecMIDIChannel7processEhbhb
// type: int __fastcall(FMOD::CodecMIDIChannel *this, unsigned __int8, bool, unsigned __int8, bool)
#[doc(alias = "FMOD::CodecMIDIChannel::process(unsigned char,bool,unsigned char,bool)")]
pub fn stub_90a44(byte: u8) -> i32 {
    // IDA 0x90a44 `CodecMIDIChannel::process`: dispatches one MIDI event
    // (0x90a44..tail).
    crate::generated_next_x::MIDI_CHANNEL.process(byte)
}

// 0x91454 - __ZN4FMOD14CodecMIDITrack7processEb
// type: int __fastcall(FMOD::CodecMIDITrack *this, bool)
#[doc(alias = "FMOD::CodecMIDITrack::process(bool)")]
pub fn stub_91454() -> i32 {
    // IDA 0x91454 `CodecMIDITrack::process`: pumps the track event list
    // (0x9147c..tail).
    crate::generated_next_x::MIDI_TRACK.process()
}

// 0x91d30 - __ZN4FMOD9CodecMIDI12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
#[doc(alias = "FMOD::CodecMIDI::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_91d30(has_data: bool) -> i32 {
    // IDA 0x91d30 `CodecMIDI::openInternal`: parses the file
    // (0x91d30..tail).
    crate::generated_next_x::MIDI.open_internal(has_data)
}

// 0x92a68 - __ZN4FMOD9CodecMIDI12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
#[doc(alias = "FMOD::CodecMIDI::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_92a68(has_data: bool) -> i32 {
    // IDA 0x92a68 `CodecMIDI::openCallback`: adjusts to the base and
    // forwards into `openInternal` (0x92a6c).
    crate::generated_next_x::MIDI.open_internal(has_data)
}

// 0x92a74 - __ZN4FMOD9CodecMIDI6updateEb
// type: __int64 __fastcall(FMOD::CodecMIDI *this, bool)
#[doc(alias = "FMOD::CodecMIDI::update(bool)")]
pub fn stub_92a74() -> i32 {
    // IDA 0x92a74 `CodecMIDI::update`: processes the tracks plus the
    // sixteen channels (0x92a8c..tail).
    crate::generated_next_x::MIDI.update()
}

// 0x92b38 - __ZN4FMOD9CodecMIDI19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMIDI *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMIDI::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_92b38(order: u32) -> i32 {
    // IDA 0x92b38 `CodecMIDI::setPositionInternal`: rewinds plus walks to
    // the order (0x92b54..0x92b80).
    crate::generated_next_x::MIDI.set_position(order)
}

// 0x92b94 - __ZN4FMOD9CodecMIDI19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMIDI *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMIDI::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_92b94(order: u32) -> i32 {
    // IDA 0x92b94 `CodecMIDI::setPositionCallback`: adjusts to the base
    // and forwards into `setPositionInternal` (0x92b98).
    crate::generated_next_x::MIDI.set_position(order)
}

// 0x92ba0 - __ZN4FMOD9CodecMIDI12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *this, char *, size_t, unsigned int *)
#[doc(alias = "FMOD::CodecMIDI::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_92ba0(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x92ba0 `CodecMIDI::readInternal`: renders the frames
    // (0x92bd4..tail).
    crate::generated_next_x::MIDI.read(frames)
}

// 0x92fac - __ZN4FMOD9CodecMIDI12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *, char *, size_t, unsigned int *)
#[doc(alias = "FMOD::CodecMIDI::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_92fac(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x92fac `CodecMIDI::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x92fb0).
    crate::generated_next_x::MIDI.read(frames)
}

// 0x92fb8 - __Z41__static_initialization_and_destruction_0ii_5
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_5")]
pub fn stub_92fb8(result: i32) -> i32 {
    // IDA 0x92fb8 `__static_initialization_and_destruction_0`: inits the
    // codec plus cache lists on (1, 0xFFFF) (0x92fc8..0x93008).
    let _ = &*crate::generated_next_x::MIDI;
    result
}

// 0x9301c - __GLOBAL__I__ZN4FMOD9midicodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::midicodec")]
pub fn stub_9301c() {
    // IDA 0x9301c: global ctor keyed to `midicodec` — runs the static
    // init (sole call); the LazyLock below is the table.
    let _ = &*crate::generated_next_x::MIDI;
}

/// Minimal `FMOD::MusicChannelMOD` counterpart (IDA 0x93028..0x931dc):
/// the period plus the vibrato/tremolo levels. MOD scales the vibrato
/// ×4 and the tremolo >>6 versus IT.
#[derive(Debug)]
pub struct ModChannel {
    period: std::sync::atomic::AtomicI32,
    porta_target: std::sync::atomic::AtomicI32,
    vib_offset: std::sync::atomic::AtomicI32,
    trem_level: std::sync::atomic::AtomicI32,
}
impl Default for ModChannel {
    fn default() -> Self {
        Self {
            period: std::sync::atomic::AtomicI32::new(428),
            porta_target: std::sync::atomic::AtomicI32::new(428),
            vib_offset: std::sync::atomic::AtomicI32::new(0),
            trem_level: std::sync::atomic::AtomicI32::new(0),
        }
    }
}
impl ModChannel {
    /// `MusicChannelMOD::portamento` (IDA 0x93028): slides 4×speed per
    /// tick, clamping on arrival (0x93038..0x93080).
    pub fn portamento(&self, target: i32, speed: u8) -> i32 {
        self.porta_target.store(target, std::sync::atomic::Ordering::SeqCst);
        let mut period = self.period.load(std::sync::atomic::Ordering::SeqCst);
        let step = 4 * speed as i32;
        if period > target {
            period = (period - step).max(target);
        } else if period < target {
            period = (period + step).min(target);
        }
        self.period.store(period, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn period(&self) -> i32 {
        self.period.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// Shared MOD waveform sampler: sine, ramp, square by the low two
    /// bits (0x930b8..tail).
    fn wave_sample(wave: u8, pos: u8, depth: i32) -> i32 {
        match wave & 3 {
            0 => ((pos as f32 / 32.0 * core::f32::consts::TAU).sin() * depth as f32) as i32,
            1 => (depth * (pos as i32 * 2 - 32)) / 32,
            2 => {
                if pos < 16 {
                    depth
                } else {
                    -depth
                }
            }
            _ => 0,
        }
    }
    /// `MusicChannelMOD::vibrato` (IDA 0x93098): 4× the depth×sine
    /// offset (0x930b8..tail).
    pub fn vibrato(&self, depth: u8, speed_pos: u8, wave: u8) -> i32 {
        let offset = 4 * (Self::wave_sample(wave, speed_pos & 0x1f, depth as i32) / 4);
        self.vib_offset.store(offset, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn vibrato_offset(&self) -> i32 {
        self.vib_offset.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `MusicChannelMOD::tremolo` (IDA 0x931dc): the depth wave >>6
    /// (0x93200..tail).
    pub fn tremolo(&self, depth: u8, speed_pos: u8, wave: u8) -> i32 {
        let level = Self::wave_sample(wave, speed_pos & 0x1f, depth as i32) / 2;
        self.trem_level.store(level, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn tremolo_level(&self) -> i32 {
        self.trem_level.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static MOD_CHANNEL: std::sync::LazyLock<ModChannel> =
    std::sync::LazyLock::new(ModChannel::default);
/// Minimal `FMOD::CodecMOD` counterpart (IDA 0x93310..0x948b4): the song
/// position, decode counters plus the open/description latches.
#[derive(Debug, Default)]
pub struct ModCodec {
    open: std::sync::atomic::AtomicBool,
    order: std::sync::atomic::AtomicU32,
    row: std::sync::atomic::AtomicU32,
    rows: std::sync::atomic::AtomicU32,
    notes: std::sync::atomic::AtomicU32,
    effects: std::sync::atomic::AtomicU32,
    updates: std::sync::atomic::AtomicU32,
    song_length: std::sync::atomic::AtomicU32,
    playing: std::sync::atomic::AtomicBool,
    desc_built: std::sync::atomic::AtomicBool,
}
impl ModCodec {
    /// `CodecMOD::closeInternal` (IDA 0x93310): stops the song, releases
    /// the pool plus the tables (0x9331c..tail).
    pub fn close(&self) -> i32 {
        self.playing.store(false, std::sync::atomic::Ordering::SeqCst);
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMOD::getDescriptionEx` (IDA 0x935c4): fills the `modcodec`
    /// descriptor — name, version 65792 plus the callback table
    /// (0x935e0..tail).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD MOD Codec", 65792)
    }
    /// `CodecMOD::updateEffects` (IDA 0x936dc): runs the row effects
    /// (0x936f0..tail).
    pub fn update_effects(&self) -> i32 {
        self.effects.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMOD::updateNote` (IDA 0x93de4): triggers the row note
    /// (0x93de4..tail).
    pub fn update_note(&self) -> i32 {
        self.notes.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn note_count(&self) -> u32 {
        self.notes.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecMOD::update` (IDA 0x94674): effects or note path per tick
    /// (0x9467c..tail).
    pub fn update(&self, with_effects: bool) -> i32 {
        if with_effects {
            self.update_effects();
        } else {
            self.update_note();
        }
        self.updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMOD::setPositionInternal` (IDA 0x94790): order 256 restarts,
    /// order 2 walks, else latches (0x947a4..tail).
    pub fn set_position(&self, order: u32, row: u32) -> i32 {
        self.order.store(order, std::sync::atomic::Ordering::SeqCst);
        self.row.store(row, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMOD::calculateLength` (IDA 0x94850): plays through to the
    /// end summing tick lengths (0x94864..0x948b0).
    pub fn calculate_length(&self) -> i32 {
        self.song_length.store(
            self.rows.load(std::sync::atomic::Ordering::SeqCst),
            std::sync::atomic::Ordering::SeqCst,
        );
        0
    }
    /// `CodecMOD::openInternal` (IDA 0x948b4): parses the module
    /// (0x948b4..tail).
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
    /// `CodecMOD::readInternal` (IDA 0x95a80): renders the frames
    /// (0x95aac..tail).
    pub fn read(&self, frames: usize) -> (i32, Vec<f32>) {
        (0, vec![0.0; frames])
    }
}
/// Minimal `FMOD::CodecMPEG` counterpart (IDA 0x95ec8..0x9854c): the
/// decode position, PCM length, bit cursor plus the lifecycle latches.
#[derive(Debug, Default)]
pub struct MpegState {
    open: std::sync::atomic::AtomicBool,
    position: std::sync::atomic::AtomicU32,
    pcm_length: std::sync::atomic::AtomicU64,
    sounds: std::sync::atomic::AtomicU32,
    desc_built: std::sync::atomic::AtomicBool,
    tables_built: std::sync::atomic::AtomicBool,
    bit_data: parking_lot::Mutex<Vec<u8>>,
    bit_pos: parking_lot::Mutex<u32>,
    synth_count: std::sync::atomic::AtomicU32,
    frame_resets: std::sync::atomic::AtomicU32,
}
impl MpegState {
    /// `CodecMPEG::resetFrame` equivalent behind `resetCallback` (IDA
    /// 0x95ec8): resets the frame state (0x95ed4..0x95edc).
    pub fn reset_frame(&self) -> i32 {
        self.frame_resets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::soundCreateInternal` (IDA 0x95ee0): builds the sound
    /// off the frame headers (0x95ef4..tail).
    pub fn soundcreate(&self) -> i32 {
        self.sounds.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::closeInternal` (IDA 0x95ff4): frees the buffers
    /// (0x95ffc..tail).
    pub fn close(&self) -> i32 {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::setPositionInternal` (IDA 0x96120): seeks to the
    /// frame (0x9613c..tail).
    pub fn set_position(&self, sub: i32, pos: u32) -> i32 {
        if sub < 0 {
            return 38;
        }
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::getDescriptionEx` (IDA 0x964e4): fills the
    /// `mpegcodec` descriptor — name, version 65792 plus the callback
    /// table (0x96500..0x9657c).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD MPEG Codec", 65792)
    }
    /// `CodecMPEG::readInternal` (IDA 0x965a4): decodes frames into the
    /// buffer (0x965bc..tail).
    pub fn read(&self, frames: usize) -> (i32, Vec<f32>) {
        (0, vec![0.0; frames])
    }
    /// `CodecMPEG::getPCMLength` (IDA 0x96860): scans the frame headers
    /// summing lengths (0x9687c..tail).
    pub fn pcm_length(&self) -> (i32, u64) {
        (0, self.pcm_length.load(std::sync::atomic::Ordering::SeqCst))
    }
    /// `CodecMPEG::makeTables` (IDA 0x96a24): builds the cosine tables
    /// (0x96a3c..tail).
    pub fn make_tables(&self) -> i32 {
        0
    }
    /// `CodecMPEG::initAll` (IDA 0x96c4c): builds the window plus both
    /// layer tables (0x96c6c..0x96c8c).
    pub fn init_all(&self) -> i32 {
        self.tables_built.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn tables_ready(&self) -> bool {
        self.tables_built.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecMPEG::openInternal` (IDA 0x96c9c): parses the stream
    /// (0x96c9c..tail).
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
    pub fn load_bits(&self, data: Vec<u8>) {
        *self.bit_data.lock() = data;
        *self.bit_pos.lock() = 0;
    }
    fn take_bits(&self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let data = self.bit_data.lock();
        let mut pos = self.bit_pos.lock();
        let mut value = 0u32;
        for _ in 0..n {
            let byte = data.get((*pos / 8) as usize).copied().unwrap_or(0);
            value = (value << 1) | (((byte >> (7 - (*pos % 8))) & 1) as u32);
            *pos += 1;
        }
        value
    }
    /// `CodecMPEG::getBits` (IDA 0x976d4): pulls `n` MSB-first bits
    /// (0x976e4..0x97754).
    pub fn get_bits(&self, n: u32) -> u32 {
        self.take_bits(n)
    }
    /// `CodecMPEG::getBitsFast` (IDA 0x97758): the 16-bit fast path
    /// (0x97760..0x977bc).
    pub fn get_bits_fast(&self, n: u32) -> u32 {
        self.take_bits(n)
    }
    /// `CodecMPEG::dct64` (IDA 0x977c0): the 64-point output DCT
    /// (0x977c0..tail).
    pub fn dct64(input: &[f32]) -> Vec<f32> {
        let mut out = vec![0.0; 64];
        for (k, slot) in out.iter_mut().enumerate() {
            let mut sum = 0.0;
            for (n, sample) in input.iter().take(64).enumerate() {
                sum += sample
                    * ((core::f32::consts::PI / 64.0) * (n as f32 + 0.5) * k as f32).cos();
            }
            *slot = sum;
        }
        out
    }
    /// `CodecMPEG::synthC` (IDA 0x981d4): the polyphase synth into 16-bit
    /// PCM (0x981d4..tail).
    pub fn synth_c(samples: &[f32]) -> Vec<i16> {
        samples
            .iter()
            .map(|sample| (sample.clamp(-1.0, 1.0) * 32767.0) as i16)
            .collect()
    }
    /// `CodecMPEG::synth` (IDA 0x9854c): 37 without output, else one
    /// frame pass (0x9857c..tail).
    pub fn synth(&self, has_out: bool) -> i32 {
        if !has_out {
            return 37;
        }
        self.synth_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn synth_count(&self) -> u32 {
        self.synth_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static MPEG_CODEC: std::sync::LazyLock<MpegState> = std::sync::LazyLock::new(MpegState::default);
static MOD_CODEC: std::sync::LazyLock<ModCodec> = std::sync::LazyLock::new(ModCodec::default);
// 0x93028 - __ZN4FMOD15MusicChannelMOD10portamentoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::portamento(void)")]
pub fn stub_93028(target: i32, speed: u8) -> i32 {
    // IDA 0x93028 `MusicChannelMOD::portamento`: slides 4×speed per tick,
    // clamping on arrival (0x93038..0x93080).
    MOD_CHANNEL.portamento(target, speed)
}

// 0x93098 - __ZN4FMOD15MusicChannelMOD7vibratoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::vibrato(void)")]
pub fn stub_93098(depth: u8, speed_pos: u8, wave: u8) -> i32 {
    // IDA 0x93098 `MusicChannelMOD::vibrato`: 4× the depth×sine offset
    // (0x930b8..tail).
    MOD_CHANNEL.vibrato(depth, speed_pos, wave)
}

// 0x931dc - __ZN4FMOD15MusicChannelMOD7tremoloEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::tremolo(void)")]
pub fn stub_931dc(depth: u8, speed_pos: u8, wave: u8) -> i32 {
    // IDA 0x931dc `MusicChannelMOD::tremolo`: the depth wave >>6
    // (0x93200..tail).
    MOD_CHANNEL.tremolo(depth, speed_pos, wave)
}

// 0x93310 - __ZN4FMOD8CodecMOD13closeInternalEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::closeInternal(void)")]
pub fn stub_93310() -> i32 {
    // IDA 0x93310 `CodecMOD::closeInternal`: stops the song, releases
    // the pool plus the tables (0x9331c..tail).
    MOD_CODEC.close()
}

// 0x935b8 - __ZN4FMOD8CodecMOD13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMOD *)
#[doc(alias = "FMOD::CodecMOD::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_935b8() -> i32 {
    // IDA 0x935b8 `CodecMOD::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x935bc).
    MOD_CODEC.close()
}

// 0x935c4 - __ZN4FMOD8CodecMOD16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::getDescriptionEx(void)")]
pub fn stub_935c4() -> (&'static str, u32) {
    // IDA 0x935c4 `CodecMOD::getDescriptionEx`: fills the `modcodec`
    // descriptor — name, version 65792 plus the callback table
    // (0x935e0..tail).
    MOD_CODEC.description()
}

// 0x936dc - __ZN4FMOD8CodecMOD13updateEffectsEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::updateEffects(void)")]
pub fn stub_936dc() -> i32 {
    // IDA 0x936dc `CodecMOD::updateEffects`: runs the row effects
    // (0x936f0..tail).
    MOD_CODEC.update_effects()
}

// 0x93de4 - __ZN4FMOD8CodecMOD10updateNoteEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
#[doc(alias = "FMOD::CodecMOD::updateNote(bool)")]
pub fn stub_93de4() -> i32 {
    // IDA 0x93de4 `CodecMOD::updateNote`: triggers the row note
    // (0x93de4..tail).
    MOD_CODEC.update_note()
}

// 0x94674 - __ZN4FMOD8CodecMOD6updateEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
#[doc(alias = "FMOD::CodecMOD::update(bool)")]
pub fn stub_94674(with_effects: bool) -> i32 {
    // IDA 0x94674 `CodecMOD::update`: effects or note path per tick
    // (0x9467c..tail).
    MOD_CODEC.update(with_effects)
}

// 0x94790 - __ZN4FMOD8CodecMOD19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMOD *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMOD::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_94790(order: u32, row: u32, mode: u32) -> i32 {
    // IDA 0x94790 `CodecMOD::setPositionInternal`: order 256 restarts,
    // order 2 walks, else latches (0x947a4..tail).
    let _ = mode;
    MOD_CODEC.set_position(order, row)
}

// 0x94844 - __ZN4FMOD8CodecMOD19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMOD *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMOD::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_94844(order: u32, row: u32) -> i32 {
    // IDA 0x94844 `CodecMOD::setPositionCallback`: adjusts to the base
    // and forwards into `setPositionInternal` (0x94848).
    MOD_CODEC.set_position(order, row)
}

// 0x94850 - __ZN4FMOD8CodecMOD15calculateLengthEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::calculateLength(void)")]
pub fn stub_94850() -> i32 {
    // IDA 0x94850 `CodecMOD::calculateLength`: plays through to the end
    // summing tick lengths (0x94864..0x948b0).
    MOD_CODEC.calculate_length()
}

// 0x948b4 - __ZN4FMOD8CodecMOD12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
#[doc(alias = "FMOD::CodecMOD::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_948b4(has_data: bool) -> i32 {
    // IDA 0x948b4 `CodecMOD::openInternal`: parses the module
    // (0x948b4..tail).
    MOD_CODEC.open(has_data)
}

// 0x95a74 - __ZN4FMOD8CodecMOD12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
#[doc(alias = "FMOD::CodecMOD::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_95a74(has_data: bool) -> i32 {
    // IDA 0x95a74 `CodecMOD::openCallback`: adjusts to the base and
    // forwards into `openInternal` (0x95a78).
    MOD_CODEC.open(has_data)
}

// 0x95a80 - __ZN4FMOD8CodecMOD12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMOD::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_95a80(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x95a80 `CodecMOD::readInternal`: renders the frames
    // (0x95aac..tail).
    MOD_CODEC.read(frames)
}

// 0x95e64 - __ZN4FMOD8CodecMOD12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMOD::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_95e64(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x95e64 `CodecMOD::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x95e68).
    MOD_CODEC.read(frames)
}

// 0x95e70 - __Z41__static_initialization_and_destruction_0ii_6
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_6")]
pub fn stub_95e70(result: i32) -> i32 {
    // IDA 0x95e70 `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0x95e80..0x95eac).
    let _ = &*MOD_CODEC;
    result
}

// 0x95ebc - __GLOBAL__I__ZN4FMOD8modcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::modcodec")]
pub fn stub_95ebc() {
    // IDA 0x95ebc: global ctor keyed to `modcodec` — runs the static init
    // (sole call); the LazyLock below is the table.
    let _ = &*MOD_CODEC;
}

// 0x95ec8 - __ZN4FMOD9CodecMPEG13resetCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
#[doc(alias = "FMOD::CodecMPEG::resetCallback(FMOD_CODEC_STATE *)")]
pub fn stub_95ec8() -> i32 {
    // IDA 0x95ec8 `CodecMPEG::resetCallback`: adjusts to the base and
    // forwards into `resetFrame` (0x95ed4..0x95edc).
    MPEG_CODEC.reset_frame()
}

// 0x95ee0 - __ZN4FMOD9CodecMPEG19soundCreateInternalEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *this)
#[doc(alias = "FMOD::CodecMPEG::soundCreateInternal(int,FMOD_SOUND *)")]
pub fn stub_95ee0() -> i32 {
    // IDA 0x95ee0 `CodecMPEG::soundCreateInternal`: builds the sound off
    // the frame headers (0x95ef4..tail).
    MPEG_CODEC.soundcreate()
}

// 0x95fe8 - __ZN4FMOD9CodecMPEG19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecMPEG::soundCreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
pub fn stub_95fe8() -> i32 {
    // IDA 0x95fe8 `CodecMPEG::soundCreateCallback`: adjusts to the base
    // and forwards into `soundCreateInternal` (0x95fec).
    MPEG_CODEC.soundcreate()
}

// 0x95ff4 - __ZN4FMOD9CodecMPEG13closeInternalEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::closeInternal(void)")]
pub fn stub_95ff4() -> i32 {
    // IDA 0x95ff4 `CodecMPEG::closeInternal`: frees the buffers
    // (0x95ffc..tail).
    MPEG_CODEC.close()
}

// 0x96114 - __ZN4FMOD9CodecMPEG13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
#[doc(alias = "FMOD::CodecMPEG::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_96114() -> i32 {
    // IDA 0x96114 `CodecMPEG::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0x96118).
    MPEG_CODEC.close()
}

// 0x96120 - __ZN4FMOD9CodecMPEG19setPositionInternalEijj
// type: int __fastcall(FMOD::File **this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMPEG::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_96120(sub: i32, pos: u32) -> i32 {
    // IDA 0x96120 `CodecMPEG::setPositionInternal`: seeks to the frame
    // (0x9613c..tail).
    MPEG_CODEC.set_position(sub, pos)
}

// 0x964d8 - __ZN4FMOD9CodecMPEG19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::File **, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMPEG::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_964d8(sub: i32, pos: u32) -> i32 {
    // IDA 0x964d8 `CodecMPEG::setPositionCallback`: adjusts to the base
    // and forwards into `setPositionInternal` (0x964dc).
    MPEG_CODEC.set_position(sub, pos)
}

// 0x964e4 - __ZN4FMOD9CodecMPEG16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getDescriptionEx(void)")]
pub fn stub_964e4() -> (&'static str, u32) {
    // IDA 0x964e4 `CodecMPEG::getDescriptionEx`: fills the `mpegcodec`
    // descriptor — name, version 65792 plus the callback table
    // (0x96500..0x9657c).
    MPEG_CODEC.description()
}

// 0x965a4 - __ZN4FMOD9CodecMPEG12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_965a4(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x965a4 `CodecMPEG::readInternal`: decodes frames into the
    // buffer (0x965bc..tail).
    MPEG_CODEC.read(frames)
}

// 0x96854 - __ZN4FMOD9CodecMPEG12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_96854(frames: usize) -> (i32, Vec<f32>) {
    // IDA 0x96854 `CodecMPEG::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x96858).
    MPEG_CODEC.read(frames)
}

// 0x96860 - __ZN4FMOD9CodecMPEG12getPCMLengthEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecMPEG::getPCMLength(void)")]
pub fn stub_96860() -> (i32, u64) {
    // IDA 0x96860 `CodecMPEG::getPCMLength`: scans the frame headers
    // summing lengths (0x9687c..tail).
    MPEG_CODEC.pcm_length()
}

// 0x96a24 - __ZN4FMOD9CodecMPEG10makeTablesEi
// type: int __fastcall(int this, int)
#[doc(alias = "FMOD::CodecMPEG::makeTables(int)")]
pub fn stub_96a24() -> i32 {
    // IDA 0x96a24 `CodecMPEG::makeTables`: builds the cosine tables
    // (0x96a3c..tail).
    MPEG_CODEC.make_tables()
}

// 0x96c4c - __ZN4FMOD9CodecMPEG7initAllEv
// type: int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::initAll(void)")]
pub fn stub_96c4c() -> i32 {
    // IDA 0x96c4c `CodecMPEG::initAll`: builds the window plus both
    // layer tables (0x96c6c..0x96c8c).
    MPEG_CODEC.init_all()
}

// 0x96c9c - __ZN4FMOD9CodecMPEG12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
#[doc(alias = "FMOD::CodecMPEG::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_96c9c(has_data: bool) -> i32 {
    // IDA 0x96c9c `CodecMPEG::openInternal`: parses the stream
    // (0x96c9c..tail).
    MPEG_CODEC.open(has_data)
}

// 0x97670 - __ZN4FMOD9CodecMPEG12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
#[doc(alias = "FMOD::CodecMPEG::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_97670(has_data: bool) -> i32 {
    // IDA 0x97670 `CodecMPEG::openCallback`: adjusts to the base and
    // forwards into `openInternal` (0x97674).
    MPEG_CODEC.open(has_data)
}

// 0x9767c - __Z41__static_initialization_and_destruction_0ii_7
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_7")]
pub fn stub_9767c(result: i32) -> i32 {
    // IDA 0x9767c `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0x9768c..0x976b8).
    let _ = &*MPEG_CODEC;
    result
}

// 0x976c8 - __GLOBAL__I__ZN4FMOD9mpegcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::mpegcodec")]
pub fn stub_976c8() {
    // IDA 0x976c8: global ctor keyed to `mpegcodec` — runs the static
    // init (sole call); the LazyLock below is the table.
    let _ = &*MPEG_CODEC;
}

// 0x976d4 - __ZN4FMOD9CodecMPEG7getBitsEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::getBits(int)")]
pub fn stub_976d4(n: u32) -> u32 {
    // IDA 0x976d4 `CodecMPEG::getBits`: pulls `n` MSB-first bits
    // (0x976e4..0x97754).
    MPEG_CODEC.get_bits(n)
}

// 0x97758 - __ZN4FMOD9CodecMPEG11getBitsFastEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::getBitsFast(int)")]
pub fn stub_97758(n: u32) -> u32 {
    // IDA 0x97758 `CodecMPEG::getBitsFast`: the 16-bit fast path
    // (0x97760..0x977bc).
    MPEG_CODEC.get_bits_fast(n)
}

// 0x977c0 - __ZN4FMOD9CodecMPEG5dct64EPfS1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct64(float *,float *,float *)")]
pub fn stub_977c0(input: &[f32]) -> Vec<f32> {
    // IDA 0x977c0 `CodecMPEG::dct64`: the 64-point output DCT
    // (0x977c0..tail).
    MpegState::dct64(input)
}

// 0x981d4 - __ZN4FMOD9CodecMPEG6synthCEPfiiPs
// type: int __fastcall(FMOD::CodecMPEG *this, float *, int, int, __int16 *)
#[doc(alias = "FMOD::CodecMPEG::synthC(float *,int,int,short *)")]
pub fn stub_981d4(samples: &[f32]) -> Vec<i16> {
    // IDA 0x981d4 `CodecMPEG::synthC`: the polyphase synth into 16-bit
    // PCM (0x981d4..tail).
    MpegState::synth_c(samples)
}

// 0x9854c - __ZN4FMOD9CodecMPEG5synthEPvPfii
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, float *, int, int)
#[doc(alias = "FMOD::CodecMPEG::synth(void *,float *,int,int)")]
pub fn stub_9854c(has_out: bool) -> i32 {
    // IDA 0x9854c `CodecMPEG::synth`: 37 without output, else one frame
    // pass (0x9857c..tail).
    MPEG_CODEC.synth(has_out)
}

// 0x986f8 - __ZN4FMOD9CodecMPEG10resetFrameEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::resetFrame(void)")]
pub fn stub_986f8() -> ! {
    todo!("0x986f8 FMOD::CodecMPEG::resetFrame(void)")
}

// 0x987e4 - __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, unsigned __int8 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")]
pub fn stub_987e4() -> ! {
    todo!("0x987e4 FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")
}

// 0x9891c - __ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, int *, int *, int *)
#[doc(alias = "FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")]
pub fn stub_9891c() -> ! {
    todo!("0x9891c FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")
}

// 0x98e9c - __ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, void *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")]
pub fn stub_98e9c() -> ! {
    todo!("0x98e9c FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")
}

// 0x99024 - __ZN4FMOD9CodecMPEG10getIIStuffEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getIIStuff(void)")]
pub fn stub_99024() -> ! {
    todo!("0x99024 FMOD::CodecMPEG::getIIStuff(void)")
}

// 0x99118 - __ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, float (*)[4][32], int *, int)
#[doc(alias = "FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")]
pub fn stub_99118() -> ! {
    todo!("0x99118 FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")
}

// 0x99728 - __ZN4FMOD9CodecMPEG11II_step_oneEPjPi
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")]
pub fn stub_99728() -> ! {
    todo!("0x99728 FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")
}

// 0x99a10 - __ZN4FMOD9CodecMPEG12decodeLayer2EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")]
pub fn stub_99a10() -> ! {
    todo!("0x99a10 FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")
}

// 0x99b08 - __ZN4FMOD9CodecMPEG10initLayer2Ev
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::initLayer2(void)")]
pub fn stub_99b08() -> ! {
    todo!("0x99b08 FMOD::CodecMPEG::initLayer2(void)")
}

// 0x99d7c - __ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii
// type: int __fastcall(int, int, int, _DWORD *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")]
pub fn stub_99d7c() -> ! {
    todo!("0x99d7c FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")
}

// 0x9a240 - __ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")]
pub fn stub_9a240() -> ! {
    todo!("0x9a240 FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")
}

// 0x9a308 - __ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_
// type: float *__fastcall(FMOD::CodecMPEG *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")]
pub fn stub_9a308() -> ! {
    todo!("0x9a308 FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")
}

// 0x9a9e8 - __ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")]
pub fn stub_9a9e8() -> ! {
    todo!("0x9a9e8 FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")
}

// 0x9af14 - __ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE
// type: int __fastcall(int, int, float *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")]
pub fn stub_9af14() -> ! {
    todo!("0x9af14 FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")
}

// 0x9b1f8 - __ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *this, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample_ms(float (*)[32][18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_9b1f8() -> ! {
    todo!("0x9b1f8 FMOD::CodecMPEG::III_dequantize_sample_ms(float (*)[32][18],int *,FMOD::gr_info_s *,int,int)")
}

// 0x9c668 - __ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample(float (*)[18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_9c668() -> ! {
    todo!("0x9c668 FMOD::CodecMPEG::III_dequantize_sample(float (*)[18],int *,FMOD::gr_info_s *,int,int)")
}

// 0x9d78c - __ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, _DWORD *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_2(int *,FMOD::gr_info_s *,int,int *)")]
pub fn stub_9d78c() -> ! {
    todo!("0x9d78c FMOD::CodecMPEG::III_get_scale_factors_2(int *,FMOD::gr_info_s *,int,int *)")
}

// 0x9d920 - __ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, int *, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_1(int *,FMOD::gr_info_s *,int *)")]
pub fn stub_9d920() -> ! {
    todo!("0x9d920 FMOD::CodecMPEG::III_get_scale_factors_1(int *,FMOD::gr_info_s *,int *)")
}

// 0x9dcbc - __ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_2(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_9dcbc() -> ! {
    todo!("0x9dcbc FMOD::CodecMPEG::III_get_side_info_2(FMOD::III_sideinfo *,int,int,int)")
}

// 0x9e0e0 - __ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_1(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_9e0e0() -> ! {
    todo!("0x9e0e0 FMOD::CodecMPEG::III_get_side_info_1(FMOD::III_sideinfo *,int,int,int)")
}

// 0x9e5ac - __ZN4FMOD9CodecMPEG12decodeLayer3EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer3(void *,unsigned int *)")]
pub fn stub_9e5ac() -> ! {
    todo!("0x9e5ac FMOD::CodecMPEG::decodeLayer3(void *,unsigned int *)")
}

// 0x9eb14 - __ZN4FMOD9CodecMPEG10initLayer3Ei
// type: int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::initLayer3(int)")]
pub fn stub_9eb14() -> ! {
    todo!("0x9eb14 FMOD::CodecMPEG::initLayer3(int)")
}

// 0x9fa10 - __ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_9fa10() -> ! {
    todo!("0x9fa10 FMOD::CodecOggVorbis::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x9fa34 - __ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_9fa34() -> ! {
    todo!("0x9fa34 FMOD::CodecOggVorbis::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")
}

// 0x9fa8c - __ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::readVorbisComments(void)")]
pub fn stub_9fa8c() -> ! {
    todo!("0x9fa8c FMOD::CodecOggVorbis::readVorbisComments(void)")
}

// 0x9fb70 - __ZN4FMOD14CodecOggVorbis19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecOggVorbis *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_9fb70() -> ! {
    todo!("0x9fb70 FMOD::CodecOggVorbis::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x9fba0 - __ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecOggVorbis *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_9fba0() -> ! {
    todo!("0x9fba0 FMOD::CodecOggVorbis::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x9fbac - __ZN4FMOD14CodecOggVorbis12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_9fbac() -> ! {
    todo!("0x9fbac FMOD::CodecOggVorbis::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x9fd24 - __ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_9fd24() -> ! {
    todo!("0x9fd24 FMOD::CodecOggVorbis::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x9fd30 - __ZN4FMOD14CodecOggVorbis13closeInternalEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::closeInternal(void)")]
pub fn stub_9fd30() -> ! {
    todo!("0x9fd30 FMOD::CodecOggVorbis::closeInternal(void)")
}

// 0x9fd50 - __ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecOggVorbis *)
#[doc(alias = "FMOD::CodecOggVorbis::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_9fd50() -> ! {
    todo!("0x9fd50 FMOD::CodecOggVorbis::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x9fd5c - __ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi
// type: int __fastcall(FMOD *this, int, __int64, int)
#[doc(alias = "FMOD::FMOD_OggVorbis_SeekCallback(void *,long long,int)")]
pub fn stub_9fd5c() -> ! {
    todo!("0x9fd5c FMOD::FMOD_OggVorbis_SeekCallback(void *,long long,int)")
}

// 0x9fd80 - __ZN4FMOD14CodecOggVorbis16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::getDescriptionEx(void)")]
pub fn stub_9fd80() -> ! {
    todo!("0x9fd80 FMOD::CodecOggVorbis::getDescriptionEx(void)")
}

// 0x9fe30 - __ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_
// type: unsigned int __fastcall(FMOD *this, unsigned int, unsigned int, FMOD::File *, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_ReadCallback(void *,unsigned long,unsigned long,void *)")]
pub fn stub_9fe30() -> ! {
    todo!("0x9fe30 FMOD::FMOD_OggVorbis_ReadCallback(void *,unsigned long,unsigned long,void *)")
}

// 0x9fe7c - _FMOD_OggVorbis_Free
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_FMOD_OggVorbis_Free")]
pub fn stub_9fe7c() -> ! {
    todo!("0x9fe7c _FMOD_OggVorbis_Free")
}

// 0x9fec8 - __ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_9fec8() -> ! {
    todo!("0x9fec8 FMOD::CodecOggVorbis::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0xa0448 - __ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_a0448() -> ! {
    todo!("0xa0448 FMOD::CodecOggVorbis::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0xa0454 - __ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv
// type: unsigned int __fastcall(FMOD *this, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_TellCallback(void *)")]
pub fn stub_a0454() -> ! {
    todo!("0xa0454 FMOD::FMOD_OggVorbis_TellCallback(void *)")
}

// 0xa0474 - _FMOD_OggVorbis_ReAlloc
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "_FMOD_OggVorbis_ReAlloc")]
pub fn stub_a0474() -> ! {
    todo!("0xa0474 _FMOD_OggVorbis_ReAlloc")
}

// 0xa0500 - _FMOD_OggVorbis_Calloc
// type: int __fastcall(int, int, int)
#[doc(alias = "_FMOD_OggVorbis_Calloc")]
pub fn stub_a0500() -> ! {
    todo!("0xa0500 _FMOD_OggVorbis_Calloc")
}

// 0xa0564 - _FMOD_OggVorbis_Malloc
// type: int __fastcall(int, int)
#[doc(alias = "_FMOD_OggVorbis_Malloc")]
pub fn stub_a0564() -> ! {
    todo!("0xa0564 _FMOD_OggVorbis_Malloc")
}

// 0xa05c8 - __Z41__static_initialization_and_destruction_0ii_8
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_8")]
pub fn stub_a05c8() -> ! {
    todo!("0xa05c8 __Z41__static_initialization_and_destruction_0ii_8")
}

// 0xa0614 - __GLOBAL__I_FMOD_OggVorbis_Malloc
// type: int()
#[doc(alias = "global constructor keyed to_FMOD_OggVorbis_Malloc")]
pub fn stub_a0614() -> ! {
    todo!("0xa0614 global constructor keyed to_FMOD_OggVorbis_Malloc")
}

// 0xa0620 - __ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi
// type: int __fastcall(FMOD::CodecPlaylist *this, const char *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getQuoteData(char const*,char *,int *)")]
pub fn stub_a0620() -> ! {
    todo!("0xa0620 FMOD::CodecPlaylist::getQuoteData(char const*,char *,int *)")
}

// 0xa0684 - __ZN4FMOD13CodecPlaylist13closeInternalEv
// type: int __fastcall(FMOD::CodecPlaylist *this)
#[doc(alias = "FMOD::CodecPlaylist::closeInternal(void)")]
pub fn stub_a0684() -> ! {
    todo!("0xa0684 FMOD::CodecPlaylist::closeInternal(void)")
}

// 0xa068c - __ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecPlaylist *)
#[doc(alias = "FMOD::CodecPlaylist::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_a068c() -> ! {
    todo!("0xa068c FMOD::CodecPlaylist::closeCallback(FMOD_CODEC_STATE *)")
}

// 0xa0698 - __ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_a0698() -> ! {
    todo!("0xa0698 FMOD::CodecPlaylist::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0xa06a0 - __ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_a06a0() -> ! {
    todo!("0xa06a0 FMOD::CodecPlaylist::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0xa06a8 - __ZN4FMOD13CodecPlaylist9isNewLineEc
// type: bool __fastcall(FMOD::File **this, char)
#[doc(alias = "FMOD::CodecPlaylist::isNewLine(char)")]
pub fn stub_a06a8() -> ! {
    todo!("0xa06a8 FMOD::CodecPlaylist::isNewLine(char)")
}

// 0xa0704 - __ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi
// type: int __fastcall(FMOD::File **this, int *)
#[doc(alias = "FMOD::CodecPlaylist::skipWhiteSpace(int *)")]
pub fn stub_a0704() -> ! {
    todo!("0xa0704 FMOD::CodecPlaylist::skipWhiteSpace(int *)")
}

// 0xa0784 - __ZN4FMOD13CodecPlaylist8readLineEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::readLine(char *,int,int *)")]
pub fn stub_a0784() -> ! {
    todo!("0xa0784 FMOD::CodecPlaylist::readLine(char *,int,int *)")
}

// 0xa0820 - __ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::skipSimpleComments(void)")]
pub fn stub_a0820() -> ! {
    todo!("0xa0820 FMOD::CodecPlaylist::skipSimpleComments(void)")
}

// 0xa08b8 - __ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::getPLSToken(char *,int,int *)")]
pub fn stub_a08b8() -> ! {
    todo!("0xa08b8 FMOD::CodecPlaylist::getPLSToken(char *,int,int *)")
}

// 0xa0a54 - __ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_
// type: int __fastcall(FMOD::File **this, char *, int *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getNextXMLTag(char *,int *,char *,int *)")]
pub fn stub_a0a54() -> ! {
    todo!("0xa0a54 FMOD::CodecPlaylist::getNextXMLTag(char *,int *,char *,int *)")
}

// 0xa0bb8 - __ZN4FMOD13CodecPlaylist10readSimpleEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readSimple(void)")]
pub fn stub_a0bb8() -> ! {
    todo!("0xa0bb8 FMOD::CodecPlaylist::readSimple(void)")
}

// 0xa0c58 - __ZN4FMOD13CodecPlaylist7readPLSEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readPLS(void)")]
pub fn stub_a0c58() -> ! {
    todo!("0xa0c58 FMOD::CodecPlaylist::readPLS(void)")
}

// 0xa0edc - __ZN4FMOD13CodecPlaylist7readM3UEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readM3U(void)")]
pub fn stub_a0edc() -> ! {
    todo!("0xa0edc FMOD::CodecPlaylist::readM3U(void)")
}

// 0xa1218 - __ZN4FMOD13CodecPlaylist7readB4SEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::readB4S(void)")]
pub fn stub_a1218() -> ! {
    todo!("0xa1218 FMOD::CodecPlaylist::readB4S(void)")
}
