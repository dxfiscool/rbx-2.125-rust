//! audio generated_04 — next 60 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2541 total; 0 strict remaining, 60 high-EA filler) | EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 60 stubs | skeleton batch | range 0xf6f600..0xf6f8c4

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
// ---- FMOD MOD/MIDI/MPEG codec host model (IDA 0x92a74..0x986f8) ----
// Target is 32-bit ARM; interior image words are plain u32/i32/f32 fields so
// the byte offsets cited below hold on any host.
// Boost mapping: no shared_ptr in this range; raw decoder pointers become
// Option<&mut T> (nullable like the C++ this), MemPool::free drops to None.
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

/// FMOD_RESULT success (IDA FMOD_OK).
pub const FMOD_OK: i32 = 0;

/// Real quarter-sine table, FMOD::gSineTable (image 0x122a6ac, 32 bytes,
/// read via IDA; index is the low 5 bits of the effect position).
pub const G_SINE_TABLE: [u8; 32] = [
    0, 24, 49, 74, 97, 120, 141, 161, 180, 197, 212, 224, 235, 244, 250, 253,
    255, 253, 250, 244, 235, 224, 212, 197, 180, 161, 141, 120, 97, 74, 49, 24,
];

/// FMOD::dword_130F4B0 — vibrato random state (image seed 0xFFFFFFFF).
static VIB_NOISE: AtomicU32 = AtomicU32::new(0xFFFF_FFFF);

/// FMOD::MusicChannelMOD song voice words (IDA 0x93028..0x93304).
#[derive(Clone, Default)]
pub struct MusicSongVoice {
    pub period: u32,
    pub volume: u32,
    pub trem_volume: u32,
    pub vib_offset: u32,
    pub flags: u8,
}

/// FMOD::MusicChannelMOD — effect channel (IDA 0x93028).
#[derive(Clone, Default)]
pub struct MusicChannelMod {
    pub voice: MusicSongVoice,
    pub target_period: u32,
    pub porta_step: u8,
    pub vib_pos: i8,
    pub vib_rate: u8,
    pub vib_depth: u8,
    pub trem_pos: i8,
    pub trem_rate: u8,
    pub trem_depth: u8,
    pub wave_form: u8,
}

/// FMOD codec wide-format words at +256/+260 off the word-8 state (IDA 0x92bf4).
#[derive(Clone, Default)]
pub struct CodecWide {
    pub format: u32,
    pub channels: u32,
}

/// FMOD::CodecMIDI host state (word indices are u32 slots from this).
#[derive(Clone, Default)]
pub struct CodecMidi {
    pub system_present: bool,
    pub crit_id: u32,
    pub wide: CodecWide,
    pub sound_present: bool,
    pub track_count: i32,
    pub tick_pos: u32,
    pub tick_step: u32,
    pub time_delta: f32,
    pub beat_acc: f32,
    pub buffered: u32,
    pub render_count: u32,
}

/// FMOD_CODEC_STATE view: 28-byte header then the codec (IDA 0x92b98: -28).
#[repr(C)]
pub struct CodecStateMidi {
    pub _header: [u8; 28],
    pub pub_codec: CodecMidi,
}

impl Default for CodecStateMidi {
    fn default() -> Self {
        CodecStateMidi { _header: [0; 28], pub_codec: CodecMidi::default() }
    }
}

/// FMOD::CodecMOD host state (close path + update-path numeric fields for the
/// deferred update cluster 0x936dc/0x93de4/0x94674/0x94790/0x94844/0x94850).
#[derive(Clone, Default)]
pub struct CodecMod {
    pub loop_counter: u32,
    pub loop_total: u32,
    pub row: u32,
    pub order: u32,
    pub loop_start: u32,
    pub pending_row: i32,
    pub pending_order: i32,
    pub restart_order: u32,
    pub order_count: u32,
    pub sample_pos: u32,
    pub sample_step: u32,
    pub saved_2172: u8,
    pub stopping: u8,
    pub stopped: u8,
    pub channel_pool: bool,
    pub codec_data: bool,
    pub samples_live: Vec<bool>,
    pub sample_data: Option<Vec<u8>>,
    pub pattern_data: Option<Vec<u8>>,
    pub patterns: Vec<Option<Vec<u8>>>,
    pub instruments: Vec<Option<Vec<u8>>>,
    pub header: Option<Vec<u8>>,
    pub tail: Option<Vec<u8>>,
}

/// FMOD_CODEC_STATE view for MOD (IDA 0x935bc: -28).
#[repr(C)]
pub struct CodecStateMod {
    pub _header: [u8; 28],
    pub pub_codec: CodecMod,
}

impl Default for CodecStateMod {
    fn default() -> Self {
        CodecStateMod { _header: [0; 28], pub_codec: CodecMod::default() }
    }
}

/// One MPEG decoder lane: seek table + VBR estimator bytes (IDA 0x96194..0x964b8).
#[derive(Clone, Default)]
pub struct MpegLane {
    pub seek_table: Vec<u32>,
    pub seek_len: u32,
    pub marker: i32,
    pub flag1: u32,
    pub vbr_curve: Vec<u8>,
    pub vbr_mode: u8,
}

/// MPEG sync-point list entry, 36-byte stride (IDA 0x95f28..0x95f50).
#[derive(Clone, Copy, Default)]
pub struct MpegSyncEntry {
    pub id: u32,
    pub pos: u32,
}

/// FMOD::CodecMPEG host state (word indices are u32 slots from this).
#[derive(Clone, Default)]
pub struct CodecMpeg {
    pub file_handle: u32,
    pub file_base: u32,
    pub flags: u32,
    pub layer_flags: u32,
    pub wide65: u32,
    pub wide67: u32,
    pub wide68: u32,
    pub total_pcm: u32,
    pub sync_list: Vec<MpegSyncEntry>,
    pub frame_out: Option<Vec<u8>>,
    pub frame_tag: Option<Vec<u8>>,
    pub aux_buf: Option<Vec<u8>>,
    pub tag_buf: Option<Vec<u8>>,
    pub lane_count: u32,
    pub lanes: Vec<MpegLane>,
}

/// FMOD_CODEC_STATE view for MPEG (IDA 0x95ed4/0x96118/0x964dc: -28, -7 words).
#[repr(C)]
pub struct CodecStateMpeg {
    pub _header: [u8; 28],
    pub pub_codec: CodecMpeg,
}

impl Default for CodecStateMpeg {
    fn default() -> Self {
        CodecStateMpeg { _header: [0; 28], pub_codec: CodecMpeg::default() }
    }
}

/// FMOD_SOUND host carrier for soundCreate (IDA 0x95f8c vtable +148 target).
#[derive(Clone, Default)]
pub struct MpegSoundOut {
    pub sync_points: Vec<MpegSyncEntry>,
}

/// FMOD codec description table (IDA 0x935c4/0x964e4, 0x7C bytes cleared then
/// filled; slots hold callee addresses on target, symbols on host).
#[derive(Clone, Default)]
pub struct FmodCodecDesc {
    pub name: &'static str,
    pub version: u32,
    pub format_tag: u32,
    pub channels: u32,
    pub open: &'static str,
    pub close: &'static str,
    pub read: &'static str,
    pub get_length: Option<&'static str>,
    pub set_position: &'static str,
    pub get_position: Option<&'static str>,
    pub extra_a: Option<&'static str>,
    pub extra_b: Option<&'static str>,
    pub extra_c: Option<&'static str>,
    pub extra_d: Option<&'static str>,
    pub extra_e: Option<&'static str>,
    pub block_count: u32,
    pub block_bytes: u32,
}

