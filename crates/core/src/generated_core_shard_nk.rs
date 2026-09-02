//! core shard nk — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet stubbed in core (lowest EA uncovered 0xdef40..0xe90ac, 42280 distinct in core before batch, 43266 uncovered, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR() -> ! { todo!("0xADDR mangled") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::SoundI::setSoundGroup(FMOD::SoundGroupI *)")]
// 0xdef40 — __ZN4FMOD6SoundI13setSoundGroupEPNS_11SoundGroupIE
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, FMOD::SoundGroupI *)
pub fn stub_0xdef40() -> ! { todo!("0xdef40 __ZN4FMOD6SoundI13setSoundGroupEPNS_11SoundGroupIE") }

#[doc(alias = "FMOD::SoundI::getOpenState(FMOD_OPENSTATE *,unsigned int *,bool *)")]
// 0xdf054 — __ZN4FMOD6SoundI12getOpenStateEP14FMOD_OPENSTATEPjPb
pub fn stub_0xdf054() -> ! { todo!("0xdf054 __ZN4FMOD6SoundI12getOpenStateEP14FMOD_OPENSTATEPjPb") }

#[doc(alias = "FMOD::SoundI::readData(void *,unsigned int,unsigned int *)")]
// 0xdf1dc — __ZN4FMOD6SoundI8readDataEPvjPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0xdf1dc() -> ! { todo!("0xdf1dc __ZN4FMOD6SoundI8readDataEPvjPj") }

#[doc(alias = "FMOD::SoundI::updateSubSound(int,bool)")]
// 0xdf524 — __ZN4FMOD6SoundI14updateSubSoundEib
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, bool)
pub fn stub_0xdf524() -> ! { todo!("0xdf524 __ZN4FMOD6SoundI14updateSubSoundEib") }

#[doc(alias = "FMOD::SoundI::getSubSound(int,FMOD::SoundI**)")]
// 0xdf760 — __ZN4FMOD6SoundI11getSubSoundEiPPS0_
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, FMOD::SoundI **)
pub fn stub_0xdf760() -> ! { todo!("0xdf760 __ZN4FMOD6SoundI11getSubSoundEiPPS0_") }

#[doc(alias = "FMOD::SoundI::setSubSoundSentence(int *,int)")]
// 0xdf974 — __ZN4FMOD6SoundI19setSubSoundSentenceEPii
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int *, int)
pub fn stub_0xdf974() -> ! { todo!("0xdf974 __ZN4FMOD6SoundI19setSubSoundSentenceEPii") }

#[doc(alias = "FMOD::SoundI::seekData(unsigned int)")]
// 0xdfeb8 — __ZN4FMOD6SoundI8seekDataEj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int)
pub fn stub_0xdfeb8() -> ! { todo!("0xdfeb8 __ZN4FMOD6SoundI8seekDataEj") }

#[doc(alias = "FMOD::SoundI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xdff14 — __ZN4FMOD6SoundI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xdff14() -> ! { todo!("0xdff14 __ZN4FMOD6SoundI17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::SoundI::getSyncPointInfo(FMOD_SYNCPOINT *,char *,int,unsigned int *,unsigned int)")]
// 0xe0204 — __ZN4FMOD6SoundI16getSyncPointInfoEP14FMOD_SYNCPOINTPciPjj
pub fn stub_0xe0204() -> ! { todo!("0xe0204 __ZN4FMOD6SoundI16getSyncPointInfoEP14FMOD_SYNCPOINTPciPjj") }

#[doc(alias = "FMOD::SoundI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")]
// 0xe0484 — __ZN4FMOD6SoundI13getLoopPointsEPjjS1_j
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *, unsigned int, unsigned int *, unsigned int)
pub fn stub_0xe0484() -> ! { todo!("0xe0484 __ZN4FMOD6SoundI13getLoopPointsEPjjS1_j") }

#[doc(alias = "FMOD::SoundI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
// 0xe0804 — __ZN4FMOD6SoundI13setLoopPointsEjjjj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
pub fn stub_0xe0804() -> ! { todo!("0xe0804 __ZN4FMOD6SoundI13setLoopPointsEjjjj") }

#[doc(alias = "FMOD::SoundI::addSyncPointInternal(unsigned int,unsigned int,char const*,FMOD_SYNCPOINT **,int,bool)")]
// 0xe0b84 — __ZN4FMOD6SoundI20addSyncPointInternalEjjPKcPP14FMOD_SYNCPOINTib
// type: int __fastcall(FMOD::SoundI *this, int, int, unsigned int, int, int, char)
pub fn stub_0xe0b84() -> ! { todo!("0xe0b84 __ZN4FMOD6SoundI20addSyncPointInternalEjjPKcPP14FMOD_SYNCPOINTib") }

