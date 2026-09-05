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

// 0x71304 — __ZN4FMOD15ChannelEmulated9isVirtualEPb
// type: int __fastcall(FMOD::ChannelEmulated *this, bool *, int, bool)
#[doc(alias = "FMOD::ChannelEmulated::isVirtual(bool *)")]
pub fn stub_71304() -> ! {
    todo!("0x71304 FMOD::ChannelEmulated::isVirtual(bool *)")
}

// 0x7131c — __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int *)
#[doc(alias = "FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7131c() -> ! {
    todo!("0x7131c FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")
}

// 0x71334 — __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelEmulated *this, int, float *, int)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")]
pub fn stub_71334() -> ! {
    todo!("0x71334 FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")
}

// 0x7133c — __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelEmulated *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_7133c() -> ! {
    todo!("0x7133c FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x71344 — __ZN4FMOD15ChannelEmulated6updateEi
// type: int __fastcall(FMOD::ChannelEmulated *this, int)
#[doc(alias = "FMOD::ChannelEmulated::update(int)")]
pub fn stub_71344() -> ! {
    todo!("0x71344 FMOD::ChannelEmulated::update(int)")
}

// 0x71540 — __ZN4FMOD15ChannelEmulated5closeEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::close(void)")]
pub fn stub_71540() -> ! {
    todo!("0x71540 FMOD::ChannelEmulated::close(void)")
}

// 0x71580 — __ZN4FMOD15ChannelEmulated5allocEv
// type: int __fastcall(FMOD::DSPI **this)
#[doc(alias = "FMOD::ChannelEmulated::alloc(void)")]
pub fn stub_71580() -> ! {
    todo!("0x71580 FMOD::ChannelEmulated::alloc(void)")
}

// 0x715e8 — __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelEmulated *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_715e8() -> ! {
    todo!("0x715e8 FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x71698 — __ZN4FMOD15ChannelEmulatedC2Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_71698() -> ! {
    todo!("0x71698 FMOD::ChannelEmulated::ChannelEmulated(void)")
}

// 0x716e4 — __ZN4FMOD15ChannelEmulatedC1Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_716e4() -> ! {
    todo!("0x716e4 FMOD::ChannelEmulated::ChannelEmulated(void)")
}

// 0x716e8 — __ZN4FMOD15ChannelEmulated4stopEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::stop(void)")]
pub fn stub_716e8() -> ! {
    todo!("0x716e8 FMOD::ChannelEmulated::stop(void)")
}

// 0x71818 — __ZN4FMOD15ChannelEmulatedD0Ev
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_71818() -> ! {
    todo!("0x71818 FMOD::ChannelEmulated::~ChannelEmulated()")
}

// 0x7183c — __ZN4FMOD15ChannelEmulatedD1Ev
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_7183c() -> ! {
    todo!("0x7183c FMOD::ChannelEmulated::~ChannelEmulated()")
}

// 0x71854 — __ZN4FMOD11ChannelRealC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelReal::ChannelReal(void)")]
pub fn stub_71854() -> ! {
    todo!("0x71854 FMOD::ChannelReal::ChannelReal(void)")
}

// 0x718a0 — __ZN4FMOD11ChannelReal4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_718a0() -> ! {
    todo!("0x718a0 FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x718dc — __ZN4FMOD11ChannelReal5closeEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::close(void)")]
pub fn stub_718dc() -> ! {
    todo!("0x718dc FMOD::ChannelReal::close(void)")
}

// 0x718e8 — __ZN4FMOD11ChannelReal5allocEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::alloc(void)")]
pub fn stub_718e8() -> ! {
    todo!("0x718e8 FMOD::ChannelReal::alloc(void)")
}

// 0x7190c — __ZN4FMOD11ChannelReal5allocEPNS_4DSPIE
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelReal::alloc(FMOD::DSPI *)")]
pub fn stub_7190c() -> ! {
    todo!("0x7190c FMOD::ChannelReal::alloc(FMOD::DSPI *)")
}

// 0x71930 — __ZN4FMOD11ChannelReal23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")]
pub fn stub_71930() -> ! {
    todo!("0x71930 FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")
}

