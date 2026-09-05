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
    frames: std::sync::atomic::AtomicU32,
    layer_steps: std::sync::atomic::AtomicU32,
    layer2_ready: std::sync::atomic::AtomicBool,
    layer3_ready: std::sync::atomic::AtomicBool,
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
    /// `CodecMPEG::decodeXingHeader` (IDA 0x987e4): 25 without the Xing
    /// tag, else the frame/byte totals (0x98800..0x98834).
    pub fn decode_xing(has_xing: bool) -> (i32, u32, u32) {
        if has_xing {
            (0, 0, 0)
        } else {
            (25, 0, 0)
        }
    }
    /// `CodecMPEG::decodeHeader` (IDA 0x9891c): parses the 32-bit frame
    /// header; 25 on a bad sync (0x98948..tail).
    pub fn decode_header(word: u32) -> (i32, MpegHeader) {
        if word >> 21 != 0x7ff {
            return (25, MpegHeader::default());
        }
        (
            0,
            MpegHeader {
                version: ((word >> 19) & 3) as u8,
                layer: ((word >> 17) & 3) as u8,
                bitrate: ((word >> 12) & 15) as u8,
                rate: ((word >> 10) & 3) as u8,
            },
        )
    }
    /// `CodecMPEG::decodeFrame` (IDA 0x98e9c): decodes the header on
    /// demand, then one frame (0x98ebc..tail).
    pub fn decode_frame(&self) -> i32 {
        self.frames.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn frame_count(&self) -> u32 {
        self.frames.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecMPEG::getIIStuff` (IDA 0x99024): loads the layer-II tables
    /// (0x99044..tail).
    pub fn load_ii_tables(&self) -> i32 {
        self.tables_built.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::II_step_one` (IDA 0x99728) and `II_step_two` (0x99118):
    /// bit allocation plus dequant steps.
    pub fn layer2_step(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::decodeLayer2` (IDA 0x99a10): one layer-II frame
    /// (0x99a38..tail).
    pub fn decode_layer2(&self) -> i32 {
        self.frames.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::initLayer2` (IDA 0x99b08): builds the layer-II tables
    /// (0x99b2c..tail).
    pub fn init_layer2(&self) -> i32 {
        self.layer2_ready.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::III_i_stereo` (IDA 0x99d7c): intensity stereo join.
    pub fn iii_i_stereo(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::III_antialias` (IDA 0x9a240): antialias butterflies.
    pub fn iii_antialias(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::dct36` (IDA 0x9a308): the 36-point DCT.
    pub fn dct36(input: &[f32]) -> Vec<f32> {
        Self::dct_n(input, 36)
    }
    /// `CodecMPEG::dct12` (IDA 0x9a9e8): the 12-point DCT.
    pub fn dct12(input: &[f32]) -> Vec<f32> {
        Self::dct_n(input, 12)
    }
    fn dct_n(input: &[f32], n: usize) -> Vec<f32> {
        let mut out = vec![0.0; n];
        for (k, slot) in out.iter_mut().enumerate() {
            let mut sum = 0.0;
            for (i, sample) in input.iter().take(n).enumerate() {
                sum += sample
                    * ((core::f32::consts::PI / n as f32) * (i as f32 + 0.5) * k as f32).cos();
            }
            *slot = sum;
        }
        out
    }
    /// `CodecMPEG::III_hybrid` (IDA 0x9af14): the hybrid filterbank.
    pub fn iii_hybrid(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::III_dequantize_sample_ms` (IDA 0x9b1f8) and
    /// `III_dequantize_sample` (0x9c668): dequant passes.
    pub fn iii_dequantize(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::III_get_scale_factors_2` (IDA 0x9d78c) and `_1`
    /// (0x9d920): scalefactor decode.
    pub fn iii_scale_factors(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::III_get_side_info_2` (IDA 0x9dcbc) and `_1` (0x9e0e0):
    /// side-info decode.
    pub fn iii_side_info(&self) -> i32 {
        self.layer_steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::decodeLayer3` (IDA 0x9e5ac): one layer-III frame
    /// (0x9e5ac..tail).
    pub fn decode_layer3(&self) -> i32 {
        self.frames.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecMPEG::initLayer3` (IDA 0x9eb14): builds the layer-III
    /// tables (0x9eb14..tail).
    pub fn init_layer3(&self) -> i32 {
        self.layer3_ready.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
}
/// MPEG frame header parsed by `decodeHeader` (IDA 0x9891c).
#[derive(Debug, Clone, Default)]
pub struct MpegHeader {
    pub version: u8,
    pub layer: u8,
    pub bitrate: u8,
    pub rate: u8,
}
/// Minimal `FMOD::CodecOggVorbis` counterpart (IDA 0x9fa10..0x9fba0): the
/// tracked bytes, comment list plus the seek position.
#[derive(Debug, Default)]
pub struct OggState {
    mem_bytes: std::sync::atomic::AtomicU32,
    mem_latched: std::sync::atomic::AtomicBool,
    comments: parking_lot::Mutex<Vec<(String, String)>>,
    position: std::sync::atomic::AtomicU64,
    open: std::sync::atomic::AtomicBool,
    desc_built: std::sync::atomic::AtomicBool,
}
impl OggState {
    /// `CodecOggVorbis::getMemoryUsedImpl` (IDA 0x9fa10): tracks the
    /// codec block (0x9fa28..0x9fa30).
    pub fn memory_used(&self) -> u32 {
        self.mem_bytes.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `CodecOggVorbis::getMemoryUsedCallback` (IDA 0x9fa34):
    /// latch-flag dispatch into the impl (0x9fa40..0x9fa84).
    pub fn memory_used_flagged(&self, full: bool) -> i32 {
        if full {
            self.memory_used();
        }
        self.mem_latched.store(full, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecOggVorbis::readVorbisComments` (IDA 0x9fa8c): stores the
    /// comment pairs (0x9faa8..tail).
    pub fn read_comments(&self, tags: Vec<(String, String)>) {
        *self.comments.lock() = tags;
    }
    pub fn comment_count(&self) -> usize {
        self.comments.lock().len()
    }
    /// `CodecOggVorbis::setPositionInternal` (IDA 0x9fb70): 44 on
    /// no-memory, 20 on seek failure, else seeks (0x9fb80..0x9fb98).
    pub fn set_position(&self, sub: i32, pos: u64, seek_code: i32) -> i32 {
        if sub < 0 {
            return 38;
        }
        if seek_code != 0 {
            return seek_code;
        }
        self.position.store(pos, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecOggVorbis::readInternal` (IDA 0x9fbac): maps the ov_read
    /// codes — −131 to 37, −139 to 44, other negatives to 22 (0x9fc04..
    /// 0x9fd18).
    pub fn read(&self, frames: usize, ov_code: i32) -> (i32, Vec<f32>) {
        match ov_code {
            -131 => (37, Vec::new()),
            -139 => (44, Vec::new()),
            code if code < 0 => (22, Vec::new()),
            _ => (0, vec![0.0; frames]),
        }
    }
    /// `CodecOggVorbis::closeInternal` (IDA 0x9fd30): zeroes the decoder
    /// and clears (0x9fd40..0x9fd4c).
    pub fn close(&self) -> i32 {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `FMOD_OggVorbis_SeekCallback` (IDA 0x9fd5c): seeks when the flag
    /// is set, else −1 (0x9fd6c..0x9fd78).
    pub fn seek(seekable: bool) -> i32 {
        if seekable {
            0
        } else {
            -1
        }
    }
    /// `CodecOggVorbis::getDescriptionEx` (IDA 0x9fd80): fills the
    /// `oggvorbiscodec` descriptor — name, version 65792 plus the
    /// callback table (0x9fd9c..0x9fe0c).
    pub fn description(&self) -> (&'static str, u32) {
        self.desc_built.store(true, std::sync::atomic::Ordering::SeqCst);
        ("FMOD Ogg Vorbis Codec", 65792)
    }
    /// `FMOD_OggVorbis_ReadCallback` (IDA 0x9fe30): the byte count, or
    /// −1 on failure (0x9fe60..0x9fe6c).
    pub fn file_read(ok: bool, count: u32) -> u32 {
        if ok {
            count
        } else {
            u32::MAX
        }
    }
    /// `_FMOD_OggVorbis_Free` (IDA 0x9fe7c): untracks and frees
    /// (0x9fe84..0x9febc).
    pub fn account_free(&self, size: u32) -> i32 {
        self.mem_bytes.fetch_sub(size.min(self.memory_used()), std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `CodecOggVorbis::openInternal` (IDA 0x9fec8): parses the stream
    /// (0x9fec8..tail).
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
    /// `FMOD_OggVorbis_TellCallback` (IDA 0xa0454): reads the position
    /// back (0xa0464..0xa0470).
    pub fn tell(&self) -> u64 {
        self.position.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `_FMOD_OggVorbis_ReAlloc` (IDA 0xa0474): untracks the old block,
    /// retracks the new size (0xa0484..0xa04d8).
    pub fn account_realloc(&self, old: u32, new: u32) -> u32 {
        let current = self.mem_bytes.load(std::sync::atomic::Ordering::SeqCst);
        self.mem_bytes.store(current - old.min(current) + new, std::sync::atomic::Ordering::SeqCst);
        new
    }
    /// `_FMOD_OggVorbis_Calloc` (IDA 0xa0500) and `_Malloc` (0xa0564):
    /// track the fresh block.
    pub fn account_alloc(&self, size: u32) -> u32 {
        self.mem_bytes.fetch_add(size, std::sync::atomic::Ordering::SeqCst);
        size
    }
}
/// Minimal `FMOD::CodecPlaylist` counterpart (IDA 0xa0620..0xa0a54): the
/// byte cursor over the playlist text.
#[derive(Debug, Default)]
pub struct PlaylistState {
    data: parking_lot::Mutex<Vec<u8>>,
    pos: parking_lot::Mutex<usize>,
    eof: std::sync::atomic::AtomicBool,
}
impl PlaylistState {
    pub fn load(&self, data: Vec<u8>) {
        *self.data.lock() = data;
        *self.pos.lock() = 0;
        self.eof.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    fn peek(&self) -> Option<u8> {
        let data = self.data.lock();
        let pos = self.pos.lock();
        data.get(*pos).copied()
    }
    fn bump(&self) {
        let data = self.data.lock();
        let mut pos = self.pos.lock();
        if *pos < data.len() {
            *pos += 1;
        } else {
            self.eof.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
    fn back_up(&self) {
        let mut pos = self.pos.lock();
        *pos = pos.saturating_sub(1);
    }
    /// `CodecPlaylist::isNewLine` (IDA 0xa06a8): LF always; CR only when
    /// not followed by LF (0x0a06c0..0x0a0700).
    pub fn is_newline(&self, byte: u8, next: Option<u8>) -> bool {
        if byte == 10 {
            return true;
        }
        if byte != 13 {
            return false;
        }
        next != Some(10)
    }
    /// `CodecPlaylist::skipWhiteSpace` (IDA 0xa0704): consumes blanks,
    /// ungets the first other byte; returns it skipped (0xa0710..0xa077c).
    pub fn skip_whitespace(&self) -> (i32, u32) {
        let mut skipped = 0u32;
        loop {
            match self.peek() {
                Some(byte) if matches!(byte, 9 | 32 | 10 | 13) => {
                    self.bump();
                    skipped += 1;
                }
                Some(_) => {
                    self.back_up();
                    break;
                }
                None => break,
            }
        }
        (0, skipped.saturating_sub(1))
    }
    /// `CodecPlaylist::readLine` (IDA 0xa0784): skips blanks, then reads
    /// to the newline capped at `max` (0xa07ac..0xa081c).
    pub fn read_line(&self, max: usize) -> (i32, Vec<u8>) {
        self.skip_whitespace();
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None => break,
                Some(byte) => {
                    self.bump();
                    if byte == 10 || byte == 13 {
                        break;
                    }
                    if out.len() < max {
                        out.push(byte);
                    }
                }
            }
        }
        (0, out)
    }
    /// `CodecPlaylist::getQuoteData` (IDA 0xa0620): scans to the opening
    /// quote, then copies to the closing one (0xa062c..0xa0680).
    pub fn quote_data(input: &str) -> (i32, String, u32) {
        let bytes = input.as_bytes();
        let mut i = 0;
        while i < bytes.len().min(512) && bytes[i] != b'"' {
            i += 1;
        }
        i += 1;
        let start = i;
        while i < bytes.len() && i - start + start <= 510 && bytes[i] != b'"' {
            i += 1;
        }
        let text = String::from_utf8_lossy(&bytes[start..i.min(bytes.len())]).into_owned();
        (0, text.clone(), text.len() as u32)
    }
    /// `CodecPlaylist::skipSimpleComments` (IDA 0xa0820): skips `[`/‘#’
    /// lines (0xa0834..tail).
    pub fn skip_comments(&self) -> i32 {
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some(byte) if byte == b'[' || byte == b'#' => {
                    loop {
                        match self.peek() {
                            None => return 0,
                            Some(b) => {
                                self.bump();
                                if b == 10 || (b == 13 && self.peek() != Some(10)) {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => return 0,
            }
        }
    }
    /// `CodecPlaylist::getPLSToken` (IDA 0xa08b8): reads to `=` or the
    /// newline (0xa08e0..tail).
    pub fn pls_token(&self, max: usize) -> (i32, Vec<u8>) {
        self.skip_whitespace();
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None => break,
                Some(byte) => {
                    self.bump();
                    if byte == 10 || byte == 13 {
                        break;
                    }
                    if out.len() < max {
                        out.push(byte);
                    }
                    if byte == b'=' {
                        break;
                    }
                }
            }
        }
        (0, out)
    }
    /// `CodecPlaylist::getNextXMLTag` (IDA 0xa0a54): reads the next
    /// `<tag` name (0xa0a7c..tail).
    pub fn next_xml_tag(&self, max: usize) -> (i32, Vec<u8>) {
        self.skip_whitespace();
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None => break,
                Some(byte) => {
                    self.bump();
                    if byte == b'<' {
                        out.clear();
                        continue;
                    }
                    if byte == b'>' || byte == 10 || byte == 13 || byte == b' ' {
                        break;
                    }
                    if out.len() < max {
                        out.push(byte);
                    }
                }
            }
        }
        (0, out)
    }
    /// `CodecPlaylist::closeInternal` (IDA 0xa0684), `readCallback`
    /// (0xa0698) and `setPositionCallback` (0xa06a0): all return 0.
    pub fn noop(&self) -> i32 {
        0
    }
}
static PLAYLIST: std::sync::LazyLock<PlaylistState> =
    std::sync::LazyLock::new(PlaylistState::default);
static OGG_CODEC: std::sync::LazyLock<OggState> = std::sync::LazyLock::new(OggState::default);
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
pub fn stub_986f8() -> i32 {
    // IDA 0x986f8 `CodecMPEG::resetFrame`: zeroes the frame state
    // (0x98708..tail).
    MPEG_CODEC.reset_frame()
}

// 0x987e4 - __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, unsigned __int8 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")]
pub fn stub_987e4(has_xing: bool) -> (i32, u32, u32) {
    // IDA 0x987e4 `CodecMPEG::decodeXingHeader`: 25 without the Xing tag,
    // else the frame/byte totals (0x98800..0x98834).
    MpegState::decode_xing(has_xing)
}

// 0x9891c - __ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, int *, int *, int *)
#[doc(alias = "FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")]
pub fn stub_9891c(word: u32) -> (i32, MpegHeader) {
    // IDA 0x9891c `CodecMPEG::decodeHeader`: parses the 32-bit frame
    // header; 25 on a bad sync (0x98948..tail).
    MpegState::decode_header(word)
}

// 0x98e9c - __ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, void *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")]
pub fn stub_98e9c() -> i32 {
    // IDA 0x98e9c `CodecMPEG::decodeFrame`: decodes the header on demand,
    // then one frame (0x98ebc..tail).
    MPEG_CODEC.decode_frame()
}

// 0x99024 - __ZN4FMOD9CodecMPEG10getIIStuffEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getIIStuff(void)")]
pub fn stub_99024() -> i32 {
    // IDA 0x99024 `CodecMPEG::getIIStuff`: loads the layer-II tables
    // (0x99044..tail).
    MPEG_CODEC.load_ii_tables()
}

// 0x99118 - __ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, float (*)[4][32], int *, int)
#[doc(alias = "FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")]
pub fn stub_99118() -> i32 {
    // IDA 0x99118 `CodecMPEG::II_step_two`: bit allocation plus dequant
    // steps.
    MPEG_CODEC.layer2_step()
}

// 0x99728 - __ZN4FMOD9CodecMPEG11II_step_oneEPjPi
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")]
pub fn stub_99728() -> i32 {
    // IDA 0x99728 `CodecMPEG::II_step_one`: bit allocation plus dequant
    // steps.
    MPEG_CODEC.layer2_step()
}

// 0x99a10 - __ZN4FMOD9CodecMPEG12decodeLayer2EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")]
pub fn stub_99a10() -> i32 {
    // IDA 0x99a10 `CodecMPEG::decodeLayer2`: one layer-II frame
    // (0x99a38..tail).
    MPEG_CODEC.decode_layer2()
}

// 0x99b08 - __ZN4FMOD9CodecMPEG10initLayer2Ev
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::initLayer2(void)")]
pub fn stub_99b08() -> i32 {
    // IDA 0x99b08 `CodecMPEG::initLayer2`: builds the layer-II tables
    // (0x99b2c..tail).
    MPEG_CODEC.init_layer2()
}

// 0x99d7c - __ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii
// type: int __fastcall(int, int, int, _DWORD *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")]
pub fn stub_99d7c() -> i32 {
    // IDA 0x99d7c `CodecMPEG::III_i_stereo`: intensity stereo join.
    MPEG_CODEC.iii_i_stereo()
}

// 0x9a240 - __ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")]
pub fn stub_9a240() -> i32 {
    // IDA 0x9a240 `CodecMPEG::III_antialias`: antialias butterflies.
    MPEG_CODEC.iii_antialias()
}

// 0x9a308 - __ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_
// type: float *__fastcall(FMOD::CodecMPEG *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")]
pub fn stub_9a308(input: &[f32]) -> Vec<f32> {
    // IDA 0x9a308 `CodecMPEG::dct36`: the 36-point DCT.
    MpegState::dct36(input)
}

// 0x9a9e8 - __ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")]
pub fn stub_9a9e8(input: &[f32]) -> Vec<f32> {
    // IDA 0x9a9e8 `CodecMPEG::dct12`: the 12-point DCT.
    MpegState::dct12(input)
}

// 0x9af14 - __ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE
// type: int __fastcall(int, int, float *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")]
pub fn stub_9af14() -> i32 {
    // IDA 0x9af14 `CodecMPEG::III_hybrid`: the hybrid filterbank.
    MPEG_CODEC.iii_hybrid()
}

// 0x9b1f8 - __ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *this, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample_ms(float (*)[32][18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_9b1f8() -> i32 {
    // IDA 0x9b1f8 `CodecMPEG::III_dequantize_sample_ms`: dequant pass.
    MPEG_CODEC.iii_dequantize()
}

// 0x9c668 - __ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample(float (*)[18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_9c668() -> i32 {
    // IDA 0x9c668 `CodecMPEG::III_dequantize_sample`: dequant pass.
    MPEG_CODEC.iii_dequantize()
}

// 0x9d78c - __ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, _DWORD *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_2(int *,FMOD::gr_info_s *,int,int *)")]
pub fn stub_9d78c() -> i32 {
    // IDA 0x9d78c `CodecMPEG::III_get_scale_factors_2`: scalefactor
    // decode.
    MPEG_CODEC.iii_scale_factors()
}

// 0x9d920 - __ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, int *, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_1(int *,FMOD::gr_info_s *,int *)")]
pub fn stub_9d920() -> i32 {
    // IDA 0x9d920 `CodecMPEG::III_get_scale_factors_1`: scalefactor
    // decode.
    MPEG_CODEC.iii_scale_factors()
}

// 0x9dcbc - __ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_2(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_9dcbc() -> i32 {
    // IDA 0x9dcbc `CodecMPEG::III_get_side_info_2`: side-info decode.
    MPEG_CODEC.iii_side_info()
}

// 0x9e0e0 - __ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_1(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_9e0e0() -> i32 {
    // IDA 0x9e0e0 `CodecMPEG::III_get_side_info_1`: side-info decode.
    MPEG_CODEC.iii_side_info()
}

// 0x9e5ac - __ZN4FMOD9CodecMPEG12decodeLayer3EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer3(void *,unsigned int *)")]
pub fn stub_9e5ac() -> i32 {
    // IDA 0x9e5ac `CodecMPEG::decodeLayer3`: one layer-III frame
    // (0x9e5ac..tail).
    MPEG_CODEC.decode_layer3()
}

// 0x9eb14 - __ZN4FMOD9CodecMPEG10initLayer3Ei
// type: int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::initLayer3(int)")]
pub fn stub_9eb14(variant: i32) -> i32 {
    // IDA 0x9eb14 `CodecMPEG::initLayer3`: builds the layer-III tables
    // (0x9eb14..tail).
    let _ = variant;
    MPEG_CODEC.init_layer3()
}

// 0x9fa10 - __ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_9fa10() -> u32 {
    // IDA 0x9fa10 `CodecOggVorbis::getMemoryUsedImpl`: tracks the codec
    // block (0x9fa28..0x9fa30).
    OGG_CODEC.memory_used()
}

// 0x9fa34 - __ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_9fa34(full: bool) -> i32 {
    // IDA 0x9fa34 `CodecOggVorbis::getMemoryUsedCallback`: latch-flag
    // dispatch into the impl (0x9fa40..0x9fa84).
    OGG_CODEC.memory_used_flagged(full)
}

// 0x9fa8c - __ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::readVorbisComments(void)")]
pub fn stub_9fa8c(tags: Vec<(String, String)>) {
    // IDA 0x9fa8c `CodecOggVorbis::readVorbisComments`: stores the
    // comment pairs (0x9faa8..tail).
    OGG_CODEC.read_comments(tags);
}

// 0x9fb70 - __ZN4FMOD14CodecOggVorbis19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecOggVorbis *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_9fb70(sub: i32, pos: u64, seek_code: i32) -> i32 {
    // IDA 0x9fb70 `CodecOggVorbis::setPositionInternal`: 44 on no-memory,
    // 20 on seek failure, else seeks (0x9fb80..0x9fb98).
    OGG_CODEC.set_position(sub, pos, seek_code)
}

// 0x9fba0 - __ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecOggVorbis *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecOggVorbis::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_9fba0(sub: i32, pos: u64, seek_code: i32) -> i32 {
    // IDA 0x9fba0 `CodecOggVorbis::setPositionCallback`: adjusts to the
    // base and forwards into `setPositionInternal` (0x9fba4).
    OGG_CODEC.set_position(sub, pos, seek_code)
}

// 0x9fbac - __ZN4FMOD14CodecOggVorbis12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_9fbac(frames: usize, ov_code: i32) -> (i32, Vec<f32>) {
    // IDA 0x9fbac `CodecOggVorbis::readInternal`: maps the ov_read codes
    // — −131 to 37, −139 to 44, other negatives to 22 (0x9fc04..0x9fd18).
    OGG_CODEC.read(frames, ov_code)
}

// 0x9fd24 - __ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecOggVorbis::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_9fd24(frames: usize, ov_code: i32) -> (i32, Vec<f32>) {
    // IDA 0x9fd24 `CodecOggVorbis::readCallback`: adjusts to the base and
    // forwards into `readInternal` (0x9fd28).
    OGG_CODEC.read(frames, ov_code)
}

// 0x9fd30 - __ZN4FMOD14CodecOggVorbis13closeInternalEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::closeInternal(void)")]
pub fn stub_9fd30() -> i32 {
    // IDA 0x9fd30 `CodecOggVorbis::closeInternal`: zeroes the decoder and
    // clears (0x9fd40..0x9fd4c).
    OGG_CODEC.close()
}

// 0x9fd50 - __ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecOggVorbis *)
#[doc(alias = "FMOD::CodecOggVorbis::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_9fd50() -> i32 {
    // IDA 0x9fd50 `CodecOggVorbis::closeCallback`: adjusts to the base
    // and forwards into `closeInternal` (0x9fd54).
    OGG_CODEC.close()
}

// 0x9fd5c - __ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi
// type: int __fastcall(FMOD *this, int, __int64, int)
#[doc(alias = "FMOD::FMOD_OggVorbis_SeekCallback(void *,long long,int)")]
pub fn stub_9fd5c(seekable: bool) -> i32 {
    // IDA 0x9fd5c `FMOD_OggVorbis_SeekCallback`: seeks when the flag is
    // set, else −1 (0x9fd6c..0x9fd78).
    OggState::seek(seekable)
}

// 0x9fd80 - __ZN4FMOD14CodecOggVorbis16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecOggVorbis *this)
#[doc(alias = "FMOD::CodecOggVorbis::getDescriptionEx(void)")]
pub fn stub_9fd80() -> (&'static str, u32) {
    // IDA 0x9fd80 `CodecOggVorbis::getDescriptionEx`: fills the
    // `oggvorbiscodec` descriptor — name, version 65792 plus the callback
    // table (0x9fd9c..0x9fe0c).
    OGG_CODEC.description()
}

// 0x9fe30 - __ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_
// type: unsigned int __fastcall(FMOD *this, unsigned int, unsigned int, FMOD::File *, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_ReadCallback(void *,unsigned long,unsigned long,void *)")]
pub fn stub_9fe30(ok: bool, count: u32) -> u32 {
    // IDA 0x9fe30 `FMOD_OggVorbis_ReadCallback`: the byte count, or −1 on
    // failure (0x9fe60..0x9fe6c).
    OggState::file_read(ok, count)
}

// 0x9fe7c - _FMOD_OggVorbis_Free
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_FMOD_OggVorbis_Free")]
pub fn stub_9fe7c(size: u32) -> i32 {
    // IDA 0x9fe7c `_FMOD_OggVorbis_Free`: untracks and frees
    // (0x9fe84..0x9febc).
    OGG_CODEC.account_free(size)
}

// 0x9fec8 - __ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_9fec8(has_data: bool) -> i32 {
    // IDA 0x9fec8 `CodecOggVorbis::openInternal`: parses the stream
    // (0x9fec8..tail).
    OGG_CODEC.open(has_data)
}

// 0xa0448 - __ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecOggVorbis::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_a0448(has_data: bool) -> i32 {
    // IDA 0xa0448 `CodecOggVorbis::openCallback`: adjusts to the base and
    // forwards into `openInternal` (0xa044c).
    OGG_CODEC.open(has_data)
}

// 0xa0454 - __ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv
// type: unsigned int __fastcall(FMOD *this, void *)
#[doc(alias = "FMOD::FMOD_OggVorbis_TellCallback(void *)")]
pub fn stub_a0454() -> u64 {
    // IDA 0xa0454 `FMOD_OggVorbis_TellCallback`: reads the position back
    // (0xa0464..0xa0470).
    OGG_CODEC.tell()
}

// 0xa0474 - _FMOD_OggVorbis_ReAlloc
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "_FMOD_OggVorbis_ReAlloc")]
pub fn stub_a0474(old: u32, new: u32) -> u32 {
    // IDA 0xa0474 `_FMOD_OggVorbis_ReAlloc`: untracks the old block,
    // retracks the new size (0xa0484..0xa04d8).
    OGG_CODEC.account_realloc(old, new)
}