static MODCODEC: LazyLock<FmodCodecDesc> = LazyLock::new(|| FmodCodecDesc {
    name: "FMOD MOD Codec", // IDA 0x935f0
    version: 65792, // IDA 0x935f8
    format_tag: 1794, // IDA 0x93600
    channels: 1, // IDA 0x93608
    open: "FMOD::CodecMOD::openCallback", // IDA 0x93614
    close: "FMOD::CodecMOD::closeCallback", // IDA 0x93620
    read: "FMOD::CodecMOD::readCallback", // IDA 0x9362c
    get_length: Some("FMOD::MusicSong::getLengthCallback"), // IDA 0x93638
    set_position: "FMOD::CodecMOD::setPositionCallback", // IDA 0x93644
    get_position: Some("FMOD::MusicSong::getPositionCallback"), // IDA 0x93650
    extra_a: Some("FMOD::MusicSong::getMusicNumChannelsCallback"), // IDA 0x9365c
    extra_b: Some("FMOD::MusicSong::setMusicChannelVolumeCallback"), // IDA 0x93668
    extra_c: Some("FMOD::MusicSong::getMusicChannelVolumeCallback"), // IDA 0x93674
    extra_d: Some("FMOD::MusicSong::setMusicSpeedCallback"), // IDA 0x93680
    extra_e: Some("FMOD::MusicSong::getMusicSpeedCallback"), // IDA 0x9368c
    block_count: 12, // IDA 0x93694
    block_bytes: 3824, // IDA 0x9369c
});
static MPEGCODEC: LazyLock<FmodCodecDesc> = LazyLock::new(|| FmodCodecDesc {
    name: "FMOD MPEG Codec", // IDA 0x96510
    version: 65792, // IDA 0x96518
    format_tag: 10, // IDA 0x96520
    channels: 0,
    open: "FMOD::CodecMPEG::openCallback", // IDA 0x9652c
    close: "FMOD::CodecMPEG::closeCallback", // IDA 0x96538
    read: "FMOD::CodecMPEG::readCallback", // IDA 0x96544
    get_length: None,
    set_position: "FMOD::CodecMPEG::setPositionCallback", // IDA 0x96550
    get_position: None,
    extra_a: Some("FMOD::CodecMPEG::soundCreateCallback"), // IDA 0x9655c
    extra_b: Some("FMOD::CodecMPEG::resetCallback"), // IDA 0x96568
    extra_c: None,
    extra_d: None,
    extra_e: None,
    block_count: 13, // IDA 0x96570
    block_bytes: 300, // IDA 0x96578
});

/// Null-callback carrier: the original adjusts null by -28 and faults; the
/// host returns FMOD_OK so unplugged graphs stay quiet.
fn null_codec_ok() -> i32 {
    FMOD_OK
}
/// IDA 0x92b88 FMOD::CodecMIDI::play (sibling; host: rewind tick).
fn music_song_play_midi(codec: &mut CodecMidi) {
    codec.tick_pos = 0;
}
/// IDA 0x93298..0x932a8 tremolo negative-position branch (LABEL_11).
fn tremolo_negative(ch: &mut MusicChannelMod, delta: i32) {
    let vol = ch.voice.volume as i32;
    let mut v = vol;
    if (vol.wrapping_sub(delta) as u32 & 0x8000) == 0 {
        v = delta;
    }
    ch.voice.trem_volume = v as u32;
}
/// IDA 0x93244..0x9326c tremolo position advance + flags.
fn tremolo_advance(ch: &mut MusicChannelMod, pos: i8) {
    let next = pos.wrapping_add(ch.trem_rate as i8);
    ch.trem_pos = if next > 31 { next.wrapping_sub(64) } else { next };
    ch.voice.flags |= 2;
}
/// Lane 0 accessor: a null word68 faults on target; the host materializes it.
fn mpeg_lane0(codec: &mut CodecMpeg) -> &mut MpegLane {
    // BUG(host): original reads through the null inner pointer and faults.
    if codec.lanes.is_empty() {
        codec.lanes.push(MpegLane::default());
    }
    &mut codec.lanes[0]
}
/// VBR estimator curve byte (IDA +18628 base, +18727 tail at index 99).
fn mpeg_curve_at(curve: &[u8], index: usize) -> f32 {
    // BUG(host): the target reads adjacent image bytes past the table; the
    // host clamps to the modeled curve.
    let last = curve.len().saturating_sub(1);
    curve.get(index.min(last)).copied().unwrap_or(0) as f32
}
/// IDA 0x9630c..0x96348 LABEL_38/39: blend toward 256.0, scale, optional +total.
fn mpeg_vbr_blend_256(wide: u32, top: f32, frac: f32, total: u32, add_total: bool) -> (u32, u32) {
    let blend = (256.0 - top) * frac + top; // IDA 0x9631c
    let scaled = blend * f32::from_bits(998244352); // IDA 0x9632c
    let mut foff = (scaled * wide as f32) as u32; // IDA 0x9633c
    if add_total {
        foff = foff.wrapping_add(total); // IDA LABEL_39 at 0x96348
    }
    (foff, wide)
}
/// IDA 0x9640c..0x9643c LABEL_53: blend between adjacent curve bytes, scale.
fn mpeg_vbr_blend_curve(wide: u32, lo: f32, hi: f32, frac: f32, total: u32, extra: i32) -> (u32, u32) {
    let blend = (hi - lo) * frac + lo; // IDA 0x9641c
    let scaled = blend * f32::from_bits(998244352); // IDA 0x96428
    let foff = (scaled * wide as f32) as u32; // IDA 0x96438
    if extra > 0 {
        return (foff.wrapping_add(total), wide); // IDA LABEL_39
    }
    (foff, wide) // IDA LABEL_11
}
/// IDA LABEL_31 flat path (0x962a0..0x963cc): byte-flag estimator.
fn mpeg_seek_flat(
    codec: &mut CodecMpeg,
    target: u32,
    bytes: u32,
    frames: u32,
    stride: u32,
    total: u32,
    wide67: u32,
    wide68: u32,
) -> (u32, u32) {
    let lane = mpeg_lane0(codec);
    if lane.vbr_mode == 0 {
        // IDA 0x962a0..0x963cc.
        if target <= bytes.wrapping_div(stride.max(1)) {
            (lane.vbr_mode as u32, wide67) // IDA 0x96444: the flag byte itself
        } else {
            // BUG(host): raw divisor may be 0 on target; guarded here.
            let div = (wide68.wrapping_mul(stride).wrapping_div(total.max(1))).max(1);
            (wide67.wrapping_div(div).wrapping_mul(frames), wide67) // IDA 0x963c8
        }
    } else {
        mpeg_seek_float(codec, target, bytes, stride, total, wide67, wide68)
    }
}
/// IDA 0x962b4..0x964b8 float VBR path inside LABEL_31.
fn mpeg_seek_float(
    codec: &mut CodecMpeg,
    target: u32,
    bytes: u32,
    stride: u32,
    total: u32,
    wide67: u32,
    wide68: u32,
) -> (u32, u32) {
    let lane = mpeg_lane0(codec);
    let curve = lane.vbr_curve.clone();
    if target <= bytes.wrapping_div(stride.max(1)) {
        // IDA 0x962b4/0x963e0..: extra = 0, base curve[0..1], frac 0.
        let lo = mpeg_curve_at(&curve, 0); // IDA 0x963ec
        let hi = mpeg_curve_at(&curve, 1); // IDA 0x9640c
        return mpeg_vbr_blend_curve(wide67, lo, hi, 0.0, total, 0);
    }
    // IDA 0x962d4..0x962dc: scaled position over wide68, 100.0f is 1120403456.
    // BUG(host): raw wide68 may be 0 on a torn header; guarded here.
    let scaled = (target.wrapping_sub(bytes.wrapping_div(stride.max(1))) as f32) / wide68.max(1) as f32 * 100.0;
    if scaled < 0.0 {
        // IDA 0x962e8/0x96474..: same as the below-target case.
        let lo = mpeg_curve_at(&curve, 0);
        let hi = mpeg_curve_at(&curve, 1);
        return mpeg_vbr_blend_curve(wide67, lo, hi, 0.0, total, 0);
    }
    if scaled <= 100.0 {
        // IDA 0x962f4..0x964bc.
        let extra = scaled as i32;
        if extra > 99 {
            // IDA LABEL_37: frac off 99.0f (1120272384), top is the tail byte.
            let top = mpeg_curve_at(&curve, 99); // IDA 0x962fc
            return mpeg_vbr_blend_256(wide67, top, scaled - 99.0, total, true);
        }
        let lo = mpeg_curve_at(&curve, extra as usize); // IDA 0x964a4 bump
        let hi = mpeg_curve_at(&curve, extra as usize + 1);
        let frac = scaled - extra as f32;
        if extra == 99 {
            // IDA LABEL_38: bumped base reads the tail neighborhood.
            return mpeg_vbr_blend_256(wide67, hi, frac, total, true);
        }
        return mpeg_vbr_blend_curve(wide67, lo, hi, frac, total, extra);
    }
    // IDA 0x962f4/0x962fc LABEL_37: over 100 -> frac 1.0 off the tail byte.
    let top = mpeg_curve_at(&curve, 99);
    mpeg_vbr_blend_256(wide67, top, 1.0, total, true)
}
/// IDA 0x9618c..0x961bc seek-table path, falling back to the flat path when
/// the table is empty (null word68+18616 word on target).
fn mpeg_seek_table(
    codec: &mut CodecMpeg,
    target: u32,
    bytes: u32,
    frames: u32,
    stride: u32,
    total: u32,
    wide67: u32,
    wide68: u32,
) -> (u32, u32) {
    let has_table = codec.lanes.first().map(|l| !l.seek_table.is_empty()).unwrap_or(false);
    if !has_table {
        return mpeg_seek_flat(codec, target, bytes, frames, stride, total, wide67, wide68);
    }
    let lane = mpeg_lane0(codec);
    // IDA 0x961a4..0x961b4: clamp the frame index, look up the file offset.
    let mut f = frames;
    if f > lane.seek_len {
        f = lane.seek_len.wrapping_sub(1);
    }
    // BUG(host): the target reads out of bounds past the table end; host clamps.
    let idx = (f as usize).min(lane.seek_table.len().saturating_sub(1));
    (lane.seek_table[idx], wide67) // IDA LABEL_11
}

