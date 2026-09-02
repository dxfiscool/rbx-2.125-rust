//! core shard nm — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet stubbed in core (lowest EA uncovered 0x101a08..0x25833c, 42480 distinct in core before batch, 43066 uncovered, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR() -> ! { todo!("0xADDR mangled") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "FMOD::MemoryTracker::getMemUsedFromBits(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD13MemoryTracker18getMemUsedFromBitsEjj")]
// 0x101a08 — __ZN4FMOD13MemoryTracker18getMemUsedFromBitsEjj
// type: _DWORD __fastcall(FMOD::MemoryTracker *__hidden this, unsigned int, unsigned int)
pub fn stub_0x101a08() -> ! { todo!("0x101a08 __ZN4FMOD13MemoryTracker18getMemUsedFromBitsEjj") }

#[doc(alias = "FMOD::MemoryTracker::clear(void)")]
#[doc(alias = "__ZN4FMOD13MemoryTracker5clearEv")]
// 0x101c40 — __ZN4FMOD13MemoryTracker5clearEv
// type: _DWORD __fastcall(FMOD::MemoryTracker *__hidden this)
pub fn stub_0x101c40() -> ! { todo!("0x101c40 __ZN4FMOD13MemoryTracker5clearEv") }

#[doc(alias = "FMOD::MemoryTracker::MemoryTracker(void)")]
#[doc(alias = "__ZN4FMOD13MemoryTrackerC2Ev")]
// 0x101c64 — __ZN4FMOD13MemoryTrackerC2Ev
// type: _DWORD __fastcall(FMOD::MemoryTracker *__hidden this)
pub fn stub_0x101c64() -> ! { todo!("0x101c64 __ZN4FMOD13MemoryTrackerC2Ev") }

#[doc(alias = "FMOD::MemoryTracker::MemoryTracker(void)")]
#[doc(alias = "__ZN4FMOD13MemoryTrackerC1Ev")]
// 0x101c68 — __ZN4FMOD13MemoryTrackerC1Ev
// type: _DWORD __fastcall(FMOD::MemoryTracker *__hidden this)
pub fn stub_0x101c68() -> ! { todo!("0x101c68 __ZN4FMOD13MemoryTrackerC1Ev") }

#[doc(alias = "FMOD::ProfileChannel::init(void)")]
#[doc(alias = "__ZN4FMOD14ProfileChannel4initEv")]
// 0x101c6c — __ZN4FMOD14ProfileChannel4initEv
// type: _DWORD __fastcall(FMOD::ProfileChannel *__hidden this)
pub fn stub_0x101c6c() -> ! { todo!("0x101c6c __ZN4FMOD14ProfileChannel4initEv") }

#[doc(alias = "FMOD::ProfileChannel::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD14ProfileChannel6updateEPNS_7SystemIEj")]
// 0x101c74 — __ZN4FMOD14ProfileChannel6updateEPNS_7SystemIEj
// type: _DWORD __fastcall(FMOD::ProfileChannel *__hidden this, FMOD::SystemI *, unsigned int)
pub fn stub_0x101c74() -> ! { todo!("0x101c74 __ZN4FMOD14ProfileChannel6updateEPNS_7SystemIEj") }

#[doc(alias = "FMOD::ProfileChannel::release(void)")]
#[doc(alias = "__ZN4FMOD14ProfileChannel7releaseEv")]
// 0x101e10 — __ZN4FMOD14ProfileChannel7releaseEv
// type: _DWORD __fastcall(FMOD::ProfileChannel *__hidden this)
pub fn stub_0x101e10() -> ! { todo!("0x101e10 __ZN4FMOD14ProfileChannel7releaseEv") }

#[doc(alias = "FMOD::ProfileChannel::ProfileChannel(void)")]
#[doc(alias = "__ZN4FMOD14ProfileChannelC2Ev")]
// 0x101e4c — __ZN4FMOD14ProfileChannelC2Ev
// type: _DWORD __fastcall(FMOD::ProfileChannel *__hidden this)
pub fn stub_0x101e4c() -> ! { todo!("0x101e4c __ZN4FMOD14ProfileChannelC2Ev") }

