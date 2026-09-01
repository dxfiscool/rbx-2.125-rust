//! core shard lm — 150 core stubs EA-sorted, next uncovered fallback after shard ll (0xccf7c..0xd3438, lowest EA first).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 9082->8932 uncovered, 38149->38299 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::Memory_DefaultRealloc(void *,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD21Memory_DefaultReallocEPvjj")]
// 0xccf7c — __ZN4FMOD21Memory_DefaultReallocEPvjj
// type: _DWORD __fastcall(FMOD *__hidden this, void *, unsigned int, unsigned int)
pub fn stub_0xccf7c() -> ! {
    todo!("0xccf7c __ZN4FMOD21Memory_DefaultReallocEPvjj")
}

#[doc(alias = "FMOD::Memory_DefaultMalloc(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD20Memory_DefaultMallocEjj")]
// 0xccf8c — __ZN4FMOD20Memory_DefaultMallocEjj
// type: _DWORD __fastcall(FMOD *__hidden this, unsigned int, unsigned int)
pub fn stub_0xccf8c() -> ! {
    todo!("0xccf8c __ZN4FMOD20Memory_DefaultMallocEjj")
}

#[doc(alias = "FMOD::MemPool::set(int,int,int)")]
#[doc(alias = "__ZN4FMOD7MemPool3setEiii")]
// 0xccf9c — __ZN4FMOD7MemPool3setEiii
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this, int, int, int)
pub fn stub_0xccf9c() -> ! {
    todo!("0xccf9c __ZN4FMOD7MemPool3setEiii")
}

#[doc(alias = "FMOD::MemPool::free(void *,char const*,int)")]
#[doc(alias = "__ZN4FMOD7MemPool4freeEPvPKci")]
// 0xcd240 — __ZN4FMOD7MemPool4freeEPvPKci
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this, void *, const char *, int)
pub fn stub_0xcd240() -> ! {
    todo!("0xcd240 __ZN4FMOD7MemPool4freeEPvPKci")
}

#[doc(alias = "FMOD::MemPool::close(void)")]
#[doc(alias = "__ZN4FMOD7MemPool5closeEv")]
// 0xcd394 — __ZN4FMOD7MemPool5closeEv
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this)
pub fn stub_0xcd394() -> ! {
    todo!("0xcd394 __ZN4FMOD7MemPool5closeEv")
}

#[doc(alias = "FMOD::MemPool::~MemPool()")]
#[doc(alias = "__ZN4FMOD7MemPoolD2Ev")]
// 0xcd454 — __ZN4FMOD7MemPoolD2Ev
// type: void __fastcall(FMOD::MemPool *__hidden this)
pub fn stub_0xcd454() -> ! {
    todo!("0xcd454 __ZN4FMOD7MemPoolD2Ev")
}

#[doc(alias = "FMOD::MemPool::~MemPool()")]
#[doc(alias = "__ZN4FMOD7MemPoolD1Ev")]
// 0xcd458 — __ZN4FMOD7MemPoolD1Ev
// type: void __fastcall(FMOD::MemPool *__hidden this)
pub fn stub_0xcd458() -> ! {
    todo!("0xcd458 __ZN4FMOD7MemPoolD1Ev")
}

#[doc(alias = "FMOD::MemSingleton::free(char const*,int)")]
#[doc(alias = "__ZN4FMOD12MemSingleton4freeEPKci")]
// 0xcd45c — __ZN4FMOD12MemSingleton4freeEPKci
// type: _DWORD __fastcall(FMOD::MemSingleton *__hidden this, const char *, int)
pub fn stub_0xcd45c() -> ! {
    todo!("0xcd45c __ZN4FMOD12MemSingleton4freeEPKci")
}

#[doc(alias = "FMOD::MemPool::alloc(int,char const*,int,unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD7MemPool5allocEiPKcijb")]
// 0xcd4c0 — __ZN4FMOD7MemPool5allocEiPKcijb
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this, int, const char *, int, unsigned int, bool)
pub fn stub_0xcd4c0() -> ! {
    todo!("0xcd4c0 __ZN4FMOD7MemPool5allocEiPKcijb")
}

#[doc(alias = "FMOD::MemPool::calloc(int,char const*,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD7MemPool6callocEiPKcij")]
// 0xcd800 — __ZN4FMOD7MemPool6callocEiPKcij
// type: int __fastcall(FMOD::MemPool *this, int, const char *, int, unsigned int)
pub fn stub_0xcd800() -> ! {
    todo!("0xcd800 __ZN4FMOD7MemPool6callocEiPKcij")
}

#[doc(alias = "FMOD::MemSingleton::alloc(int,char const*,int)")]
#[doc(alias = "__ZN4FMOD12MemSingleton5allocEiPKci")]
// 0xcd828 — __ZN4FMOD12MemSingleton5allocEiPKci
// type: _DWORD __fastcall(FMOD::MemSingleton *__hidden this, int, const char *, int)
pub fn stub_0xcd828() -> ! {
    todo!("0xcd828 __ZN4FMOD12MemSingleton5allocEiPKci")
}

#[doc(alias = "FMOD::MemPool::realloc(void *,int,char const*,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD7MemPool7reallocEPviPKcij")]
// 0xcd880 — __ZN4FMOD7MemPool7reallocEPviPKcij
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this, void *, int, const char *, int, unsigned int)
pub fn stub_0xcd880() -> ! {
    todo!("0xcd880 __ZN4FMOD7MemPool7reallocEPviPKcij")
}

#[doc(alias = "FMOD::Metadata::addTag(FMOD::TagNode *)")]
#[doc(alias = "__ZN4FMOD8Metadata6addTagEPNS_7TagNodeE")]
// 0xcdcc4 — __ZN4FMOD8Metadata6addTagEPNS_7TagNodeE
pub fn stub_0xcdcc4() -> ! {
    todo!("0xcdcc4 __ZN4FMOD8Metadata6addTagEPNS_7TagNodeE")
}

#[doc(alias = "FMOD::TagNode::release(void)")]
#[doc(alias = "__ZN4FMOD7TagNode7releaseEv")]
// 0xcdce4 — __ZN4FMOD7TagNode7releaseEv
// type: _DWORD __fastcall(FMOD::TagNode *__hidden this)
pub fn stub_0xcdce4() -> ! {
    todo!("0xcdce4 __ZN4FMOD7TagNode7releaseEv")
}

#[doc(alias = "FMOD::TagNode::update(void *,unsigned int)")]
#[doc(alias = "__ZN4FMOD7TagNode6updateEPvj")]
// 0xcdd9c — __ZN4FMOD7TagNode6updateEPvj
// type: _DWORD __fastcall(FMOD::TagNode *__hidden this, void *, unsigned int)
pub fn stub_0xcdd9c() -> ! {
    todo!("0xcdd9c __ZN4FMOD7TagNode6updateEPvj")
}

#[doc(alias = "FMOD::TagNode::init(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE)")]
#[doc(alias = "__ZN4FMOD7TagNode4initE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPE")]
// 0xcde94 — __ZN4FMOD7TagNode4initE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPE
// type: int __fastcall(int, int, int, int, size_t __n, int)
pub fn stub_0xcde94() -> ! {
    todo!("0xcde94 __ZN4FMOD7TagNode4initE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPE")
}

#[doc(alias = "FMOD::Metadata::release(void)")]
#[doc(alias = "__ZN4FMOD8Metadata7releaseEv")]
// 0xcdf64 — __ZN4FMOD8Metadata7releaseEv
// type: _DWORD __fastcall(FMOD::Metadata *__hidden this)
pub fn stub_0xcdf64() -> ! {
    todo!("0xcdf64 __ZN4FMOD8Metadata7releaseEv")
}

#[doc(alias = "FMOD::Metadata::addTag(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE,bool)")]
#[doc(alias = "__ZN4FMOD8Metadata6addTagE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb")]
// 0xcdfe0 — __ZN4FMOD8Metadata6addTagE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb
// type: int __fastcall(int, int, int, int, size_t, int, char)
pub fn stub_0xcdfe0() -> ! {
    todo!("0xcdfe0 __ZN4FMOD8Metadata6addTagE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb")
}

#[doc(alias = "FMOD::Metadata::getTag(char const*,int,FMOD_TAG *)")]
#[doc(alias = "__ZN4FMOD8Metadata6getTagEPKciP8FMOD_TAG")]
// 0xce128 — __ZN4FMOD8Metadata6getTagEPKciP8FMOD_TAG
pub fn stub_0xce128() -> ! {
    todo!("0xce128 __ZN4FMOD8Metadata6getTagEPKciP8FMOD_TAG")
}