// --- unported-callee shims (sibling EAs outside this batch; contracts only) ---
/// IDA 0x92aa8 FMOD::CodecMIDITrack::process (sibling; host: no track image).
fn midi_track_process(_index: usize) {}
/// IDA 0x92ac4/0x92aec FMOD::CodecMIDIChannel::update (sibling; host: no-op).
fn midi_channel_update(_index: usize) {}
/// IDA 0x92c90 FMOD::SystemI::flushDSPConnectionRequests (sibling; host: no-op).
fn midi_flush_dsp() {}
/// IDA 0x92c98/0x92de4 FMOD_OS_CriticalSection_Enter/Leave (host: no threads).
fn midi_crit_enter(_id: u32) {}
/// IDA 0x92d60/0x92f18 FMOD_OS_CriticalSection_Leave (host: no threads).
fn midi_crit_leave(_id: u32) {}
/// IDA 0x92cd4 codec vtable +8 render (sibling; host: silence, all consumed).
fn midi_sound_render(_buf: &mut [u8], frames: u32) -> (i32, u32) {
    (FMOD_OK, frames)
}
/// IDA 0x9331c FMOD::MusicSong::stop (sibling; host: stopping flag).
fn music_song_stop_mod(codec: &mut CodecMod) {
    codec.stopping = 1;
}
/// IDA 0x9332c FMOD::ChannelPool::release (sibling; host: presence cleared).
fn channel_pool_release(_present: bool) {}
/// IDA 0x947a4/0x9481c/0x94830 FMOD::MusicSong::play (sibling; host: restart).
fn music_song_play_mod(_codec: &mut CodecMod) {}
/// IDA 0x961d8/0x96464 FMOD::File::seek (sibling; host: success).
fn mpeg_file_seek(_handle: u32, _offset: u32, _whence: u32) -> i32 {
    FMOD_OK
}
/// IDA 0x9622c FMOD::Codec::read (sibling; host: all requested bytes consumed).
fn mpeg_codec_read(_want: u32) -> (i32, u32) {
    (FMOD_OK, _want)
}
/// IDA 0x95fa0 FMOD::SoundI::syncPointFixIndicies (sibling; host: no-op).
fn mpeg_sound_fix_indices(_sound: &mut MpegSoundOut) {}
/// IDA 0x95f8c FMOD::SoundI vtable +148 addSyncPoint (sibling; host: record).
fn mpeg_sound_add_sync_point(sound: &mut MpegSoundOut, id: u32, pos: u32) {
    sound.sync_points.push(MpegSyncEntry { id, pos });
}
/// Bits per sample for the MIDI render format code (IDA 0x92c04/0x92cfc).
fn midi_bits_per_sample(fmt: u32) -> Option<u32> {
    match fmt {
        1 => Some(8),
        2 => Some(16),
        3 => Some(24),
        4 | 5 => Some(32),
        _ => None,
    }
}