#[doc(alias = "FMOD::SoundI::getLength(unsigned int *,unsigned int)")]
// 0xe12c8 — __ZN4FMOD6SoundI9getLengthEPjj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int *, unsigned int)
pub fn stub_0xe12c8() -> ! { todo!("0xe12c8 __ZN4FMOD6SoundI9getLengthEPjj") }

#[doc(alias = "FMOD::SoundI::read(unsigned int,unsigned int,unsigned int *)")]
// 0xe1510 — __ZN4FMOD6SoundI4readEjjPj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int, unsigned int *)
pub fn stub_0xe1510() -> ! { todo!("0xe1510 __ZN4FMOD6SoundI4readEjjPj") }

#[doc(alias = "FMOD::SoundI::loadSubSound(int,unsigned int)")]
// 0xe1f1c — __ZN4FMOD6SoundI12loadSubSoundEij
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, int, unsigned int)
pub fn stub_0xe1f1c() -> ! { todo!("0xe1f1c __ZN4FMOD6SoundI12loadSubSoundEij") }

#[doc(alias = "FMOD::SoundI::clear(unsigned int,unsigned int)")]
// 0xe20bc — __ZN4FMOD6SoundI5clearEjj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int)
pub fn stub_0xe20bc() -> ! { todo!("0xe20bc __ZN4FMOD6SoundI5clearEjj") }

#[doc(alias = "FMOD::SoundI::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xe24a0 — __ZN4FMOD6SoundI13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xe24a0() -> ! { todo!("0xe24a0 __ZN4FMOD6SoundI13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::SoundI::isStream(void)")]
// 0xe24f8 — __ZN4FMOD6SoundI8isStreamEv
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this)
pub fn stub_0xe24f8() -> ! { todo!("0xe24f8 __ZN4FMOD6SoundI8isStreamEv") }

#[doc(alias = "FMOD::SpeakerLevelsPool::SpeakerLevelsPool(void)")]
// 0xe2500 — __ZN4FMOD17SpeakerLevelsPoolC2Ev
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this)
pub fn stub_0xe2500() -> ! { todo!("0xe2500 __ZN4FMOD17SpeakerLevelsPoolC2Ev") }

#[doc(alias = "FMOD::SpeakerLevelsPool::SpeakerLevelsPool(void)")]
// 0xe2520 — __ZN4FMOD17SpeakerLevelsPoolC1Ev
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this)
pub fn stub_0xe2520() -> ! { todo!("0xe2520 __ZN4FMOD17SpeakerLevelsPoolC1Ev") }

#[doc(alias = "FMOD::SpeakerLevelsPool::free(float *)")]
// 0xe2524 — __ZN4FMOD17SpeakerLevelsPool4freeEPf
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this, float *)
pub fn stub_0xe2524() -> ! { todo!("0xe2524 __ZN4FMOD17SpeakerLevelsPool4freeEPf") }

#[doc(alias = "FMOD::SpeakerLevelsPool::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xe2598 — __ZN4FMOD17SpeakerLevelsPool17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xe2598() -> ! { todo!("0xe2598 __ZN4FMOD17SpeakerLevelsPool17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::SpeakerLevelsPool::release(void)")]
// 0xe2650 — __ZN4FMOD17SpeakerLevelsPool7releaseEv
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this)
pub fn stub_0xe2650() -> ! { todo!("0xe2650 __ZN4FMOD17SpeakerLevelsPool7releaseEv") }

#[doc(alias = "FMOD::SpeakerLevelsPool::alloc(float **)")]
// 0xe271c — __ZN4FMOD17SpeakerLevelsPool5allocEPPf
// type: _DWORD __fastcall(FMOD::SpeakerLevelsPool *__hidden this, float **)
pub fn stub_0xe271c() -> ! { todo!("0xe271c __ZN4FMOD17SpeakerLevelsPool5allocEPPf") }

#[doc(alias = "FMOD::SpeakerLevelsPool::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0xe28e8 — __ZN4FMOD17SpeakerLevelsPool13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xe28e8() -> ! { todo!("0xe28e8 __ZN4FMOD17SpeakerLevelsPool13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "_FMOD_strlen")]
// 0xe2940 — _FMOD_strlen
// type: int __fastcall(_DWORD)
pub fn stub_0xe2940() -> ! { todo!("0xe2940 _FMOD_strlen") }

