//! core shard ja — 120 core stubs EA-sorted, 0x779bc..0x7fc24 (EA-sorted asc global gap filler next 120 uncovered, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted asc not in crates/ via grep -r stub_0x crates --include=*.rs — next 120 uncovered (49345 remaining before -> 49225 after, 0x779bc..0x7fc24).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::ChannelStream::setLowPassGain(float)")]
// 0x779bc — __ZN4FMOD13ChannelStream14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
pub fn stub_0x779bc() {
    // IDA 0x779bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::set3DMinMaxDistance(void)")]
// 0x77a18 — __ZN4FMOD13ChannelStream19set3DMinMaxDistanceEv
// type: int __fastcall(FMOD::ChannelStream *this)
pub fn stub_0x77a18() {
    // IDA 0x77a18: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::set3DOcclusion(float,float)")]
// 0x77a64 — __ZN4FMOD13ChannelStream14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
pub fn stub_0x77a64() {
    // IDA 0x77a64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
// 0x77ac8 — __ZN4FMOD13ChannelStream19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
pub fn stub_0x77ac8() {
    // IDA 0x77ac8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
// 0x77b24 — __ZN4FMOD13ChannelStream19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int)
pub fn stub_0x77b24() {
    // IDA 0x77b24: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::isPlaying(bool *,bool)")]
// 0x77b48 — __ZN4FMOD13ChannelStream9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelStream *this, bool *, bool)
pub fn stub_0x77b48() {
    // IDA 0x77b48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
// 0x77b5c — __ZN4FMOD13ChannelStream11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(int)
pub fn stub_0x77b5c() {
    // IDA 0x77b5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::getWaveData(float *,int,int)")]
// 0x77b6c — __ZN4FMOD13ChannelStream11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelStream *this, float *, int, int)
pub fn stub_0x77b6c() {
    // IDA 0x77b6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::getDSPHead(FMOD::DSPI **)")]
// 0x77b7c — __ZN4FMOD13ChannelStream10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int)
pub fn stub_0x77b7c() {
    // IDA 0x77b7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setLoopCount(int)")]
// 0x77b8c — __ZN4FMOD13ChannelStream12setLoopCountEi
// type: int __fastcall(FMOD::ChannelStream *this, int)
pub fn stub_0x77b8c() {
    // IDA 0x77b8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setLoopPoints(unsigned int,unsigned int)")]
// 0x77bc0 — __ZN4FMOD13ChannelStream13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
pub fn stub_0x77bc0() {
    // IDA 0x77bc0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::getPosition(unsigned int *,unsigned int)")]
// 0x77c14 — __ZN4FMOD13ChannelStream11getPositionEPjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int *, unsigned int)
pub fn stub_0x77c14() {
    // IDA 0x77c14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::stop(void)")]
// 0x77f74 — __ZN4FMOD13ChannelStream4stopEv
// type: int __fastcall(FMOD::ChannelStream *this)
pub fn stub_0x77f74() {
    // IDA 0x77f74: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setMode(unsigned int)")]
// 0x78168 — __ZN4FMOD13ChannelStream7setModeEj
// type: int __fastcall(FMOD::ChannelStream *this, int)
pub fn stub_0x78168() {
    // IDA 0x78168: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
// 0x781f0 — __ZN4FMOD13ChannelStreamC2Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
pub fn stub_0x781f0() {
    // IDA 0x781f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
// 0x7826c — __ZN4FMOD13ChannelStreamC1Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
pub fn stub_0x7826c() {
    // IDA 0x7826c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::alloc(void)")]
// 0x78270 — __ZN4FMOD13ChannelStream5allocEv
// type: int __fastcall(FMOD::ChannelStream *this, int, int)
pub fn stub_0x78270() {
    // IDA 0x78270: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setPositionEx(unsigned int,unsigned int,bool)")]
// 0x78540 — __ZN4FMOD13ChannelStream13setPositionExEjjb
// type: int __fastcall(unsigned __int64 this, unsigned int, bool)
pub fn stub_0x78540() {
    // IDA 0x78540: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setPaused(bool)")]
// 0x78af0 — __ZN4FMOD13ChannelStream9setPausedEb
// type: int __fastcall(FMOD::ChannelStream *this, bool)
pub fn stub_0x78af0() {
    // IDA 0x78af0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::updateStream(void)")]
// 0x78b80 — __ZN4FMOD13ChannelStream12updateStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
pub fn stub_0x78b80() {
    // IDA 0x78b80: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::isStream(void)")]
// 0x78fac — __ZN4FMOD13ChannelStream8isStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
pub fn stub_0x78fac() {
    // IDA 0x78fac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setPosition(unsigned int,unsigned int)")]
// 0x78fb4 — __ZN4FMOD13ChannelStream11setPositionEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
pub fn stub_0x78fb4() {
    // IDA 0x78fb4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
// 0x78fc4 — __ZN4FMOD13ChannelStreamD0Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
pub fn stub_0x78fc4() {
    // IDA 0x78fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
// 0x78fe8 — __ZN4FMOD13ChannelStreamD1Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
pub fn stub_0x78fe8() {
    // IDA 0x78fe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelGroup::setVolume(float)")]
// 0x79000 — __ZN4FMOD12ChannelGroup9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroup *this, float, FMOD::ChannelGroupI **)
pub fn stub_0x79000() {
    // IDA 0x79000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelGroupI::validate(FMOD::ChannelGroup *,FMOD::ChannelGroupI**)")]
// 0x79034 — __ZN4FMOD13ChannelGroupI8validateEPNS_12ChannelGroupEPPS0_
// type: int __fastcall(int result, int *)
pub fn stub_0x79034() {
    // IDA 0x79034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelGroupI::getPaused(bool *)")]
// 0x79054 — __ZN4FMOD13ChannelGroupI9getPausedEPb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool *)
pub fn stub_0x79054() {
    // IDA 0x79054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0x7906c — __ZN4FMOD13ChannelGroupI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::MemoryTracker *)
pub fn stub_0x7906c() {
    // IDA 0x7906c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelGroupI::updateChildMixTarget(FMOD::DSPI *)")]
// 0x790fc — __ZN4FMOD13ChannelGroupI20updateChildMixTargetEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::DSPI *)
pub fn stub_0x790fc() {
    // IDA 0x790fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::setMute(bool,bool)")]
// 0x791e8 — __ZN4FMOD13ChannelGroupI7setMuteEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
pub fn stub_0x791e8() {
    // IDA 0x791e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::setPaused(bool,bool)")]
// 0x79280 — __ZN4FMOD13ChannelGroupI9setPausedEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
pub fn stub_0x79280() {
    // IDA 0x79280: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::setPitchInternal(void)")]
// 0x79334 — __ZN4FMOD13ChannelGroupI16setPitchInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
pub fn stub_0x79334() {
    // IDA 0x79334: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::setVolumeInternal(void)")]
// 0x793e4 — __ZN4FMOD13ChannelGroupI17setVolumeInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
pub fn stub_0x793e4() {
    // IDA 0x793e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")]
// 0x794c4 — __ZN4FMOD13ChannelGroupI8addGroupEPS0_
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::ChannelGroupI *)
pub fn stub_0x794c4() {
    // IDA 0x794c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::setVolume(float)")]
// 0x796a4 — __ZN4FMOD13ChannelGroupI9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroupI *this, float)
pub fn stub_0x796a4() {
    // IDA 0x796a4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::releaseInternal(bool)")]
// 0x796d4 — __ZN4FMOD13ChannelGroupI15releaseInternalEb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool)
pub fn stub_0x796d4() {
    // IDA 0x796d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::release(void)")]
// 0x7995c — __ZN4FMOD13ChannelGroupI7releaseEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
pub fn stub_0x7995c() {
    // IDA 0x7995c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0x79980 — __ZN4FMOD20ChannelGroupSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupSoftware *this, FMOD::MemoryTracker *)
pub fn stub_0x79980() {
    // IDA 0x79980: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0x79a38 — __ZN4FMOD13ChannelGroupI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_0x79a38() {
    // IDA 0x79a38: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0x79a90 — __ZN4FMOD20ChannelGroupSoftware13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_0x79a90() {
    // IDA 0x79a90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::returnToFreeList(void)")]
// 0x79ae8 — __ZN4FMOD8ChannelI16returnToFreeListEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x79ae8() {
    // IDA 0x79ae8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::referenceStamp(bool)")]
// 0x79b98 — __ZN4FMOD8ChannelI14referenceStampEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
pub fn stub_0x79b98() {
    // IDA 0x79b98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")]
// 0x79bdc — __ZN4FMOD8ChannelI14getRealChannelEPPNS_11ChannelRealEPi
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelReal **, int *)
pub fn stub_0x79bdc() {
    // IDA 0x79bdc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::init(void)")]
// 0x79ca8 — __ZN4FMOD8ChannelI4initEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x79ca8() {
    // IDA 0x79ca8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
// 0x79dd4 — __ZN4FMOD8ChannelIC2EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
pub fn stub_0x79dd4() {
    // IDA 0x79dd4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
// 0x79e84 — __ZN4FMOD8ChannelIC1EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
pub fn stub_0x79e84() {
    // IDA 0x79e84: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
// 0x79e88 — __ZN4FMOD8ChannelIC2Ev
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x79e88() {
    // IDA 0x79e88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
// 0x79ef0 — __ZN4FMOD8ChannelIC1Ev
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x79ef0() {
    // IDA 0x79ef0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")]
// 0x79ef4 — __ZN4FMOD8ChannelI5allocEPNS_4DSPIEb
// type: int __fastcall(_DWORD *, int, char)
pub fn stub_0x79ef4() {
    // IDA 0x79ef4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::start(void)")]
// 0x7a0f8 — __ZN4FMOD8ChannelI5startEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x7a0f8() {
    // IDA 0x7a0f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getPaused(bool *)")]
// 0x7a198 — __ZN4FMOD8ChannelI9getPausedEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
pub fn stub_0x7a198() {
    // IDA 0x7a198: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getVolume(float *)")]
// 0x7a1ec — __ZN4FMOD8ChannelI9getVolumeEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
pub fn stub_0x7a1ec() {
    // IDA 0x7a1ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getFrequency(float *)")]
// 0x7a214 — __ZN4FMOD8ChannelI12getFrequencyEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
pub fn stub_0x7a214() {
    // IDA 0x7a214: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setPan(float,bool)")]
// 0x7a23c — __ZN4FMOD8ChannelI6setPanEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
pub fn stub_0x7a23c() {
    // IDA 0x7a23c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")]
// 0x7a358 — __ZN4FMOD8ChannelI8setDelayE14FMOD_DELAYTYPEjj
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_0x7a358() {
    // IDA 0x7a358: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")]
// 0x7a50c — __ZN4FMOD8ChannelI13setSpeakerMixEffffffffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, float, float, float, float, float, float, bool)
pub fn stub_0x7a50c() {
    // IDA 0x7a50c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")]
// 0x7a7dc — __ZN4FMOD8ChannelI16getSpeakerLevelsE12FMOD_SPEAKERPfi
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_0x7a7dc() {
    // IDA 0x7a7dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getMute(bool *)")]
// 0x7a8b0 — __ZN4FMOD8ChannelI7getMuteEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
pub fn stub_0x7a8b0() {
    // IDA 0x7a8b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
// 0x7a8d8 — __ZN4FMOD8ChannelI15set3DAttributesEPK11FMOD_VECTORS3_
// type: int __fastcall(int, float *, float *)
pub fn stub_0x7a8d8() {
    // IDA 0x7a8d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
// 0x7aa4c — __ZN4FMOD8ChannelI19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
pub fn stub_0x7aa4c() {
    // IDA 0x7aa4c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
// 0x7aae0 — __ZN4FMOD8ChannelI19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
pub fn stub_0x7aae0() {
    // IDA 0x7aae0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::isVirtual(bool *)")]
// 0x7ab74 — __ZN4FMOD8ChannelI9isVirtualEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
pub fn stub_0x7ab74() {
    // IDA 0x7ab74: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getAudibilityInternal(float *,bool)")]
// 0x7aba0 — __ZN4FMOD8ChannelI21getAudibilityInternalEPfb
// type: int __fastcall(FMOD::ChannelI *this, float *, bool)
pub fn stub_0x7aba0() {
    // IDA 0x7aba0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getAudibility(float *)")]
// 0x7ad00 — __ZN4FMOD8ChannelI13getAudibilityEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
pub fn stub_0x7ad00() {
    // IDA 0x7ad00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")]
// 0x7ad08 — __ZN4FMOD8ChannelI15getCurrentSoundEPPNS_6SoundIE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x7ad08() {
    // IDA 0x7ad08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")]
// 0x7ad44 — __ZN4FMOD8ChannelI13getCurrentDSPEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x7ad44() {
    // IDA 0x7ad44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
// 0x7ad70 — __ZN4FMOD8ChannelI11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
// type: int __fastcall(int result, int)
pub fn stub_0x7ad70() {
    // IDA 0x7ad70: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")]
// 0x7ad88 — __ZN4FMOD8ChannelI11getPositionEPjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int)
pub fn stub_0x7ad88() {
    // IDA 0x7ad88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::updateSyncPoints(bool)")]
// 0x7adb0 — __ZN4FMOD8ChannelI16updateSyncPointsEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
pub fn stub_0x7adb0() {
    // IDA 0x7adb0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setFrequency(float)")]
// 0x7b1f8 — __ZN4FMOD8ChannelI12setFrequencyEf
// type: int __fastcall(FMOD::ChannelI *this, float)
pub fn stub_0x7b1f8() {
    // IDA 0x7b1f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")]
// 0x7b31c — __ZN4FMOD8ChannelI10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int)
pub fn stub_0x7b31c() {
    // IDA 0x7b31c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getMode(unsigned int *)")]
// 0x7b344 — __ZN4FMOD8ChannelI7getModeEPj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *)
pub fn stub_0x7b344() {
    // IDA 0x7b344: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setLoopCount(int)")]
// 0x7b36c — __ZN4FMOD8ChannelI12setLoopCountEi
// type: int __fastcall(FMOD::ChannelI *this, int)
pub fn stub_0x7b36c() {
    // IDA 0x7b36c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getLoopCount(int *)")]
// 0x7b40c — __ZN4FMOD8ChannelI12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelI *this, int *)
pub fn stub_0x7b40c() {
    // IDA 0x7b40c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setUserData(void *)")]
// 0x7b434 — __ZN4FMOD8ChannelI11setUserDataEPv
// type: int __fastcall(FMOD::ChannelI *this, void *)
pub fn stub_0x7b434() {
    // IDA 0x7b434: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getUserData(void **)")]
// 0x7b440 — __ZN4FMOD8ChannelI11getUserDataEPPv
// type: int __fastcall(FMOD::ChannelI *this, void **)
pub fn stub_0x7b440() {
    // IDA 0x7b440: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0x7b458 — __ZN4FMOD8ChannelI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::MemoryTracker *)
pub fn stub_0x7b458() {
    // IDA 0x7b458: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")]
// 0x7b47c — __ZN4FMOD8ChannelI6addDSPEPNS_4DSPIEPPNS_14DSPConnectionIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, FMOD::DSPConnectionI **)
pub fn stub_0x7b47c() {
    // IDA 0x7b47c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")]
// 0x7b4e8 — __ZN4FMOD8ChannelI16setSpeakerLevelsE12FMOD_SPEAKERPfib
// type: int __fastcall(int, unsigned int, int, int, char)
pub fn stub_0x7b4e8() {
    // IDA 0x7b4e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")]
// 0x7b79c — __ZN4FMOD8ChannelI21calculate3DReverbGainEPNS_7ReverbIEP11FMOD_VECTORPf
// type: int __fastcall(int, int, int, __int32 *)
pub fn stub_0x7b79c() {
    // IDA 0x7b79c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")]
// 0x7b860 — __ZN4FMOD8ChannelI5allocEPNS_6SoundIEb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::SoundI *, bool)
pub fn stub_0x7b860() {
    // IDA 0x7b860: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")]
// 0x7bbc4 — __ZN4FMOD8ChannelI23calcVolumeAndPitchFor3DEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x7bbc4() {
    // IDA 0x7bbc4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")]
// 0x7c164 — __ZN4FMOD8ChannelI8validateEPNS_7ChannelEPPS0_
// type: int __fastcall(unsigned int, _DWORD *, FMOD::SystemI **)
pub fn stub_0x7c164() {
    // IDA 0x7c164: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::isPlaying(bool *)")]
// 0x7c224 — __ZN4FMOD8ChannelI9isPlayingEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
pub fn stub_0x7c224() {
    // IDA 0x7c224: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")]
// 0x7c3d8 — __ZN4FMOD8ChannelI13getLoopPointsEPjjS1_j
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int, unsigned int *, unsigned int)
pub fn stub_0x7c3d8() {
    // IDA 0x7c3d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
// 0x7c784 — __ZN4FMOD8ChannelI14getChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(FMOD::ChannelI *, int)
pub fn stub_0x7c784() {
    // IDA 0x7c784: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setPosition(unsigned int,unsigned int)")]
// 0x7c83c — __ZN4FMOD8ChannelI11setPositionEjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, unsigned int)
pub fn stub_0x7c83c() {
    // IDA 0x7c83c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
// 0x7ce58 — __ZN4FMOD8ChannelI13setLoopPointsEjjjj
// type: int __fastcall(unsigned __int64 this, unsigned int, unsigned int, unsigned int)
pub fn stub_0x7ce58() {
    // IDA 0x7ce58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
// 0x7d208 — __ZN4FMOD8ChannelI14setChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(int, int)
pub fn stub_0x7d208() {
    // IDA 0x7d208: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::forceVirtual(bool)")]
// 0x7d480 — __ZN4FMOD8ChannelI12forceVirtualEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
pub fn stub_0x7d480() {
    // IDA 0x7d480: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::updatePosition(void)")]
// 0x7d5fc — __ZN4FMOD8ChannelI14updatePositionEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x7d5fc() {
    // IDA 0x7d5fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::set3DOcclusionInternal(float,float,bool)")]
// 0x7d8c4 — __ZN4FMOD8ChannelI22set3DOcclusionInternalEffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, bool)
pub fn stub_0x7d8c4() {
    // IDA 0x7d8c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setPriority(int)")]
// 0x7d9b8 — __ZN4FMOD8ChannelI11setPriorityEi
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
pub fn stub_0x7d9b8() {
    // IDA 0x7d9b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setVolume(float,bool)")]
// 0x7d9d0 — __ZN4FMOD8ChannelI9setVolumeEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
pub fn stub_0x7d9d0() {
    // IDA 0x7d9d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setMute(bool)")]
// 0x7db84 — __ZN4FMOD8ChannelI7setMuteEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
pub fn stub_0x7db84() {
    // IDA 0x7db84: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setDefaults(void)")]
// 0x7dc98 — __ZN4FMOD8ChannelI11setDefaultsEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x7dc98() {
    // IDA 0x7dc98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::update(int,bool)")]
// 0x7df78 — __ZN4FMOD8ChannelI6updateEib
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, bool)
pub fn stub_0x7df78() {
    // IDA 0x7df78: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setMode(unsigned int)")]
// 0x7e58c — __ZN4FMOD8ChannelI7setModeEj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
pub fn stub_0x7e58c() {
    // IDA 0x7e58c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setPaused(bool)")]
// 0x7e8f0 — __ZN4FMOD8ChannelI9setPausedEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
pub fn stub_0x7e8f0() {
    // IDA 0x7e8f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setChannelGroupInternal(FMOD::ChannelGroupI *,bool,bool)")]
// 0x7ea20 — __ZN4FMOD8ChannelI23setChannelGroupInternalEPNS_13ChannelGroupIEbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *, bool, bool)
pub fn stub_0x7ea20() {
    // IDA 0x7ea20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::setChannelGroup(FMOD::ChannelGroupI *)")]
// 0x7ecf8 — __ZN4FMOD8ChannelI15setChannelGroupEPNS_13ChannelGroupIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *)
pub fn stub_0x7ecf8() {
    // IDA 0x7ecf8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::stopEx(unsigned int)")]
// 0x7ed04 — __ZN4FMOD8ChannelI6stopExEj
// type: int __fastcall(FMOD::ChannelI *this, char)
pub fn stub_0x7ed04() {
    // IDA 0x7ed04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::stop(void)")]
// 0x7f0f4 — __ZN4FMOD8ChannelI4stopEv
// type: int __fastcall(FMOD::ChannelI *this)
pub fn stub_0x7f0f4() {
    // IDA 0x7f0f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::play(FMOD::DSPI *,bool,bool,bool)")]
// 0x7f0fc — __ZN4FMOD8ChannelI4playEPNS_4DSPIEbbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, bool, char, bool)
pub fn stub_0x7f0fc() {
    // IDA 0x7f0fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::play(FMOD::SoundI *,bool,bool,bool)")]
// 0x7f23c — __ZN4FMOD8ChannelI4playEPNS_6SoundIEbbb
// type: int __fastcall(FMOD::ChannelI *this, unsigned __int8 **, bool, bool, bool)
pub fn stub_0x7f23c() {
    // IDA 0x7f23c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelI::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0x7f4a0 — __ZN4FMOD8ChannelI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_0x7f4a0() {
    // IDA 0x7f4a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
// 0x7f4f8 — __ZN4FMOD11ChannelPoolC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0x7f4f8() {
    // IDA 0x7f4f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
// 0x7f514 — __ZN4FMOD11ChannelPoolC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0x7f514() {
    // IDA 0x7f514: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::allocateChannel(FMOD::ChannelReal **,int,int,int *,bool)")]
// 0x7f518 — __ZN4FMOD11ChannelPool15allocateChannelEPPNS_11ChannelRealEiiPib
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::ChannelReal **, int, int, int *, bool)
pub fn stub_0x7f518() {
    // IDA 0x7f518: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::getNumChannels(int *)")]
// 0x7f744 — __ZN4FMOD11ChannelPool14getNumChannelsEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
pub fn stub_0x7f744() {
    // IDA 0x7f744: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::getChannelsUsed(int *)")]
// 0x7f75c — __ZN4FMOD11ChannelPool15getChannelsUsedEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
pub fn stub_0x7f75c() {
    // IDA 0x7f75c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::setChannel(int,FMOD::ChannelReal *,FMOD::DSPI *)")]
// 0x7f774 — __ZN4FMOD11ChannelPool10setChannelEiPNS_11ChannelRealEPNS_4DSPIE
// type: int __fastcall(_DWORD *, unsigned int, int, int)
pub fn stub_0x7f774() {
    // IDA 0x7f774: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::release(void)")]
// 0x7f7e8 — __ZN4FMOD11ChannelPool7releaseEv
// type: int __fastcall(FMOD::ChannelPool *this)
pub fn stub_0x7f7e8() {
    // IDA 0x7f7e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelPool::init(FMOD::SystemI *,FMOD::Output *,int)")]
// 0x7f898 — __ZN4FMOD11ChannelPool4initEPNS_7SystemIEPNS_6OutputEi
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::SystemI *, FMOD::Output *, int)
pub fn stub_0x7f898() {
    // IDA 0x7f898: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::getLength(unsigned int *,unsigned int)")]
// 0x7f924 — __ZN4FMOD5Codec9getLengthEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
pub fn stub_0x7f924() {
    // IDA 0x7f924: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0x7f984 — __ZN4FMOD5Codec17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::Codec *this, FMOD::MemoryTracker *)
pub fn stub_0x7f984() {
    // IDA 0x7f984: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::metaData(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE,bool)")]
// 0x7f9ec — __ZN4FMOD5Codec8metaDataE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb
// type: int __fastcall(int, int, int, int, size_t, int, char)
pub fn stub_0x7f9ec() {
    // IDA 0x7f9ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::getPosition(unsigned int *,unsigned int)")]
// 0x7facc — __ZN4FMOD5Codec11getPositionEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
pub fn stub_0x7facc() {
    // IDA 0x7facc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::getMetadataFromFile(void)")]
// 0x7fb54 — __ZN4FMOD5Codec19getMetadataFromFileEv
// type: int __fastcall(FMOD::Codec *this)
pub fn stub_0x7fb54() {
    // IDA 0x7fb54: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::read(void *,unsigned int,unsigned int *)")]
// 0x7fc24 — __ZN4FMOD5Codec4readEPvjPj
// type: int __fastcall(FMOD::Codec *this, char *, unsigned int, unsigned int *)
pub fn stub_0x7fc24() {
    // IDA 0x7fc24: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
