//! core shard nj — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet stubbed in core (lowest EA uncovered 0xd9ac0..0xde840, 43366 uncovered in core before batch, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR() -> ! { todo!("0xADDR mangled") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::ReverbI::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xd9ac0 — __ZN4FMOD7ReverbI13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xd9ac0() -> ! { todo!("0xd9ac0 __ZN4FMOD7ReverbI13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::SampleSoftware::setBufferData(void *)")]
// 0xd9b18 — __ZN4FMOD14SampleSoftware13setBufferDataEPv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, void *)
pub fn stub_0xd9b18() -> ! { todo!("0xd9b18 __ZN4FMOD14SampleSoftware13setBufferDataEPv") }

#[doc(alias = "FMOD::SampleSoftware::release(bool)")]
// 0xd9b24 — __ZN4FMOD14SampleSoftware7releaseEb
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, bool)
pub fn stub_0xd9b24() -> ! { todo!("0xd9b24 __ZN4FMOD14SampleSoftware7releaseEb") }

#[doc(alias = "FMOD::SampleSoftware::SampleSoftware(void)")]
// 0xd9c30 — __ZN4FMOD14SampleSoftwareC2Ev
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9c30() -> ! { todo!("0xd9c30 __ZN4FMOD14SampleSoftwareC2Ev") }

#[doc(alias = "FMOD::SampleSoftware::SampleSoftware(void)")]
// 0xd9c68 — __ZN4FMOD14SampleSoftwareC1Ev
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9c68() -> ! { todo!("0xd9c68 __ZN4FMOD14SampleSoftwareC1Ev") }

#[doc(alias = "FMOD::SampleSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xd9c6c — __ZN4FMOD14SampleSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xd9c6c() -> ! { todo!("0xd9c6c __ZN4FMOD14SampleSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::SampleSoftware::setLoopPointData(void)")]
// 0xd9edc — __ZN4FMOD14SampleSoftware16setLoopPointDataEv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xd9edc() -> ! { todo!("0xd9edc __ZN4FMOD14SampleSoftware16setLoopPointDataEv") }

#[doc(alias = "FMOD::SampleSoftware::unlockInternal(void *,void *,unsigned int,unsigned int)")]
// 0xda46c — __ZN4FMOD14SampleSoftware14unlockInternalEPvS1_jj
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xda46c() -> ! { todo!("0xda46c __ZN4FMOD14SampleSoftware14unlockInternalEPvS1_jj") }

#[doc(alias = "FMOD::SampleSoftware::setMode(unsigned int)")]
// 0xda470 — __ZN4FMOD14SampleSoftware7setModeEj
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, unsigned int)
pub fn stub_0xda470() -> ! { todo!("0xda470 __ZN4FMOD14SampleSoftware7setModeEj") }

#[doc(alias = "FMOD::SampleSoftware::restoreLoopPointData(void)")]
// 0xda494 — __ZN4FMOD14SampleSoftware20restoreLoopPointDataEv
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this)
pub fn stub_0xda494() -> ! { todo!("0xda494 __ZN4FMOD14SampleSoftware20restoreLoopPointDataEv") }

#[doc(alias = "FMOD::SampleSoftware::lockInternal(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xda5f0 — __ZN4FMOD14SampleSoftware12lockInternalEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::SampleSoftware *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xda5f0() -> ! { todo!("0xda5f0 __ZN4FMOD14SampleSoftware12lockInternalEjjPPvS2_PjS3_") }

#[doc(alias = "FMOD::SampleSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xda914 — __ZN4FMOD14SampleSoftware13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xda914() -> ! { todo!("0xda914 __ZN4FMOD14SampleSoftware13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::Sound::setSoundGroup(FMOD::SoundGroup *)")]
// 0xda96c — __ZN4FMOD5Sound13setSoundGroupEPNS_10SoundGroupE
pub fn stub_0xda96c() -> ! { todo!("0xda96c __ZN4FMOD5Sound13setSoundGroupEPNS_10SoundGroupE") }