#[doc(alias = "_FMOD_strcpy")]
// 0xe2968 — _FMOD_strcpy
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xe2968() -> ! { todo!("0xe2968 _FMOD_strcpy") }

#[doc(alias = "_FMOD_strncpy")]
// 0xe2980 — _FMOD_strncpy
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe2980() -> ! { todo!("0xe2980 _FMOD_strncpy") }

#[doc(alias = "_FMOD_strcat")]
// 0xe29b0 — _FMOD_strcat
pub fn stub_0xe29b0() -> ! { todo!("0xe29b0 _FMOD_strcat") }

#[doc(alias = "_FMOD_strncat")]
// 0xe29e4 — _FMOD_strncat
pub fn stub_0xe29e4() -> ! { todo!("0xe29e4 _FMOD_strncat") }

#[doc(alias = "_FMOD_tolower")]
// 0xe2a34 — _FMOD_tolower
pub fn stub_0xe2a34() -> ! { todo!("0xe2a34 _FMOD_tolower") }

#[doc(alias = "_FMOD_strupr")]
// 0xe2a58 — _FMOD_strupr
// type: int __fastcall(_DWORD)
pub fn stub_0xe2a58() -> ! { todo!("0xe2a58 _FMOD_strupr") }

#[doc(alias = "_FMOD_strcmp")]
// 0xe2a90 — _FMOD_strcmp
pub fn stub_0xe2a90() -> ! { todo!("0xe2a90 _FMOD_strcmp") }

#[doc(alias = "_FMOD_strncmp")]
// 0xe2abc — _FMOD_strncmp
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe2abc() -> ! { todo!("0xe2abc _FMOD_strncmp") }

#[doc(alias = "_FMOD_stricmp")]
// 0xe2b00 — _FMOD_stricmp
pub fn stub_0xe2b00() -> ! { todo!("0xe2b00 _FMOD_stricmp") }

#[doc(alias = "_FMOD_strnicmp")]
// 0xe2b4c — _FMOD_strnicmp
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe2b4c() -> ! { todo!("0xe2b4c _FMOD_strnicmp") }

#[doc(alias = "_FMOD_memcmp")]
// 0xe2bb4 — _FMOD_memcmp
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe2bb4() -> ! { todo!("0xe2bb4 _FMOD_memcmp") }

#[doc(alias = "_FMOD_strstr")]
// 0xe2c18 — _FMOD_strstr
pub fn stub_0xe2c18() -> ! { todo!("0xe2c18 _FMOD_strstr") }

#[doc(alias = "_FMOD_memmove")]
// 0xe2c94 — _FMOD_memmove
pub fn stub_0xe2c94() -> ! { todo!("0xe2c94 _FMOD_memmove") }

#[doc(alias = "FMOD::System::release(void)")]
// 0xe3380 — __ZN4FMOD6System7releaseEv
// type: _DWORD __fastcall(FMOD::System *__hidden this)
pub fn stub_0xe3380() -> ! { todo!("0xe3380 __ZN4FMOD6System7releaseEv") }

#[doc(alias = "FMOD::SystemI::validate(FMOD::System *,FMOD::SystemI**)")]
// 0xe33ac — __ZN4FMOD7SystemI8validateEPNS_6SystemEPPS0_
pub fn stub_0xe33ac() -> ! { todo!("0xe33ac __ZN4FMOD7SystemI8validateEPNS_6SystemEPPS0_") }

#[doc(alias = "FMOD::SystemI::getDriver(int *)")]
// 0xe341c — __ZN4FMOD7SystemI9getDriverEPi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int *)
pub fn stub_0xe341c() -> ! { todo!("0xe341c __ZN4FMOD7SystemI9getDriverEPi") }

#[doc(alias = "FMOD::SystemI::setDSPBufferSize(unsigned int,int)")]
// 0xe3434 — __ZN4FMOD7SystemI16setDSPBufferSizeEji
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, unsigned int, int)
pub fn stub_0xe3434() -> ! { todo!("0xe3434 __ZN4FMOD7SystemI16setDSPBufferSizeEji") }