#[doc(alias = "FMOD::Metadata::getNumTags(int *,int *)")]
#[doc(alias = "__ZN4FMOD8Metadata10getNumTagsEPiS1_")]
// 0xce298 — __ZN4FMOD8Metadata10getNumTagsEPiS1_
// type: _DWORD __fastcall(FMOD::Metadata *__hidden this, int *, int *)
pub fn stub_0xce298() -> ! {
    todo!("0xce298 __ZN4FMOD8Metadata10getNumTagsEPiS1_")
}

#[doc(alias = "FMOD::Metadata::add(FMOD::Metadata*)")]
#[doc(alias = "__ZN4FMOD8Metadata3addEPS0_")]
// 0xce2f0 — __ZN4FMOD8Metadata3addEPS0_
// type: _DWORD __fastcall(FMOD::Metadata *__hidden this, FMOD::Metadata *)
pub fn stub_0xce2f0() -> ! {
    todo!("0xce2f0 __ZN4FMOD8Metadata3addEPS0_")
}

#[doc(alias = "FMOD::MusicSong::spawnNewVirtualChannel(FMOD::MusicChannel *,FMOD::MusicSample *,FMOD::MusicVirtualChannel **)")]
#[doc(alias = "__ZN4FMOD9MusicSong22spawnNewVirtualChannelEPNS_12MusicChannelEPNS_11MusicSampleEPPNS_19MusicVirtualChannelE")]
// 0xce3b4 — __ZN4FMOD9MusicSong22spawnNewVirtualChannelEPNS_12MusicChannelEPNS_11MusicSampleEPPNS_19MusicVirtualChannelE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xce3b4() -> ! {
    todo!("0xce3b4 __ZN4FMOD9MusicSong22spawnNewVirtualChannelEPNS_12MusicChannelEPNS_11MusicSampleEPPNS_19MusicVirtualChannelE")
}

#[doc(alias = "FMOD::MusicSong::setBPM(int)")]
#[doc(alias = "__ZN4FMOD9MusicSong6setBPMEi")]
// 0xce4ac — __ZN4FMOD9MusicSong6setBPMEi
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, int)
pub fn stub_0xce4ac() -> ! {
    todo!("0xce4ac __ZN4FMOD9MusicSong6setBPMEi")
}

#[doc(alias = "FMOD::MusicSong::fineTune2Hz(unsigned char,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9MusicSong11fineTune2HzEhPj")]
// 0xce524 — __ZN4FMOD9MusicSong11fineTune2HzEhPj
// type: int __fastcall(FMOD::MusicSong *this, unsigned __int8, unsigned int *)
pub fn stub_0xce524() -> ! {
    todo!("0xce524 __ZN4FMOD9MusicSong11fineTune2HzEhPj")
}

#[doc(alias = "FMOD::MusicSong::getLengthInternal(unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD9MusicSong17getLengthInternalEPjj")]
// 0xce690 — __ZN4FMOD9MusicSong17getLengthInternalEPjj
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, unsigned int *, unsigned int)
pub fn stub_0xce690() -> ! {
    todo!("0xce690 __ZN4FMOD9MusicSong17getLengthInternalEPjj")
}

#[doc(alias = "FMOD::MusicSong::getPositionInternal(unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD9MusicSong19getPositionInternalEPjj")]
// 0xce6e0 — __ZN4FMOD9MusicSong19getPositionInternalEPjj
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, unsigned int *, unsigned int)
pub fn stub_0xce6e0() -> ! {
    todo!("0xce6e0 __ZN4FMOD9MusicSong19getPositionInternalEPjj")
}

#[doc(alias = "FMOD::MusicSong::getMusicNumChannelsInternal(int *)")]
#[doc(alias = "__ZN4FMOD9MusicSong27getMusicNumChannelsInternalEPi")]
// 0xce724 — __ZN4FMOD9MusicSong27getMusicNumChannelsInternalEPi
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, int *)
pub fn stub_0xce724() -> ! {
    todo!("0xce724 __ZN4FMOD9MusicSong27getMusicNumChannelsInternalEPi")
}

#[doc(alias = "FMOD::MusicSong::setMusicChannelVolumeInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9MusicSong29setMusicChannelVolumeInternalEif")]
// 0xce73c — __ZN4FMOD9MusicSong29setMusicChannelVolumeInternalEif
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, int, float)
pub fn stub_0xce73c() -> ! {
    todo!("0xce73c __ZN4FMOD9MusicSong29setMusicChannelVolumeInternalEif")
}

#[doc(alias = "FMOD::MusicSong::getMusicChannelVolumeInternal(int,float *)")]
#[doc(alias = "__ZN4FMOD9MusicSong29getMusicChannelVolumeInternalEiPf")]
// 0xce790 — __ZN4FMOD9MusicSong29getMusicChannelVolumeInternalEiPf
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, int, float *)
pub fn stub_0xce790() -> ! {
    todo!("0xce790 __ZN4FMOD9MusicSong29getMusicChannelVolumeInternalEiPf")
}

#[doc(alias = "FMOD::MusicSong::setMusicSpeedInternal(float)")]
#[doc(alias = "__ZN4FMOD9MusicSong21setMusicSpeedInternalEf")]
// 0xce7d4 — __ZN4FMOD9MusicSong21setMusicSpeedInternalEf
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, float)
pub fn stub_0xce7d4() -> ! {
    todo!("0xce7d4 __ZN4FMOD9MusicSong21setMusicSpeedInternalEf")
}

#[doc(alias = "FMOD::MusicSong::getMusicSpeedInternal(float *)")]
#[doc(alias = "__ZN4FMOD9MusicSong21getMusicSpeedInternalEPf")]
// 0xce7e8 — __ZN4FMOD9MusicSong21getMusicSpeedInternalEPf
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, float *)
pub fn stub_0xce7e8() -> ! {
    todo!("0xce7e8 __ZN4FMOD9MusicSong21getMusicSpeedInternalEPf")
}

#[doc(alias = "FMOD::MusicSong::getLengthCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD9MusicSong17getLengthCallbackEP16FMOD_CODEC_STATEPjj")]
// 0xce800 — __ZN4FMOD9MusicSong17getLengthCallbackEP16FMOD_CODEC_STATEPjj
pub fn stub_0xce800() -> ! {
    todo!("0xce800 __ZN4FMOD9MusicSong17getLengthCallbackEP16FMOD_CODEC_STATEPjj")
}

#[doc(alias = "FMOD::MusicSong::getPositionCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD9MusicSong19getPositionCallbackEP16FMOD_CODEC_STATEPjj")]
// 0xce80c — __ZN4FMOD9MusicSong19getPositionCallbackEP16FMOD_CODEC_STATEPjj
pub fn stub_0xce80c() -> ! {
    todo!("0xce80c __ZN4FMOD9MusicSong19getPositionCallbackEP16FMOD_CODEC_STATEPjj")
}

#[doc(alias = "FMOD::MusicSong::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")]
#[doc(alias = "__ZN4FMOD9MusicSong27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi")]
// 0xce818 — __ZN4FMOD9MusicSong27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi
pub fn stub_0xce818() -> ! {
    todo!("0xce818 __ZN4FMOD9MusicSong27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi")
}

#[doc(alias = "FMOD::MusicSong::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9MusicSong29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif")]
// 0xce824 — __ZN4FMOD9MusicSong29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif
pub fn stub_0xce824() -> ! {
    todo!("0xce824 __ZN4FMOD9MusicSong29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif")
}

#[doc(alias = "FMOD::MusicSong::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")]
#[doc(alias = "__ZN4FMOD9MusicSong29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf")]
// 0xce830 — __ZN4FMOD9MusicSong29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf
pub fn stub_0xce830() -> ! {
    todo!("0xce830 __ZN4FMOD9MusicSong29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf")
}

#[doc(alias = "FMOD::MusicSong::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")]
#[doc(alias = "__ZN4FMOD9MusicSong21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf")]
// 0xce83c — __ZN4FMOD9MusicSong21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf
pub fn stub_0xce83c() -> ! {
    todo!("0xce83c __ZN4FMOD9MusicSong21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf")
}

#[doc(alias = "FMOD::MusicSong::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")]
#[doc(alias = "__ZN4FMOD9MusicSong21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf")]
// 0xce848 — __ZN4FMOD9MusicSong21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf
// type: int __fastcall(FMOD::MusicSong *, float *)
pub fn stub_0xce848() -> ! {
    todo!("0xce848 __ZN4FMOD9MusicSong21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf")
}

#[doc(alias = "FMOD::ChannelMusic::setVolume(float)")]
#[doc(alias = "__ZN4FMOD12ChannelMusic9setVolumeEf")]
// 0xce854 — __ZN4FMOD12ChannelMusic9setVolumeEf
// type: _DWORD __fastcall(FMOD::ChannelMusic *__hidden this, float)
pub fn stub_0xce854() -> ! {
    todo!("0xce854 __ZN4FMOD12ChannelMusic9setVolumeEf")
}