// 0x71938 — __ZN4FMOD11ChannelReal6updateEi
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::update(int)")]
pub fn stub_71938() -> ! {
    todo!("0x71938 FMOD::ChannelReal::update(int)")
}

// 0x71940 — __ZN4FMOD11ChannelReal12updateStreamEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::updateStream(void)")]
pub fn stub_71940() -> ! {
    todo!("0x71940 FMOD::ChannelReal::updateStream(void)")
}

// 0x71948 — __ZN4FMOD11ChannelReal5startEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::start(void)")]
pub fn stub_71948() -> ! {
    todo!("0x71948 FMOD::ChannelReal::start(void)")
}

// 0x71950 — __ZN4FMOD11ChannelReal4stopEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::stop(void)")]
pub fn stub_71950() -> ! {
    todo!("0x71950 FMOD::ChannelReal::stop(void)")
}

// 0x7197c — __ZN4FMOD11ChannelReal9setPausedEb
// type: int __fastcall(FMOD::ChannelReal *this, bool)
#[doc(alias = "FMOD::ChannelReal::setPaused(bool)")]
pub fn stub_7197c() -> ! {
    todo!("0x7197c FMOD::ChannelReal::setPaused(bool)")
}

// 0x719a0 — __ZN4FMOD11ChannelReal9getPausedEPb
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::getPaused(bool *)")]
pub fn stub_719a0() -> ! {
    todo!("0x719a0 FMOD::ChannelReal::getPaused(bool *)")
}

// 0x719c0 — __ZN4FMOD11ChannelReal9setVolumeEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setVolume(float)")]
pub fn stub_719c0() -> ! {
    todo!("0x719c0 FMOD::ChannelReal::setVolume(float)")
}

// 0x719c8 — __ZN4FMOD11ChannelReal12setFrequencyEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setFrequency(float)")]
pub fn stub_719c8() -> ! {
    todo!("0x719c8 FMOD::ChannelReal::setFrequency(float)")
}

// 0x719d0 — __ZN4FMOD11ChannelReal6setPanEff
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::setPan(float,float)")]
pub fn stub_719d0() -> ! {
    todo!("0x719d0 FMOD::ChannelReal::setPan(float,float)")
}

// 0x719d8 — __ZN4FMOD11ChannelReal16setDSPClockDelayEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::setDSPClockDelay(void)")]
pub fn stub_719d8() -> ! {
    todo!("0x719d8 FMOD::ChannelReal::setDSPClockDelay(void)")
}

// 0x719e0 — __ZN4FMOD11ChannelReal13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelReal *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t)
#[doc(alias = "FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_719e0() -> ! {
    todo!("0x719e0 FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x71e34 — __ZN4FMOD11ChannelReal11setPositionEjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")]
pub fn stub_71e34() -> ! {
    todo!("0x71e34 FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")
}

// 0x72008 — __ZN4FMOD11ChannelReal11getPositionEPjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")]
pub fn stub_72008() -> ! {
    todo!("0x72008 FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")
}

// 0x722f0 — __ZN4FMOD11ChannelReal13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_722f0() -> ! {
    todo!("0x722f0 FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")
}

// 0x72328 — __ZN4FMOD11ChannelReal12setLoopCountEi
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setLoopCount(int)")]
pub fn stub_72328() -> ! {
    todo!("0x72328 FMOD::ChannelReal::setLoopCount(int)")
}

// 0x72334 — __ZN4FMOD11ChannelReal12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelReal *this, int *)
#[doc(alias = "FMOD::ChannelReal::getLoopCount(int *)")]
pub fn stub_72334() -> ! {
    todo!("0x72334 FMOD::ChannelReal::getLoopCount(int *)")
}

// 0x7234c — __ZN4FMOD11ChannelReal14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setLowPassGain(float)")]
pub fn stub_7234c() -> ! {
    todo!("0x7234c FMOD::ChannelReal::setLowPassGain(float)")
}