#[doc(alias = "FMOD::SystemI::getDSPBufferSize(unsigned int *,int *)")]
// 0xe346c — __ZN4FMOD7SystemI16getDSPBufferSizeEPjPi
// type: int __fastcall(FMOD::SystemI *this, unsigned int *, int *)
pub fn stub_0xe346c() -> ! { todo!("0xe346c __ZN4FMOD7SystemI16getDSPBufferSizeEPjPi") }

#[doc(alias = "FMOD::SystemI::set3DSettings(float,float,float)")]
// 0xe34a8 — __ZN4FMOD7SystemI13set3DSettingsEfff
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, float, float, float)
pub fn stub_0xe34a8() -> ! { todo!("0xe34a8 __ZN4FMOD7SystemI13set3DSettingsEfff") }

#[doc(alias = "FMOD::SystemI::set3DListenerAttributes(int,FMOD_VECTOR const*,FMOD_VECTOR const*,FMOD_VECTOR const*,FMOD_VECTOR const*)")]
// 0xe3508 — __ZN4FMOD7SystemI23set3DListenerAttributesEiPK11FMOD_VECTORS3_S3_S3_
pub fn stub_0xe3508() -> ! { todo!("0xe3508 __ZN4FMOD7SystemI23set3DListenerAttributesEiPK11FMOD_VECTORS3_S3_S3_") }

#[doc(alias = "FMOD::SystemI::get3DNumListeners(int *)")]
// 0xe3880 — __ZN4FMOD7SystemI17get3DNumListenersEPi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int *)
pub fn stub_0xe3880() -> ! { todo!("0xe3880 __ZN4FMOD7SystemI17get3DNumListenersEPi") }

#[doc(alias = "FMOD::SystemI::get3DSettings(float *,float *,float *)")]
// 0xe38a0 — __ZN4FMOD7SystemI13get3DSettingsEPfS1_S1_
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, float *, float *, float *)
pub fn stub_0xe38a0() -> ! { todo!("0xe38a0 __ZN4FMOD7SystemI13get3DSettingsEPfS1_S1_") }

#[doc(alias = "FMOD::SystemI::getVersion(unsigned int *)")]
// 0xe38e0 — __ZN4FMOD7SystemI10getVersionEPj
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, unsigned int *)
pub fn stub_0xe38e0() -> ! { todo!("0xe38e0 __ZN4FMOD7SystemI10getVersionEPj") }

#[doc(alias = "FMOD::SystemI::getChannelsPlaying(int *)")]
// 0xe38fc — __ZN4FMOD7SystemI18getChannelsPlayingEPi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int *)
pub fn stub_0xe38fc() -> ! { todo!("0xe38fc __ZN4FMOD7SystemI18getChannelsPlayingEPi") }

#[doc(alias = "FMOD::SystemI::getMasterChannelGroup(FMOD::ChannelGroupI **)")]
// 0xe393c — __ZN4FMOD7SystemI21getMasterChannelGroupEPPNS_13ChannelGroupIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, FMOD::ChannelGroupI **)
pub fn stub_0xe393c() -> ! { todo!("0xe393c __ZN4FMOD7SystemI21getMasterChannelGroupEPPNS_13ChannelGroupIE") }

#[doc(alias = "FMOD::SystemI::set3DReverbActive(bool)")]
// 0xe396c — __ZN4FMOD7SystemI17set3DReverbActiveEb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, bool)
pub fn stub_0xe396c() -> ! { todo!("0xe396c __ZN4FMOD7SystemI17set3DReverbActiveEb") }

#[doc(alias = "FMOD::SystemI::get3DReverbActive(bool *)")]
// 0xe397c — __ZN4FMOD7SystemI17get3DReverbActiveEPb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, bool *)
pub fn stub_0xe397c() -> ! { todo!("0xe397c __ZN4FMOD7SystemI17get3DReverbActiveEPb") }

#[doc(alias = "FMOD::SystemI::getDSPHead(FMOD::DSPI **)")]
// 0xe3998 — __ZN4FMOD7SystemI10getDSPHeadEPPNS_4DSPIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, FMOD::DSPI **)
pub fn stub_0xe3998() -> ! { todo!("0xe3998 __ZN4FMOD7SystemI10getDSPHeadEPPNS_4DSPIE") }

#[doc(alias = "FMOD::SystemI::getListenerObject(int,FMOD::Listener **)")]
// 0xe39c4 — __ZN4FMOD7SystemI17getListenerObjectEiPPNS_8ListenerE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int, FMOD::Listener **)
pub fn stub_0xe39c4() -> ! { todo!("0xe39c4 __ZN4FMOD7SystemI17getListenerObjectEiPPNS_8ListenerE") }