#[doc(alias = "FMOD::Sound::set3DMinMaxDistance(float,float)")]
// 0xda9b8 — __ZN4FMOD5Sound19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::Sound *__hidden this, float, float)
pub fn stub_0xda9b8() -> ! { todo!("0xda9b8 __ZN4FMOD5Sound19set3DMinMaxDistanceEff") }

#[doc(alias = "FMOD::Sound::getDefaults(float *,float *,float *,int *)")]
// 0xdaa0c — __ZN4FMOD5Sound11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::Sound *__hidden this, float *, float *, float *, int *)
pub fn stub_0xdaa0c() -> ! { todo!("0xdaa0c __ZN4FMOD5Sound11getDefaultsEPfS1_S1_Pi") }

#[doc(alias = "FMOD::Sound::release(void)")]
// 0xdaa70 — __ZN4FMOD5Sound7releaseEv
// type: _DWORD __fastcall(FMOD::Sound *__hidden this)
pub fn stub_0xdaa70() -> ! { todo!("0xdaa70 __ZN4FMOD5Sound7releaseEv") }

#[doc(alias = "FMOD::Sample::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xdaaa4 — __ZN4FMOD6Sample17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xdaaa4() -> ! { todo!("0xdaaa4 __ZN4FMOD6Sample17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::Sample::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
// 0xdaad8 — __ZN4FMOD6Sample13setLoopPointsEjjjj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
pub fn stub_0xdaad8() -> ! { todo!("0xdaad8 __ZN4FMOD6Sample13setLoopPointsEjjjj") }

#[doc(alias = "FMOD::Sample::setLoopCount(int)")]
// 0xdab6c — __ZN4FMOD6Sample12setLoopCountEi
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, int)
pub fn stub_0xdab6c() -> ! { todo!("0xdab6c __ZN4FMOD6Sample12setLoopCountEi") }

#[doc(alias = "FMOD::Sample::setMode(unsigned int)")]
// 0xdabd4 — __ZN4FMOD6Sample7setModeEj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int)
pub fn stub_0xdabd4() -> ! { todo!("0xdabd4 __ZN4FMOD6Sample7setModeEj") }

#[doc(alias = "FMOD::Sample::set3DConeSettings(float,float,float)")]
// 0xdac3c — __ZN4FMOD6Sample17set3DConeSettingsEfff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float)
pub fn stub_0xdac3c() -> ! { todo!("0xdac3c __ZN4FMOD6Sample17set3DConeSettingsEfff") }

#[doc(alias = "FMOD::Sample::set3DMinMaxDistance(float,float)")]
// 0xdacc0 — __ZN4FMOD6Sample19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float)
pub fn stub_0xdacc0() -> ! { todo!("0xdacc0 __ZN4FMOD6Sample19set3DMinMaxDistanceEff") }

#[doc(alias = "FMOD::Sample::setVariations(float,float,float)")]
// 0xdad30 — __ZN4FMOD6Sample13setVariationsEfff
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float)
pub fn stub_0xdad30() -> ! { todo!("0xdad30 __ZN4FMOD6Sample13setVariationsEfff") }

#[doc(alias = "FMOD::Sample::setDefaults(float,float,float,int)")]
// 0xdadb4 — __ZN4FMOD6Sample11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, float, float, float, int)
pub fn stub_0xdadb4() -> ! { todo!("0xdadb4 __ZN4FMOD6Sample11setDefaultsEfffi") }

#[doc(alias = "FMOD::Sample::release(bool)")]
// 0xdae48 — __ZN4FMOD6Sample7releaseEb
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, bool)
pub fn stub_0xdae48() -> ! { todo!("0xdae48 __ZN4FMOD6Sample7releaseEb") }

#[doc(alias = "FMOD::Sample::Sample(void)")]
// 0xdaf1c — __ZN4FMOD6SampleC2Ev
// type: _DWORD __fastcall(FMOD::Sample *__hidden this)
pub fn stub_0xdaf1c() -> ! { todo!("0xdaf1c __ZN4FMOD6SampleC2Ev") }

#[doc(alias = "FMOD::Sample::Sample(void)")]
// 0xdaf54 — __ZN4FMOD6SampleC1Ev
// type: _DWORD __fastcall(FMOD::Sample *__hidden this)
pub fn stub_0xdaf54() -> ! { todo!("0xdaf54 __ZN4FMOD6SampleC1Ev") }