#[doc(alias = "FMOD::ProfileChannel::ProfileChannel(void)")]
#[doc(alias = "__ZN4FMOD14ProfileChannelC1Ev")]
// 0x101e74 — __ZN4FMOD14ProfileChannelC1Ev
// type: _DWORD __fastcall(FMOD::ProfileChannel *__hidden this)
pub fn stub_0x101e74() -> ! { todo!("0x101e74 __ZN4FMOD14ProfileChannelC1Ev") }

#[doc(alias = "FMOD::FMOD_ProfileChannel_Create(void)")]
#[doc(alias = "__ZN4FMOD26FMOD_ProfileChannel_CreateEv")]
// 0x101e78 — __ZN4FMOD26FMOD_ProfileChannel_CreateEv
// type: _DWORD __fastcall(FMOD *__hidden this)
pub fn stub_0x101e78() -> ! { todo!("0x101e78 __ZN4FMOD26FMOD_ProfileChannel_CreateEv") }

#[doc(alias = "FMOD::ProfileCodec::init(void)")]
#[doc(alias = "__ZN4FMOD12ProfileCodec4initEv")]
// 0x101f1c — __ZN4FMOD12ProfileCodec4initEv
// type: _DWORD __fastcall(FMOD::ProfileCodec *__hidden this)
pub fn stub_0x101f1c() -> ! { todo!("0x101f1c __ZN4FMOD12ProfileCodec4initEv") }

#[doc(alias = "FMOD::ProfileCodec::getNumFreeCodecs(FMOD::DSPCodecPool const&)const")]
#[doc(alias = "__ZNK4FMOD12ProfileCodec16getNumFreeCodecsERKNS_12DSPCodecPoolE")]
// 0x101f24 — __ZNK4FMOD12ProfileCodec16getNumFreeCodecsERKNS_12DSPCodecPoolE
pub fn stub_0x101f24() -> ! { todo!("0x101f24 __ZNK4FMOD12ProfileCodec16getNumFreeCodecsERKNS_12DSPCodecPoolE") }

#[doc(alias = "FMOD::ProfileCodec::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD12ProfileCodec6updateEPNS_7SystemIEj")]
// 0x101f94 — __ZN4FMOD12ProfileCodec6updateEPNS_7SystemIEj
// type: _DWORD __fastcall(FMOD::ProfileCodec *__hidden this, FMOD::SystemI *, unsigned int)
pub fn stub_0x101f94() -> ! { todo!("0x101f94 __ZN4FMOD12ProfileCodec6updateEPNS_7SystemIEj") }

#[doc(alias = "FMOD::ProfileCodec::release(void)")]
#[doc(alias = "__ZN4FMOD12ProfileCodec7releaseEv")]
// 0x102040 — __ZN4FMOD12ProfileCodec7releaseEv
// type: _DWORD __fastcall(FMOD::ProfileCodec *__hidden this)
pub fn stub_0x102040() -> ! { todo!("0x102040 __ZN4FMOD12ProfileCodec7releaseEv") }

#[doc(alias = "FMOD::ProfileCodec::ProfileCodec(void)")]
#[doc(alias = "__ZN4FMOD12ProfileCodecC2Ev")]
// 0x10207c — __ZN4FMOD12ProfileCodecC2Ev
// type: _DWORD __fastcall(FMOD::ProfileCodec *__hidden this)
pub fn stub_0x10207c() -> ! { todo!("0x10207c __ZN4FMOD12ProfileCodecC2Ev") }

#[doc(alias = "FMOD::ProfileCodec::ProfileCodec(void)")]
#[doc(alias = "__ZN4FMOD12ProfileCodecC1Ev")]
// 0x1020a4 — __ZN4FMOD12ProfileCodecC1Ev
// type: _DWORD __fastcall(FMOD::ProfileCodec *__hidden this)
pub fn stub_0x1020a4() -> ! { todo!("0x1020a4 __ZN4FMOD12ProfileCodecC1Ev") }