#[doc(alias = "FMOD::SystemI::getReverbAmbientProperties(FMOD_REVERB_PROPERTIES *)")]
// 0xe3a20 — __ZN4FMOD7SystemI26getReverbAmbientPropertiesEP22FMOD_REVERB_PROPERTIES
pub fn stub_0xe3a20() -> ! { todo!("0xe3a20 __ZN4FMOD7SystemI26getReverbAmbientPropertiesEP22FMOD_REVERB_PROPERTIES") }

#[doc(alias = "FMOD::SystemI::setReverbAmbientProperties(FMOD_REVERB_PROPERTIES *)")]
// 0xe3a50 — __ZN4FMOD7SystemI26setReverbAmbientPropertiesEP22FMOD_REVERB_PROPERTIES
pub fn stub_0xe3a50() -> ! { todo!("0xe3a50 __ZN4FMOD7SystemI26setReverbAmbientPropertiesEP22FMOD_REVERB_PROPERTIES") }

#[doc(alias = "FMOD::SystemI::lockDSP(void)")]
// 0xe3a98 — __ZN4FMOD7SystemI7lockDSPEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe3a98() -> ! { todo!("0xe3a98 __ZN4FMOD7SystemI7lockDSPEv") }

#[doc(alias = "FMOD::SystemI::unlockDSP(void)")]
// 0xe3ab4 — __ZN4FMOD7SystemI9unlockDSPEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe3ab4() -> ! { todo!("0xe3ab4 __ZN4FMOD7SystemI9unlockDSPEv") }

#[doc(alias = "FMOD::SystemI::stopSound(FMOD::SoundI *)")]
// 0xe3ad0 — __ZN4FMOD7SystemI9stopSoundEPNS_6SoundIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, FMOD::SoundI *)
pub fn stub_0xe3ad0() -> ! { todo!("0xe3ad0 __ZN4FMOD7SystemI9stopSoundEPNS_6SoundIE") }

#[doc(alias = "FMOD::SystemI::set3DReverbProperties(FMOD_REVERB_PROPERTIES const*,bool)")]
// 0xe3e30 — __ZN4FMOD7SystemI21set3DReverbPropertiesEPK22FMOD_REVERB_PROPERTIESb
pub fn stub_0xe3e30() -> ! { todo!("0xe3e30 __ZN4FMOD7SystemI21set3DReverbPropertiesEPK22FMOD_REVERB_PROPERTIESb") }

#[doc(alias = "FMOD::SystemI::update3DReverbs(void)")]
// 0xe3f78 — __ZN4FMOD7SystemI15update3DReverbsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe3f78() -> ! { todo!("0xe3f78 __ZN4FMOD7SystemI15update3DReverbsEv") }

#[doc(alias = "FMOD::SystemI::setReverbProperties(FMOD_REVERB_PROPERTIES const*,bool)")]
// 0xe41c4 — __ZN4FMOD7SystemI19setReverbPropertiesEPK22FMOD_REVERB_PROPERTIESb
pub fn stub_0xe41c4() -> ! { todo!("0xe41c4 __ZN4FMOD7SystemI19setReverbPropertiesEPK22FMOD_REVERB_PROPERTIESb") }

#[doc(alias = "FMOD::SystemI::createSoundGroup(char const*,FMOD::SoundGroupI **)")]
// 0xe438c — __ZN4FMOD7SystemI16createSoundGroupEPKcPPNS_11SoundGroupIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, const char *, FMOD::SoundGroupI **)
pub fn stub_0xe438c() -> ! { todo!("0xe438c __ZN4FMOD7SystemI16createSoundGroupEPKcPPNS_11SoundGroupIE") }

#[doc(alias = "FMOD::SystemI::createDSP(FMOD::FMOD_DSP_DESCRIPTION_EX *,FMOD::DSPI **,bool)")]
// 0xe44c0 — __ZN4FMOD7SystemI9createDSPEPNS_23FMOD_DSP_DESCRIPTION_EXEPPNS_4DSPIEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xe44c0() -> ! { todo!("0xe44c0 __ZN4FMOD7SystemI9createDSPEPNS_23FMOD_DSP_DESCRIPTION_EXEPPNS_4DSPIEb") }