#[doc(alias = "FMOD::Sample::unlock(void *,void *,unsigned int,unsigned int)")]
// 0xdaf58 — __ZN4FMOD6Sample6unlockEPvS1_jj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdaf58() -> ! { todo!("0xdaf58 __ZN4FMOD6Sample6unlockEPvS1_jj") }

#[doc(alias = "FMOD::Sample::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdb774 — __ZN4FMOD6Sample4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdb774() -> ! { todo!("0xdb774 __ZN4FMOD6Sample4lockEjjPPvS2_PjS3_") }

#[doc(alias = "FMOD::Sample::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xdc278 — __ZN4FMOD6Sample13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xdc278() -> ! { todo!("0xdc278 __ZN4FMOD6Sample13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::Sample::lockInternal(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdc2d0 — __ZN4FMOD6Sample12lockInternalEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdc2d0() -> ! { todo!("0xdc2d0 __ZN4FMOD6Sample12lockInternalEjjPPvS2_PjS3_") }

#[doc(alias = "FMOD::Sample::unlockInternal(void *,void *,unsigned int,unsigned int)")]
// 0xdc2d8 — __ZN4FMOD6Sample14unlockInternalEPvS1_jj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdc2d8() -> ! { todo!("0xdc2d8 __ZN4FMOD6Sample14unlockInternalEPvS1_jj") }

#[doc(alias = "FMOD::Sample::setBufferData(void *)")]
// 0xdc2e0 — __ZN4FMOD6Sample13setBufferDataEPv
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, void *)
pub fn stub_0xdc2e0() -> ! { todo!("0xdc2e0 __ZN4FMOD6Sample13setBufferDataEPv") }

#[doc(alias = "FMOD::Stream::setLoopCount(int)")]
// 0xdc2e8 — __ZN4FMOD6Stream12setLoopCountEi
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, int)
pub fn stub_0xdc2e8() -> ! { todo!("0xdc2e8 __ZN4FMOD6Stream12setLoopCountEi") }

#[doc(alias = "FMOD::Stream::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xdc310 — __ZN4FMOD6Stream17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xdc310() -> ! { todo!("0xdc310 __ZN4FMOD6Stream17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::Stream::setPosition(unsigned int,unsigned int)")]
// 0xdc3b8 — __ZN4FMOD6Stream11setPositionEjj
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int, unsigned int)
pub fn stub_0xdc3b8() -> ! { todo!("0xdc3b8 __ZN4FMOD6Stream11setPositionEjj") }

#[doc(alias = "FMOD::Stream::fill(unsigned int,unsigned int,unsigned int *,bool)")]
// 0xdc648 — __ZN4FMOD6Stream4fillEjjPjb
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int, unsigned int, unsigned int *, bool)
pub fn stub_0xdc648() -> ! { todo!("0xdc648 __ZN4FMOD6Stream4fillEjjPjb") }

#[doc(alias = "FMOD::Stream::flush(void)")]
// 0xdcdec — __ZN4FMOD6Stream5flushEv
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcdec() -> ! { todo!("0xdcdec __ZN4FMOD6Stream5flushEv") }

#[doc(alias = "FMOD::Stream::Stream(void)")]
// 0xdcea4 — __ZN4FMOD6StreamC2Ev
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcea4() -> ! { todo!("0xdcea4 __ZN4FMOD6StreamC2Ev") }

#[doc(alias = "FMOD::Stream::Stream(void)")]
// 0xdcf00 — __ZN4FMOD6StreamC1Ev
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdcf00() -> ! { todo!("0xdcf00 __ZN4FMOD6StreamC1Ev") }

#[doc(alias = "FMOD::Stream::getPosition(unsigned int *,unsigned int)")]
// 0xdcf04 — __ZN4FMOD6Stream11getPositionEPjj
// type: _DWORD __fastcall(FMOD::Stream *__hidden this, unsigned int *, unsigned int)
pub fn stub_0xdcf04() -> ! { todo!("0xdcf04 __ZN4FMOD6Stream11getPositionEPjj") }