#[doc(alias = "FMOD::FMOD_ProfileCodec_Create(void)")]
#[doc(alias = "__ZN4FMOD24FMOD_ProfileCodec_CreateEv")]
// 0x1020a8 — __ZN4FMOD24FMOD_ProfileCodec_CreateEv
// type: _DWORD __fastcall(FMOD *__hidden this)
pub fn stub_0x1020a8() -> ! { todo!("0x1020a8 __ZN4FMOD24FMOD_ProfileCodec_CreateEv") }

#[doc(alias = "_FMOD_DSP_Connection_MixMonoToStereo_SIMD")]
// 0x102150 — _FMOD_DSP_Connection_MixMonoToStereo_SIMD
// type: int __fastcall(int, int, int, int, float)
pub fn stub_0x102150() -> ! { todo!("0x102150 _FMOD_DSP_Connection_MixMonoToStereo_SIMD") }

#[doc(alias = "_FMOD_DSP_Connection_MixStereoToStereo_SIMD")]
// 0x1021e0 — _FMOD_DSP_Connection_MixStereoToStereo_SIMD
// type: int __fastcall(int, int, int, int, float)
pub fn stub_0x1021e0() -> ! { todo!("0x1021e0 _FMOD_DSP_Connection_MixStereoToStereo_SIMD") }

#[doc(alias = "FMOD::DSPI::convert(void *,void *,FMOD_SOUND_FORMAT,FMOD_SOUND_FORMAT,unsigned int,int,int,float)")]
#[doc(alias = "__ZN4FMOD4DSPI7convertEPvS1_17FMOD_SOUND_FORMATS2_jiif")]
// 0x102264 — __ZN4FMOD4DSPI7convertEPvS1_17FMOD_SOUND_FORMATS2_jiif
// type: int __fastcall(int, int, int, int, int, int, int, float)
pub fn stub_0x102264() -> ! { todo!("0x102264 __ZN4FMOD4DSPI7convertEPvS1_17FMOD_SOUND_FORMATS2_jiif") }

#[doc(alias = "_FMOD_Resampler_Linear")]
// 0x103020 — _FMOD_Resampler_Linear
pub fn stub_0x103020() -> ! { todo!("0x103020 _FMOD_Resampler_Linear") }

#[doc(alias = "FMOD::DSPDelay::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD8DSPDelay20setParameterInternalEif")]
// 0x103fdc — __ZN4FMOD8DSPDelay20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this, int, float)
pub fn stub_0x103fdc() -> ! { todo!("0x103fdc __ZN4FMOD8DSPDelay20setParameterInternalEif") }

#[doc(alias = "FMOD::DSPDelay::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD8DSPDelay20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0x104000 — __ZN4FMOD8DSPDelay20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0x104000() -> ! { todo!("0x104000 __ZN4FMOD8DSPDelay20setParameterCallbackEP14FMOD_DSP_STATEif") }

#[doc(alias = "FMOD::DSPDelay::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x10400c — __ZN4FMOD8DSPDelay17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0x10400c() -> ! { todo!("0x10400c __ZN4FMOD8DSPDelay17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::DSPDelay::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0x104040 — __ZN4FMOD8DSPDelay21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPDelay *this)
pub fn stub_0x104040() -> ! { todo!("0x104040 __ZN4FMOD8DSPDelay21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::DSPDelay::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay20getParameterInternalEiPfPc")]
// 0x104098 — __ZN4FMOD8DSPDelay20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this, int, float *, char *)
pub fn stub_0x104098() -> ! { todo!("0x104098 __ZN4FMOD8DSPDelay20getParameterInternalEiPfPc") }

#[doc(alias = "FMOD::DSPDelay::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0x104120 — __ZN4FMOD8DSPDelay20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0x104120() -> ! { todo!("0x104120 __ZN4FMOD8DSPDelay20getParameterCallbackEP14FMOD_DSP_STATEiPfPc") }

#[doc(alias = "FMOD::DSPDelay::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPDelay15releaseInternalEv")]
// 0x10412c — __ZN4FMOD8DSPDelay15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this)
pub fn stub_0x10412c() -> ! { todo!("0x10412c __ZN4FMOD8DSPDelay15releaseInternalEv") }