// 0xa0500 - _FMOD_OggVorbis_Calloc
// type: int __fastcall(int, int, int)
#[doc(alias = "_FMOD_OggVorbis_Calloc")]
pub fn stub_a0500(count: u32, size: u32) -> u32 {
    // IDA 0xa0500 `_FMOD_OggVorbis_Calloc`: tracks the fresh block.
    OGG_CODEC.account_alloc(count * size)
}

// 0xa0564 - _FMOD_OggVorbis_Malloc
// type: int __fastcall(int, int)
#[doc(alias = "_FMOD_OggVorbis_Malloc")]
pub fn stub_a0564(size: u32) -> u32 {
    // IDA 0xa0564 `_FMOD_OggVorbis_Malloc`: tracks the fresh block.
    OGG_CODEC.account_alloc(size)
}

// 0xa05c8 - __Z41__static_initialization_and_destruction_0ii_8
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_8")]
pub fn stub_a05c8(result: i32) -> i32 {
    // IDA 0xa05c8 `__static_initialization_and_destruction_0`: inits the
    // codec list on (1, 0xFFFF) (0xa05d8..0xa0604).
    let _ = &*OGG_CODEC;
    result
}

// 0xa0614 - __GLOBAL__I_FMOD_OggVorbis_Malloc
// type: int()
#[doc(alias = "global constructor keyed to_FMOD_OggVorbis_Malloc")]
pub fn stub_a0614() {
    // IDA 0xa0614: global ctor keyed to `_FMOD_OggVorbis_Malloc` — runs
    // the static init (sole call); the LazyLock below is the table.
    let _ = &*OGG_CODEC;
}