#[doc(alias = "FMOD::SystemI::createChannelGroupInternal(char const*,FMOD::ChannelGroupI **,bool,bool)")]
// 0xe45fc — __ZN4FMOD7SystemI26createChannelGroupInternalEPKcPPNS_13ChannelGroupIEbb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, const char *, FMOD::ChannelGroupI **, bool, bool)
pub fn stub_0xe45fc() -> ! { todo!("0xe45fc __ZN4FMOD7SystemI26createChannelGroupInternalEPKcPPNS_13ChannelGroupIEbb") }

#[doc(alias = "FMOD::SystemI::createChannelGroup(char const*,FMOD::ChannelGroupI **)")]
// 0xe499c — __ZN4FMOD7SystemI18createChannelGroupEPKcPPNS_13ChannelGroupIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, const char *, FMOD::ChannelGroupI **)
pub fn stub_0xe499c() -> ! { todo!("0xe499c __ZN4FMOD7SystemI18createChannelGroupEPKcPPNS_13ChannelGroupIE") }

#[doc(alias = "FMOD::SystemI::createDSP(FMOD_DSP_DESCRIPTION *,FMOD::DSPI **)")]
// 0xe49ec — __ZN4FMOD7SystemI9createDSPEP20FMOD_DSP_DESCRIPTIONPPNS_4DSPIE
pub fn stub_0xe49ec() -> ! { todo!("0xe49ec __ZN4FMOD7SystemI9createDSPEP20FMOD_DSP_DESCRIPTIONPPNS_4DSPIE") }

#[doc(alias = "FMOD::SystemI::createDSPByType(FMOD_DSP_TYPE,FMOD::DSPI **)")]
// 0xe4b18 — __ZN4FMOD7SystemI15createDSPByTypeE13FMOD_DSP_TYPEPPNS_4DSPIE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe4b18() -> ! { todo!("0xe4b18 __ZN4FMOD7SystemI15createDSPByTypeE13FMOD_DSP_TYPEPPNS_4DSPIE") }

#[doc(alias = "FMOD::SystemI::updateStreams(void)")]
// 0xe4c48 — __ZN4FMOD7SystemI13updateStreamsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe4c48() -> ! { todo!("0xe4c48 __ZN4FMOD7SystemI13updateStreamsEv") }

#[doc(alias = "FMOD::SystemI::streamThread(void *)")]
// 0xe4dbc — __ZN4FMOD7SystemI12streamThreadEPv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, void *)
pub fn stub_0xe4dbc() -> ! { todo!("0xe4dbc __ZN4FMOD7SystemI12streamThreadEPv") }

#[doc(alias = "FMOD::SystemI::getCPUUsage(float *,float *,float *,float *,float *)")]
// 0xe4dc0 — __ZN4FMOD7SystemI11getCPUUsageEPfS1_S1_S1_S1_
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, float *, float *, float *, float *, float *)
pub fn stub_0xe4dc0() -> ! { todo!("0xe4dc0 __ZN4FMOD7SystemI11getCPUUsageEPfS1_S1_S1_S1_") }

#[doc(alias = "FMOD::SystemI::allocateDSPCodec(FMOD_SOUND_FORMAT,FMOD::DSPCodec **)")]
// 0xe4eb8 — __ZN4FMOD7SystemI16allocateDSPCodecE17FMOD_SOUND_FORMATPPNS_8DSPCodecE
// type: int __fastcall(int, int, FMOD::DSPCodec **)
pub fn stub_0xe4eb8() -> ! { todo!("0xe4eb8 __ZN4FMOD7SystemI16allocateDSPCodecE17FMOD_SOUND_FORMATPPNS_8DSPCodecE") }

#[doc(alias = "FMOD::SystemI::setUpPlugins(void)")]
// 0xe4efc — __ZN4FMOD7SystemI12setUpPluginsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe4efc() -> ! { todo!("0xe4efc __ZN4FMOD7SystemI12setUpPluginsEv") }

#[doc(alias = "FMOD::SystemI::setOutput(FMOD_OUTPUTTYPE)")]
// 0xe553c — __ZN4FMOD7SystemI9setOutputE15FMOD_OUTPUTTYPE
// type: int __fastcall(FMOD::SystemI *this)
pub fn stub_0xe553c() -> ! { todo!("0xe553c __ZN4FMOD7SystemI9setOutputE15FMOD_OUTPUTTYPE") }

#[doc(alias = "FMOD::SystemI::getHardwareChannels(int *,int *,int *)")]
// 0xe5684 — __ZN4FMOD7SystemI19getHardwareChannelsEPiS1_S1_
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int *, int *, int *)
pub fn stub_0xe5684() -> ! { todo!("0xe5684 __ZN4FMOD7SystemI19getHardwareChannelsEPiS1_S1_") }