#[doc(alias = "FMOD::DSPDelay::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay15releaseCallbackEP14FMOD_DSP_STATE")]
// 0x104180 — __ZN4FMOD8DSPDelay15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0x104180() -> ! { todo!("0x104180 __ZN4FMOD8DSPDelay15releaseCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPDelay::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPDelay13resetInternalEv")]
// 0x10418c — __ZN4FMOD8DSPDelay13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this)
pub fn stub_0x10418c() -> ! { todo!("0x10418c __ZN4FMOD8DSPDelay13resetInternalEv") }

#[doc(alias = "FMOD::DSPDelay::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay13resetCallbackEP14FMOD_DSP_STATE")]
// 0x1041e8 — __ZN4FMOD8DSPDelay13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0x1041e8() -> ! { todo!("0x1041e8 __ZN4FMOD8DSPDelay13resetCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPDelay::createInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPDelay14createInternalEv")]
// 0x1041f4 — __ZN4FMOD8DSPDelay14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this)
pub fn stub_0x1041f4() -> ! { todo!("0x1041f4 __ZN4FMOD8DSPDelay14createInternalEv") }

#[doc(alias = "FMOD::DSPDelay::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPDelay14createCallbackEP14FMOD_DSP_STATE")]
// 0x1043d0 — __ZN4FMOD8DSPDelay14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0x1043d0() -> ! { todo!("0x1043d0 __ZN4FMOD8DSPDelay14createCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPDelay::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD8DSPDelay16getDescriptionExEv")]
// 0x1043dc — __ZN4FMOD8DSPDelay16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this)
pub fn stub_0x1043dc() -> ! { todo!("0x1043dc __ZN4FMOD8DSPDelay16getDescriptionExEv") }

#[doc(alias = "FMOD::DSPDelay::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD8DSPDelay12readInternalEPfS1_jii")]
// 0x1044c8 — __ZN4FMOD8DSPDelay12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0x1044c8() -> ! { todo!("0x1044c8 __ZN4FMOD8DSPDelay12readInternalEPfS1_jii") }

#[doc(alias = "FMOD::DSPDelay::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD8DSPDelay12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0x1050ac — __ZN4FMOD8DSPDelay12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0x1050ac() -> ! { todo!("0x1050ac __ZN4FMOD8DSPDelay12readCallbackEP14FMOD_DSP_STATEPfS3_jii") }

#[doc(alias = "global constructor keyed toFMOD::dspdelay_desc")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dspdelay_descE")]
// 0x105118 — __GLOBAL__I__ZN4FMOD13dspdelay_descE
pub fn stub_0x105118() -> ! { todo!("0x105118 __GLOBAL__I__ZN4FMOD13dspdelay_descE") }

#[doc(alias = "FMOD::DSPTremolo::readLFOTable(int,bool,float *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo12readLFOTableEibPf")]
// 0x105124 — __ZN4FMOD10DSPTremolo12readLFOTableEibPf
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, int, bool, float *)
pub fn stub_0x105124() -> ! { todo!("0x105124 __ZN4FMOD10DSPTremolo12readLFOTableEibPf") }

#[doc(alias = "FMOD::DSPTremolo::updateWaveform(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo14updateWaveformEv")]
// 0x1051c0 — __ZN4FMOD10DSPTremolo14updateWaveformEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x1051c0() -> ! { todo!("0x1051c0 __ZN4FMOD10DSPTremolo14updateWaveformEv") }

#[doc(alias = "FMOD::DSPTremolo::applyPhase(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo10applyPhaseEv")]
// 0x105244 — __ZN4FMOD10DSPTremolo10applyPhaseEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105244() -> ! { todo!("0x105244 __ZN4FMOD10DSPTremolo10applyPhaseEv") }

#[doc(alias = "FMOD::DSPTremolo::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo15releaseInternalEv")]
// 0x105328 — __ZN4FMOD10DSPTremolo15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105328() -> ! { todo!("0x105328 __ZN4FMOD10DSPTremolo15releaseInternalEv") }

#[doc(alias = "FMOD::DSPTremolo::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo13resetInternalEv")]
// 0x105330 — __ZN4FMOD10DSPTremolo13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105330() -> ! { todo!("0x105330 __ZN4FMOD10DSPTremolo13resetInternalEv") }