#[doc(alias = "FMOD::Stream::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xdcfe0 — __ZN4FMOD6Stream13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xdcfe0() -> ! { todo!("0xdcfe0 __ZN4FMOD6Stream13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::Stream::isStream(void)")]
// 0xdd038 — __ZN4FMOD6Stream8isStreamEv
// type: _DWORD __fastcall(FMOD::Stream *__hidden this)
pub fn stub_0xdd038() -> ! { todo!("0xdd038 __ZN4FMOD6Stream8isStreamEv") }

#[doc(alias = "FMOD::SoundGroup::setMaxAudibleBehavior(FMOD_SOUNDGROUP_BEHAVIOR)")]
// 0xdd040 — __ZN4FMOD10SoundGroup21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR
pub fn stub_0xdd040() -> ! { todo!("0xdd040 __ZN4FMOD10SoundGroup21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR") }

#[doc(alias = "FMOD::SoundGroup::setMaxAudible(int)")]
// 0xdd074 — __ZN4FMOD10SoundGroup13setMaxAudibleEi
// type: _DWORD __fastcall(FMOD::SoundGroup *__hidden this, int)
pub fn stub_0xdd074() -> ! { todo!("0xdd074 __ZN4FMOD10SoundGroup13setMaxAudibleEi") }

#[doc(alias = "FMOD::SoundGroupI::validate(FMOD::SoundGroup *,FMOD::SoundGroupI**)")]
// 0xdd0a8 — __ZN4FMOD11SoundGroupI8validateEPNS_10SoundGroupEPPS0_
pub fn stub_0xdd0a8() -> ! { todo!("0xdd0a8 __ZN4FMOD11SoundGroupI8validateEPNS_10SoundGroupEPPS0_") }

#[doc(alias = "FMOD::SoundGroupI::SoundGroupI(void)")]
// 0xdd0c8 — __ZN4FMOD11SoundGroupIC2Ev
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd0c8() -> ! { todo!("0xdd0c8 __ZN4FMOD11SoundGroupIC2Ev") }

#[doc(alias = "FMOD::SoundGroupI::SoundGroupI(void)")]
// 0xdd118 — __ZN4FMOD11SoundGroupIC1Ev
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd118() -> ! { todo!("0xdd118 __ZN4FMOD11SoundGroupIC1Ev") }

#[doc(alias = "FMOD::SoundGroupI::setMaxAudible(int)")]
// 0xdd11c — __ZN4FMOD11SoundGroupI13setMaxAudibleEi
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this, int)
pub fn stub_0xdd11c() -> ! { todo!("0xdd11c __ZN4FMOD11SoundGroupI13setMaxAudibleEi") }

#[doc(alias = "FMOD::SoundGroupI::releaseInternal(void)")]
// 0xdd130 — __ZN4FMOD11SoundGroupI15releaseInternalEv
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd130() -> ! { todo!("0xdd130 __ZN4FMOD11SoundGroupI15releaseInternalEv") }

#[doc(alias = "FMOD::SoundGroupI::setMaxAudibleBehavior(FMOD_SOUNDGROUP_BEHAVIOR)")]
// 0xdd1c8 — __ZN4FMOD11SoundGroupI21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR
pub fn stub_0xdd1c8() -> ! { todo!("0xdd1c8 __ZN4FMOD11SoundGroupI21setMaxAudibleBehaviorE24FMOD_SOUNDGROUP_BEHAVIOR") }

#[doc(alias = "FMOD::SoundGroupI::release(void)")]
// 0xdd260 — __ZN4FMOD11SoundGroupI7releaseEv
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this)
pub fn stub_0xdd260() -> ! { todo!("0xdd260 __ZN4FMOD11SoundGroupI7releaseEv") }

#[doc(alias = "FMOD::SoundGroupI::getNumPlaying(int *)")]
// 0xdd3c4 — __ZN4FMOD11SoundGroupI13getNumPlayingEPi
// type: _DWORD __fastcall(FMOD::SoundGroupI *__hidden this, int *)
pub fn stub_0xdd3c4() -> ! { todo!("0xdd3c4 __ZN4FMOD11SoundGroupI13getNumPlayingEPi") }