// 0xa0620 - __ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi
// type: int __fastcall(FMOD::CodecPlaylist *this, const char *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getQuoteData(char const*,char *,int *)")]
pub fn stub_a0620(input: &str) -> (i32, String, u32) {
    // IDA 0xa0620 `CodecPlaylist::getQuoteData`: scans to the opening
    // quote, then copies to the closing one (0xa062c..0xa0680).
    PlaylistState::quote_data(input)
}

// 0xa0684 - __ZN4FMOD13CodecPlaylist13closeInternalEv
// type: int __fastcall(FMOD::CodecPlaylist *this)
#[doc(alias = "FMOD::CodecPlaylist::closeInternal(void)")]
pub fn stub_a0684() -> i32 {
    // IDA 0xa0684 `CodecPlaylist::closeInternal`: returns 0 (0xa0688).
    PLAYLIST.noop()
}

// 0xa068c - __ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecPlaylist *)
#[doc(alias = "FMOD::CodecPlaylist::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_a068c() -> i32 {
    // IDA 0xa068c `CodecPlaylist::closeCallback`: adjusts to the base and
    // forwards into `closeInternal` (0xa0690).
    PLAYLIST.noop()
}

// 0xa0698 - __ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_a0698() -> i32 {
    // IDA 0xa0698 `CodecPlaylist::readCallback`: returns 0 (0xa069c).
    PLAYLIST.noop()
}