#[doc(alias = "FMOD::SystemI::checkDriverList(bool)")]
// 0xe5774 — __ZN4FMOD7SystemI15checkDriverListEb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, bool)
pub fn stub_0xe5774() -> ! { todo!("0xe5774 __ZN4FMOD7SystemI15checkDriverListEb") }

#[doc(alias = "FMOD::SystemI::getNumDrivers(int *)")]
// 0xe5884 — __ZN4FMOD7SystemI13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int *)
pub fn stub_0xe5884() -> ! { todo!("0xe5884 __ZN4FMOD7SystemI13getNumDriversEPi") }

#[doc(alias = "FMOD::SystemI::setDriver(int)")]
// 0xe5910 — __ZN4FMOD7SystemI9setDriverEi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int)
pub fn stub_0xe5910() -> ! { todo!("0xe5910 __ZN4FMOD7SystemI9setDriverEi") }

#[doc(alias = "FMOD::SystemI::getDriverCaps(int,unsigned int *,int *,int *,FMOD_SPEAKERMODE *)")]
// 0xe5c60 — __ZN4FMOD7SystemI13getDriverCapsEiPjPiS2_P16FMOD_SPEAKERMODE
pub fn stub_0xe5c60() -> ! { todo!("0xe5c60 __ZN4FMOD7SystemI13getDriverCapsEiPjPiS2_P16FMOD_SPEAKERMODE") }

#[doc(alias = "FMOD::SystemI::getDriverInfo(int,char *,int,FMOD_GUID *)")]
// 0xe5e80 — __ZN4FMOD7SystemI13getDriverInfoEiPciP9FMOD_GUID
pub fn stub_0xe5e80() -> ! { todo!("0xe5e80 __ZN4FMOD7SystemI13getDriverInfoEiPciP9FMOD_GUID") }

#[doc(alias = "FMOD::SystemI::createSample(unsigned int,FMOD_CODEC_WAVEFORMAT *,FMOD::Sample **)")]
// 0xe5f98 — __ZN4FMOD7SystemI12createSampleEjP21FMOD_CODEC_WAVEFORMATPPNS_6SampleE
pub fn stub_0xe5f98() -> ! { todo!("0xe5f98 __ZN4FMOD7SystemI12createSampleEjP21FMOD_CODEC_WAVEFORMATPPNS_6SampleE") }

#[doc(alias = "FMOD::SystemI::findChannel(FMOD_CHANNELINDEX,FMOD::SoundI *,FMOD::ChannelI **)")]
// 0xe6594 — __ZN4FMOD7SystemI11findChannelE17FMOD_CHANNELINDEXPNS_6SoundIEPPNS_8ChannelIE
pub fn stub_0xe6594() -> ! { todo!("0xe6594 __ZN4FMOD7SystemI11findChannelE17FMOD_CHANNELINDEXPNS_6SoundIEPPNS_8ChannelIE") }

#[doc(alias = "FMOD::SystemI::updateSoundGroups(int)")]
// 0xe69e0 — __ZN4FMOD7SystemI17updateSoundGroupsEi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int)
pub fn stub_0xe69e0() -> ! { todo!("0xe69e0 __ZN4FMOD7SystemI17updateSoundGroupsEi") }

#[doc(alias = "FMOD::SystemI::updateChannels(int)")]
// 0xe6be0 — __ZN4FMOD7SystemI14updateChannelsEi
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int)
pub fn stub_0xe6be0() -> ! { todo!("0xe6be0 __ZN4FMOD7SystemI14updateChannelsEi") }

#[doc(alias = "FMOD::SystemI::update(void)")]
// 0xe7628 — __ZN4FMOD7SystemI6updateEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe7628() -> ! { todo!("0xe7628 __ZN4FMOD7SystemI6updateEv") }

#[doc(alias = "FMOD::SystemI::closeEx(bool)")]
// 0xe78c8 — __ZN4FMOD7SystemI7closeExEb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, bool)
pub fn stub_0xe78c8() -> ! { todo!("0xe78c8 __ZN4FMOD7SystemI7closeExEb") }

#[doc(alias = "FMOD::SystemI::close(void)")]
// 0xe7ed0 — __ZN4FMOD7SystemI5closeEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe7ed0() -> ! { todo!("0xe7ed0 __ZN4FMOD7SystemI5closeEv") }