#[doc(alias = "FMOD::SoundI::validate(FMOD::Sound *,FMOD::SoundI**)")]
// 0xdd408 — __ZN4FMOD6SoundI8validateEPNS_5SoundEPPS0_
pub fn stub_0xdd408() -> ! { todo!("0xdd408 __ZN4FMOD6SoundI8validateEPNS_5SoundEPPS0_") }

#[doc(alias = "FMOD::SoundI::SoundI(void)")]
// 0xdd428 — __ZN4FMOD6SoundIC2Ev
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xdd428() -> ! { todo!("0xdd428 __ZN4FMOD6SoundIC2Ev") }

#[doc(alias = "FMOD::SoundI::SoundI(void)")]
// 0xdd52c — __ZN4FMOD6SoundIC1Ev
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xdd52c() -> ! { todo!("0xdd52c __ZN4FMOD6SoundIC1Ev") }

#[doc(alias = "FMOD::SoundI::getSystemObject(FMOD::System **)")]
// 0xdd530 — __ZN4FMOD6SoundI15getSystemObjectEPPNS_6SystemE
pub fn stub_0xdd530() -> ! { todo!("0xdd530 __ZN4FMOD6SoundI15getSystemObjectEPPNS_6SystemE") }

#[doc(alias = "FMOD::SoundI::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xdd548 — __ZN4FMOD6SoundI4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xdd548() -> ! { todo!("0xdd548 __ZN4FMOD6SoundI4lockEjjPPvS2_PjS3_") }

#[doc(alias = "FMOD::SoundI::unlock(void *,void *,unsigned int,unsigned int)")]
// 0xdd550 — __ZN4FMOD6SoundI6unlockEPvS1_jj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void *, void *, unsigned int, unsigned int)
pub fn stub_0xdd550() -> ! { todo!("0xdd550 __ZN4FMOD6SoundI6unlockEPvS1_jj") }

#[doc(alias = "FMOD::SoundI::setDefaults(float,float,float,int)")]
// 0xdd558 — __ZN4FMOD6SoundI11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float, int)
pub fn stub_0xdd558() -> ! { todo!("0xdd558 __ZN4FMOD6SoundI11setDefaultsEfffi") }

#[doc(alias = "FMOD::SoundI::getDefaults(float *,float *,float *,int *)")]
// 0xdd5e0 — __ZN4FMOD6SoundI11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *, int *)
pub fn stub_0xdd5e0() -> ! { todo!("0xdd5e0 __ZN4FMOD6SoundI11getDefaultsEPfS1_S1_Pi") }

#[doc(alias = "FMOD::SoundI::setVariations(float,float,float)")]
// 0xdd624 — __ZN4FMOD6SoundI13setVariationsEfff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float)
pub fn stub_0xdd624() -> ! { todo!("0xdd624 __ZN4FMOD6SoundI13setVariationsEfff") }

#[doc(alias = "FMOD::SoundI::getVariations(float *,float *,float *)")]
// 0xdd65c — __ZN4FMOD6SoundI13getVariationsEPfS1_S1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *)
pub fn stub_0xdd65c() -> ! { todo!("0xdd65c __ZN4FMOD6SoundI13getVariationsEPfS1_S1_") }

#[doc(alias = "FMOD::SoundI::set3DMinMaxDistance(float,float)")]
// 0xdd688 — __ZN4FMOD6SoundI19set3DMinMaxDistanceEff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float)
pub fn stub_0xdd688() -> ! { todo!("0xdd688 __ZN4FMOD6SoundI19set3DMinMaxDistanceEff") }

#[doc(alias = "FMOD::SoundI::get3DMinMaxDistance(float *,float *)")]
// 0xdd6c8 — __ZN4FMOD6SoundI19get3DMinMaxDistanceEPfS1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *)
pub fn stub_0xdd6c8() -> ! { todo!("0xdd6c8 __ZN4FMOD6SoundI19get3DMinMaxDistanceEPfS1_") }

#[doc(alias = "FMOD::SoundI::set3DConeSettings(float,float,float)")]
// 0xdd6e8 — __ZN4FMOD6SoundI17set3DConeSettingsEfff
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float, float, float)
pub fn stub_0xdd6e8() -> ! { todo!("0xdd6e8 __ZN4FMOD6SoundI17set3DConeSettingsEfff") }