#[doc(alias = "FMOD::ChannelMusic::updateStream(void)")]
#[doc(alias = "__ZN4FMOD12ChannelMusic12updateStreamEv")]
// 0xce874 — __ZN4FMOD12ChannelMusic12updateStreamEv
// type: _DWORD __fastcall(FMOD::ChannelMusic *__hidden this)
pub fn stub_0xce874() -> ! {
    todo!("0xce874 __ZN4FMOD12ChannelMusic12updateStreamEv")
}

#[doc(alias = "FMOD::MusicSong::getHardwareMusicChannel(FMOD::ChannelReal **)")]
#[doc(alias = "__ZN4FMOD9MusicSong23getHardwareMusicChannelEPPNS_11ChannelRealE")]
// 0xce8bc — __ZN4FMOD9MusicSong23getHardwareMusicChannelEPPNS_11ChannelRealE
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, FMOD::ChannelReal **)
pub fn stub_0xce8bc() -> ! {
    todo!("0xce8bc __ZN4FMOD9MusicSong23getHardwareMusicChannelEPPNS_11ChannelRealE")
}

#[doc(alias = "FMOD::MusicSong::getHardwareMusicChannelCallback(FMOD_CODEC_STATE *,FMOD::ChannelReal **)")]
#[doc(alias = "__ZN4FMOD9MusicSong31getHardwareMusicChannelCallbackEP16FMOD_CODEC_STATEPPNS_11ChannelRealE")]
// 0xce91c — __ZN4FMOD9MusicSong31getHardwareMusicChannelCallbackEP16FMOD_CODEC_STATEPPNS_11ChannelRealE
pub fn stub_0xce91c() -> ! {
    todo!("0xce91c __ZN4FMOD9MusicSong31getHardwareMusicChannelCallbackEP16FMOD_CODEC_STATEPPNS_11ChannelRealE")
}

#[doc(alias = "FMOD::MusicSong::playSound(FMOD::MusicSample *,FMOD::MusicVirtualChannel *,bool,FMOD::_SNDMIXPLUGIN *)")]
#[doc(alias = "__ZN4FMOD9MusicSong9playSoundEPNS_11MusicSampleEPNS_19MusicVirtualChannelEbPNS_13_SNDMIXPLUGINE")]
// 0xce928 — __ZN4FMOD9MusicSong9playSoundEPNS_11MusicSampleEPNS_19MusicVirtualChannelEbPNS_13_SNDMIXPLUGINE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xce928() -> ! {
    todo!("0xce928 __ZN4FMOD9MusicSong9playSoundEPNS_11MusicSampleEPNS_19MusicVirtualChannelEbPNS_13_SNDMIXPLUGINE")
}

#[doc(alias = "FMOD::MusicVirtualChannel::cleanUp(void)")]
#[doc(alias = "__ZN4FMOD19MusicVirtualChannel7cleanUpEv")]
// 0xceabc — __ZN4FMOD19MusicVirtualChannel7cleanUpEv
// type: _DWORD __fastcall(FMOD::MusicVirtualChannel *__hidden this)
pub fn stub_0xceabc() -> ! {
    todo!("0xceabc __ZN4FMOD19MusicVirtualChannel7cleanUpEv")
}

#[doc(alias = "FMOD::MusicSong::stop(void)")]
#[doc(alias = "__ZN4FMOD9MusicSong4stopEv")]
// 0xceb40 — __ZN4FMOD9MusicSong4stopEv
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this)
pub fn stub_0xceb40() -> ! {
    todo!("0xceb40 __ZN4FMOD9MusicSong4stopEv")
}

#[doc(alias = "FMOD::ChannelMusic::stop(void)")]
#[doc(alias = "__ZN4FMOD12ChannelMusic4stopEv")]
// 0xcec00 — __ZN4FMOD12ChannelMusic4stopEv
// type: _DWORD __fastcall(FMOD::ChannelMusic *__hidden this)
pub fn stub_0xcec00() -> ! {
    todo!("0xcec00 __ZN4FMOD12ChannelMusic4stopEv")
}

#[doc(alias = "FMOD::MusicSong::play(bool)")]
#[doc(alias = "__ZN4FMOD9MusicSong4playEb")]
// 0xcec08 — __ZN4FMOD9MusicSong4playEb
// type: _DWORD __fastcall(FMOD::MusicSong *__hidden this, bool)
pub fn stub_0xcec08() -> ! {
    todo!("0xcec08 __ZN4FMOD9MusicSong4playEb")
}

#[doc(alias = "FMOD::ChannelMusic::start(void)")]
#[doc(alias = "__ZN4FMOD12ChannelMusic5startEv")]
// 0xcee38 — __ZN4FMOD12ChannelMusic5startEv
// type: _DWORD __fastcall(FMOD::ChannelMusic *__hidden this)
pub fn stub_0xcee38() -> ! {
    todo!("0xcee38 __ZN4FMOD12ChannelMusic5startEv")
}

#[doc(alias = "FMOD::ChannelMusic::setPaused(bool)")]
#[doc(alias = "__ZN4FMOD12ChannelMusic9setPausedEb")]
// 0xcee44 — __ZN4FMOD12ChannelMusic9setPausedEb
// type: _DWORD __fastcall(FMOD::ChannelMusic *__hidden this, bool)
pub fn stub_0xcee44() -> ! {
    todo!("0xcee44 __ZN4FMOD12ChannelMusic9setPausedEb")
}

#[doc(alias = "FMOD::ChannelReal::isStream(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal8isStreamEv")]
// 0xceec8 — __ZN4FMOD11ChannelReal8isStreamEv
// type: _DWORD __fastcall(FMOD::ChannelReal *__hidden this)
pub fn stub_0xceec8() -> ! {
    todo!("0xceec8 __ZN4FMOD11ChannelReal8isStreamEv")
}

#[doc(alias = "FMOD::ChannelReal::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
#[doc(alias = "__ZN4FMOD11ChannelReal16moveChannelGroupEPNS_13ChannelGroupIES2_b")]
// 0xceed0 — __ZN4FMOD11ChannelReal16moveChannelGroupEPNS_13ChannelGroupIES2_b
// type: _DWORD __fastcall(FMOD::ChannelReal *__hidden this, FMOD::ChannelGroupI *, FMOD::ChannelGroupI *, bool)
pub fn stub_0xceed0() -> ! {
    todo!("0xceed0 __ZN4FMOD11ChannelReal16moveChannelGroupEPNS_13ChannelGroupIES2_b")
}

#[doc(alias = "FMOD::ChannelReal::setPositionEx(unsigned int,unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD11ChannelReal13setPositionExEjjb")]
// 0xceed8 — __ZN4FMOD11ChannelReal13setPositionExEjjb
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int, bool)
pub fn stub_0xceed8() -> ! {
    todo!("0xceed8 __ZN4FMOD11ChannelReal13setPositionExEjjb")
}

#[doc(alias = "FMOD::ChannelMusic::~ChannelMusic()")]
#[doc(alias = "__ZN4FMOD12ChannelMusicD0Ev")]
// 0xceee4 — __ZN4FMOD12ChannelMusicD0Ev
// type: void __fastcall(FMOD::ChannelMusic *__hidden this)
pub fn stub_0xceee4() -> ! {
    todo!("0xceee4 __ZN4FMOD12ChannelMusicD0Ev")
}

#[doc(alias = "FMOD::ChannelMusic::~ChannelMusic()")]
#[doc(alias = "__ZN4FMOD12ChannelMusicD1Ev")]
// 0xcef08 — __ZN4FMOD12ChannelMusicD1Ev
// type: void __fastcall(FMOD::ChannelMusic *__hidden this)
pub fn stub_0xcef08() -> ! {
    todo!("0xcef08 __ZN4FMOD12ChannelMusicD1Ev")
}

#[doc(alias = "`global constructor keyed toFMOD::gDummySample")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12gDummySampleE")]
// 0xcf00c — __GLOBAL__I__ZN4FMOD12gDummySampleE
pub fn stub_0xcf00c() -> ! {
    todo!("0xcf00c __GLOBAL__I__ZN4FMOD12gDummySampleE")
}

#[doc(alias = "FMOD_Net_EncodeBase64(char *,char *,int)")]
#[doc(alias = "__Z21FMOD_Net_EncodeBase64PcS_i")]
// 0xcf018 — __Z21FMOD_Net_EncodeBase64PcS_i
// type: _DWORD __fastcall(char *, char *, int)
pub fn stub_0xcf018() -> ! {
    todo!("0xcf018 __Z21FMOD_Net_EncodeBase64PcS_i")
}

#[doc(alias = "FMOD_Net_ParseHTTPStatus(char *,int,int *,int *)")]
#[doc(alias = "__Z24FMOD_Net_ParseHTTPStatusPciPiS0_")]
// 0xcf1a8 — __Z24FMOD_Net_ParseHTTPStatusPciPiS0_
// type: _DWORD __fastcall(char *, int, int *, int *)
pub fn stub_0xcf1a8() -> ! {
    todo!("0xcf1a8 __Z24FMOD_Net_ParseHTTPStatusPciPiS0_")
}