#[doc(alias = "FMOD::DSPTremolo::getRampValues(int,float *,float *,int *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo13getRampValuesEiPfS1_Pi")]
// 0x105384 — __ZN4FMOD10DSPTremolo13getRampValuesEiPfS1_Pi
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, int, float *, float *, int *)
pub fn stub_0x105384() -> ! { todo!("0x105384 __ZN4FMOD10DSPTremolo13getRampValuesEiPfS1_Pi") }

#[doc(alias = "FMOD::DSPTremolo::getLFOLevel(int)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo11getLFOLevelEi")]
// 0x105648 — __ZN4FMOD10DSPTremolo11getLFOLevelEi
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, int)
pub fn stub_0x105648() -> ! { todo!("0x105648 __ZN4FMOD10DSPTremolo11getLFOLevelEi") }

#[doc(alias = "FMOD::DSPTremolo::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo20setParameterInternalEif")]
// 0x10575c — __ZN4FMOD10DSPTremolo20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, int, float)
pub fn stub_0x10575c() -> ! { todo!("0x10575c __ZN4FMOD10DSPTremolo20setParameterInternalEif") }

#[doc(alias = "FMOD::DSPTremolo::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x105800 — __ZN4FMOD10DSPTremolo17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0x105800() -> ! { todo!("0x105800 __ZN4FMOD10DSPTremolo17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::DSPTremolo::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo15releaseCallbackEP14FMOD_DSP_STATE")]
// 0x105808 — __ZN4FMOD10DSPTremolo15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0x105808() -> ! { todo!("0x105808 __ZN4FMOD10DSPTremolo15releaseCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPTremolo::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo13resetCallbackEP14FMOD_DSP_STATE")]
// 0x105814 — __ZN4FMOD10DSPTremolo13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0x105814() -> ! { todo!("0x105814 __ZN4FMOD10DSPTremolo13resetCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPTremolo::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0x105820 — __ZN4FMOD10DSPTremolo20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0x105820() -> ! { todo!("0x105820 __ZN4FMOD10DSPTremolo20setParameterCallbackEP14FMOD_DSP_STATEif") }

#[doc(alias = "FMOD::DSPTremolo::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0x10582c — __ZN4FMOD10DSPTremolo21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPTremolo *this)
pub fn stub_0x10582c() -> ! { todo!("0x10582c __ZN4FMOD10DSPTremolo21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::DSPTremolo::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo20getParameterInternalEiPfPc")]
// 0x105884 — __ZN4FMOD10DSPTremolo20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, int, float *, char *)
pub fn stub_0x105884() -> ! { todo!("0x105884 __ZN4FMOD10DSPTremolo20getParameterInternalEiPfPc") }

#[doc(alias = "FMOD::DSPTremolo::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0x105a38 — __ZN4FMOD10DSPTremolo20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0x105a38() -> ! { todo!("0x105a38 __ZN4FMOD10DSPTremolo20getParameterCallbackEP14FMOD_DSP_STATEiPfPc") }

#[doc(alias = "FMOD::DSPTremolo::updateTiming(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo12updateTimingEv")]
// 0x105a44 — __ZN4FMOD10DSPTremolo12updateTimingEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105a44() -> ! { todo!("0x105a44 __ZN4FMOD10DSPTremolo12updateTimingEv") }

#[doc(alias = "FMOD::DSPTremolo::createLFOTable(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo14createLFOTableEv")]
// 0x105c50 — __ZN4FMOD10DSPTremolo14createLFOTableEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105c50() -> ! { todo!("0x105c50 __ZN4FMOD10DSPTremolo14createLFOTableEv") }

#[doc(alias = "FMOD::DSPTremolo::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo14createInternalEv")]
// 0x105cdc — __ZN4FMOD10DSPTremolo14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105cdc() -> ! { todo!("0x105cdc __ZN4FMOD10DSPTremolo14createInternalEv") }

#[doc(alias = "FMOD::DSPTremolo::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo14createCallbackEP14FMOD_DSP_STATE")]
// 0x105e00 — __ZN4FMOD10DSPTremolo14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0x105e00() -> ! { todo!("0x105e00 __ZN4FMOD10DSPTremolo14createCallbackEP14FMOD_DSP_STATE") }