#[doc(alias = "FMOD::SoundI::get3DConeSettings(float *,float *,float *)")]
// 0xdd76c — __ZN4FMOD6SoundI17get3DConeSettingsEPfS1_S1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *, float *, float *)
pub fn stub_0xdd76c() -> ! { todo!("0xdd76c __ZN4FMOD6SoundI17get3DConeSettingsEPfS1_S1_") }

#[doc(alias = "FMOD::SoundI::set3DCustomRolloff(FMOD_VECTOR *,int)")]
// 0xdd798 — __ZN4FMOD6SoundI18set3DCustomRolloffEP11FMOD_VECTORi
pub fn stub_0xdd798() -> ! { todo!("0xdd798 __ZN4FMOD6SoundI18set3DCustomRolloffEP11FMOD_VECTORi") }

#[doc(alias = "FMOD::SoundI::get3DCustomRolloff(FMOD_VECTOR **,int *)")]
// 0xdd850 — __ZN4FMOD6SoundI18get3DCustomRolloffEPP11FMOD_VECTORPi
pub fn stub_0xdd850() -> ! { todo!("0xdd850 __ZN4FMOD6SoundI18get3DCustomRolloffEPP11FMOD_VECTORPi") }

#[doc(alias = "FMOD::SoundI::getFormat(FMOD_SOUND_TYPE *,FMOD_SOUND_FORMAT *,int *,int *)")]
// 0xdd870 — __ZN4FMOD6SoundI9getFormatEP15FMOD_SOUND_TYPEP17FMOD_SOUND_FORMATPiS5_
pub fn stub_0xdd870() -> ! { todo!("0xdd870 __ZN4FMOD6SoundI9getFormatEP15FMOD_SOUND_TYPEP17FMOD_SOUND_FORMATPiS5_") }

#[doc(alias = "FMOD::SoundI::getNumSubSounds(int *)")]
// 0xdd930 — __ZN4FMOD6SoundI15getNumSubSoundsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xdd930() -> ! { todo!("0xdd930 __ZN4FMOD6SoundI15getNumSubSoundsEPi") }

#[doc(alias = "FMOD::SoundI::getSoundGroup(FMOD::SoundGroupI **)")]
// 0xdd948 — __ZN4FMOD6SoundI13getSoundGroupEPPNS_11SoundGroupIE
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, FMOD::SoundGroupI **)
pub fn stub_0xdd948() -> ! { todo!("0xdd948 __ZN4FMOD6SoundI13getSoundGroupEPPNS_11SoundGroupIE") }

#[doc(alias = "FMOD::SoundI::addSyncPoint(unsigned int,unsigned int,char const*,FMOD_SYNCPOINT **,int,bool)")]
// 0xdd960 — __ZN4FMOD6SoundI12addSyncPointEjjPKcPP14FMOD_SYNCPOINTib
pub fn stub_0xdd960() -> ! { todo!("0xdd960 __ZN4FMOD6SoundI12addSyncPointEjjPKcPP14FMOD_SYNCPOINTib") }

#[doc(alias = "FMOD::SoundI::setMode(unsigned int)")]
// 0xdd99c — __ZN4FMOD6SoundI7setModeEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xdd99c() -> ! { todo!("0xdd99c __ZN4FMOD6SoundI7setModeEj") }

#[doc(alias = "FMOD::SoundI::getMode(unsigned int *)")]
// 0xddb2c — __ZN4FMOD6SoundI7getModeEPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *)
pub fn stub_0xddb2c() -> ! { todo!("0xddb2c __ZN4FMOD6SoundI7getModeEPj") }

#[doc(alias = "FMOD::SoundI::setLoopCount(int)")]
// 0xddb44 — __ZN4FMOD6SoundI12setLoopCountEi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int)
pub fn stub_0xddb44() -> ! { todo!("0xddb44 __ZN4FMOD6SoundI12setLoopCountEi") }