#[doc(alias = "FMOD::SystemI::getInstance(unsigned int,FMOD::SystemI**)")]
// 0xe7ed8 — __ZN4FMOD7SystemI11getInstanceEjPPS0_
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, unsigned int, FMOD::SystemI **)
pub fn stub_0xe7ed8() -> ! { todo!("0xe7ed8 __ZN4FMOD7SystemI11getInstanceEjPPS0_") }

#[doc(alias = "FMOD::SystemI::stopDSP(FMOD::DSPI *)")]
// 0xe7f50 — __ZN4FMOD7SystemI7stopDSPEPNS_4DSPIE
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, FMOD::DSPI *)
pub fn stub_0xe7f50() -> ! { todo!("0xe7f50 __ZN4FMOD7SystemI7stopDSPEPNS_4DSPIE") }

#[doc(alias = "FMOD::SystemI::count3DPhysicalReverbs(void)")]
// 0xe7fdc — __ZN4FMOD7SystemI22count3DPhysicalReverbsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe7fdc() -> ! { todo!("0xe7fdc __ZN4FMOD7SystemI22count3DPhysicalReverbsEv") }

#[doc(alias = "FMOD::SystemI::playSound(FMOD_CHANNELINDEX,FMOD::SoundI *,bool,FMOD::ChannelI **)")]
// 0xe8024 — __ZN4FMOD7SystemI9playSoundE17FMOD_CHANNELINDEXPNS_6SoundIEbPPNS_8ChannelIE
pub fn stub_0xe8024() -> ! { todo!("0xe8024 __ZN4FMOD7SystemI9playSoundE17FMOD_CHANNELINDEXPNS_6SoundIEbPPNS_8ChannelIE") }

#[doc(alias = "FMOD::SystemI::flushDSPConnectionRequests(bool)")]
// 0xe82a0 — __ZN4FMOD7SystemI26flushDSPConnectionRequestsEb
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, bool)
pub fn stub_0xe82a0() -> ! { todo!("0xe82a0 __ZN4FMOD7SystemI26flushDSPConnectionRequestsEb") }

#[doc(alias = "FMOD::SystemI::count3DVirtualReverbs(void)")]
// 0xe853c — __ZN4FMOD7SystemI21count3DVirtualReverbsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe853c() -> ! { todo!("0xe853c __ZN4FMOD7SystemI21count3DVirtualReverbsEv") }

#[doc(alias = "FMOD::SystemI::release(void)")]
// 0xe8584 — __ZN4FMOD7SystemI7releaseEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe8584() -> ! { todo!("0xe8584 __ZN4FMOD7SystemI7releaseEv") }

#[doc(alias = "FMOD::SystemI::prepareSpeakerPairs(void)")]
// 0xe8634 — __ZN4FMOD7SystemI19prepareSpeakerPairsEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe8634() -> ! { todo!("0xe8634 __ZN4FMOD7SystemI19prepareSpeakerPairsEv") }

#[doc(alias = "FMOD::SystemI::sortSpeakerList(void)")]
// 0xe8b14 — __ZN4FMOD7SystemI15sortSpeakerListEv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe8b14() -> ! { todo!("0xe8b14 __ZN4FMOD7SystemI15sortSpeakerListEv") }

#[doc(alias = "FMOD::SystemI::set3DSpeakerPosition(FMOD_SPEAKER,float,float,bool)")]
// 0xe8c80 — __ZN4FMOD7SystemI20set3DSpeakerPositionE12FMOD_SPEAKERffb
pub fn stub_0xe8c80() -> ! { todo!("0xe8c80 __ZN4FMOD7SystemI20set3DSpeakerPositionE12FMOD_SPEAKERffb") }

#[doc(alias = "FMOD::SystemI::setSpeakerMode(FMOD_SPEAKERMODE)")]
// 0xe8d90 — __ZN4FMOD7SystemI14setSpeakerModeE16FMOD_SPEAKERMODE
// type: int __fastcall(FMOD::SystemI *this)
pub fn stub_0xe8d90() -> ! { todo!("0xe8d90 __ZN4FMOD7SystemI14setSpeakerModeE16FMOD_SPEAKERMODE") }

#[doc(alias = "FMOD::SystemI::init(int,unsigned int,void *)")]
// 0xe90ac — __ZN4FMOD7SystemI4initEijPv
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this, int, unsigned int, void *)
pub fn stub_0xe90ac() -> ! { todo!("0xe90ac __ZN4FMOD7SystemI4initEijPv") }