#[doc(alias = "FMOD::HighestBit(unsigned int)")]
#[doc(alias = "__ZN4FMODL10HighestBitEj")]
// 0xcf2dc — __ZN4FMODL10HighestBitEj
// type: _DWORD __fastcall(FMOD *__hidden this, unsigned int)
pub fn stub_0xcf2dc() -> ! {
    todo!("0xcf2dc __ZN4FMODL10HighestBitEj")
}

#[doc(alias = "FMOD::aabbAdd(FMOD::FMOD_AABB &,FMOD::FMOD_AABB &,FMOD::FMOD_AABB &)")]
#[doc(alias = "__ZN4FMOD7aabbAddERNS_9FMOD_AABBES1_S1_")]
// 0xcf300 — __ZN4FMOD7aabbAddERNS_9FMOD_AABBES1_S1_
pub fn stub_0xcf300() -> ! {
    todo!("0xcf300 __ZN4FMOD7aabbAddERNS_9FMOD_AABBES1_S1_")
}

#[doc(alias = "FMOD::Octree::getAABB(FMOD::FMOD_AABB *)")]
#[doc(alias = "__ZN4FMOD6Octree7getAABBEPNS_9FMOD_AABBE")]
// 0xcf394 — __ZN4FMOD6Octree7getAABBEPNS_9FMOD_AABBE
pub fn stub_0xcf394() -> ! {
    todo!("0xcf394 __ZN4FMOD6Octree7getAABBEPNS_9FMOD_AABBE")
}

#[doc(alias = "FMOD::Octree::addToFreeList(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree13addToFreeListEPNS_10OctreeNodeE")]
// 0xcf418 — __ZN4FMOD6Octree13addToFreeListEPNS_10OctreeNodeE
pub fn stub_0xcf418() -> ! {
    todo!("0xcf418 __ZN4FMOD6Octree13addToFreeListEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD::Octree::addInternalNode(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree15addInternalNodeEPNS_10OctreeNodeE")]
// 0xcf448 — __ZN4FMOD6Octree15addInternalNodeEPNS_10OctreeNodeE
pub fn stub_0xcf448() -> ! {
    todo!("0xcf448 __ZN4FMOD6Octree15addInternalNodeEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD::Octree::getFreeNode(void)")]
#[doc(alias = "__ZN4FMOD6Octree11getFreeNodeEv")]
// 0xcf460 — __ZN4FMOD6Octree11getFreeNodeEv
// type: _DWORD __fastcall(FMOD::Octree *__hidden this)
pub fn stub_0xcf460() -> ! {
    todo!("0xcf460 __ZN4FMOD6Octree11getFreeNodeEv")
}

#[doc(alias = "FMOD::Octree::adjustAABBs(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree11adjustAABBsEPNS_10OctreeNodeE")]
// 0xcf494 — __ZN4FMOD6Octree11adjustAABBsEPNS_10OctreeNodeE
pub fn stub_0xcf494() -> ! {
    todo!("0xcf494 __ZN4FMOD6Octree11adjustAABBsEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD::Octree::addListItem(FMOD::OctreeNode *,FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree11addListItemEPNS_10OctreeNodeES2_")]
// 0xcf584 — __ZN4FMOD6Octree11addListItemEPNS_10OctreeNodeES2_
// type: int __fastcall(int result, _DWORD *, _DWORD *)
pub fn stub_0xcf584() -> ! {
    todo!("0xcf584 __ZN4FMOD6Octree11addListItemEPNS_10OctreeNodeES2_")
}

#[doc(alias = "FMOD::Octree::insertInternal(FMOD::OctreeNode *,FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree14insertInternalEPNS_10OctreeNodeES2_")]
// 0xcf688 — __ZN4FMOD6Octree14insertInternalEPNS_10OctreeNodeES2_
pub fn stub_0xcf688() -> ! {
    todo!("0xcf688 __ZN4FMOD6Octree14insertInternalEPNS_10OctreeNodeES2_")
}

#[doc(alias = "FMOD::Octree::deleteItem(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree10deleteItemEPNS_10OctreeNodeE")]
// 0xcfa50 — __ZN4FMOD6Octree10deleteItemEPNS_10OctreeNodeE
pub fn stub_0xcfa50() -> ! {
    todo!("0xcfa50 __ZN4FMOD6Octree10deleteItemEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD::Octree::insertItem(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree10insertItemEPNS_10OctreeNodeE")]
// 0xcfc08 — __ZN4FMOD6Octree10insertItemEPNS_10OctreeNodeE
pub fn stub_0xcfc08() -> ! {
    todo!("0xcfc08 __ZN4FMOD6Octree10insertItemEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD::Octree::testLine(FMOD::OctreeNode *,FMOD_VECTOR,FMOD_VECTOR,FMOD::Octree::RecursionData *)")]
#[doc(alias = "__ZN4FMOD6Octree8testLineEPNS_10OctreeNodeE11FMOD_VECTORS3_PNS0_13RecursionDataE")]
// 0xcfd48 — __ZN4FMOD6Octree8testLineEPNS_10OctreeNodeE11FMOD_VECTORS3_PNS0_13RecursionDataE
// type: int __fastcall(int, int, int, int, float, float, float, int)
pub fn stub_0xcfd48() -> ! {
    todo!("0xcfd48 __ZN4FMOD6Octree8testLineEPNS_10OctreeNodeE11FMOD_VECTORS3_PNS0_13RecursionDataE")
}

#[doc(alias = "FMOD::Octree::testLine(bool (*)(FMOD::OctreeNode *,void *),void *,FMOD_VECTOR const&,FMOD_VECTOR const&)")]
#[doc(alias = "__ZN4FMOD6Octree8testLineEPFbPNS_10OctreeNodeEPvES3_RK11FMOD_VECTORS8_")]
// 0xd0370 — __ZN4FMOD6Octree8testLineEPFbPNS_10OctreeNodeEPvES3_RK11FMOD_VECTORS8_
pub fn stub_0xd0370() -> ! {
    todo!("0xd0370 __ZN4FMOD6Octree8testLineEPFbPNS_10OctreeNodeEPvES3_RK11FMOD_VECTORS8_")
}

#[doc(alias = "FMOD::Octree::updateItem(FMOD::OctreeNode *)")]
#[doc(alias = "__ZN4FMOD6Octree10updateItemEPNS_10OctreeNodeE")]
// 0xd03cc — __ZN4FMOD6Octree10updateItemEPNS_10OctreeNodeE
pub fn stub_0xd03cc() -> ! {
    todo!("0xd03cc __ZN4FMOD6Octree10updateItemEPNS_10OctreeNodeE")
}

#[doc(alias = "FMOD_OS_File_Cancel(void *)")]
#[doc(alias = "__Z19FMOD_OS_File_CancelPv")]
// 0xd0548 — __Z19FMOD_OS_File_CancelPv
// type: _DWORD __fastcall(void *)
pub fn stub_0xd0548() -> ! {
    todo!("0xd0548 __Z19FMOD_OS_File_CancelPv")
}

#[doc(alias = "FMOD_OS_Thread_Destroy(void *)")]
#[doc(alias = "__Z22FMOD_OS_Thread_DestroyPv")]
// 0xd0550 — __Z22FMOD_OS_Thread_DestroyPv
// type: _DWORD __fastcall(void *)
pub fn stub_0xd0550() -> ! {
    todo!("0xd0550 __Z22FMOD_OS_Thread_DestroyPv")
}

#[doc(alias = "FMOD_OS_CheckDriverList(bool *)")]
#[doc(alias = "__Z23FMOD_OS_CheckDriverListPb")]
// 0xd0558 — __Z23FMOD_OS_CheckDriverListPb
// type: _DWORD __fastcall(bool *)
pub fn stub_0xd0558() -> ! {
    todo!("0xd0558 __Z23FMOD_OS_CheckDriverListPb")
}

#[doc(alias = "FMOD_OS_Semaphore_Signal(FMOD_OS_SEMAPHORE *,bool)")]
#[doc(alias = "__Z24FMOD_OS_Semaphore_SignalP17FMOD_OS_SEMAPHOREb")]
// 0xd0560 — __Z24FMOD_OS_Semaphore_SignalP17FMOD_OS_SEMAPHOREb
pub fn stub_0xd0560() -> ! {
    todo!("0xd0560 __Z24FMOD_OS_Semaphore_SignalP17FMOD_OS_SEMAPHOREb")
}