#[doc(alias = "FMOD::SoundI::getLoopCount(int *)")]
// 0xddb58 — __ZN4FMOD6SoundI12getLoopCountEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xddb58() -> ! { todo!("0xddb58 __ZN4FMOD6SoundI12getLoopCountEPi") }

#[doc(alias = "FMOD::SoundI::setPositionInternal(unsigned int)")]
// 0xddb70 — __ZN4FMOD6SoundI19setPositionInternalEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xddb70() -> ! { todo!("0xddb70 __ZN4FMOD6SoundI19setPositionInternalEj") }

#[doc(alias = "FMOD::SoundI::setPosition(unsigned int)")]
// 0xddb7c — __ZN4FMOD6SoundI11setPositionEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xddb7c() -> ! { todo!("0xddb7c __ZN4FMOD6SoundI11setPositionEj") }

#[doc(alias = "FMOD::SoundI::getPosition(unsigned int *)")]
// 0xddb80 — __ZN4FMOD6SoundI11getPositionEPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *)
pub fn stub_0xddb80() -> ! { todo!("0xddb80 __ZN4FMOD6SoundI11getPositionEPj") }

#[doc(alias = "FMOD::SoundI::getMusicNumChannels(int *)")]
// 0xddb98 — __ZN4FMOD6SoundI19getMusicNumChannelsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xddb98() -> ! { todo!("0xddb98 __ZN4FMOD6SoundI19getMusicNumChannelsEPi") }

#[doc(alias = "FMOD::SoundI::setMusicChannelVolume(int,float)")]
// 0xddbbc — __ZN4FMOD6SoundI21setMusicChannelVolumeEif
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, float)
pub fn stub_0xddbbc() -> ! { todo!("0xddbbc __ZN4FMOD6SoundI21setMusicChannelVolumeEif") }

#[doc(alias = "FMOD::SoundI::getMusicChannelVolume(int,float *)")]
// 0xddbe0 — __ZN4FMOD6SoundI21getMusicChannelVolumeEiPf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, float *)
pub fn stub_0xddbe0() -> ! { todo!("0xddbe0 __ZN4FMOD6SoundI21getMusicChannelVolumeEiPf") }

#[doc(alias = "FMOD::SoundI::setMusicSpeed(float)")]
// 0xddc04 — __ZN4FMOD6SoundI13setMusicSpeedEf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float)
pub fn stub_0xddc04() -> ! { todo!("0xddc04 __ZN4FMOD6SoundI13setMusicSpeedEf") }

#[doc(alias = "FMOD::SoundI::getMusicSpeed(float *)")]
// 0xddc5c — __ZN4FMOD6SoundI13getMusicSpeedEPf
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, float *)
pub fn stub_0xddc5c() -> ! { todo!("0xddc5c __ZN4FMOD6SoundI13getMusicSpeedEPf") }

#[doc(alias = "FMOD::SoundI::setUserData(void *)")]
// 0xddc94 — __ZN4FMOD6SoundI11setUserDataEPv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void *)
pub fn stub_0xddc94() -> ! { todo!("0xddc94 __ZN4FMOD6SoundI11setUserDataEPv") }

#[doc(alias = "FMOD::SoundI::getUserData(void **)")]
// 0xddca0 — __ZN4FMOD6SoundI11getUserDataEPPv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void **)
pub fn stub_0xddca0() -> ! { todo!("0xddca0 __ZN4FMOD6SoundI11getUserDataEPPv") }

#[doc(alias = "FMOD::SoundI::syncPointFixIndicies(void)")]
// 0xddcb8 — __ZN4FMOD6SoundI20syncPointFixIndiciesEv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xddcb8() -> ! { todo!("0xddcb8 __ZN4FMOD6SoundI20syncPointFixIndiciesEv") }

#[doc(alias = "FMOD::SoundI::getMemoryInfo(unsigned int,unsigned int,unsigned int *,FMOD_MEMORY_USAGE_DETAILS *)")]
// 0xddd44 — __ZN4FMOD6SoundI13getMemoryInfoEjjPjP25FMOD_MEMORY_USAGE_DETAILS
// type: int __fastcall(int, int, int, int, void *)
pub fn stub_0xddd44() -> ! { todo!("0xddd44 __ZN4FMOD6SoundI13getMemoryInfoEjjPjP25FMOD_MEMORY_USAGE_DETAILS") }