#[doc(alias = "FMOD::DSPTremolo::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo16getDescriptionExEv")]
// 0x105e0c — __ZN4FMOD10DSPTremolo16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
pub fn stub_0x105e0c() -> ! { todo!("0x105e0c __ZN4FMOD10DSPTremolo16getDescriptionExEv") }

#[doc(alias = "FMOD::DSPTremolo::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo12readInternalEPfS1_jii")]
// 0x105ef8 — __ZN4FMOD10DSPTremolo12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0x105ef8() -> ! { todo!("0x105ef8 __ZN4FMOD10DSPTremolo12readInternalEPfS1_jii") }

#[doc(alias = "FMOD::DSPTremolo::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPTremolo12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0x106428 — __ZN4FMOD10DSPTremolo12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0x106428() -> ! { todo!("0x106428 __ZN4FMOD10DSPTremolo12readCallbackEP14FMOD_DSP_STATEPfS3_jii") }

#[doc(alias = "global constructor keyed toFMOD::dsptremolo_desc")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD15dsptremolo_descE")]
// 0x106494 — __GLOBAL__I__ZN4FMOD15dsptremolo_descE
pub fn stub_0x106494() -> ! { todo!("0x106494 __GLOBAL__I__ZN4FMOD15dsptremolo_descE") }

#[doc(alias = "FMOD::HistoryBufferPool::HistoryBufferPool(void)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPoolC2Ev")]
// 0x1064a0 — __ZN4FMOD17HistoryBufferPoolC2Ev
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
pub fn stub_0x1064a0() -> ! { todo!("0x1064a0 __ZN4FMOD17HistoryBufferPoolC2Ev") }

#[doc(alias = "FMOD::HistoryBufferPool::HistoryBufferPool(void)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPoolC1Ev")]
// 0x1064cc — __ZN4FMOD17HistoryBufferPoolC1Ev
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
pub fn stub_0x1064cc() -> ! { todo!("0x1064cc __ZN4FMOD17HistoryBufferPoolC1Ev") }

#[doc(alias = "FMOD::HistoryBufferPool::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x1064d0 — __ZN4FMOD17HistoryBufferPool17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0x1064d0() -> ! { todo!("0x1064d0 __ZN4FMOD17HistoryBufferPool17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::HistoryBufferPool::release(void)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool7releaseEv")]
// 0x106528 — __ZN4FMOD17HistoryBufferPool7releaseEv
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
pub fn stub_0x106528() -> ! { todo!("0x106528 __ZN4FMOD17HistoryBufferPool7releaseEv") }

#[doc(alias = "FMOD::HistoryBufferPool::free(float *)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool4freeEPf")]
// 0x1065b4 — __ZN4FMOD17HistoryBufferPool4freeEPf
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, float *)
pub fn stub_0x1065b4() -> ! { todo!("0x1065b4 __ZN4FMOD17HistoryBufferPool4freeEPf") }

#[doc(alias = "FMOD::HistoryBufferPool::alloc(float **,int)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool5allocEPPfi")]
// 0x1066bc — __ZN4FMOD17HistoryBufferPool5allocEPPfi
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, float **, int)
pub fn stub_0x1066bc() -> ! { todo!("0x1066bc __ZN4FMOD17HistoryBufferPool5allocEPPfi") }

#[doc(alias = "FMOD::HistoryBufferPool::init(int,int)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool4initEii")]
// 0x106868 — __ZN4FMOD17HistoryBufferPool4initEii
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, int, int)
pub fn stub_0x106868() -> ! { todo!("0x106868 __ZN4FMOD17HistoryBufferPool4initEii") }