#[doc(alias = "FMOD_OS_Semaphore_Wait(FMOD_OS_SEMAPHORE *)")]
#[doc(alias = "__Z22FMOD_OS_Semaphore_WaitP17FMOD_OS_SEMAPHORE")]
// 0xd058c — __Z22FMOD_OS_Semaphore_WaitP17FMOD_OS_SEMAPHORE
pub fn stub_0xd058c() -> ! {
    todo!("0xd058c __Z22FMOD_OS_Semaphore_WaitP17FMOD_OS_SEMAPHORE")
}

#[doc(alias = "FMOD_OS_Semaphore_Free(FMOD_OS_SEMAPHORE *)")]
#[doc(alias = "__Z22FMOD_OS_Semaphore_FreeP17FMOD_OS_SEMAPHORE")]
// 0xd05b8 — __Z22FMOD_OS_Semaphore_FreeP17FMOD_OS_SEMAPHORE
pub fn stub_0xd05b8() -> ! {
    todo!("0xd05b8 __Z22FMOD_OS_Semaphore_FreeP17FMOD_OS_SEMAPHORE")
}

#[doc(alias = "FMOD_OS_CriticalSection_Leave(FMOD_OS_CRITICALSECTION *)")]
#[doc(alias = "__Z29FMOD_OS_CriticalSection_LeaveP23FMOD_OS_CRITICALSECTION")]
// 0xd066c — __Z29FMOD_OS_CriticalSection_LeaveP23FMOD_OS_CRITICALSECTION
// type: int __fastcall(_DWORD)
pub fn stub_0xd066c() -> ! {
    todo!("0xd066c __Z29FMOD_OS_CriticalSection_LeaveP23FMOD_OS_CRITICALSECTION")
}

#[doc(alias = "FMOD_OS_CriticalSection_Enter(FMOD_OS_CRITICALSECTION *)")]
#[doc(alias = "__Z29FMOD_OS_CriticalSection_EnterP23FMOD_OS_CRITICALSECTION")]
// 0xd0694 — __Z29FMOD_OS_CriticalSection_EnterP23FMOD_OS_CRITICALSECTION
// type: int __fastcall(pthread_mutex_t *)
pub fn stub_0xd0694() -> ! {
    todo!("0xd0694 __Z29FMOD_OS_CriticalSection_EnterP23FMOD_OS_CRITICALSECTION")
}

#[doc(alias = "FMOD_OS_CriticalSection_Free(FMOD_OS_CRITICALSECTION *,bool)")]
#[doc(alias = "__Z28FMOD_OS_CriticalSection_FreeP23FMOD_OS_CRITICALSECTIONb")]
// 0xd06bc — __Z28FMOD_OS_CriticalSection_FreeP23FMOD_OS_CRITICALSECTIONb
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xd06bc() -> ! {
    todo!("0xd06bc __Z28FMOD_OS_CriticalSection_FreeP23FMOD_OS_CRITICALSECTIONb")
}

#[doc(alias = "FMOD_OS_CriticalSection_Create(FMOD_OS_CRITICALSECTION **,bool)")]
#[doc(alias = "__Z30FMOD_OS_CriticalSection_CreatePP23FMOD_OS_CRITICALSECTIONb")]
// 0xd0718 — __Z30FMOD_OS_CriticalSection_CreatePP23FMOD_OS_CRITICALSECTIONb
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xd0718() -> ! {
    todo!("0xd0718 __Z30FMOD_OS_CriticalSection_CreatePP23FMOD_OS_CRITICALSECTIONb")
}

#[doc(alias = "FMOD_OS_Thread_Create(char const*,void * (*)(void *),void *,FMOD_THREAD_PRIORITY,void *,int,void **)")]
#[doc(alias = "__Z21FMOD_OS_Thread_CreatePKcPFPvS1_ES1_20FMOD_THREAD_PRIORITYS1_iPS1_")]
// 0xd0844 — __Z21FMOD_OS_Thread_CreatePKcPFPvS1_ES1_20FMOD_THREAD_PRIORITYS1_iPS1_
pub fn stub_0xd0844() -> ! {
    todo!("0xd0844 __Z21FMOD_OS_Thread_CreatePKcPFPvS1_ES1_20FMOD_THREAD_PRIORITYS1_iPS1_")
}

#[doc(alias = "FMOD_OS_Thread_GetCurrentID(unsigned int *)")]
#[doc(alias = "__Z27FMOD_OS_Thread_GetCurrentIDPj")]
// 0xd0984 — __Z27FMOD_OS_Thread_GetCurrentIDPj
// type: _DWORD __fastcall(unsigned int *)
pub fn stub_0xd0984() -> ! {
    todo!("0xd0984 __Z27FMOD_OS_Thread_GetCurrentIDPj")
}

#[doc(alias = "FMOD_OS_Time_Sleep(unsigned int)")]
#[doc(alias = "__Z18FMOD_OS_Time_Sleepj")]
// 0xd09a0 — __Z18FMOD_OS_Time_Sleepj
// type: _DWORD __fastcall(unsigned int)
pub fn stub_0xd09a0() -> ! {
    todo!("0xd09a0 __Z18FMOD_OS_Time_Sleepj")
}

#[doc(alias = "FMOD_OS_Time_GetMs(unsigned int *)")]
#[doc(alias = "__Z18FMOD_OS_Time_GetMsPj")]
// 0xd09c8 — __Z18FMOD_OS_Time_GetMsPj
// type: _DWORD __fastcall(unsigned int *)
pub fn stub_0xd09c8() -> ! {
    todo!("0xd09c8 __Z18FMOD_OS_Time_GetMsPj")
}

#[doc(alias = "FMOD_OS_Time_GetNs(unsigned int *)")]
#[doc(alias = "__Z18FMOD_OS_Time_GetNsPj")]
// 0xd0a80 — __Z18FMOD_OS_Time_GetNsPj
// type: _DWORD __fastcall(unsigned int *)
pub fn stub_0xd0a80() -> ! {
    todo!("0xd0a80 __Z18FMOD_OS_Time_GetNsPj")
}

#[doc(alias = "FMOD_OS_Semaphore_Create(FMOD_OS_SEMAPHORE **)")]
#[doc(alias = "__Z24FMOD_OS_Semaphore_CreatePP17FMOD_OS_SEMAPHORE")]
// 0xd0b38 — __Z24FMOD_OS_Semaphore_CreatePP17FMOD_OS_SEMAPHORE
pub fn stub_0xd0b38() -> ! {
    todo!("0xd0b38 __Z24FMOD_OS_Semaphore_CreatePP17FMOD_OS_SEMAPHORE")
}

#[doc(alias = "FMOD_OS_File_Seek(void *,unsigned int)")]
#[doc(alias = "__Z17FMOD_OS_File_SeekPvj")]
// 0xd0c34 — __Z17FMOD_OS_File_SeekPvj
// type: _DWORD __fastcall(void *, unsigned int)
pub fn stub_0xd0c34() -> ! {
    todo!("0xd0c34 __Z17FMOD_OS_File_SeekPvj")
}

#[doc(alias = "FMOD_OS_File_Read(void *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__Z17FMOD_OS_File_ReadPvS_jPj")]
// 0xd0c60 — __Z17FMOD_OS_File_ReadPvS_jPj
// type: _DWORD __fastcall(void *, void *__ptr, unsigned int, unsigned int *)
pub fn stub_0xd0c60() -> ! {
    todo!("0xd0c60 __Z17FMOD_OS_File_ReadPvS_jPj")
}

#[doc(alias = "FMOD_OS_File_Close(void *)")]
#[doc(alias = "__Z18FMOD_OS_File_ClosePv")]
// 0xd0cb8 — __Z18FMOD_OS_File_ClosePv
// type: _DWORD __fastcall(void *)
pub fn stub_0xd0cb8() -> ! {
    todo!("0xd0cb8 __Z18FMOD_OS_File_ClosePv")
}

#[doc(alias = "FMOD_OS_File_Open(char const*,char *,int,unsigned int *,void **)")]
#[doc(alias = "__Z17FMOD_OS_File_OpenPKcPciPjPPv")]
// 0xd0cd4 — __Z17FMOD_OS_File_OpenPKcPciPjPPv
// type: _DWORD __fastcall(const char *, char *, int, unsigned int *, void **)
pub fn stub_0xd0cd4() -> ! {
    todo!("0xd0cd4 __Z17FMOD_OS_File_OpenPKcPciPjPPv")
}

#[doc(alias = "FMOD_OS_Memory_Free(void *,unsigned int)")]
#[doc(alias = "__Z19FMOD_OS_Memory_FreePvj")]
// 0xd0dc8 — __Z19FMOD_OS_Memory_FreePvj
// type: _DWORD __fastcall(void *, unsigned int)
pub fn stub_0xd0dc8() -> ! {
    todo!("0xd0dc8 __Z19FMOD_OS_Memory_FreePvj")
}