#[doc(alias = "FMOD::SoundI::deleteSyncPointInternal(FMOD_SYNCPOINT *,bool)")]
// 0xdde0c — __ZN4FMOD6SoundI23deleteSyncPointInternalEP14FMOD_SYNCPOINTb
pub fn stub_0xdde0c() -> ! { todo!("0xdde0c __ZN4FMOD6SoundI23deleteSyncPointInternalEP14FMOD_SYNCPOINTb") }

#[doc(alias = "FMOD::SoundI::deleteSyncPoint(FMOD_SYNCPOINT *)")]
// 0xddf90 — __ZN4FMOD6SoundI15deleteSyncPointEP14FMOD_SYNCPOINT
pub fn stub_0xddf90() -> ! { todo!("0xddf90 __ZN4FMOD6SoundI15deleteSyncPointEP14FMOD_SYNCPOINT") }

#[doc(alias = "FMOD::SoundI::getTag(char const*,int,FMOD_TAG *)")]
// 0xddf98 — __ZN4FMOD6SoundI6getTagEPKciP8FMOD_TAG
pub fn stub_0xddf98() -> ! { todo!("0xddf98 __ZN4FMOD6SoundI6getTagEPKciP8FMOD_TAG") }

#[doc(alias = "FMOD::SoundI::getNumTags(int *,int *)")]
// 0xddfd0 — __ZN4FMOD6SoundI10getNumTagsEPiS1_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *, int *)
pub fn stub_0xddfd0() -> ! { todo!("0xddfd0 __ZN4FMOD6SoundI10getNumTagsEPiS1_") }

#[doc(alias = "FMOD::SoundI::getName(char *,int)")]
// 0xde028 — __ZN4FMOD6SoundI7getNameEPci
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, char *, int)
pub fn stub_0xde028() -> ! { todo!("0xde028 __ZN4FMOD6SoundI7getNameEPci") }

#[doc(alias = "FMOD::SoundI::setSubSoundInternal(int,FMOD::SoundI*,bool)")]
// 0xde0d8 — __ZN4FMOD6SoundI19setSubSoundInternalEiPS0_b
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, FMOD::SoundI *, bool)
pub fn stub_0xde0d8() -> ! { todo!("0xde0d8 __ZN4FMOD6SoundI19setSubSoundInternalEiPS0_b") }

#[doc(alias = "FMOD::SoundI::setSubSound(int,FMOD::SoundI*)")]
// 0xde618 — __ZN4FMOD6SoundI11setSubSoundEiPS0_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, FMOD::SoundI *)
pub fn stub_0xde618() -> ! { todo!("0xde618 __ZN4FMOD6SoundI11setSubSoundEiPS0_") }

#[doc(alias = "FMOD::SoundI::seek(int,unsigned int)")]
// 0xde620 — __ZN4FMOD6SoundI4seekEij
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, unsigned int)
pub fn stub_0xde620() -> ! { todo!("0xde620 __ZN4FMOD6SoundI4seekEij") }

#[doc(alias = "FMOD::SoundI::getSyncPoint(int,FMOD_SYNCPOINT **)")]
// 0xde6a0 — __ZN4FMOD6SoundI12getSyncPointEiPP14FMOD_SYNCPOINT
pub fn stub_0xde6a0() -> ! { todo!("0xde6a0 __ZN4FMOD6SoundI12getSyncPointEiPP14FMOD_SYNCPOINT") }

#[doc(alias = "FMOD::SoundI::getNumSyncPoints(int *)")]
// 0xde788 — __ZN4FMOD6SoundI16getNumSyncPointsEPi
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *)
pub fn stub_0xde788() -> ! { todo!("0xde788 __ZN4FMOD6SoundI16getNumSyncPointsEPi") }

#[doc(alias = "FMOD::SoundI::release(bool)")]
// 0xde840 — __ZN4FMOD6SoundI7releaseEb
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, bool)
pub fn stub_0xde840() -> ! { todo!("0xde840 __ZN4FMOD6SoundI7releaseEb") }