// 0xf6f600 — sub_F6F600
#[doc(alias = "sub_F6F600")]
pub fn stub_f6f600() {
    // IDA 0xf6f600: __stub_helper — LDR R12, =0x4F9C; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f60c — sub_F6F60C
#[doc(alias = "sub_F6F60C")]
pub fn stub_f6f60c() {
    // IDA 0xf6f60c: __stub_helper — LDR R12, =0x4FAC; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f618 — sub_F6F618
#[doc(alias = "sub_F6F618")]
pub fn stub_f6f618() {
    // IDA 0xf6f618: __stub_helper — LDR R12, =0x4FBD; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f624 — sub_F6F624
#[doc(alias = "sub_F6F624")]
pub fn stub_f6f624() {
    // IDA 0xf6f624: __stub_helper — LDR R12, =0x4FCC; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f630 — sub_F6F630
#[doc(alias = "sub_F6F630")]
pub fn stub_f6f630() {
    // IDA 0xf6f630: __stub_helper — LDR R12, =0x4FE1; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f63c — sub_F6F63C
#[doc(alias = "sub_F6F63C")]
pub fn stub_f6f63c() {
    // IDA 0xf6f63c: __stub_helper — LDR R12, =0x4FF2; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f648 — sub_F6F648
#[doc(alias = "sub_F6F648")]
pub fn stub_f6f648() {
    // IDA 0xf6f648: __stub_helper — LDR R12, =0x5003; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f654 — sub_F6F654
#[doc(alias = "sub_F6F654")]
pub fn stub_f6f654() {
    // IDA 0xf6f654: __stub_helper — LDR R12, =0x5014; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f660 — sub_F6F660
#[doc(alias = "sub_F6F660")]
pub fn stub_f6f660() {
    // IDA 0xf6f660: __stub_helper — LDR R12, =0x5026; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f66c — sub_F6F66C
#[doc(alias = "sub_F6F66C")]
pub fn stub_f6f66c() {
    // IDA 0xf6f66c: __stub_helper — LDR R12, =0x5037; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f678 — sub_F6F678
#[doc(alias = "sub_F6F678")]
pub fn stub_f6f678() {
    // IDA 0xf6f678: __stub_helper — LDR R12, =0x5049; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f684 — sub_F6F684
#[doc(alias = "sub_F6F684")]
pub fn stub_f6f684() {
    // IDA 0xf6f684: __stub_helper — LDR R12, =0x505A; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f690 — sub_F6F690
#[doc(alias = "sub_F6F690")]
pub fn stub_f6f690() {
    // IDA 0xf6f690: __stub_helper — LDR R12, =0x506D; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f69c — sub_F6F69C
#[doc(alias = "sub_F6F69C")]
pub fn stub_f6f69c() {
    // IDA 0xf6f69c: __stub_helper — LDR R12, =0x5082; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6a8 — sub_F6F6A8
#[doc(alias = "sub_F6F6A8")]
pub fn stub_f6f6a8() {
    // IDA 0xf6f6a8: __stub_helper — LDR R12, =0x5094; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6b4 — sub_F6F6B4
#[doc(alias = "sub_F6F6B4")]
pub fn stub_f6f6b4() {
    // IDA 0xf6f6b4: __stub_helper — LDR R12, =0x50A6; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6c0 — sub_F6F6C0
#[doc(alias = "sub_F6F6C0")]
pub fn stub_f6f6c0() {
    // IDA 0xf6f6c0: __stub_helper — LDR R12, =0x50B7; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6cc — sub_F6F6CC
#[doc(alias = "sub_F6F6CC")]
pub fn stub_f6f6cc() {
    // IDA 0xf6f6cc: __stub_helper — LDR R12, =0x50CD; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6d8 — sub_F6F6D8
#[doc(alias = "sub_F6F6D8")]
pub fn stub_f6f6d8() {
    // IDA 0xf6f6d8: __stub_helper — LDR R12, =0x50DF; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6e4 — sub_F6F6E4
#[doc(alias = "sub_F6F6E4")]
pub fn stub_f6f6e4() {
    // IDA 0xf6f6e4: __stub_helper — LDR R12, =0x50F1; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6f0 — sub_F6F6F0
#[doc(alias = "sub_F6F6F0")]
pub fn stub_f6f6f0() {
    // IDA 0xf6f6f0: __stub_helper — LDR R12, =0x5103; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f6fc — sub_F6F6FC
#[doc(alias = "sub_F6F6FC")]
pub fn stub_f6f6fc() {
    // IDA 0xf6f6fc: __stub_helper — LDR R12, =0x5115; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f708 — sub_F6F708
#[doc(alias = "sub_F6F708")]
pub fn stub_f6f708() {
    // IDA 0xf6f708: __stub_helper — LDR R12, =0x5127; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f714 — sub_F6F714
#[doc(alias = "sub_F6F714")]
pub fn stub_f6f714() {
    // IDA 0xf6f714: __stub_helper — LDR R12, =0x513B; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f720 — sub_F6F720
#[doc(alias = "sub_F6F720")]
pub fn stub_f6f720() {
    // IDA 0xf6f720: __stub_helper — LDR R12, =0x514C; B _stub_helpers (dyld lazy-bind resolver, decompile `return _stub_helpers()`). Host: no dyld — no-op carrier.
}

// 0xf6f72c — sub_F6F72C
#[doc(alias = "sub_F6F72C")]
pub fn stub_f6f72c() -> ! {
    todo!("0xf6f72c sub_F6F72C")
}

// 0xf6f738 — sub_F6F738
#[doc(alias = "sub_F6F738")]
pub fn stub_f6f738() -> ! {
    todo!("0xf6f738 sub_F6F738")
}

// 0xf6f744 — sub_F6F744
#[doc(alias = "sub_F6F744")]
pub fn stub_f6f744() -> ! {
    todo!("0xf6f744 sub_F6F744")
}

// 0xf6f750 — sub_F6F750
#[doc(alias = "sub_F6F750")]
pub fn stub_f6f750() -> ! {
    todo!("0xf6f750 sub_F6F750")
}

// 0xf6f75c — sub_F6F75C
#[doc(alias = "sub_F6F75C")]
pub fn stub_f6f75c() -> ! {
    todo!("0xf6f75c sub_F6F75C")
}

// 0xf6f768 — sub_F6F768
#[doc(alias = "sub_F6F768")]
pub fn stub_f6f768() -> ! {
    todo!("0xf6f768 sub_F6F768")
}

// 0xf6f774 — sub_F6F774
#[doc(alias = "sub_F6F774")]
pub fn stub_f6f774() -> ! {
    todo!("0xf6f774 sub_F6F774")
}

// 0xf6f780 — sub_F6F780
#[doc(alias = "sub_F6F780")]
pub fn stub_f6f780() -> ! {
    todo!("0xf6f780 sub_F6F780")
}

// 0xf6f78c — sub_F6F78C
#[doc(alias = "sub_F6F78C")]
pub fn stub_f6f78c() -> ! {
    todo!("0xf6f78c sub_F6F78C")
}

// 0xf6f798 — sub_F6F798
#[doc(alias = "sub_F6F798")]
pub fn stub_f6f798() -> ! {
    todo!("0xf6f798 sub_F6F798")
}

// 0xf6f7a4 — sub_F6F7A4
#[doc(alias = "sub_F6F7A4")]
pub fn stub_f6f7a4() -> ! {
    todo!("0xf6f7a4 sub_F6F7A4")
}

// 0xf6f7b0 — sub_F6F7B0
#[doc(alias = "sub_F6F7B0")]
pub fn stub_f6f7b0() -> ! {
    todo!("0xf6f7b0 sub_F6F7B0")
}

// 0xf6f7bc — sub_F6F7BC
#[doc(alias = "sub_F6F7BC")]
pub fn stub_f6f7bc() -> ! {
    todo!("0xf6f7bc sub_F6F7BC")
}

// 0xf6f7c8 — sub_F6F7C8
#[doc(alias = "sub_F6F7C8")]
pub fn stub_f6f7c8() -> ! {
    todo!("0xf6f7c8 sub_F6F7C8")
}

// 0xf6f7d4 — sub_F6F7D4
#[doc(alias = "sub_F6F7D4")]
pub fn stub_f6f7d4() -> ! {
    todo!("0xf6f7d4 sub_F6F7D4")
}

// 0xf6f7e0 — sub_F6F7E0
#[doc(alias = "sub_F6F7E0")]
pub fn stub_f6f7e0() -> ! {
    todo!("0xf6f7e0 sub_F6F7E0")
}

// 0xf6f7ec — sub_F6F7EC
#[doc(alias = "sub_F6F7EC")]
pub fn stub_f6f7ec() -> ! {
    todo!("0xf6f7ec sub_F6F7EC")
}

// 0xf6f7f8 — sub_F6F7F8
#[doc(alias = "sub_F6F7F8")]
pub fn stub_f6f7f8() -> ! {
    todo!("0xf6f7f8 sub_F6F7F8")
}

// 0xf6f804 — sub_F6F804
#[doc(alias = "sub_F6F804")]
pub fn stub_f6f804() -> ! {
    todo!("0xf6f804 sub_F6F804")
}

// 0xf6f810 — sub_F6F810
#[doc(alias = "sub_F6F810")]
pub fn stub_f6f810() -> ! {
    todo!("0xf6f810 sub_F6F810")
}

// 0xf6f81c — sub_F6F81C
#[doc(alias = "sub_F6F81C")]
pub fn stub_f6f81c() -> ! {
    todo!("0xf6f81c sub_F6F81C")
}

// 0xf6f828 — sub_F6F828
#[doc(alias = "sub_F6F828")]
pub fn stub_f6f828() -> ! {
    todo!("0xf6f828 sub_F6F828")
}

// 0xf6f834 — sub_F6F834
#[doc(alias = "sub_F6F834")]
pub fn stub_f6f834() -> ! {
    todo!("0xf6f834 sub_F6F834")
}

// 0xf6f840 — sub_F6F840
#[doc(alias = "sub_F6F840")]
pub fn stub_f6f840() -> ! {
    todo!("0xf6f840 sub_F6F840")
}

// 0xf6f84c — sub_F6F84C
#[doc(alias = "sub_F6F84C")]
pub fn stub_f6f84c() -> ! {
    todo!("0xf6f84c sub_F6F84C")
}

// 0xf6f858 — sub_F6F858
#[doc(alias = "sub_F6F858")]
pub fn stub_f6f858() -> ! {
    todo!("0xf6f858 sub_F6F858")
}

// 0xf6f864 — sub_F6F864
#[doc(alias = "sub_F6F864")]
pub fn stub_f6f864() -> ! {
    todo!("0xf6f864 sub_F6F864")
}

// 0xf6f870 — sub_F6F870
#[doc(alias = "sub_F6F870")]
pub fn stub_f6f870() -> ! {
    todo!("0xf6f870 sub_F6F870")
}

// 0xf6f87c — sub_F6F87C
#[doc(alias = "sub_F6F87C")]
pub fn stub_f6f87c() -> ! {
    todo!("0xf6f87c sub_F6F87C")
}

// 0xf6f888 — sub_F6F888
#[doc(alias = "sub_F6F888")]
pub fn stub_f6f888() -> ! {
    todo!("0xf6f888 sub_F6F888")
}

// 0xf6f894 — sub_F6F894
#[doc(alias = "sub_F6F894")]
pub fn stub_f6f894() -> ! {
    todo!("0xf6f894 sub_F6F894")
}

// 0xf6f8a0 — sub_F6F8A0
#[doc(alias = "sub_F6F8A0")]
pub fn stub_f6f8a0() -> ! {
    todo!("0xf6f8a0 sub_F6F8A0")
}

// 0xf6f8ac — sub_F6F8AC
#[doc(alias = "sub_F6F8AC")]
pub fn stub_f6f8ac() -> ! {
    todo!("0xf6f8ac sub_F6F8AC")
}

// 0xf6f8b8 — sub_F6F8B8
#[doc(alias = "sub_F6F8B8")]
pub fn stub_f6f8b8() -> ! {
    todo!("0xf6f8b8 sub_F6F8B8")
}

// 0xf6f8c4 — sub_F6F8C4
#[doc(alias = "sub_F6F8C4")]
pub fn stub_f6f8c4() -> ! {
    todo!("0xf6f8c4 sub_F6F8C4")
}

// --- appended next 60 stubs EA-sorted, FMOD|Sound|Audio (2541 total; 1559 strict remaining) | range 0x91d30..0x9af14 | skeleton batch

// 0x91d30 — __ZN4FMOD9CodecMIDI12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
#[doc(alias = "FMOD::CodecMIDI::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_91d30() -> ! {
    todo!("0x91d30 FMOD::CodecMIDI::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x92a68 — __ZN4FMOD9CodecMIDI12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
#[doc(alias = "FMOD::CodecMIDI::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_92a68() -> ! {
    todo!("0x92a68 FMOD::CodecMIDI::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x92a74 — __ZN4FMOD9CodecMIDI6updateEb
// type: __int64 __fastcall(FMOD::CodecMIDI *this, bool)
#[doc(alias = "FMOD::CodecMIDI::update(bool)")]
pub fn stub_92a74(codec: &mut CodecMidi, _a2: bool) -> i64 {
    // IDA 0x92a74: process each track (0x92a8c..0x92abc), update channel 0
    // (0x92ac4) then channels 1..15 (0x92ac8..0x92aec), accumulate the float
    // beat words (0x92b08..0x92b20) and the int tick word (0x92b30).
    for i in 0..codec.track_count.max(0) as usize {
        midi_track_process(i);
    }
    midi_channel_update(0);
    for i in 1..16 {
        let _stride = 47 * i;
        midi_channel_update(i);
    }
    codec.beat_acc += codec.time_delta;
    codec.tick_pos = codec.tick_pos.wrapping_add(codec.tick_step);
    // IDA 0x92b34: exact 64-bit constant.
    0x309C_0000_0000_i64
}

// 0x92b38 — __ZN4FMOD9CodecMIDI19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMIDI *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMIDI::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_92b38(codec: &mut CodecMidi, _a2: i32, pos: u32, _a4: u32) -> i32 {
    // IDA 0x92b38: same tick -> done (0x92b54); rewind via play when seeking
    // back (0x92b58..0x92b8c); fast-forward with update(1) (0x92b60..0x92b78).
    if codec.tick_pos != pos {
        if codec.tick_pos > pos {
            music_song_play_midi(codec);
        }
        while pos > codec.tick_pos {
            stub_92a74(codec, true);
        }
    }
    FMOD_OK // IDA 0x92b80
}

// 0x92b94 — __ZN4FMOD9CodecMIDI19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMIDI *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMIDI::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_92b94(state: Option<&mut CodecStateMidi>, a2: i32, a3: u32, a4: u32) -> i32 {
    // IDA 0x92b94: container_of -28 (0x92b98) then setPositionInternal.
    match state {
        Some(s) => stub_92b38(&mut s.pub_codec, a2, a3, a4),
        None => null_codec_ok(),
    }
}

// 0x92ba0 — __ZN4FMOD9CodecMIDI12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *this, char *, size_t, unsigned int *)
#[doc(alias = "FMOD::CodecMIDI::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_92ba0(codec: &mut CodecMidi, mut out: Option<&mut [u8]>, len: usize, written: Option<&mut u32>) -> i32 {
    // IDA 0x92ba0: zero the buffer (0x92be4); sample count from the format
    // code (0x92bf8..0x92f9c); render chunks until the count is met
    // (0x92c4c..0x92e34); null error-pointer return is FMOD_OK on host.
    // BUG(host): error returns are pointers on target; they collapse to
    // FMOD_OK/nonzero i32 here. Stack-garbage v23/v25/v5 seed as 0.
    let crit = codec.crit_id;
    let channels = codec.wide.channels;
    let format = codec.wide.format;
    if let Some(buf) = out.as_deref_mut() {
        let n = buf.len().min(len);
        buf[..n].fill(0);
    }
    // IDA 0x92bf8: frames wanted; v24 == 0 skips the switch (0x92bf8).
    let mut want: u32 = 0; // IDA v23
    if channels != 0 {
        want = match midi_bits_per_sample(format) {
            // IDA 0x92c20..0x92c48: samples = 8 * len / bits, per channel.
            Some(bits) => ((8 * len as u64 / bits as u64) as u32).wrapping_div(channels),
            None => match format {
                // IDA 0x92edc..0x92f9c.
                0 => 0,
                6 => ((14 * len) >> 3) as u32,
                7 => ((len << 6) / 0x24) as u32,
                8 => ((28 * len) >> 4) as u32,
                9 | 10 | 11 => len as u32,
                // BUG(host): default leaves v23 as stack garbage; host uses 0.
                _ => 0,
            },
        };
        if midi_bits_per_sample(format).is_none() && !matches!(format, 0 | 6 | 7 | 8 | 9 | 10 | 11) {
            want = 0;
        } else if midi_bits_per_sample(format).is_some() {
            // already divided above
        } else if matches!(format, 6 | 7 | 8) {
            want = want.wrapping_div(channels);
        }
    }
    let mut have: u32 = 0; // IDA v11
    let mut left = codec.buffered; // IDA v9
    let mut offset: usize = 0;
    let mut carry: u32 = 0; // IDA v25
    if want != 0 {
        // IDA 0x92c4c..0x92e34 chunk loop (while(2) at 0x92c70).
        'chunk: loop {
            let mut frames = left; // IDA v12/v27
            if left.wrapping_add(have) > want {
                frames = want - have;
            }
            midi_flush_dsp();
            midi_crit_enter(crit);
            // IDA 0x92ca4: null buffer skips the render; else vtable +8.
            let render_err = if let Some(buf) = out.as_deref_mut() {
                let len = buf.len();
                let end = offset.saturating_add(len.saturating_sub(offset)).min(len);
                let start = offset.min(len);
                let (err, _got) = midi_sound_render(&mut buf[start..end], frames);
                if err == FMOD_OK {
                    codec.render_count = codec.render_count.wrapping_add(1);
                    None
                } else {
                    Some(err)
                }
            } else {
                None
            };
            if let Some(err) = render_err {
                midi_crit_leave(crit);
                return err; // IDA 0x92f18: leave then return v14
            }
            // IDA LABEL_13: byte accounting by format (0x92cec..0x92d58).
            let mut bytes = frames; // IDA v15/v17
            if let Some(bits) = midi_bits_per_sample(format) {
                // IDA 0x92dac..0x92db4.
                bytes = channels.wrapping_mul((bits as u64 * frames as u64 >> 3) as u32);
                midi_crit_leave(crit);
                have = have.wrapping_add(frames);
                left = left.wrapping_sub(frames);
                if have >= want {
                    break 'chunk; // IDA LABEL_23 at 0x92e00
                }
            } else {
                bytes = match format {
                    0 => 0, // IDA 0x92d54
                    6 => channels * 8 * ((frames + 13) / 0xE), // IDA 0x92ec0
                    7 => channels * 36 * ((frames + 63) >> 6), // IDA 0x92e98
                    8 => channels * 16 * ((frames + 27) / 0x1C), // IDA 0x92e74
                    9 | 10 | 11 => bytes, // IDA LABEL_16: stale v15; host 0-seed
                    // BUG(host): default reads stale v25; host carries 0-seed.
                    _ => carry,
                };
                midi_crit_leave(crit);
                have = have.wrapping_add(frames);
                left = left.wrapping_sub(frames);
                if have >= want {
                    break 'chunk; // IDA LABEL_23 at 0x92d78
                }
            }
            // IDA LABEL_18 (0x92d7c..0x92d88).
            offset = offset.saturating_add(bytes as usize);
            carry = bytes;
            if left == 0 {
                // IDA LABEL_19 (0x92d8c..0x92da4).
                let err = stub_92a74(codec, true);
                // BUG(0x92d9c): (u32)0x309C00000000 is 0, so the error return
                // never fires on target either; kept verbatim.
                if (err as u32) != 0 {
                    return (err as u32) as i32;
                }
                left = codec.tick_step;
            }
        }
        // IDA LABEL_23 (0x92e04..0x92e24).
        codec.buffered = left;
        if let Some(w) = written {
            *w = len as u32;
        }
    }
    FMOD_OK // IDA 0x92e24/0x92e34
}

// 0x92fac — __ZN4FMOD9CodecMIDI12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *, char *, size_t, unsigned int *)
#[doc(alias = "FMOD::CodecMIDI::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_92fac(state: Option<&mut CodecStateMidi>, out: Option<&mut [u8]>, len: usize, written: Option<&mut u32>) -> i32 {
    // IDA 0x92fac: container_of -28 (0x92fb0) then readInternal.
    match state {
        Some(s) => stub_92ba0(&mut s.pub_codec, out, len, written),
        None => null_codec_ok(),
    }
}

// 0x9301c — __GLOBAL__I__ZN4FMOD9midicodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::midicodec")]
pub fn stub_9301c() {
    // IDA 0x9301c: __static_initialization_and_destruction_0(1, 0xFFFF) runs
    // the midicodec static ctor (sibling cluster 0xa8400..; host: G_SINE_TABLE
    // and codec descs are const/LazyLock-initialized, nothing to run).
}

// 0x93028 — __ZN4FMOD15MusicChannelMOD10portamentoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::portamento(void)")]
pub fn stub_93028(ch: &mut MusicChannelMod) -> i32 {
    // IDA 0x93028: slide voice.period toward target_period by 4 * porta_step
    // with a clamp on each side (0x93038..0x93080); flags |= 1 (0x93084).
    let step = 4 * ch.porta_step as u32;
    if ch.voice.period < ch.target_period {
        // IDA 0x93044..0x93058.
        let slid = ch.voice.period.wrapping_add(step);
        ch.voice.period = slid;
        if slid > ch.target_period {
            ch.voice.period = ch.target_period;
        }
    }
    // IDA 0x9304c/0x93064: period and target reloaded before the down-slide.
    if ch.voice.period > ch.target_period {
        // IDA 0x93070..0x93080.
        let slid = ch.voice.period.wrapping_sub(step);
        ch.voice.period = slid;
        if slid < ch.target_period {
            ch.voice.period = ch.target_period;
        }
    }
    ch.voice.flags |= 1; // IDA 0x93084
    FMOD_OK // IDA 0x93094
}

// 0x93098 — __ZN4FMOD15MusicChannelMOD7vibratoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::vibrato(void)")]
pub fn stub_93098(ch: &mut MusicChannelMod) -> i32 {
    // IDA 0x93098: waveform = wave_form & 3 (0x930b8); depth at +696,
    // position +694, rate +695; position is a signed char, the table index
    // its low 5 bits (0x930b0).
    let pos = ch.vib_pos;
    let idx = (pos as i32 & 0x1F) as usize;
    let depth = ch.vib_depth as i32;
    let (v4, delta) = match ch.wave_form & 3 {
        // IDA 0x931c0: sine.
        0 => (pos as i32, 4 * ((depth * G_SINE_TABLE[idx] as i32) >> 7)),
        // IDA 0x93184..0x931a0: ramp.
        1 => {
            let mut ramp = 8 * idx as i32;
            if pos < 0 {
                ramp ^= 0xFF;
            }
            (pos as i32, 4 * ((depth * ramp) >> 7))
        }
        // IDA 0x93178: square.
        2 => (pos as i32, 4 * ((255 * depth) >> 7)),
        // IDA 0x93140..0x93160: random (LCG, BYTE2 of the state).
        _ => {
            let state = VIB_NOISE.load(Ordering::Relaxed).wrapping_mul(214013).wrapping_add(2531011);
            VIB_NOISE.store(state, Ordering::Relaxed);
            let noise = ((state >> 16) & 0xFF) as i8 as i32;
            (pos as i32, 4 * ((depth * noise) >> 7))
        }
    };
    // IDA 0x930d8..0x930e4: negative position negates the offset.
    ch.voice.vib_offset = (if v4 >= 0 { delta } else { -delta }) as u32;
    // IDA 0x930f0..0x93108: advance with signed-char wrap at 32.
    let next = pos.wrapping_add(ch.vib_rate as i8);
    ch.vib_pos = if next > 31 { next.wrapping_sub(64) } else { next };
    ch.voice.flags |= 1; // IDA 0x93118
    FMOD_OK // IDA 0x9311c
}

// 0x931dc — __ZN4FMOD15MusicChannelMOD7tremoloEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
#[doc(alias = "FMOD::MusicChannelMOD::tremolo(void)")]
pub fn stub_931dc(ch: &mut MusicChannelMod) -> i32 {
    // IDA 0x931dc: waveform = (wave_form >> 4) & 3 (0x93200); depth at +700,
    // position +698, rate +699. Case 3 skips LABEL_3 (0x93288..0x93294).
    let pos = ch.trem_pos;
    let idx = (pos as i32 & 0x1F) as usize;
    let depth = ch.trem_depth as i32;
    let wave = (ch.wave_form >> 4) & 3;
    // (sign, delta); sign picks the LABEL_3 branch (0x93224).
    let (sign, delta) = match wave {
        // IDA 0x932e4: sine.
        0 => (pos as i32, (depth * G_SINE_TABLE[idx] as i32) >> 6),
        // IDA 0x932b0..0x932c8: ramp.
        1 => {
            let mut ramp = 8 * idx as i32;
            if pos < 0 {
                ramp ^= 0xFF;
            }
            (pos as i32, (depth * ramp) >> 6)
        }
        // IDA 0x932fc: square.
        2 => (pos as i32, (255 * depth) >> 6),
        // IDA 0x93288..0x93294: sine with the negative-position branch taken
        // directly (LABEL_11) instead of via LABEL_3.
        _ => {
            let d = (depth * G_SINE_TABLE[idx] as i32) >> 6;
            if pos < 0 {
                tremolo_negative(ch, d);
                tremolo_advance(ch, pos);
                return FMOD_OK;
            }
            (pos as i32, d)
        }
    };
    // IDA LABEL_3 (0x93224): negative position ducks toward the delta.
    if sign < 0 {
        tremolo_negative(ch, delta);
    } else {
        // IDA LABEL_4 (0x93228..0x93238): positive side clamps at 64.
        let vol = ch.voice.volume as i32;
        let mut d = delta;
        if d + vol > 64 {
            d = 64 - vol;
        }
        ch.voice.trem_volume = d as u32;
    }
    tremolo_advance(ch, pos);
    FMOD_OK
}

// 0x93310 — __ZN4FMOD8CodecMOD13closeInternalEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::closeInternal(void)")]
pub fn stub_93310(codec: &mut CodecMod) -> i32 {
    // IDA 0x93310: MusicSong::stop (0x9331c); release the channel pool
    // (0x93320..0x93334); delete the word-133 object (0x93338..0x93358);
    // delete each live sample slot, re-reading the count (0x9335c..0x933a4);
    // MemPool::free each heap slot and null it (0x933ac..0x93574, host: drop).
    music_song_stop_mod(codec);
    if codec.channel_pool {
        channel_pool_release(true);
        codec.channel_pool = false;
    }
    codec.codec_data = false;
    for live in &mut codec.samples_live {
        *live = false;
    }
    codec.sample_data = None;
    codec.pattern_data = None;
    for slot in &mut codec.patterns {
        *slot = None;
    }
    for slot in &mut codec.instruments {
        *slot = None;
    }
    codec.header = None;
    codec.tail = None;
    FMOD_OK // IDA 0x9357c
}

// 0x935b8 — __ZN4FMOD8CodecMOD13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMOD *)
#[doc(alias = "FMOD::CodecMOD::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_935b8(state: Option<&mut CodecStateMod>) -> i32 {
    // IDA 0x935b8: container_of -28 (0x935bc) then closeInternal.
    match state {
        Some(s) => stub_93310(&mut s.pub_codec),
        None => null_codec_ok(),
    }
}

// 0x935c4 — __ZN4FMOD8CodecMOD16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::getDescriptionEx(void)")]
pub fn stub_935c4() -> &'static FmodCodecDesc {
    // IDA 0x935c4: memset 0x7C (0x935e0) then fill name/version/callback
    // slots (0x935f0..0x9369c); host table is const-initialized once.
    &MODCODEC // IDA 0x936a0
}

// 0x936dc — __ZN4FMOD8CodecMOD13updateEffectsEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::updateEffects(void)")]
pub fn stub_936dc() -> ! {
    todo!("0x936dc FMOD::CodecMOD::updateEffects(void)")
}

// 0x93de4 — __ZN4FMOD8CodecMOD10updateNoteEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
#[doc(alias = "FMOD::CodecMOD::updateNote(bool)")]
pub fn stub_93de4() -> ! {
    todo!("0x93de4 FMOD::CodecMOD::updateNote(bool)")
}

// 0x94674 — __ZN4FMOD8CodecMOD6updateEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
#[doc(alias = "FMOD::CodecMOD::update(bool)")]
pub fn stub_94674() -> ! {
    todo!("0x94674 FMOD::CodecMOD::update(bool)")
}

// 0x94790 — __ZN4FMOD8CodecMOD19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMOD *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMOD::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_94790() -> ! {
    todo!("0x94790 FMOD::CodecMOD::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x94844 — __ZN4FMOD8CodecMOD19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMOD *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMOD::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_94844() -> ! {
    todo!("0x94844 FMOD::CodecMOD::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x94850 — __ZN4FMOD8CodecMOD15calculateLengthEv
// type: int __fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::calculateLength(void)")]
pub fn stub_94850() -> ! {
    todo!("0x94850 FMOD::CodecMOD::calculateLength(void)")
}

// 0x948b4 — __ZN4FMOD8CodecMOD12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
#[doc(alias = "FMOD::CodecMOD::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_948b4() -> ! {
    todo!("0x948b4 FMOD::CodecMOD::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x95a74 — __ZN4FMOD8CodecMOD12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
#[doc(alias = "FMOD::CodecMOD::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_95a74() -> ! {
    todo!("0x95a74 FMOD::CodecMOD::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x95a80 — __ZN4FMOD8CodecMOD12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMOD::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_95a80() -> ! {
    todo!("0x95a80 FMOD::CodecMOD::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x95e64 — __ZN4FMOD8CodecMOD12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMOD::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_95e64() -> ! {
    todo!("0x95e64 FMOD::CodecMOD::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x95ebc — __GLOBAL__I__ZN4FMOD8modcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::modcodec")]
pub fn stub_95ebc() {
    // IDA 0x95ebc: __static_initialization_and_destruction_0(1, 0xFFFF) runs
    // the modcodec static ctor (sibling cluster 0xa8400..; host: MODCODEC is
    // LazyLock-initialized, nothing to run).
}

// 0x95ec8 — __ZN4FMOD9CodecMPEG13resetCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
#[doc(alias = "FMOD::CodecMPEG::resetCallback(FMOD_CODEC_STATE *)")]
pub fn stub_95ec8(state: Option<&mut CodecStateMpeg>) -> i32 {
    // IDA 0x95ec8: container_of -7 words (0x95ed4) then resetFrame (0x95edc).
    match state {
        Some(s) => stub_986f8(&mut s.pub_codec),
        None => null_codec_ok(),
    }
}

// 0x95ee0 — __ZN4FMOD9CodecMPEG19soundCreateInternalEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *this)
#[doc(alias = "FMOD::CodecMPEG::soundCreateInternal(int,FMOD_SOUND *)")]
pub fn stub_95ee0(codec: &mut CodecMpeg, _a2: i32, sound: &mut MpegSoundOut) -> i32 {
    // IDA 0x95ee0: for each of the word-284 entries at word-280 (stride 36),
    // build the sync id from bytes 24..27 and the position from bytes 16..19
    // (0x95f28..0x95f8c); fix indices (0x95fa0); free the list (0x95fc4..0x95fcc).
    for entry in codec.sync_list.drain(..).collect::<Vec<_>>() {
        mpeg_sound_add_sync_point(sound, entry.id, entry.pos);
    }
    mpeg_sound_fix_indices(sound);
    FMOD_OK // IDA 0x95fdc
}

// 0x95fe8 — __ZN4FMOD9CodecMPEG19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecMPEG::soundCreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
pub fn stub_95fe8(state: Option<&mut CodecStateMpeg>, kind: i32, sound: &mut MpegSoundOut) -> i32 {
    // IDA 0x95fe8: container_of -28 (0x95fec) then soundCreateInternal.
    match state {
        Some(s) => stub_95ee0(&mut s.pub_codec, kind, sound),
        None => null_codec_ok(),
    }
}

// 0x95ff4 — __ZN4FMOD9CodecMPEG13closeInternalEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::closeInternal(void)")]
pub fn stub_95ff4(codec: &mut CodecMpeg) -> i32 {
    // IDA 0x95ff4: free word73, zero words 57/73 (0x95ffc..0x96034); free
    // word16 (0x96038..0x96068); free the +18616 seek table, free word69,
    // zero words 68/69 (0x9606c..0x960e8). MemPool::free is drop on host.
    // BUG(host): the target only nulls word57 without freeing it; the host
    // drops the buffer with the same observable result (slot empty).
    codec.frame_tag = None;
    codec.frame_out = None;
    codec.tag_buf = None;
    if !codec.lanes.is_empty() {
        codec.lanes[0].seek_table.clear();
        codec.lanes.clear();
    }
    FMOD_OK // IDA 0x960f0
}

// 0x96114 — __ZN4FMOD9CodecMPEG13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
#[doc(alias = "FMOD::CodecMPEG::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_96114(state: Option<&mut CodecStateMpeg>) -> i32 {
    // IDA 0x96114: container_of -28 (0x96118) then closeInternal.
    match state {
        Some(s) => stub_95ff4(&mut s.pub_codec),
        None => null_codec_ok(),
    }
}

// 0x96120 — __ZN4FMOD9CodecMPEG19setPositionInternalEijj
// type: int __fastcall(FMOD::File **this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMPEG::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_96120(codec: &mut CodecMpeg, _a2: i32, pos: u32, mode: u32) -> i32 {
    // IDA 0x96120: mode 8 is a raw byte seek (0x96140..0x96464); otherwise
    // frames/bytes derive from wide65/total_pcm (0x9614c..0x96178), then the
    // seek-table path (0x9618c..0x961bc) or the flat/float path (LABEL_31,
    // 0x962a0..0x96480) feeds LABEL_11 (0x961bc..0x9628c): seek + drain.
    if mode == 8 {
        return mpeg_file_seek(codec.file_handle, codec.file_base.wrapping_add(pos), 0);
    }
    // BUG(host): the target divides by the raw total_pcm word (nonzero in
    // practice); the host guards the zero case deterministically.
    let total = codec.total_pcm.max(1);
    let stride = codec.wide65.wrapping_mul(2); // IDA v9
    let prod = stride.wrapping_mul(pos);
    let mut frames = prod.wrapping_div(total); // IDA v11
    let mut bytes = prod; // IDA v10
    let mut target = pos; // IDA v8
    if bytes == 0 {
        target = 0; // IDA 0x96170
    }
    let wide67 = codec.wide67;
    let wide68 = codec.wide68;
    let (foff, lane_frames) = if bytes != 0 {
        // IDA 0x96360..0x96378: skip-ahead adjustment.
        let mut skip = if codec.flags & 2 != 0 { 3 } else { 9 };
        if skip >= frames {
            skip = frames;
        }
        frames -= skip;
        bytes = prod.wrapping_rem(total).wrapping_add(total.wrapping_mul(skip));
        if codec.layer_flags & 0x4000 == 0 {
            mpeg_seek_flat(codec, target, bytes, frames, stride, total, wide67, wide68)
        } else {
            mpeg_seek_table(codec, target, bytes, frames, stride, total, wide67, wide68)
        }
    } else if codec.layer_flags & 0x4000 == 0 {
        mpeg_seek_flat(codec, target, bytes, frames, stride, total, wide67, wide68)
    } else {
        mpeg_seek_table(codec, target, bytes, frames, stride, total, wide67, wide68)
    };
    // IDA LABEL_11 (0x961bc..0x9628c).
    let lane_total = lane_frames;
    let mut fpos = codec.file_base;
    if foff.wrapping_add(fpos) <= lane_total.wrapping_add(fpos) {
        fpos = fpos.wrapping_add(foff);
    }
    let mut result = mpeg_file_seek(codec.file_handle, fpos, 0);
    if result == FMOD_OK {
        if codec.flags & 2 == 0 {
            codec.flags |= 4; // IDA 0x961f0
        }
        let mut rest = bytes;
        while rest != 0 {
            // IDA 0x96210..0x9622c.
            let chunk = if rest >= 0x1200 { 4608 } else { rest };
            let (err, got) = mpeg_codec_read(chunk);
            result = err;
            if result != FMOD_OK {
                break;
            }
            // IDA 0x9623c..0x96258: zero read counts as the full chunk.
            let mut used = got;
            if used == 0 {
                used = chunk;
            }
            if used > rest {
                rest = 0;
            } else {
                rest -= used;
            }
            // IDA 0x9625c..0x96274.
            if (codec.frame_out.is_none() || codec.aux_buf.is_none()) && rest < used {
                break;
            }
        }
        codec.flags &= !4; // IDA 0x96280
    }
    result // IDA 0x9628c
}

// 0x964d8 — __ZN4FMOD9CodecMPEG19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::File **, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecMPEG::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_964d8(state: Option<&mut CodecStateMpeg>, a2: i32, a3: u32, a4: u32) -> i32 {
    // IDA 0x964d8: container_of -7 words (0x964dc) then setPositionInternal.
    match state {
        Some(s) => stub_96120(&mut s.pub_codec, a2, a3, a4),
        None => null_codec_ok(),
    }
}

// 0x964e4 — __ZN4FMOD9CodecMPEG16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getDescriptionEx(void)")]
pub fn stub_964e4() -> &'static FmodCodecDesc {
    // IDA 0x964e4: memset 0x7C (0x96500) then fill name/version/callback
    // slots (0x96510..0x96578); host table is const-initialized once.
    &MPEGCODEC // IDA 0x9657c
}

// 0x965a4 — __ZN4FMOD9CodecMPEG12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_965a4() -> ! {
    todo!("0x965a4 FMOD::CodecMPEG::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x96854 — __ZN4FMOD9CodecMPEG12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_96854() -> ! {
    todo!("0x96854 FMOD::CodecMPEG::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x96860 — __ZN4FMOD9CodecMPEG12getPCMLengthEv
// type: int __fastcall(FMOD::File **this)
#[doc(alias = "FMOD::CodecMPEG::getPCMLength(void)")]
pub fn stub_96860() -> ! {
    todo!("0x96860 FMOD::CodecMPEG::getPCMLength(void)")
}

// 0x96a24 — __ZN4FMOD9CodecMPEG10makeTablesEi
// type: int __fastcall(int this, int)
#[doc(alias = "FMOD::CodecMPEG::makeTables(int)")]
pub fn stub_96a24() -> ! {
    todo!("0x96a24 FMOD::CodecMPEG::makeTables(int)")
}

// 0x96c4c — __ZN4FMOD9CodecMPEG7initAllEv
// type: int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::initAll(void)")]
pub fn stub_96c4c() -> ! {
    todo!("0x96c4c FMOD::CodecMPEG::initAll(void)")
}

// 0x96c9c — __ZN4FMOD9CodecMPEG12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
#[doc(alias = "FMOD::CodecMPEG::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_96c9c() -> ! {
    todo!("0x96c9c FMOD::CodecMPEG::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x97670 — __ZN4FMOD9CodecMPEG12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
#[doc(alias = "FMOD::CodecMPEG::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_97670() -> ! {
    todo!("0x97670 FMOD::CodecMPEG::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x976c8 — __GLOBAL__I__ZN4FMOD9mpegcodecE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::mpegcodec")]
pub fn stub_976c8() {
    // IDA 0x976c8: __static_initialization_and_destruction_0(1, 0xFFFF) runs
    // the mpegcodec static ctor (sibling cluster 0xa8400..; host: MPEGCODEC is
    // LazyLock-initialized, nothing to run).
}

// 0x976d4 — __ZN4FMOD9CodecMPEG7getBitsEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::getBits(int)")]
pub fn stub_976d4() -> ! {
    todo!("0x976d4 FMOD::CodecMPEG::getBits(int)")
}

// 0x97758 — __ZN4FMOD9CodecMPEG11getBitsFastEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::getBitsFast(int)")]
pub fn stub_97758() -> ! {
    todo!("0x97758 FMOD::CodecMPEG::getBitsFast(int)")
}

// 0x977c0 — __ZN4FMOD9CodecMPEG5dct64EPfS1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct64(float *,float *,float *)")]
pub fn stub_977c0() -> ! {
    todo!("0x977c0 FMOD::CodecMPEG::dct64(float *,float *,float *)")
}

// 0x981d4 — __ZN4FMOD9CodecMPEG6synthCEPfiiPs
// type: int __fastcall(FMOD::CodecMPEG *this, float *, int, int, __int16 *)
#[doc(alias = "FMOD::CodecMPEG::synthC(float *,int,int,short *)")]
pub fn stub_981d4() -> ! {
    todo!("0x981d4 FMOD::CodecMPEG::synthC(float *,int,int,short *)")
}

// 0x9854c — __ZN4FMOD9CodecMPEG5synthEPvPfii
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, float *, int, int)
#[doc(alias = "FMOD::CodecMPEG::synth(void *,float *,int,int)")]
pub fn stub_9854c() -> ! {
    todo!("0x9854c FMOD::CodecMPEG::synth(void *,float *,int,int)")
}

// 0x986f8 — __ZN4FMOD9CodecMPEG10resetFrameEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::resetFrame(void)")]
pub fn stub_986f8(codec: &mut CodecMpeg) -> i32 {
    // IDA 0x986f8: per lane (count word74 ?: 1, stride 18732): save the seek
    // words, memset 0x492C, restore table/len, marker -1 (0x98774), flag 1
    // (0x98788), aligned self pointer (0x987a4, address identity: host no-op).
    // No-op when word68 is null (0x98708).
    if !codec.lanes.is_empty() {
        let n = (if codec.lane_count == 0 { 1 } else { codec.lane_count }) as usize;
        if codec.lanes.len() < n {
            codec.lanes.resize_with(n, MpegLane::default);
        }
        for lane in codec.lanes.iter_mut().take(n) {
            let table = std::mem::take(&mut lane.seek_table);
            let len = lane.seek_len;
            *lane = MpegLane::default(); // IDA 0x98748 memset
            lane.seek_table = table; // IDA 0x98758
            lane.seek_len = len; // IDA 0x98764
            lane.marker = -1; // IDA 0x98774
            lane.flag1 = 1; // IDA 0x98788
        }
    }
    FMOD_OK // IDA 0x987e0
}

// 0x987e4 — __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, unsigned __int8 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")]
pub fn stub_987e4() -> ! {
    todo!("0x987e4 FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")
}

// 0x9891c — __ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, int *, int *, int *)
#[doc(alias = "FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")]
pub fn stub_9891c() -> ! {
    todo!("0x9891c FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")
}

// 0x98e9c — __ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, void *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")]
pub fn stub_98e9c() -> ! {
    todo!("0x98e9c FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")
}

// 0x99024 — __ZN4FMOD9CodecMPEG10getIIStuffEv
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getIIStuff(void)")]
pub fn stub_99024() -> ! {
    todo!("0x99024 FMOD::CodecMPEG::getIIStuff(void)")
}

// 0x99118 — __ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, float (*)[4][32], int *, int)
#[doc(alias = "FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")]
pub fn stub_99118() -> ! {
    todo!("0x99118 FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")
}

// 0x99728 — __ZN4FMOD9CodecMPEG11II_step_oneEPjPi
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")]
pub fn stub_99728() -> ! {
    todo!("0x99728 FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")
}

// 0x99a10 — __ZN4FMOD9CodecMPEG12decodeLayer2EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")]
pub fn stub_99a10() -> ! {
    todo!("0x99a10 FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")
}

// 0x99b08 — __ZN4FMOD9CodecMPEG10initLayer2Ev
// type: int __fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::initLayer2(void)")]
pub fn stub_99b08() -> ! {
    todo!("0x99b08 FMOD::CodecMPEG::initLayer2(void)")
}

// 0x99d7c — __ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii
// type: int __fastcall(int, int, int, _DWORD *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")]
pub fn stub_99d7c() -> ! {
    todo!("0x99d7c FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")
}

// 0x9a240 — __ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")]
pub fn stub_9a240() -> ! {
    todo!("0x9a240 FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")
}

// 0x9a308 — __ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_
// type: float *__fastcall(FMOD::CodecMPEG *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")]
pub fn stub_9a308() -> ! {
    todo!("0x9a308 FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")
}

// 0x9a9e8 — __ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *, float *, float *)
#[doc(alias = "FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")]
pub fn stub_9a9e8() -> ! {
    todo!("0x9a9e8 FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")
}

// 0x9af14 — __ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE
// type: int __fastcall(int, int, float *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")]
pub fn stub_9af14() -> ! {
    todo!("0x9af14 FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")
}