#[doc(alias = "FMOD_OS_Memory_Realloc(void *,int,unsigned int)")]
#[doc(alias = "__Z22FMOD_OS_Memory_ReallocPvij")]
// 0xd0dd8 — __Z22FMOD_OS_Memory_ReallocPvij
// type: void *__fastcall(void *, size_t, unsigned int)
pub fn stub_0xd0dd8() -> ! {
    todo!("0xd0dd8 __Z22FMOD_OS_Memory_ReallocPvij")
}

#[doc(alias = "FMOD_OS_Memory_Alloc(int,unsigned int)")]
#[doc(alias = "__Z20FMOD_OS_Memory_Allocij")]
// 0xd0de8 — __Z20FMOD_OS_Memory_Allocij
// type: _DWORD __fastcall(int, unsigned int)
pub fn stub_0xd0de8() -> ! {
    todo!("0xd0de8 __Z20FMOD_OS_Memory_Allocij")
}

#[doc(alias = "FMOD_OS_Net_Read(void const*,char *,unsigned int,unsigned int *)")]
#[doc(alias = "__Z16FMOD_OS_Net_ReadPKvPcjPj")]
// 0xd0df8 — __Z16FMOD_OS_Net_ReadPKvPcjPj
// type: _DWORD __fastcall(const void *, char *, size_t, unsigned int *)
pub fn stub_0xd0df8() -> ! {
    todo!("0xd0df8 __Z16FMOD_OS_Net_ReadPKvPcjPj")
}

#[doc(alias = "FMOD_OS_Net_ReadLine(void const*,char *,unsigned int)")]
#[doc(alias = "__Z20FMOD_OS_Net_ReadLinePKvPcj")]
// 0xd0ec0 — __Z20FMOD_OS_Net_ReadLinePKvPcj
// type: _DWORD __fastcall(const void *, char *, unsigned int)
pub fn stub_0xd0ec0() -> ! {
    todo!("0xd0ec0 __Z20FMOD_OS_Net_ReadLinePKvPcj")
}

#[doc(alias = "FMOD_OS_Net_Write(void const*,char const*,unsigned int,unsigned int *)")]
#[doc(alias = "__Z17FMOD_OS_Net_WritePKvPKcjPj")]
// 0xd0f84 — __Z17FMOD_OS_Net_WritePKvPKcjPj
// type: _DWORD __fastcall(const void *, const char *, size_t, unsigned int *)
pub fn stub_0xd0f84() -> ! {
    todo!("0xd0f84 __Z17FMOD_OS_Net_WritePKvPKcjPj")
}

#[doc(alias = "FMOD_OS_Net_Close(void const*)")]
#[doc(alias = "__Z17FMOD_OS_Net_ClosePKv")]
// 0xd103c — __Z17FMOD_OS_Net_ClosePKv
// type: _DWORD __fastcall(const void *)
pub fn stub_0xd103c() -> ! {
    todo!("0xd103c __Z17FMOD_OS_Net_ClosePKv")
}

#[doc(alias = "FMOD_OS_Net_Accept(void const*,void **)")]
#[doc(alias = "__Z18FMOD_OS_Net_AcceptPKvPPv")]
// 0xd1058 — __Z18FMOD_OS_Net_AcceptPKvPPv
// type: _DWORD __fastcall(const void *, void **)
pub fn stub_0xd1058() -> ! {
    todo!("0xd1058 __Z18FMOD_OS_Net_AcceptPKvPPv")
}

#[doc(alias = "FMOD_OS_Net_Listen(unsigned short,void **)")]
#[doc(alias = "__Z18FMOD_OS_Net_ListentPPv")]
// 0xd10d0 — __Z18FMOD_OS_Net_ListentPPv
// type: _DWORD __fastcall(unsigned __int16, void **)
pub fn stub_0xd10d0() -> ! {
    todo!("0xd10d0 __Z18FMOD_OS_Net_ListentPPv")
}

#[doc(alias = "FMOD_OS_Net_Shutdown(void)")]
#[doc(alias = "__Z20FMOD_OS_Net_Shutdownv")]
// 0xd11b8 — __Z20FMOD_OS_Net_Shutdownv
// type: _DWORD __fastcall()
pub fn stub_0xd11b8() -> ! {
    todo!("0xd11b8 __Z20FMOD_OS_Net_Shutdownv")
}

#[doc(alias = "FMOD_OS_Net_Init(void)")]
#[doc(alias = "__Z16FMOD_OS_Net_Initv")]
// 0xd1214 — __Z16FMOD_OS_Net_Initv
// type: _DWORD __fastcall()
pub fn stub_0xd1214() -> ! {
    todo!("0xd1214 __Z16FMOD_OS_Net_Initv")
}

#[doc(alias = "FMOD_OS_Net_Connect(char const*,unsigned short,void **)")]
#[doc(alias = "__Z19FMOD_OS_Net_ConnectPKctPPv")]
// 0xd125c — __Z19FMOD_OS_Net_ConnectPKctPPv
// type: _DWORD __fastcall(const char *, unsigned __int16, void **)
pub fn stub_0xd125c() -> ! {
    todo!("0xd125c __Z19FMOD_OS_Net_ConnectPKctPPv")
}

#[doc(alias = "FMOD_OS_Output_GetDefault(FMOD_OUTPUTTYPE *)")]
#[doc(alias = "__Z25FMOD_OS_Output_GetDefaultP15FMOD_OUTPUTTYPE")]
// 0xd148c — __Z25FMOD_OS_Output_GetDefaultP15FMOD_OUTPUTTYPE
pub fn stub_0xd148c() -> ! {
    todo!("0xd148c __Z25FMOD_OS_Output_GetDefaultP15FMOD_OUTPUTTYPE")
}

#[doc(alias = "FMOD_OS_Output_Register(FMOD::PluginFactory *)")]
#[doc(alias = "__Z23FMOD_OS_Output_RegisterPN4FMOD13PluginFactoryE")]
// 0xd14a4 — __Z23FMOD_OS_Output_RegisterPN4FMOD13PluginFactoryE
// type: _DWORD __fastcall(FMOD::PluginFactory *)
pub fn stub_0xd14a4() -> ! {
    todo!("0xd14a4 __Z23FMOD_OS_Output_RegisterPN4FMOD13PluginFactoryE")
}

#[doc(alias = "FMOD::Output::getFreeChannel(unsigned int,FMOD::ChannelReal **,int,int,int *,bool)")]
#[doc(alias = "__ZN4FMOD6Output14getFreeChannelEjPPNS_11ChannelRealEiiPib")]
// 0xd14c8 — __ZN4FMOD6Output14getFreeChannelEjPPNS_11ChannelRealEiiPib
// type: _DWORD __fastcall(FMOD::Output *__hidden this, unsigned int, FMOD::ChannelReal **, int, int, int *, bool)
pub fn stub_0xd14c8() -> ! {
    todo!("0xd14c8 __ZN4FMOD6Output14getFreeChannelEjPPNS_11ChannelRealEiiPib")
}

#[doc(alias = "FMOD::Output::recordStop(FMOD::FMOD_RECORDING_INFO *)")]
#[doc(alias = "__ZN4FMOD6Output10recordStopEPNS_19FMOD_RECORDING_INFOE")]
// 0xd1528 — __ZN4FMOD6Output10recordStopEPNS_19FMOD_RECORDING_INFOE
pub fn stub_0xd1528() -> ! {
    todo!("0xd1528 __ZN4FMOD6Output10recordStopEPNS_19FMOD_RECORDING_INFOE")
}

#[doc(alias = "FMOD::Output::release(void)")]
#[doc(alias = "__ZN4FMOD6Output7releaseEv")]
// 0xd16a8 — __ZN4FMOD6Output7releaseEv
// type: _DWORD __fastcall(FMOD::Output *__hidden this)
pub fn stub_0xd16a8() -> ! {
    todo!("0xd16a8 __ZN4FMOD6Output7releaseEv")
}

#[doc(alias = "FMOD::Output::Output(void)")]
#[doc(alias = "__ZN4FMOD6OutputC2Ev")]
// 0xd1718 — __ZN4FMOD6OutputC2Ev
// type: _DWORD __fastcall(FMOD::Output *__hidden this)
pub fn stub_0xd1718() -> ! {
    todo!("0xd1718 __ZN4FMOD6OutputC2Ev")
}

#[doc(alias = "FMOD::Output::Output(void)")]
#[doc(alias = "__ZN4FMOD6OutputC1Ev")]
// 0xd17f0 — __ZN4FMOD6OutputC1Ev
// type: _DWORD __fastcall(FMOD::Output *__hidden this)
pub fn stub_0xd17f0() -> ! {
    todo!("0xd17f0 __ZN4FMOD6OutputC1Ev")
}

#[doc(alias = "FMOD::Output::recordStopAll(bool)")]
#[doc(alias = "__ZN4FMOD6Output13recordStopAllEb")]
// 0xd17f4 — __ZN4FMOD6Output13recordStopAllEb
// type: _DWORD __fastcall(FMOD::Output *__hidden this, bool)
pub fn stub_0xd17f4() -> ! {
    todo!("0xd17f4 __ZN4FMOD6Output13recordStopAllEb")
}