// 0x72354 — __ZN4FMOD11ChannelReal15set3DAttributesEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DAttributes(void)")]
pub fn stub_72354() -> ! {
    todo!("0x72354 FMOD::ChannelReal::set3DAttributes(void)")
}

// 0x7235c — __ZN4FMOD11ChannelReal19set3DMinMaxDistanceEv
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DMinMaxDistance(void)")]
pub fn stub_7235c() -> ! {
    todo!("0x7235c FMOD::ChannelReal::set3DMinMaxDistance(void)")
}

// 0x72364 — __ZN4FMOD11ChannelReal14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::set3DOcclusion(float,float)")]
pub fn stub_72364() -> ! {
    todo!("0x72364 FMOD::ChannelReal::set3DOcclusion(float,float)")
}

// 0x72388 — __ZN4FMOD11ChannelReal9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelReal *this, bool *, bool)
#[doc(alias = "FMOD::ChannelReal::isPlaying(bool *,bool)")]
pub fn stub_72388() -> ! {
    todo!("0x72388 FMOD::ChannelReal::isPlaying(bool *,bool)")
}

// 0x723b0 — __ZN4FMOD11ChannelReal9isVirtualEPb
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::isVirtual(bool *)")]
pub fn stub_723b0() -> ! {
    todo!("0x723b0 FMOD::ChannelReal::isVirtual(bool *)")
}

// 0x723c4 — __ZN4FMOD11ChannelReal11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int()
#[doc(alias = "FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_723c4() -> ! {
    todo!("0x723c4 FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")
}

// 0x723cc — __ZN4FMOD11ChannelReal11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelReal *this, float *, int, int)
#[doc(alias = "FMOD::ChannelReal::getWaveData(float *,int,int)")]
pub fn stub_723cc() -> ! {
    todo!("0x723cc FMOD::ChannelReal::getWaveData(float *,int,int)")
}

// 0x723d4 — __ZN4FMOD11ChannelReal10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")]
pub fn stub_723d4() -> ! {
    todo!("0x723d4 FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")
}

// 0x723e4 — __ZN4FMOD11ChannelReal7setModeEj
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setMode(unsigned int)")]
pub fn stub_723e4() -> ! {
    todo!("0x723e4 FMOD::ChannelReal::setMode(unsigned int)")
}

// 0x72528 — __ZN4FMOD11ChannelReal19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_72528() -> ! {
    todo!("0x72528 FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x725a0 — __ZN4FMOD11ChannelReal19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_725a0() -> ! {
    todo!("0x725a0 FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x726d8 — __ZN4FMOD11ChannelReal19updateSpeakerLevelsEf
// type: int __fastcall(FMOD::ChannelReal *this, float32_t)
#[doc(alias = "FMOD::ChannelReal::updateSpeakerLevels(float)")]
pub fn stub_726d8() -> ! {
    todo!("0x726d8 FMOD::ChannelReal::updateSpeakerLevels(float)")
}

// 0x72910 — __ZN4FMOD11ChannelReal16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelReal *this, int, float *, int)
#[doc(alias = "FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")]
pub fn stub_72910() -> ! {
    todo!("0x72910 FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")
}

// 0x72a04 — __ZN4FMOD11ChannelRealD0Ev
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a04() -> ! {
    todo!("0x72a04 FMOD::ChannelReal::~ChannelReal()")
}

// 0x72a28 — __ZN4FMOD11ChannelRealD1Ev
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a28() -> ! {
    todo!("0x72a28 FMOD::ChannelReal::~ChannelReal()")
}

// 0x72a40 — __ZN4FMOD19ChannelRealManual3D5allocEv
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::alloc(void)")]
pub fn stub_72a40() -> ! {
    todo!("0x72a40 FMOD::ChannelRealManual3D::alloc(void)")
}

// 0x72a58 — __ZN4FMOD19ChannelRealManual3DC2Ev
// type: _DWORD *__fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")]
pub fn stub_72a58() -> ! {
    todo!("0x72a58 FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")
}

// 0x72a88 — __ZN4FMOD19ChannelRealManual3D23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")]
pub fn stub_72a88() -> ! {
    todo!("0x72a88 FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")
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