#[doc(alias = "FMOD::HistoryBufferPool::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17HistoryBufferPool13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0x106974 — __ZN4FMOD17HistoryBufferPool13getMemoryUsedEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0x106974() -> ! { todo!("0x106974 __ZN4FMOD17HistoryBufferPool13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::CodecAudioQueue::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x1069cc — __ZN4FMOD15CodecAudioQueue17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0x1069cc() -> ! { todo!("0x1069cc __ZN4FMOD15CodecAudioQueue17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::CodecAudioQueue::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE")]
// 0x1069fc — __ZN4FMOD15CodecAudioQueue21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecAudioQueue *this)
pub fn stub_0x1069fc() -> ! { todo!("0x1069fc __ZN4FMOD15CodecAudioQueue21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::CodecAudioQueue::fileGetSize(long long *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue11fileGetSizeEPx")]
// 0x106a54 — __ZN4FMOD15CodecAudioQueue11fileGetSizeEPx
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, __int64 *)
pub fn stub_0x106a54() -> ! { todo!("0x106a54 __ZN4FMOD15CodecAudioQueue11fileGetSizeEPx") }

#[doc(alias = "FMOD::CodecAudioQueue::fileGetSizeCallback(void *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19fileGetSizeCallbackEPv")]
// 0x106ad0 — __ZN4FMOD15CodecAudioQueue19fileGetSizeCallbackEPv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *)
pub fn stub_0x106ad0() -> ! { todo!("0x106ad0 __ZN4FMOD15CodecAudioQueue19fileGetSizeCallbackEPv") }

#[doc(alias = "FMOD::CodecAudioQueue::fileRead(long long,unsigned long,void *,unsigned long *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue8fileReadExmPvPm")]
// 0x106afc — __ZN4FMOD15CodecAudioQueue8fileReadExmPvPm
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, __int64, unsigned int, void *, unsigned int *)
pub fn stub_0x106afc() -> ! { todo!("0x106afc __ZN4FMOD15CodecAudioQueue8fileReadExmPvPm") }

#[doc(alias = "FMOD::CodecAudioQueue::fileReadCallback(void *,long long,unsigned long,void *,unsigned long *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue16fileReadCallbackEPvxmS1_Pm")]
// 0x106c1c — __ZN4FMOD15CodecAudioQueue16fileReadCallbackEPvxmS1_Pm
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, __int64, unsigned int, void *, unsigned int *)
pub fn stub_0x106c1c() -> ! { todo!("0x106c1c __ZN4FMOD15CodecAudioQueue16fileReadCallbackEPvxmS1_Pm") }

#[doc(alias = "FMOD::CodecAudioQueue::processAudioQueue(OpaqueAudioQueue *,AudioQueueBuffer *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer")]
// 0x106c20 — __ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, OpaqueAudioQueue *, AudioQueueBuffer *)
pub fn stub_0x106c20() -> ! { todo!("0x106c20 __ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer") }

#[doc(alias = "FMOD::CodecAudioQueue::audioQueueOutputCallback(void *,OpaqueAudioQueue *,AudioQueueBuffer *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue24audioQueueOutputCallbackEPvP16OpaqueAudioQueueP16AudioQueueBuffer")]
// 0x106cf8 — __ZN4FMOD15CodecAudioQueue24audioQueueOutputCallbackEPvP16OpaqueAudioQueueP16AudioQueueBuffer
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, OpaqueAudioQueue *, AudioQueueBuffer *)
pub fn stub_0x106cf8() -> ! { todo!("0x106cf8 __ZN4FMOD15CodecAudioQueue24audioQueueOutputCallbackEPvP16OpaqueAudioQueueP16AudioQueueBuffer") }

#[doc(alias = "FMOD::CodecAudioQueue::setupAudioFile(bool)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue14setupAudioFileEb")]
// 0x106cfc — __ZN4FMOD15CodecAudioQueue14setupAudioFileEb
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, bool)
pub fn stub_0x106cfc() -> ! { todo!("0x106cfc __ZN4FMOD15CodecAudioQueue14setupAudioFileEb") }

#[doc(alias = "FMOD::CodecAudioQueue::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19setPositionInternalEijj")]
// 0x106eac — __ZN4FMOD15CodecAudioQueue19setPositionInternalEijj
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, int, unsigned int, unsigned int)
pub fn stub_0x106eac() -> ! { todo!("0x106eac __ZN4FMOD15CodecAudioQueue19setPositionInternalEijj") }

#[doc(alias = "FMOD::CodecAudioQueue::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x107090 — __ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj
pub fn stub_0x107090() -> ! { todo!("0x107090 __ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj") }