#[doc(alias = "FMOD::Output::recordRead(FMOD::FMOD_RECORDING_INFO *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD6Output10recordReadEPNS_19FMOD_RECORDING_INFOEPfS3_jii")]
// 0xd1850 — __ZN4FMOD6Output10recordReadEPNS_19FMOD_RECORDING_INFOEPfS3_jii
pub fn stub_0xd1850() -> ! {
    todo!("0xd1850 __ZN4FMOD6Output10recordReadEPNS_19FMOD_RECORDING_INFOEPfS3_jii")
}

#[doc(alias = "FMOD::Output::recordFill(FMOD::FMOD_RECORDING_INFO *,unsigned int)")]
#[doc(alias = "__ZN4FMOD6Output10recordFillEPNS_19FMOD_RECORDING_INFOEj")]
// 0xd1d88 — __ZN4FMOD6Output10recordFillEPNS_19FMOD_RECORDING_INFOEj
pub fn stub_0xd1d88() -> ! {
    todo!("0xd1d88 __ZN4FMOD6Output10recordFillEPNS_19FMOD_RECORDING_INFOEj")
}

#[doc(alias = "FMOD::Output::recordUpdate(void)")]
#[doc(alias = "__ZN4FMOD6Output12recordUpdateEv")]
// 0xd23b0 — __ZN4FMOD6Output12recordUpdateEv
// type: _DWORD __fastcall(FMOD::Output *__hidden this)
pub fn stub_0xd23b0() -> ! {
    todo!("0xd23b0 __ZN4FMOD6Output12recordUpdateEv")
}

#[doc(alias = "FMOD::Output::mix(void *,unsigned int)")]
#[doc(alias = "__ZN4FMOD6Output3mixEPvj")]
// 0xd24ec — __ZN4FMOD6Output3mixEPvj
// type: _DWORD __fastcall(FMOD::Output *__hidden this, void *, unsigned int)
pub fn stub_0xd24ec() -> ! {
    todo!("0xd24ec __ZN4FMOD6Output3mixEPvj")
}

#[doc(alias = "FMOD::Output::mixCallback(FMOD_OUTPUT_STATE *,void *,unsigned int)")]
#[doc(alias = "__ZN4FMOD6Output11mixCallbackEP17FMOD_OUTPUT_STATEPvj")]
// 0xd27a8 — __ZN4FMOD6Output11mixCallbackEP17FMOD_OUTPUT_STATEPvj
pub fn stub_0xd27a8() -> ! {
    todo!("0xd27a8 __ZN4FMOD6Output11mixCallbackEP17FMOD_OUTPUT_STATEPvj")
}

