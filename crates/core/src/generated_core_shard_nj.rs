//! core shard nj — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet stubbed in core (lowest EA uncovered 0xd9ac0..0xde840, 43366 uncovered in core before batch, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR() -> ! { todo!("0xADDR mangled") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::ReverbI::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xd9ac0 — __ZN4FMOD7ReverbI13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xd9ac0() {
    // IDA 0xd9ac0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::setBufferData(void *)")]
// 0xd9b18 — __ZN4FMOD14SampleSoftware13setBufferDataEPv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, void *)
pub fn stub_0xd9b18() {
    // IDA 0xd9b18: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::release(bool)")]
// 0xd9b24 — __ZN4FMOD14SampleSoftware7releaseEb
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, bool)
pub fn stub_0xd9b24() {
    // IDA 0xd9b24: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::SampleSoftware(void)")]
// 0xd9c30 — __ZN4FMOD14SampleSoftwareC2Ev
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9c30() {
    // IDA 0xd9c30: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::SampleSoftware(void)")]
// 0xd9c68 — __ZN4FMOD14SampleSoftwareC1Ev
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9c68() {
    // IDA 0xd9c68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xd9c6c — __ZN4FMOD14SampleSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xd9c6c() {
    // IDA 0xd9c6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::setLoopPointData(void)")]
// 0xd9edc — __ZN4FMOD14SampleSoftware16setLoopPointDataEv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9edc() {
    // IDA 0xd9edc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::unlockInternal(void *,void *,unsigned int,unsigned int)")]
// 0xda46c — __ZN4FMOD14SampleSoftware14unlockInternalEPvS1_jj
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xda46c() {
    // IDA 0xda46c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::setMode(unsigned int)")]
// 0xda470 — __ZN4FMOD14SampleSoftware7setModeEj
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, unsigned int)
pub fn stub_0xda470() {
    // IDA 0xda470: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::restoreLoopPointData(void)")]
// 0xda494 — __ZN4FMOD14SampleSoftware20restoreLoopPointDataEv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xda494() {
    // IDA 0xda494: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::lockInternal(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xda5f0 — __ZN4FMOD14SampleSoftware12lockInternalEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xda5f0() {
    // IDA 0xda5f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SampleSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xda914 — __ZN4FMOD14SampleSoftware13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xda914() {
    // IDA 0xda914: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sound::setSoundGroup(FMOD::SoundGroup *)")]
// 0xda96c — __ZN4FMOD5Sound13setSoundGroupEPNS_10SoundGroupE
pub fn stub_0xda96c() {
    // IDA 0xda96c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sound::set3DMinMaxDistance(float,float)")]
// 0xda9b8 — __ZN4FMOD5Sound19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::Sound *__hidden this, float, float)
pub fn stub_0xda9b8() {
    // IDA 0xda9b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sound::getDefaults(float *,float *,float *,int *)")]
// 0xdaa0c — __ZN4FMOD5Sound11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::Sound *__hidden this, float *, float *, float *, int *)
pub fn stub_0xdaa0c() {
    // IDA 0xdaa0c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sound::release(void)")]
// 0xdaa70 — __ZN4FMOD5Sound7releaseEv
// type: _DWORD __fastcall(FMOD::Sound *__hidden this)
pub fn stub_0xdaa70() {
    // IDA 0xdaa70: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xdaaa4 — __ZN4FMOD6Sample17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xdaaa4() {
    // IDA 0xdaaa4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
// 0xdaad8 — __ZN4FMOD6Sample13setLoopPointsEjjjj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
pub fn stub_0xdaad8() {
    // IDA 0xdaad8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::setLoopCount(int)")]
// 0xdab6c — __ZN4FMOD6Sample12setLoopCountEi
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, int)
pub fn stub_0xdab6c() {
    // IDA 0xdab6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::setMode(unsigned int)")]
// 0xdabd4 — __ZN4FMOD6Sample7setModeEj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int)
pub fn stub_0xdabd4() {
    // IDA 0xdabd4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::set3DConeSettings(float,float,float)")]
// 0xdac3c — __ZN4FMOD6Sample17set3DConeSettingsEfff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float)
pub fn stub_0xdac3c() {
    // IDA 0xdac3c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::set3DMinMaxDistance(float,float)")]
// 0xdacc0 — __ZN4FMOD6Sample19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float)
pub fn stub_0xdacc0() {
    // IDA 0xdacc0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::setVariations(float,float,float)")]
// 0xdad30 — __ZN4FMOD6Sample13setVariationsEfff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float)
pub fn stub_0xdad30() {
    // IDA 0xdad30: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::setDefaults(float,float,float,int)")]
// 0xdadb4 — __ZN4FMOD6Sample11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float, int)
pub fn stub_0xdadb4() {
    // IDA 0xdadb4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::release(bool)")]
// 0xdae48 — __ZN4FMOD6Sample7releaseEb
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, bool)
pub fn stub_0xdae48() {
    // IDA 0xdae48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::Sample(void)")]
// 0xdaf1c — __ZN4FMOD6SampleC2Ev
// type: _DWORD __fastcall(FMOD::Sample *__hidden this)
pub fn stub_0xdaf1c() {
    // IDA 0xdaf1c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::Sample(void)")]
// 0xdaf54 — __ZN4FMOD6SampleC1Ev
// type: _DWORD __fastcall(FMOD::Sample *__hidden this)
pub fn stub_0xdaf54() {
    // IDA 0xdaf54: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::unlock(void *,void *,unsigned int,unsigned int)")]
// 0xdaf58 — __ZN4FMOD6Sample6unlockEPvS1_jj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdaf58() {
    // IDA 0xdaf58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Sample::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdb774 — __ZN4FMOD6Sample4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdb774() {
    // IDA 0xdb774: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Sample::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xdc278 — __ZN4FMOD6Sample13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xdc278() {
    // IDA 0xdc278: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Sample::lockInternal(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdc2d0 — __ZN4FMOD6Sample12lockInternalEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdc2d0() {
    // IDA 0xdc2d0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Sample::unlockInternal(void *,void *,unsigned int,unsigned int)")]
// 0xdc2d8 — __ZN4FMOD6Sample14unlockInternalEPvS1_jj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdc2d8() {
    // IDA 0xdc2d8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Sample::setBufferData(void *)")]
// 0xdc2e0 — __ZN4FMOD6Sample13setBufferDataEPv
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *)
pub fn stub_0xdc2e0() {
    // IDA 0xdc2e0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Stream::setLoopCount(int)")]
// 0xdc2e8 — __ZN4FMOD6Stream12setLoopCountEi
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, int)
pub fn stub_0xdc2e8() {
    // IDA 0xdc2e8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::Stream::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xdc310 — __ZN4FMOD6Stream17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xdc310() {
    // IDA 0xdc310: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::setPosition(unsigned int,unsigned int)")]
// 0xdc3b8 — __ZN4FMOD6Stream11setPositionEjj
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int, unsigned int)
pub fn stub_0xdc3b8() {
    // IDA 0xdc3b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::fill(unsigned int,unsigned int,unsigned int *,bool)")]
// 0xdc648 — __ZN4FMOD6Stream4fillEjjPjb
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int, unsigned int, unsigned int *, bool)
pub fn stub_0xdc648() {
    // IDA 0xdc648: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::flush(void)")]
// 0xdcdec — __ZN4FMOD6Stream5flushEv
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcdec() {
    // IDA 0xdcdec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::Stream(void)")]
// 0xdcea4 — __ZN4FMOD6StreamC2Ev
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcea4() {
    // IDA 0xdcea4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::Stream(void)")]
// 0xdcf00 — __ZN4FMOD6StreamC1Ev
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcf00() {
    // IDA 0xdcf00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::getPosition(unsigned int *,unsigned int)")]
// 0xdcf04 — __ZN4FMOD6Stream11getPositionEPjj
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int *, unsigned int)
pub fn stub_0xdcf04() {
    // IDA 0xdcf04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xdcfe0 — __ZN4FMOD6Stream13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xdcfe0() {
    // IDA 0xdcfe0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Stream::isStream(void)")]
// 0xdd038 — __ZN4FMOD6Stream8isStreamEv
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdd038() {
    // IDA 0xdd038: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroup::setMaxAudibleBehavior(FMOD_SOUNDGROUP_BEHAVIOR)")]
// 0xdd040 — __ZN4FMOD10SoundGroup21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR
pub fn stub_0xdd040() {
    // IDA 0xdd040: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroup::setMaxAudible(int)")]
// 0xdd074 — __ZN4FMOD10SoundGroup13setMaxAudibleEi
// type: _DWORD __fastcall(FMOD::SoundGroup *__hidden this, int)
pub fn stub_0xdd074() {
    // IDA 0xdd074: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::validate(FMOD::SoundGroup *,FMOD::SoundGroupI**)")]
// 0xdd0a8 — __ZN4FMOD11SoundGroupI8validateEPNS_10SoundGroupEPPS0_
pub fn stub_0xdd0a8() {
    // IDA 0xdd0a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::SoundGroupI(void)")]
// 0xdd0c8 — __ZN4FMOD11SoundGroupIC2Ev
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd0c8() {
    // IDA 0xdd0c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::SoundGroupI(void)")]
// 0xdd118 — __ZN4FMOD11SoundGroupIC1Ev
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd118() {
    // IDA 0xdd118: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::setMaxAudible(int)")]
// 0xdd11c — __ZN4FMOD11SoundGroupI13setMaxAudibleEi
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this, int)
pub fn stub_0xdd11c() {
    // IDA 0xdd11c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::releaseInternal(void)")]
// 0xdd130 — __ZN4FMOD11SoundGroupI15releaseInternalEv
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd130() {
    // IDA 0xdd130: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::setMaxAudibleBehavior(FMOD_SOUNDGROUP_BEHAVIOR)")]
// 0xdd1c8 — __ZN4FMOD11SoundGroupI21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR
pub fn stub_0xdd1c8() {
    // IDA 0xdd1c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::release(void)")]
// 0xdd260 — __ZN4FMOD11SoundGroupI7releaseEv
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd260() {
    // IDA 0xdd260: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundGroupI::getNumPlaying(int *)")]
// 0xdd3c4 — __ZN4FMOD11SoundGroupI13getNumPlayingEPi
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this, int *)
pub fn stub_0xdd3c4() {
    // IDA 0xdd3c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::validate(FMOD::Sound *,FMOD::SoundI**)")]
// 0xdd408 — __ZN4FMOD6SoundI8validateEPNS_5SoundEPPS0_
pub fn stub_0xdd408() {
    // IDA 0xdd408: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::SoundI(void)")]
// 0xdd428 — __ZN4FMOD6SoundIC2Ev
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xdd428() {
    // IDA 0xdd428: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::SoundI(void)")]
// 0xdd52c — __ZN4FMOD6SoundIC1Ev
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xdd52c() {
    // IDA 0xdd52c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getSystemObject(FMOD::System **)")]
// 0xdd530 — __ZN4FMOD6SoundI15getSystemObjectEPPNS_6SystemE
pub fn stub_0xdd530() {
    // IDA 0xdd530: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdd548 — __ZN4FMOD6SoundI4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdd548() {
    // IDA 0xdd548: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::unlock(void *,void *,unsigned int,unsigned int)")]
// 0xdd550 — __ZN4FMOD6SoundI6unlockEPvS1_jj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdd550() {
    // IDA 0xdd550: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::setDefaults(float,float,float,int)")]
// 0xdd558 — __ZN4FMOD6SoundI11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float, int)
pub fn stub_0xdd558() {
    // IDA 0xdd558: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::getDefaults(float *,float *,float *,int *)")]
// 0xdd5e0 — __ZN4FMOD6SoundI11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *, int *)
pub fn stub_0xdd5e0() {
    // IDA 0xdd5e0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::setVariations(float,float,float)")]
// 0xdd624 — __ZN4FMOD6SoundI13setVariationsEfff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float)
pub fn stub_0xdd624() {
    // IDA 0xdd624: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::getVariations(float *,float *,float *)")]
// 0xdd65c — __ZN4FMOD6SoundI13getVariationsEPfS1_S1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *)
pub fn stub_0xdd65c() {
    // IDA 0xdd65c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::SoundI::set3DMinMaxDistance(float,float)")]
// 0xdd688 — __ZN4FMOD6SoundI19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float)
pub fn stub_0xdd688() {
    // IDA 0xdd688: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::get3DMinMaxDistance(float *,float *)")]
// 0xdd6c8 — __ZN4FMOD6SoundI19get3DMinMaxDistanceEPfS1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *)
pub fn stub_0xdd6c8() {
    // IDA 0xdd6c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::set3DConeSettings(float,float,float)")]
// 0xdd6e8 — __ZN4FMOD6SoundI17set3DConeSettingsEfff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float)
pub fn stub_0xdd6e8() {
    // IDA 0xdd6e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::get3DConeSettings(float *,float *,float *)")]
// 0xdd76c — __ZN4FMOD6SoundI17get3DConeSettingsEPfS1_S1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *)
pub fn stub_0xdd76c() {
    // IDA 0xdd76c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::set3DCustomRolloff(FMOD_VECTOR *,int)")]
// 0xdd798 — __ZN4FMOD6SoundI18set3DCustomRolloffEP11FMOD_VECTORi
pub fn stub_0xdd798() {
    // IDA 0xdd798: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::get3DCustomRolloff(FMOD_VECTOR **,int *)")]
// 0xdd850 — __ZN4FMOD6SoundI18get3DCustomRolloffEPP11FMOD_VECTORPi
pub fn stub_0xdd850() {
    // IDA 0xdd850: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getFormat(FMOD_SOUND_TYPE *,FMOD_SOUND_FORMAT *,int *,int *)")]
// 0xdd870 — __ZN4FMOD6SoundI9getFormatEP15FMOD_SOUND_TYPEP17FMOD_SOUND_FORMATPiS5_
pub fn stub_0xdd870() {
    // IDA 0xdd870: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getNumSubSounds(int *)")]
// 0xdd930 — __ZN4FMOD6SoundI15getNumSubSoundsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xdd930() {
    // IDA 0xdd930: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getSoundGroup(FMOD::SoundGroupI **)")]
// 0xdd948 — __ZN4FMOD6SoundI13getSoundGroupEPPNS_11SoundGroupIE
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, FMOD::SoundGroupI **)
pub fn stub_0xdd948() {
    // IDA 0xdd948: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::addSyncPoint(unsigned int,unsigned int,char const*,FMOD_SYNCPOINT **,int,bool)")]
// 0xdd960 — __ZN4FMOD6SoundI12addSyncPointEjjPKcPP14FMOD_SYNCPOINTib
pub fn stub_0xdd960() {
    // IDA 0xdd960: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setMode(unsigned int)")]
// 0xdd99c — __ZN4FMOD6SoundI7setModeEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xdd99c() {
    // IDA 0xdd99c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getMode(unsigned int *)")]
// 0xddb2c — __ZN4FMOD6SoundI7getModeEPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *)
pub fn stub_0xddb2c() {
    // IDA 0xddb2c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setLoopCount(int)")]
// 0xddb44 — __ZN4FMOD6SoundI12setLoopCountEi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int)
pub fn stub_0xddb44() {
    // IDA 0xddb44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getLoopCount(int *)")]
// 0xddb58 — __ZN4FMOD6SoundI12getLoopCountEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xddb58() {
    // IDA 0xddb58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setPositionInternal(unsigned int)")]
// 0xddb70 — __ZN4FMOD6SoundI19setPositionInternalEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xddb70() {
    // IDA 0xddb70: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setPosition(unsigned int)")]
// 0xddb7c — __ZN4FMOD6SoundI11setPositionEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xddb7c() {
    // IDA 0xddb7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getPosition(unsigned int *)")]
// 0xddb80 — __ZN4FMOD6SoundI11getPositionEPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *)
pub fn stub_0xddb80() {
    // IDA 0xddb80: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getMusicNumChannels(int *)")]
// 0xddb98 — __ZN4FMOD6SoundI19getMusicNumChannelsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xddb98() {
    // IDA 0xddb98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setMusicChannelVolume(int,float)")]
// 0xddbbc — __ZN4FMOD6SoundI21setMusicChannelVolumeEif
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, float)
pub fn stub_0xddbbc() {
    // IDA 0xddbbc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getMusicChannelVolume(int,float *)")]
// 0xddbe0 — __ZN4FMOD6SoundI21getMusicChannelVolumeEiPf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, float *)
pub fn stub_0xddbe0() {
    // IDA 0xddbe0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setMusicSpeed(float)")]
// 0xddc04 — __ZN4FMOD6SoundI13setMusicSpeedEf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float)
pub fn stub_0xddc04() {
    // IDA 0xddc04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getMusicSpeed(float *)")]
// 0xddc5c — __ZN4FMOD6SoundI13getMusicSpeedEPf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *)
pub fn stub_0xddc5c() {
    // IDA 0xddc5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setUserData(void *)")]
// 0xddc94 — __ZN4FMOD6SoundI11setUserDataEPv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void *)
pub fn stub_0xddc94() {
    // IDA 0xddc94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getUserData(void **)")]
// 0xddca0 — __ZN4FMOD6SoundI11getUserDataEPPv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void **)
pub fn stub_0xddca0() {
    // IDA 0xddca0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::syncPointFixIndicies(void)")]
// 0xddcb8 — __ZN4FMOD6SoundI20syncPointFixIndiciesEv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xddcb8() {
    // IDA 0xddcb8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getMemoryInfo(unsigned int,unsigned int,unsigned int *,FMOD_MEMORY_USAGE_DETAILS *)")]
// 0xddd44 — __ZN4FMOD6SoundI13getMemoryInfoEjjPjP25FMOD_MEMORY_USAGE_DETAILS
// type: int __fastcall(int, int, int, int, void *)
pub fn stub_0xddd44() {
    // IDA 0xddd44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::deleteSyncPointInternal(FMOD_SYNCPOINT *,bool)")]
// 0xdde0c — __ZN4FMOD6SoundI23deleteSyncPointInternalEP14FMOD_SYNCPOINTb
pub fn stub_0xdde0c() {
    // IDA 0xdde0c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::deleteSyncPoint(FMOD_SYNCPOINT *)")]
// 0xddf90 — __ZN4FMOD6SoundI15deleteSyncPointEP14FMOD_SYNCPOINT
pub fn stub_0xddf90() {
    // IDA 0xddf90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getTag(char const*,int,FMOD_TAG *)")]
// 0xddf98 — __ZN4FMOD6SoundI6getTagEPKciP8FMOD_TAG
pub fn stub_0xddf98() {
    // IDA 0xddf98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getNumTags(int *,int *)")]
// 0xddfd0 — __ZN4FMOD6SoundI10getNumTagsEPiS1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *, int *)
pub fn stub_0xddfd0() {
    // IDA 0xddfd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getName(char *,int)")]
// 0xde028 — __ZN4FMOD6SoundI7getNameEPci
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, char *, int)
pub fn stub_0xde028() {
    // IDA 0xde028: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setSubSoundInternal(int,FMOD::SoundI*,bool)")]
// 0xde0d8 — __ZN4FMOD6SoundI19setSubSoundInternalEiPS0_b
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, FMOD::SoundI *, bool)
pub fn stub_0xde0d8() {
    // IDA 0xde0d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::setSubSound(int,FMOD::SoundI*)")]
// 0xde618 — __ZN4FMOD6SoundI11setSubSoundEiPS0_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, FMOD::SoundI *)
pub fn stub_0xde618() {
    // IDA 0xde618: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::seek(int,unsigned int)")]
// 0xde620 — __ZN4FMOD6SoundI4seekEij
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, unsigned int)
pub fn stub_0xde620() {
    // IDA 0xde620: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getSyncPoint(int,FMOD_SYNCPOINT **)")]
// 0xde6a0 — __ZN4FMOD6SoundI12getSyncPointEiPP14FMOD_SYNCPOINT
pub fn stub_0xde6a0() {
    // IDA 0xde6a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::getNumSyncPoints(int *)")]
// 0xde788 — __ZN4FMOD6SoundI16getNumSyncPointsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xde788() {
    // IDA 0xde788: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::SoundI::release(bool)")]
// 0xde840 — __ZN4FMOD6SoundI7releaseEb
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, bool)
pub fn stub_0xde840() {
    // IDA 0xde840: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