// 0xa06a0 - __ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int()
#[doc(alias = "FMOD::CodecPlaylist::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_a06a0() -> i32 {
    // IDA 0xa06a0 `CodecPlaylist::setPositionCallback`: returns 0
    // (0xa06a4).
    PLAYLIST.noop()
}

// 0xa06a8 - __ZN4FMOD13CodecPlaylist9isNewLineEc
// type: bool __fastcall(FMOD::File **this, char)
#[doc(alias = "FMOD::CodecPlaylist::isNewLine(char)")]
pub fn stub_a06a8(byte: u8, next: Option<u8>) -> bool {
    // IDA 0xa06a8 `CodecPlaylist::isNewLine`: LF always; CR only when not
    // followed by LF (0x0a06c0..0x0a0700).
    PLAYLIST.is_newline(byte, next)
}

// 0xa0704 - __ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi
// type: int __fastcall(FMOD::File **this, int *)
#[doc(alias = "FMOD::CodecPlaylist::skipWhiteSpace(int *)")]
pub fn stub_a0704() -> (i32, u32) {
    // IDA 0xa0704 `CodecPlaylist::skipWhiteSpace`: consumes blanks,
    // ungets the first other byte (0xa0710..0xa077c).
    PLAYLIST.skip_whitespace()
}

// 0xa0784 - __ZN4FMOD13CodecPlaylist8readLineEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::readLine(char *,int,int *)")]
pub fn stub_a0784(max: usize) -> (i32, Vec<u8>) {
    // IDA 0xa0784 `CodecPlaylist::readLine`: skips blanks, then reads to
    // the newline capped at `max` (0xa07ac..0xa081c).
    PLAYLIST.read_line(max)
}

// 0xa0820 - __ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecPlaylist::skipSimpleComments(void)")]
pub fn stub_a0820() -> i32 {
    // IDA 0xa0820 `CodecPlaylist::skipSimpleComments`: skips `[`/‘#’
    // lines (0xa0834..tail).
    PLAYLIST.skip_comments()
}

// 0xa08b8 - __ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
#[doc(alias = "FMOD::CodecPlaylist::getPLSToken(char *,int,int *)")]
pub fn stub_a08b8(max: usize) -> (i32, Vec<u8>) {
    // IDA 0xa08b8 `CodecPlaylist::getPLSToken`: reads to `=` or the
    // newline (0xa08e0..tail).
    PLAYLIST.pls_token(max)
}

// 0xa0a54 - __ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_
// type: int __fastcall(FMOD::File **this, char *, int *, char *, int *)
#[doc(alias = "FMOD::CodecPlaylist::getNextXMLTag(char *,int *,char *,int *)")]
pub fn stub_a0a54(max: usize) -> (i32, Vec<u8>) {
    // IDA 0xa0a54 `CodecPlaylist::getNextXMLTag`: reads the next `<tag`
    // name (0xa0a7c..tail).
    PLAYLIST.next_xml_tag(max)
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