#[doc(alias = "FMOD::OutputCoreAudio::getNumDrivers(int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13getNumDriversEPi")]
// 0xd27b4 — __ZN4FMOD15OutputCoreAudio13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, int *)
pub fn stub_0xd27b4() -> ! {
    todo!("0xd27b4 __ZN4FMOD15OutputCoreAudio13getNumDriversEPi")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetNumDrivers(int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio19recordGetNumDriversEPi")]
// 0xd27c8 — __ZN4FMOD15OutputCoreAudio19recordGetNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, int *)
pub fn stub_0xd27c8() -> ! {
    todo!("0xd27c8 __ZN4FMOD15OutputCoreAudio19recordGetNumDriversEPi")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetPosition(FMOD::FMOD_RECORDING_INFO *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio17recordGetPositionEPNS_19FMOD_RECORDING_INFOEPj")]
// 0xd27dc — __ZN4FMOD15OutputCoreAudio17recordGetPositionEPNS_19FMOD_RECORDING_INFOEPj
pub fn stub_0xd27dc() -> ! {
    todo!("0xd27dc __ZN4FMOD15OutputCoreAudio17recordGetPositionEPNS_19FMOD_RECORDING_INFOEPj")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordLock(FMOD::FMOD_RECORDING_INFO *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio10recordLockEPNS_19FMOD_RECORDING_INFOEjjPPvS4_PjS5_")]
// 0xd27ec — __ZN4FMOD15OutputCoreAudio10recordLockEPNS_19FMOD_RECORDING_INFOEjjPPvS4_PjS5_
pub fn stub_0xd27ec() -> ! {
    todo!("0xd27ec __ZN4FMOD15OutputCoreAudio10recordLockEPNS_19FMOD_RECORDING_INFOEjjPPvS4_PjS5_")
}

#[doc(alias = "FMOD::OutputCoreAudio::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi")]
// 0xd2894 — __ZN4FMOD15OutputCoreAudio21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd2894() -> ! {
    todo!("0xd2894 __ZN4FMOD15OutputCoreAudio21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio27recordGetNumDriversCallbackEP17FMOD_OUTPUT_STATEPi")]
// 0xd28a0 — __ZN4FMOD15OutputCoreAudio27recordGetNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd28a0() -> ! {
    todo!("0xd28a0 __ZN4FMOD15OutputCoreAudio27recordGetNumDriversCallbackEP17FMOD_OUTPUT_STATEPi")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetPositionCallback(FMOD_OUTPUT_STATE *,FMOD::FMOD_RECORDING_INFO *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio25recordGetPositionCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEPj")]
// 0xd28ac — __ZN4FMOD15OutputCoreAudio25recordGetPositionCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEPj
pub fn stub_0xd28ac() -> ! {
    todo!("0xd28ac __ZN4FMOD15OutputCoreAudio25recordGetPositionCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEPj")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordLockCallback(FMOD_OUTPUT_STATE *,FMOD::FMOD_RECORDING_INFO *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio18recordLockCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEjjPPvS6_PjS7_")]
// 0xd28b8 — __ZN4FMOD15OutputCoreAudio18recordLockCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEjjPPvS6_PjS7_
pub fn stub_0xd28b8() -> ! {
    todo!("0xd28b8 __ZN4FMOD15OutputCoreAudio18recordLockCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEjjPPvS6_PjS7_")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordPause(bool)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio11recordPauseEb")]
// 0xd28f8 — __ZN4FMOD15OutputCoreAudio11recordPauseEb
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, bool)
pub fn stub_0xd28f8() -> ! {
    todo!("0xd28f8 __ZN4FMOD15OutputCoreAudio11recordPauseEb")
}

#[doc(alias = "FMOD::OutputCoreAudio::updateRecord(unsigned long *,AudioTimeStamp const*,unsigned long,unsigned long)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio12updateRecordEPmPK14AudioTimeStampmm")]
// 0xd296c — __ZN4FMOD15OutputCoreAudio12updateRecordEPmPK14AudioTimeStampmm
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int *, const AudioTimeStamp *, unsigned int, unsigned int)
pub fn stub_0xd296c() -> ! {
    todo!("0xd296c __ZN4FMOD15OutputCoreAudio12updateRecordEPmPK14AudioTimeStampmm")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordCallback(void *,unsigned long *,AudioTimeStamp const*,unsigned long,unsigned long,AudioBufferList *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio14recordCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList")]
// 0xd2a3c — __ZN4FMOD15OutputCoreAudio14recordCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *, unsigned int *, const AudioTimeStamp *, unsigned int, unsigned int, AudioBufferList *)
pub fn stub_0xd2a3c() -> ! {
    todo!("0xd2a3c __ZN4FMOD15OutputCoreAudio14recordCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList")
}

#[doc(alias = "FMOD::OutputCoreAudio::setupAudioUnit(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio14setupAudioUnitEjj")]
// 0xd2a64 — __ZN4FMOD15OutputCoreAudio14setupAudioUnitEjj
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int, unsigned int)
pub fn stub_0xd2a64() -> ! {
    todo!("0xd2a64 __ZN4FMOD15OutputCoreAudio14setupAudioUnitEjj")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetDriverInfo(int,char *,int,FMOD_GUID *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio19recordGetDriverInfoEiPciP9FMOD_GUID")]
// 0xd2bd8 — __ZN4FMOD15OutputCoreAudio19recordGetDriverInfoEiPciP9FMOD_GUID
pub fn stub_0xd2bd8() -> ! {
    todo!("0xd2bd8 __ZN4FMOD15OutputCoreAudio19recordGetDriverInfoEiPciP9FMOD_GUID")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordGetDriverInfoCallback(FMOD_OUTPUT_STATE *,int,char *,int,FMOD_GUID *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio27recordGetDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID")]
// 0xd2c20 — __ZN4FMOD15OutputCoreAudio27recordGetDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID
pub fn stub_0xd2c20() -> ! {
    todo!("0xd2c20 __ZN4FMOD15OutputCoreAudio27recordGetDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID")
}

#[doc(alias = "FMOD::OutputCoreAudio::getDriverInfo(int,char *,int,FMOD_GUID *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13getDriverInfoEiPciP9FMOD_GUID")]
// 0xd2c34 — __ZN4FMOD15OutputCoreAudio13getDriverInfoEiPciP9FMOD_GUID
// type: int __fastcall(int, int, int, int)
pub fn stub_0xd2c34() -> ! {
    todo!("0xd2c34 __ZN4FMOD15OutputCoreAudio13getDriverInfoEiPciP9FMOD_GUID")
}

#[doc(alias = "FMOD::OutputCoreAudio::getDriverInfoCallback(FMOD_OUTPUT_STATE *,int,char *,int,FMOD_GUID *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio21getDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID")]
// 0xd2c7c — __ZN4FMOD15OutputCoreAudio21getDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID
pub fn stub_0xd2c7c() -> ! {
    todo!("0xd2c7c __ZN4FMOD15OutputCoreAudio21getDriverInfoCallbackEP17FMOD_OUTPUT_STATEiPciP9FMOD_GUID")
}

#[doc(alias = "FMOD::OutputCoreAudio::handleRouteChange(__CFDictionary const*)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio17handleRouteChangeEPK14__CFDictionary")]
// 0xd2c90 — __ZN4FMOD15OutputCoreAudio17handleRouteChangeEPK14__CFDictionary
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, const __CFDictionary *)
pub fn stub_0xd2c90() -> ! {
    todo!("0xd2c90 __ZN4FMOD15OutputCoreAudio17handleRouteChangeEPK14__CFDictionary")
}

#[doc(alias = "FMOD::OutputCoreAudio::routeChangeCallback(void *,unsigned long,unsigned long,void const*)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio19routeChangeCallbackEPvmmPKv")]
// 0xd2dd0 — __ZN4FMOD15OutputCoreAudio19routeChangeCallbackEPvmmPKv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *, unsigned int, __CFDictionary *, const void *)
pub fn stub_0xd2dd0() -> ! {
    todo!("0xd2dd0 __ZN4FMOD15OutputCoreAudio19routeChangeCallbackEPvmmPKv")
}

#[doc(alias = "FMOD::OutputCoreAudio::updateRender(unsigned long,AudioBufferList *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio12updateRenderEmP15AudioBufferList")]
// 0xd2e00 — __ZN4FMOD15OutputCoreAudio12updateRenderEmP15AudioBufferList
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int, AudioBufferList *)
pub fn stub_0xd2e00() -> ! {
    todo!("0xd2e00 __ZN4FMOD15OutputCoreAudio12updateRenderEmP15AudioBufferList")
}

#[doc(alias = "FMOD::OutputCoreAudio::renderCallback(void *,unsigned long *,AudioTimeStamp const*,unsigned long,unsigned long,AudioBufferList *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio14renderCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList")]
// 0xd2ecc — __ZN4FMOD15OutputCoreAudio14renderCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *, unsigned int *, const AudioTimeStamp *, unsigned int, AudioBufferList *, AudioBufferList *)
pub fn stub_0xd2ecc() -> ! {
    todo!("0xd2ecc __ZN4FMOD15OutputCoreAudio14renderCallbackEPvPmPK14AudioTimeStampmmP15AudioBufferList")
}

#[doc(alias = "FMOD::OutputCoreAudio::updateMixer(void)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio11updateMixerEv")]
// 0xd2eec — __ZN4FMOD15OutputCoreAudio11updateMixerEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd2eec() -> ! {
    todo!("0xd2eec __ZN4FMOD15OutputCoreAudio11updateMixerEv")
}

#[doc(alias = "FMOD::OutputCoreAudio::mixerCallback(void *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13mixerCallbackEPv")]
// 0xd2f9c — __ZN4FMOD15OutputCoreAudio13mixerCallbackEPv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *)
pub fn stub_0xd2f9c() -> ! {
    todo!("0xd2f9c __ZN4FMOD15OutputCoreAudio13mixerCallbackEPv")
}

#[doc(alias = "FMOD::OutputCoreAudio::stop(void)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio4stopEv")]
// 0xd2fa0 — __ZN4FMOD15OutputCoreAudio4stopEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd2fa0() -> ! {
    todo!("0xd2fa0 __ZN4FMOD15OutputCoreAudio4stopEv")
}

#[doc(alias = "FMOD::OutputCoreAudio::stopCallback(FMOD_OUTPUT_STATE *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio12stopCallbackEP17FMOD_OUTPUT_STATE")]
// 0xd2fdc — __ZN4FMOD15OutputCoreAudio12stopCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd2fdc() -> ! {
    todo!("0xd2fdc __ZN4FMOD15OutputCoreAudio12stopCallbackEP17FMOD_OUTPUT_STATE")
}

#[doc(alias = "FMOD::OutputCoreAudio::start(void)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio5startEv")]
// 0xd2fe8 — __ZN4FMOD15OutputCoreAudio5startEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd2fe8() -> ! {
    todo!("0xd2fe8 __ZN4FMOD15OutputCoreAudio5startEv")
}

#[doc(alias = "FMOD::OutputCoreAudio::startCallback(FMOD_OUTPUT_STATE *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13startCallbackEP17FMOD_OUTPUT_STATE")]
// 0xd3064 — __ZN4FMOD15OutputCoreAudio13startCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd3064() -> ! {
    todo!("0xd3064 __ZN4FMOD15OutputCoreAudio13startCallbackEP17FMOD_OUTPUT_STATE")
}

#[doc(alias = "FMOD::OutputCoreAudio::shutdownAudio(void)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13shutdownAudioEv")]
// 0xd3070 — __ZN4FMOD15OutputCoreAudio13shutdownAudioEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd3070() -> ! {
    todo!("0xd3070 __ZN4FMOD15OutputCoreAudio13shutdownAudioEv")
}

#[doc(alias = "FMOD::OutputCoreAudio::close(void)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio5closeEv")]
// 0xd30c8 — __ZN4FMOD15OutputCoreAudio5closeEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd30c8() -> ! {
    todo!("0xd30c8 __ZN4FMOD15OutputCoreAudio5closeEv")
}

#[doc(alias = "FMOD::OutputCoreAudio::closeCallback(FMOD_OUTPUT_STATE *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio13closeCallbackEP17FMOD_OUTPUT_STATE")]
// 0xd3138 — __ZN4FMOD15OutputCoreAudio13closeCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd3138() -> ! {
    todo!("0xd3138 __ZN4FMOD15OutputCoreAudio13closeCallbackEP17FMOD_OUTPUT_STATE")
}

#[doc(alias = "FMOD::OutputCoreAudio::setupAudioSession(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio17setupAudioSessionEjj")]
// 0xd3144 — __ZN4FMOD15OutputCoreAudio17setupAudioSessionEjj
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int, unsigned int)
pub fn stub_0xd3144() -> ! {
    todo!("0xd3144 __ZN4FMOD15OutputCoreAudio17setupAudioSessionEjj")
}

#[doc(alias = "FMOD::OutputCoreAudio::reset(bool,bool)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio5resetEbb")]
// 0xd32ac — __ZN4FMOD15OutputCoreAudio5resetEbb
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, bool, bool)
pub fn stub_0xd32ac() -> ! {
    todo!("0xd32ac __ZN4FMOD15OutputCoreAudio5resetEbb")
}

#[doc(alias = "FMOD::OutputCoreAudio::handleInputAvailable(bool)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio20handleInputAvailableEb")]
// 0xd33c8 — __ZN4FMOD15OutputCoreAudio20handleInputAvailableEb
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, bool)
pub fn stub_0xd33c8() -> ! {
    todo!("0xd33c8 __ZN4FMOD15OutputCoreAudio20handleInputAvailableEb")
}

#[doc(alias = "FMOD::OutputCoreAudio::inputAvailableCallback(void *,unsigned long,unsigned long,void const*)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio22inputAvailableCallbackEPvmmPKv")]
// 0xd3408 — __ZN4FMOD15OutputCoreAudio22inputAvailableCallbackEPvmmPKv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *, unsigned int, unsigned int, const void *)
pub fn stub_0xd3408() -> ! {
    todo!("0xd3408 __ZN4FMOD15OutputCoreAudio22inputAvailableCallbackEPvmmPKv")
}

#[doc(alias = "FMOD::OutputCoreAudio::recordStop(FMOD::FMOD_RECORDING_INFO *)")]
#[doc(alias = "__ZN4FMOD15OutputCoreAudio10recordStopEPNS_19FMOD_RECORDING_INFOE")]
// 0xd3438 — __ZN4FMOD15OutputCoreAudio10recordStopEPNS_19FMOD_RECORDING_INFOE
pub fn stub_0xd3438() -> ! {
    todo!("0xd3438 __ZN4FMOD15OutputCoreAudio10recordStopEPNS_19FMOD_RECORDING_INFOE")
}