#[doc(alias = "FMOD::CodecAudioQueue::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeInternalEv")]
// 0x10709c — __ZN4FMOD15CodecAudioQueue13closeInternalEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
pub fn stub_0x10709c() -> ! { todo!("0x10709c __ZN4FMOD15CodecAudioQueue13closeInternalEv") }

#[doc(alias = "FMOD::CodecAudioQueue::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x107164 — __ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_0x107164() -> ! { todo!("0x107164 __ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE") }

#[doc(alias = "FMOD::CodecAudioQueue::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue16getDescriptionExEv")]
// 0x107170 — __ZN4FMOD15CodecAudioQueue16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
pub fn stub_0x107170() -> ! { todo!("0x107170 __ZN4FMOD15CodecAudioQueue16getDescriptionExEv") }

#[doc(alias = "FMOD::CodecAudioQueue::setupAudioQueue(void)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue15setupAudioQueueEv")]
// 0x107284 — __ZN4FMOD15CodecAudioQueue15setupAudioQueueEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
pub fn stub_0x107284() -> ! { todo!("0x107284 __ZN4FMOD15CodecAudioQueue15setupAudioQueueEv") }

#[doc(alias = "FMOD::CodecAudioQueue::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readInternalEPvjPj")]
// 0x107598 — __ZN4FMOD15CodecAudioQueue12readInternalEPvjPj
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0x107598() -> ! { todo!("0x107598 __ZN4FMOD15CodecAudioQueue12readInternalEPvjPj") }

#[doc(alias = "FMOD::CodecAudioQueue::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x10773c — __ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj
pub fn stub_0x10773c() -> ! { todo!("0x10773c __ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj") }

#[doc(alias = "FMOD::CodecAudioQueue::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x107748 — __ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_0x107748() -> ! { todo!("0x107748 __ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO") }

#[doc(alias = "FMOD::CodecAudioQueue::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x1078d4 — __ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_0x1078d4() -> ! { todo!("0x1078d4 __ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO") }

#[doc(alias = "FMOD::CodecAudioQueue::resetAll(bool,bool)")]
#[doc(alias = "__ZN4FMOD15CodecAudioQueue8resetAllEbb")]
// 0x1078e0 — __ZN4FMOD15CodecAudioQueue8resetAllEbb
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, bool, bool)
pub fn stub_0x1078e0() -> ! { todo!("0x1078e0 __ZN4FMOD15CodecAudioQueue8resetAllEbb") }

#[doc(alias = "global constructor keyed toFMOD::CodecAudioQueue::gCodecHead")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE")]
// 0x107a1c — __GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE
pub fn stub_0x107a1c() -> ! { todo!("0x107a1c __GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE") }

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC1Ev")]
// 0x256ef8 — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC1Ev
// type: int()
pub fn stub_0x256ef8() -> ! { todo!("0x256ef8 __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC1Ev") }

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC2Ev")]
// 0x256efc — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x256efc() -> ! { todo!("0x256efc __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC2Ev") }

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED1Ev")]
// 0x257980 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x257980() -> ! { todo!("0x257980 __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED1Ev") }

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED1Ev")]
// 0x2579c0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x2579c0() -> ! { todo!("0x2579c0 __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED1Ev") }

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc")]
// 0x257b5c — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0x257b5c() -> ! { todo!("0x257b5c __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc") }

#[doc(alias = "RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v")]
// 0x257ebc — __ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v
// type: int __fastcall(_UNKNOWN ****)
pub fn stub_0x257ebc() -> ! { todo!("0x257ebc __ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v") }

#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev")]
// 0x2580a8 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev
// type: int()
pub fn stub_0x2580a8() -> ! { todo!("0x2580a8 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev") }

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")]
// 0x258150 — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_0x258150() -> ! { todo!("0x258150 __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv") }

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")]
// 0x25820c — __ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_0x25820c() -> ! { todo!("0x25820c __ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv") }

#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv")]
// 0x2582c8 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv
// type: void *()
pub fn stub_0x2582c8() -> ! { todo!("0x2582c8 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv") }

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv")]
// 0x25833c — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv
pub fn stub_0x25833c() -> ! { todo!("0x25833c __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv") }
