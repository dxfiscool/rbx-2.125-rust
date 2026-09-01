//! core shard jw — 150 stubs EA-sorted 0x88a34..0xadb24 (global EA-sorted, next 150 not yet in core after jv 0x88818, rbx_core::SharedPtr not boost).
//! Source: `ida/export.json` (85545 funcs) global EA-sorted ascending, next 150 not yet in rbx_core (34309 before -> 34459 after, gap 51237->51087).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::CodecIT::decompress8(void **,void *,int,bool,int)")]
#[doc(alias = "__ZN4FMOD7CodecIT11decompress8EPPvS1_ibi")]
// 0x88a34 — __ZN4FMOD7CodecIT11decompress8EPPvS1_ibi
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **, _BYTE *, int, bool, int)
pub fn stub_88a34() -> ! {
    todo!("0x88a34 __ZN4FMOD7CodecIT11decompress8EPPvS1_ibi")
}

#[doc(alias = "FMOD::CodecIT::play(bool)")]
#[doc(alias = "__ZN4FMOD7CodecIT4playEb")]
// 0x88c44 — __ZN4FMOD7CodecIT4playEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
pub fn stub_88c44() -> ! {
    todo!("0x88c44 __ZN4FMOD7CodecIT4playEb")
}

#[doc(alias = "FMOD::CodecIT::updateRow(bool)")]
#[doc(alias = "__ZN4FMOD7CodecIT9updateRowEb")]
// 0x88ccc — __ZN4FMOD7CodecIT9updateRowEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
pub fn stub_88ccc() -> ! {
    todo!("0x88ccc __ZN4FMOD7CodecIT9updateRowEb")
}

#[doc(alias = "FMOD::CodecIT::update(bool)")]
#[doc(alias = "__ZN4FMOD7CodecIT6updateEb")]
// 0x8b660 — __ZN4FMOD7CodecIT6updateEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
pub fn stub_8b660() -> ! {
    todo!("0x8b660 __ZN4FMOD7CodecIT6updateEb")
}

#[doc(alias = "FMOD::CodecIT::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD7CodecIT19setPositionInternalEijj")]
// 0x8b854 — __ZN4FMOD7CodecIT19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecIT *this, int, unsigned int, unsigned int)
pub fn stub_8b854() -> ! {
    todo!("0x8b854 __ZN4FMOD7CodecIT19setPositionInternalEijj")
}

#[doc(alias = "FMOD::CodecIT::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD7CodecIT19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x8b908 — __ZN4FMOD7CodecIT19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecIT *, int, unsigned int, unsigned int)
pub fn stub_8b908() -> ! {
    todo!("0x8b908 __ZN4FMOD7CodecIT19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecIT::calculateLength(void)")]
#[doc(alias = "__ZN4FMOD7CodecIT15calculateLengthEv")]
// 0x8b914 — __ZN4FMOD7CodecIT15calculateLengthEv
// type: int __fastcall(FMOD::CodecIT *this)
pub fn stub_8b914() -> ! {
    todo!("0x8b914 __ZN4FMOD7CodecIT15calculateLengthEv")
}

#[doc(alias = "FMOD::CodecIT::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD7CodecIT12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x8b978 — __ZN4FMOD7CodecIT12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
pub fn stub_8b978() -> ! {
    todo!("0x8b978 __ZN4FMOD7CodecIT12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecIT::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD7CodecIT12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x8e7bc — __ZN4FMOD7CodecIT12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
pub fn stub_8e7bc() -> ! {
    todo!("0x8e7bc __ZN4FMOD7CodecIT12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecIT::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7CodecIT12readInternalEPvjPj")]
// 0x8e7c8 — __ZN4FMOD7CodecIT12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *this, char *, unsigned int, unsigned int *)
pub fn stub_8e7c8() -> ! {
    todo!("0x8e7c8 __ZN4FMOD7CodecIT12readInternalEPvjPj")
}

#[doc(alias = "FMOD::CodecIT::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7CodecIT12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x8ebc0 — __ZN4FMOD7CodecIT12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *, char *, unsigned int, unsigned int *)
pub fn stub_8ebc0() -> ! {
    todo!("0x8ebc0 __ZN4FMOD7CodecIT12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "global constructor keyed toFMOD::itcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD7itcodecE")]
// 0x8ec18 — __GLOBAL__I__ZN4FMOD7itcodecE
// type: int()
pub fn stub_8ec18() -> ! {
    todo!("0x8ec18 __GLOBAL__I__ZN4FMOD7itcodecE")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::findArticulator(int,int)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel15findArticulatorEii")]
// 0x8ec24 — __ZN4FMOD19CodecMIDISubChannel15findArticulatorEii
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int, int)
pub fn stub_8ec24() -> ! {
    todo!("0x8ec24 __ZN4FMOD19CodecMIDISubChannel15findArticulatorEii")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::articulateDest(FMOD::CONN_SRC_FLAGS,int,int *)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel14articulateDestENS_14CONN_SRC_FLAGSEiPi")]
// 0x8ec8c — __ZN4FMOD19CodecMIDISubChannel14articulateDestENS_14CONN_SRC_FLAGSEiPi
// type: int __fastcall(int, __int16, int, _DWORD *)
pub fn stub_8ec8c() -> ! {
    todo!("0x8ec8c __ZN4FMOD19CodecMIDISubChannel14articulateDestENS_14CONN_SRC_FLAGSEiPi")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::getTimeCentsFromlScale(int)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel22getTimeCentsFromlScaleEi")]
// 0x8ef90 — __ZN4FMOD19CodecMIDISubChannel22getTimeCentsFromlScaleEi
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int)
pub fn stub_8ef90() -> ! {
    todo!("0x8ef90 __ZN4FMOD19CodecMIDISubChannel22getTimeCentsFromlScaleEi")
}

#[doc(alias = "FMOD::CodecMIDIChannel::getSound(int,FMOD::SoundI **,FMOD::CodecDLSInstrument **,int *,int *,int *,bool *,int *,int *,FMOD::DLS_CONNECTIONBLOCK **)")]
#[doc(alias = "__ZN4FMOD16CodecMIDIChannel8getSoundEiPPNS_6SoundIEPPNS_18CodecDLSInstrumentEPiS7_S7_PbS7_S7_PPNS_19DLS_CONNECTIONBLOCKE")]
// 0x8f00c — __ZN4FMOD16CodecMIDIChannel8getSoundEiPPNS_6SoundIEPPNS_18CodecDLSInstrumentEPiS7_S7_PbS7_S7_PPNS_19DLS_CONNECTIONBLOCKE
// type: int __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *, int, _DWORD *, _DWORD *, _DWORD *)
pub fn stub_8f00c() -> ! {
    todo!("0x8f00c __ZN4FMOD16CodecMIDIChannel8getSoundEiPPNS_6SoundIEPPNS_18CodecDLSInstrumentEPiS7_S7_PbS7_S7_PPNS_19DLS_CONNECTIONBLOCKE")
}

#[doc(alias = "FMOD::CodecMIDITrack::readVarLen(unsigned int *)")]
#[doc(alias = "__ZN4FMOD14CodecMIDITrack10readVarLenEPj")]
// 0x8f274 — __ZN4FMOD14CodecMIDITrack10readVarLenEPj
// type: int __fastcall(FMOD::CodecMIDITrack *this, unsigned int *)
pub fn stub_8f274() -> ! {
    todo!("0x8f274 __ZN4FMOD14CodecMIDITrack10readVarLenEPj")
}

#[doc(alias = "FMOD::CodecMIDITrack::readByte(unsigned char *)")]
#[doc(alias = "__ZN4FMOD14CodecMIDITrack8readByteEPh")]
// 0x8f2ec — __ZN4FMOD14CodecMIDITrack8readByteEPh
// type: int __fastcall(int this, unsigned __int8 *)
pub fn stub_8f2ec() -> ! {
    todo!("0x8f2ec __ZN4FMOD14CodecMIDITrack8readByteEPh")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsInternal(int *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI27getMusicNumChannelsInternalEPi")]
// 0x8f320 — __ZN4FMOD9CodecMIDI27getMusicNumChannelsInternalEPi
// type: int __fastcall(FMOD::CodecMIDI *this, int *)
pub fn stub_8f320() -> ! {
    todo!("0x8f320 __ZN4FMOD9CodecMIDI27getMusicNumChannelsInternalEPi")
}

#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI29setMusicChannelVolumeInternalEif")]
// 0x8f35c — __ZN4FMOD9CodecMIDI29setMusicChannelVolumeInternalEif
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float)
pub fn stub_8f35c() -> ! {
    todo!("0x8f35c __ZN4FMOD9CodecMIDI29setMusicChannelVolumeInternalEif")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeInternal(int,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI29getMusicChannelVolumeInternalEiPf")]
// 0x8f3fc — __ZN4FMOD9CodecMIDI29getMusicChannelVolumeInternalEiPf
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float *)
pub fn stub_8f3fc() -> ! {
    todo!("0x8f3fc __ZN4FMOD9CodecMIDI29getMusicChannelVolumeInternalEiPf")
}

#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedInternal(float)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI21setMusicSpeedInternalEf")]
// 0x8f488 — __ZN4FMOD9CodecMIDI21setMusicSpeedInternalEf
// type: int __fastcall(FMOD::CodecMIDI *this, float)
pub fn stub_8f488() -> ! {
    todo!("0x8f488 __ZN4FMOD9CodecMIDI21setMusicSpeedInternalEf")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedInternal(float *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI21getMusicSpeedInternalEPf")]
// 0x8f528 — __ZN4FMOD9CodecMIDI21getMusicSpeedInternalEPf
// type: int __fastcall(FMOD::CodecMIDI *this, float *)
pub fn stub_8f528() -> ! {
    todo!("0x8f528 __ZN4FMOD9CodecMIDI21getMusicSpeedInternalEPf")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi")]
// 0x8f540 — __ZN4FMOD9CodecMIDI27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi
// type: int __fastcall(FMOD::CodecMIDI *, int *)
pub fn stub_8f540() -> ! {
    todo!("0x8f540 __ZN4FMOD9CodecMIDI27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi")
}

#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif")]
// 0x8f54c — __ZN4FMOD9CodecMIDI29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float)
pub fn stub_8f54c() -> ! {
    todo!("0x8f54c __ZN4FMOD9CodecMIDI29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf")]
// 0x8f558 — __ZN4FMOD9CodecMIDI29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float *)
pub fn stub_8f558() -> ! {
    todo!("0x8f558 __ZN4FMOD9CodecMIDI29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf")
}

#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf")]
// 0x8f564 — __ZN4FMOD9CodecMIDI21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf
// type: int __fastcall(FMOD::CodecMIDI *, float)
pub fn stub_8f564() -> ! {
    todo!("0x8f564 __ZN4FMOD9CodecMIDI21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf")
}

#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf")]
// 0x8f570 — __ZN4FMOD9CodecMIDI21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf
// type: int __fastcall(FMOD::CodecMIDI *, float *)
pub fn stub_8f570() -> ! {
    todo!("0x8f570 __ZN4FMOD9CodecMIDI21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf")
}

#[doc(alias = "FMOD::CodecMIDI::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI16getDescriptionExEv")]
// 0x8f57c — __ZN4FMOD9CodecMIDI16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMIDI *this)
pub fn stub_8f57c() -> ! {
    todo!("0x8f57c __ZN4FMOD9CodecMIDI16getDescriptionExEv")
}

#[doc(alias = "FMOD::CodecMIDI::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI13closeInternalEv")]
// 0x8f674 — __ZN4FMOD9CodecMIDI13closeInternalEv
// type: int __fastcall(FMOD::CodecMIDI *this)
pub fn stub_8f674() -> ! {
    todo!("0x8f674 __ZN4FMOD9CodecMIDI13closeInternalEv")
}

#[doc(alias = "FMOD::CodecMIDI::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x8f8d0 — __ZN4FMOD9CodecMIDI13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMIDI *)
pub fn stub_8f8d0() -> ! {
    todo!("0x8f8d0 __ZN4FMOD9CodecMIDI13closeCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::CodecMIDITrack::read(void *,int)")]
#[doc(alias = "__ZN4FMOD14CodecMIDITrack4readEPvi")]
// 0x8f8dc — __ZN4FMOD14CodecMIDITrack4readEPvi
// type: int __fastcall(FMOD::CodecMIDITrack *this, void *, size_t)
pub fn stub_8f8dc() -> ! {
    todo!("0x8f8dc __ZN4FMOD14CodecMIDITrack4readEPvi")
}

#[doc(alias = "FMOD::CodecMIDITrack::addTag(char const*,int,bool)")]
#[doc(alias = "__ZN4FMOD14CodecMIDITrack6addTagEPKcib")]
// 0x8f944 — __ZN4FMOD14CodecMIDITrack6addTagEPKcib
// type: int __fastcall(FMOD::CodecMIDITrack *this, const char *, size_t, bool)
pub fn stub_8f944() -> ! {
    todo!("0x8f944 __ZN4FMOD14CodecMIDITrack6addTagEPKcib")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::setUpArticulators(void)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel17setUpArticulatorsEv")]
// 0x8fa30 — __ZN4FMOD19CodecMIDISubChannel17setUpArticulatorsEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
pub fn stub_8fa30() -> ! {
    todo!("0x8fa30 __ZN4FMOD19CodecMIDISubChannel17setUpArticulatorsEv")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::updatePan(void)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel9updatePanEv")]
// 0x8ff60 — __ZN4FMOD19CodecMIDISubChannel9updatePanEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
pub fn stub_8ff60() -> ! {
    todo!("0x8ff60 __ZN4FMOD19CodecMIDISubChannel9updatePanEv")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::updatePitch(void)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel11updatePitchEv")]
// 0x8ffa4 — __ZN4FMOD19CodecMIDISubChannel11updatePitchEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
pub fn stub_8ffa4() -> ! {
    todo!("0x8ffa4 __ZN4FMOD19CodecMIDISubChannel11updatePitchEv")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::stop(void)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel4stopEv")]
// 0x9034c — __ZN4FMOD19CodecMIDISubChannel4stopEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
pub fn stub_9034c() -> ! {
    todo!("0x9034c __ZN4FMOD19CodecMIDISubChannel4stopEv")
}

#[doc(alias = "FMOD::CodecMIDI::play(bool)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI4playEb")]
// 0x903bc — __ZN4FMOD9CodecMIDI4playEb
// type: int __fastcall(FMOD::CodecMIDI *this, bool)
pub fn stub_903bc() -> ! {
    todo!("0x903bc __ZN4FMOD9CodecMIDI4playEb")
}

#[doc(alias = "FMOD::CodecMIDISubChannel::updateVolume(void)")]
#[doc(alias = "__ZN4FMOD19CodecMIDISubChannel12updateVolumeEv")]
// 0x90584 — __ZN4FMOD19CodecMIDISubChannel12updateVolumeEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
pub fn stub_90584() -> ! {
    todo!("0x90584 __ZN4FMOD19CodecMIDISubChannel12updateVolumeEv")
}

#[doc(alias = "FMOD::CodecMIDIChannel::update(void)")]
#[doc(alias = "__ZN4FMOD16CodecMIDIChannel6updateEv")]
// 0x90984 — __ZN4FMOD16CodecMIDIChannel6updateEv
// type: int __fastcall(FMOD::CodecMIDIChannel *this)
pub fn stub_90984() -> ! {
    todo!("0x90984 __ZN4FMOD16CodecMIDIChannel6updateEv")
}

#[doc(alias = "FMOD::CodecMIDIChannel::process(unsigned char,bool,unsigned char,bool)")]
#[doc(alias = "__ZN4FMOD16CodecMIDIChannel7processEhbhb")]
// 0x90a44 — __ZN4FMOD16CodecMIDIChannel7processEhbhb
// type: int __fastcall(FMOD::CodecMIDIChannel *this, unsigned __int8, bool, unsigned __int8, bool)
pub fn stub_90a44() -> ! {
    todo!("0x90a44 __ZN4FMOD16CodecMIDIChannel7processEhbhb")
}

#[doc(alias = "FMOD::CodecMIDITrack::process(bool)")]
#[doc(alias = "__ZN4FMOD14CodecMIDITrack7processEb")]
// 0x91454 — __ZN4FMOD14CodecMIDITrack7processEb
// type: int __fastcall(FMOD::CodecMIDITrack *this, bool)
pub fn stub_91454() -> ! {
    todo!("0x91454 __ZN4FMOD14CodecMIDITrack7processEb")
}

#[doc(alias = "FMOD::CodecMIDI::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x91d30 — __ZN4FMOD9CodecMIDI12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
pub fn stub_91d30() -> ! {
    todo!("0x91d30 __ZN4FMOD9CodecMIDI12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecMIDI::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x92a68 — __ZN4FMOD9CodecMIDI12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, char, _DWORD *)
pub fn stub_92a68() -> ! {
    todo!("0x92a68 __ZN4FMOD9CodecMIDI12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecMIDI::update(bool)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI6updateEb")]
// 0x92a74 — __ZN4FMOD9CodecMIDI6updateEb
// type: __int64 __fastcall(FMOD::CodecMIDI *this, bool)
pub fn stub_92a74() -> ! {
    todo!("0x92a74 __ZN4FMOD9CodecMIDI6updateEb")
}

#[doc(alias = "FMOD::CodecMIDI::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI19setPositionInternalEijj")]
// 0x92b38 — __ZN4FMOD9CodecMIDI19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMIDI *this, int, unsigned int, unsigned int)
pub fn stub_92b38() -> ! {
    todo!("0x92b38 __ZN4FMOD9CodecMIDI19setPositionInternalEijj")
}

#[doc(alias = "FMOD::CodecMIDI::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x92b94 — __ZN4FMOD9CodecMIDI19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMIDI *, int, unsigned int, unsigned int)
pub fn stub_92b94() -> ! {
    todo!("0x92b94 __ZN4FMOD9CodecMIDI19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecMIDI::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI12readInternalEPvjPj")]
// 0x92ba0 — __ZN4FMOD9CodecMIDI12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *this, char *, size_t, unsigned int *)
pub fn stub_92ba0() -> ! {
    todo!("0x92ba0 __ZN4FMOD9CodecMIDI12readInternalEPvjPj")
}

#[doc(alias = "FMOD::CodecMIDI::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMIDI12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x92fac — __ZN4FMOD9CodecMIDI12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMIDI *, char *, size_t, unsigned int *)
pub fn stub_92fac() -> ! {
    todo!("0x92fac __ZN4FMOD9CodecMIDI12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "global constructor keyed toFMOD::midicodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9midicodecE")]
// 0x9301c — __GLOBAL__I__ZN4FMOD9midicodecE
// type: int()
pub fn stub_9301c() -> ! {
    todo!("0x9301c __GLOBAL__I__ZN4FMOD9midicodecE")
}

#[doc(alias = "FMOD::MusicChannelMOD::portamento(void)")]
#[doc(alias = "__ZN4FMOD15MusicChannelMOD10portamentoEv")]
// 0x93028 — __ZN4FMOD15MusicChannelMOD10portamentoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
pub fn stub_93028() -> ! {
    todo!("0x93028 __ZN4FMOD15MusicChannelMOD10portamentoEv")
}

#[doc(alias = "FMOD::MusicChannelMOD::vibrato(void)")]
#[doc(alias = "__ZN4FMOD15MusicChannelMOD7vibratoEv")]
// 0x93098 — __ZN4FMOD15MusicChannelMOD7vibratoEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
pub fn stub_93098() -> ! {
    todo!("0x93098 __ZN4FMOD15MusicChannelMOD7vibratoEv")
}

#[doc(alias = "FMOD::MusicChannelMOD::tremolo(void)")]
#[doc(alias = "__ZN4FMOD15MusicChannelMOD7tremoloEv")]
// 0x931dc — __ZN4FMOD15MusicChannelMOD7tremoloEv
// type: int __fastcall(FMOD::MusicChannelMOD *this)
pub fn stub_931dc() -> ! {
    todo!("0x931dc __ZN4FMOD15MusicChannelMOD7tremoloEv")
}

#[doc(alias = "FMOD::CodecMOD::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD8CodecMOD13closeInternalEv")]
// 0x93310 — __ZN4FMOD8CodecMOD13closeInternalEv
// type: int __fastcall(FMOD::CodecMOD *this)
pub fn stub_93310() -> ! {
    todo!("0x93310 __ZN4FMOD8CodecMOD13closeInternalEv")
}

#[doc(alias = "FMOD::CodecMOD::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD8CodecMOD13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x935b8 — __ZN4FMOD8CodecMOD13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMOD *)
pub fn stub_935b8() -> ! {
    todo!("0x935b8 __ZN4FMOD8CodecMOD13closeCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::CodecMOD::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD8CodecMOD16getDescriptionExEv")]
// 0x935c4 — __ZN4FMOD8CodecMOD16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMOD *this)
pub fn stub_935c4() -> ! {
    todo!("0x935c4 __ZN4FMOD8CodecMOD16getDescriptionExEv")
}

#[doc(alias = "FMOD::CodecMOD::updateEffects(void)")]
#[doc(alias = "__ZN4FMOD8CodecMOD13updateEffectsEv")]
// 0x936dc — __ZN4FMOD8CodecMOD13updateEffectsEv
// type: int __fastcall(FMOD::CodecMOD *this)
pub fn stub_936dc() -> ! {
    todo!("0x936dc __ZN4FMOD8CodecMOD13updateEffectsEv")
}

#[doc(alias = "FMOD::CodecMOD::updateNote(bool)")]
#[doc(alias = "__ZN4FMOD8CodecMOD10updateNoteEb")]
// 0x93de4 — __ZN4FMOD8CodecMOD10updateNoteEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
pub fn stub_93de4() -> ! {
    todo!("0x93de4 __ZN4FMOD8CodecMOD10updateNoteEb")
}

#[doc(alias = "FMOD::CodecMOD::update(bool)")]
#[doc(alias = "__ZN4FMOD8CodecMOD6updateEb")]
// 0x94674 — __ZN4FMOD8CodecMOD6updateEb
// type: int __fastcall(FMOD::CodecMOD *this, bool)
pub fn stub_94674() -> ! {
    todo!("0x94674 __ZN4FMOD8CodecMOD6updateEb")
}

#[doc(alias = "FMOD::CodecMOD::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecMOD19setPositionInternalEijj")]
// 0x94790 — __ZN4FMOD8CodecMOD19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecMOD *this, int, unsigned int, unsigned int)
pub fn stub_94790() -> ! {
    todo!("0x94790 __ZN4FMOD8CodecMOD19setPositionInternalEijj")
}

#[doc(alias = "FMOD::CodecMOD::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecMOD19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x94844 — __ZN4FMOD8CodecMOD19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecMOD *, int, unsigned int, unsigned int)
pub fn stub_94844() -> ! {
    todo!("0x94844 __ZN4FMOD8CodecMOD19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecMOD::calculateLength(void)")]
#[doc(alias = "__ZN4FMOD8CodecMOD15calculateLengthEv")]
// 0x94850 — __ZN4FMOD8CodecMOD15calculateLengthEv
// type: int __fastcall(FMOD::CodecMOD *this)
pub fn stub_94850() -> ! {
    todo!("0x94850 __ZN4FMOD8CodecMOD15calculateLengthEv")
}

#[doc(alias = "FMOD::CodecMOD::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecMOD12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x948b4 — __ZN4FMOD8CodecMOD12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_948b4() -> ! {
    todo!("0x948b4 __ZN4FMOD8CodecMOD12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecMOD::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecMOD12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x95a74 — __ZN4FMOD8CodecMOD12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_95a74() -> ! {
    todo!("0x95a74 __ZN4FMOD8CodecMOD12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecMOD::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecMOD12readInternalEPvjPj")]
// 0x95a80 — __ZN4FMOD8CodecMOD12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *this, char *, unsigned int, unsigned int *)
pub fn stub_95a80() -> ! {
    todo!("0x95a80 __ZN4FMOD8CodecMOD12readInternalEPvjPj")
}

#[doc(alias = "FMOD::CodecMOD::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecMOD12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x95e64 — __ZN4FMOD8CodecMOD12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecMOD *, char *, unsigned int, unsigned int *)
pub fn stub_95e64() -> ! {
    todo!("0x95e64 __ZN4FMOD8CodecMOD12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "global constructor keyed toFMOD::modcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD8modcodecE")]
// 0x95ebc — __GLOBAL__I__ZN4FMOD8modcodecE
// type: int()
pub fn stub_95ebc() -> ! {
    todo!("0x95ebc __GLOBAL__I__ZN4FMOD8modcodecE")
}

#[doc(alias = "FMOD::CodecMPEG::resetCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG13resetCallbackEP16FMOD_CODEC_STATE")]
// 0x95ec8 — __ZN4FMOD9CodecMPEG13resetCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
pub fn stub_95ec8() -> ! {
    todo!("0x95ec8 __ZN4FMOD9CodecMPEG13resetCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::CodecMPEG::soundCreateInternal(int,FMOD_SOUND *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19soundCreateInternalEiP10FMOD_SOUND")]
// 0x95ee0 — __ZN4FMOD9CodecMPEG19soundCreateInternalEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *this)
pub fn stub_95ee0() -> ! {
    todo!("0x95ee0 __ZN4FMOD9CodecMPEG19soundCreateInternalEiP10FMOD_SOUND")
}

#[doc(alias = "FMOD::CodecMPEG::soundCreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND")]
// 0x95fe8 — __ZN4FMOD9CodecMPEG19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *)
pub fn stub_95fe8() -> ! {
    todo!("0x95fe8 __ZN4FMOD9CodecMPEG19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND")
}

#[doc(alias = "FMOD::CodecMPEG::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG13closeInternalEv")]
// 0x95ff4 — __ZN4FMOD9CodecMPEG13closeInternalEv
// type: int __fastcall(FMOD::CodecMPEG *this)
pub fn stub_95ff4() -> ! {
    todo!("0x95ff4 __ZN4FMOD9CodecMPEG13closeInternalEv")
}

#[doc(alias = "FMOD::CodecMPEG::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x96114 — __ZN4FMOD9CodecMPEG13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMPEG *)
pub fn stub_96114() -> ! {
    todo!("0x96114 __ZN4FMOD9CodecMPEG13closeCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::CodecMPEG::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19setPositionInternalEijj")]
// 0x96120 — __ZN4FMOD9CodecMPEG19setPositionInternalEijj
// type: int __fastcall(FMOD::File **this, int, unsigned int, unsigned int)
pub fn stub_96120() -> ! {
    todo!("0x96120 __ZN4FMOD9CodecMPEG19setPositionInternalEijj")
}

#[doc(alias = "FMOD::CodecMPEG::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x964d8 — __ZN4FMOD9CodecMPEG19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::File **, int, unsigned int, unsigned int)
pub fn stub_964d8() -> ! {
    todo!("0x964d8 __ZN4FMOD9CodecMPEG19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecMPEG::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG16getDescriptionExEv")]
// 0x964e4 — __ZN4FMOD9CodecMPEG16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMPEG *this)
pub fn stub_964e4() -> ! {
    todo!("0x964e4 __ZN4FMOD9CodecMPEG16getDescriptionExEv")
}

#[doc(alias = "FMOD::CodecMPEG::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12readInternalEPvjPj")]
// 0x965a4 — __ZN4FMOD9CodecMPEG12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *this, char *, unsigned int, unsigned int *)
pub fn stub_965a4() -> ! {
    todo!("0x965a4 __ZN4FMOD9CodecMPEG12readInternalEPvjPj")
}

#[doc(alias = "FMOD::CodecMPEG::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x96854 — __ZN4FMOD9CodecMPEG12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecMPEG *, char *, unsigned int, unsigned int *)
pub fn stub_96854() -> ! {
    todo!("0x96854 __ZN4FMOD9CodecMPEG12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "FMOD::CodecMPEG::getPCMLength(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12getPCMLengthEv")]
// 0x96860 — __ZN4FMOD9CodecMPEG12getPCMLengthEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_96860() -> ! {
    todo!("0x96860 __ZN4FMOD9CodecMPEG12getPCMLengthEv")
}

#[doc(alias = "FMOD::CodecMPEG::makeTables(int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10makeTablesEi")]
// 0x96a24 — __ZN4FMOD9CodecMPEG10makeTablesEi
// type: int __fastcall(int this, int)
pub fn stub_96a24() -> ! {
    todo!("0x96a24 __ZN4FMOD9CodecMPEG10makeTablesEi")
}

#[doc(alias = "FMOD::CodecMPEG::initAll(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG7initAllEv")]
// 0x96c4c — __ZN4FMOD9CodecMPEG7initAllEv
// type: int __fastcall(FMOD::CodecMPEG *this, int)
pub fn stub_96c4c() -> ! {
    todo!("0x96c4c __ZN4FMOD9CodecMPEG7initAllEv")
}

#[doc(alias = "FMOD::CodecMPEG::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x96c9c — __ZN4FMOD9CodecMPEG12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
pub fn stub_96c9c() -> ! {
    todo!("0x96c9c __ZN4FMOD9CodecMPEG12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecMPEG::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x97670 — __ZN4FMOD9CodecMPEG12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
pub fn stub_97670() -> ! {
    todo!("0x97670 __ZN4FMOD9CodecMPEG12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "global constructor keyed toFMOD::mpegcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9mpegcodecE")]
// 0x976c8 — __GLOBAL__I__ZN4FMOD9mpegcodecE
// type: int()
pub fn stub_976c8() -> ! {
    todo!("0x976c8 __GLOBAL__I__ZN4FMOD9mpegcodecE")
}

#[doc(alias = "FMOD::CodecMPEG::getBits(int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG7getBitsEi")]
// 0x976d4 — __ZN4FMOD9CodecMPEG7getBitsEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
pub fn stub_976d4() -> ! {
    todo!("0x976d4 __ZN4FMOD9CodecMPEG7getBitsEi")
}

#[doc(alias = "FMOD::CodecMPEG::getBitsFast(int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG11getBitsFastEi")]
// 0x97758 — __ZN4FMOD9CodecMPEG11getBitsFastEi
// type: unsigned int __fastcall(FMOD::CodecMPEG *this, int)
pub fn stub_97758() -> ! {
    todo!("0x97758 __ZN4FMOD9CodecMPEG11getBitsFastEi")
}

#[doc(alias = "FMOD::CodecMPEG::dct64(float *,float *,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG5dct64EPfS1_S1_")]
// 0x977c0 — __ZN4FMOD9CodecMPEG5dct64EPfS1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *)
pub fn stub_977c0() -> ! {
    todo!("0x977c0 __ZN4FMOD9CodecMPEG5dct64EPfS1_S1_")
}

#[doc(alias = "FMOD::CodecMPEG::synthC(float *,int,int,short *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG6synthCEPfiiPs")]
// 0x981d4 — __ZN4FMOD9CodecMPEG6synthCEPfiiPs
// type: int __fastcall(FMOD::CodecMPEG *this, float *, int, int, __int16 *)
pub fn stub_981d4() -> ! {
    todo!("0x981d4 __ZN4FMOD9CodecMPEG6synthCEPfiiPs")
}

#[doc(alias = "FMOD::CodecMPEG::synth(void *,float *,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG5synthEPvPfii")]
// 0x9854c — __ZN4FMOD9CodecMPEG5synthEPvPfii
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, float *, int, int)
pub fn stub_9854c() -> ! {
    todo!("0x9854c __ZN4FMOD9CodecMPEG5synthEPvPfii")
}

#[doc(alias = "FMOD::CodecMPEG::resetFrame(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10resetFrameEv")]
// 0x986f8 — __ZN4FMOD9CodecMPEG10resetFrameEv
// type: int __fastcall(FMOD::CodecMPEG *this)
pub fn stub_986f8() -> ! {
    todo!("0x986f8 __ZN4FMOD9CodecMPEG10resetFrameEv")
}

#[doc(alias = "FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj")]
// 0x987e4 — __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, unsigned __int8 *, unsigned int *)
pub fn stub_987e4() -> ! {
    todo!("0x987e4 __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj")
}

#[doc(alias = "FMOD::CodecMPEG::decodeHeader(void *,int *,int *,int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_")]
// 0x9891c — __ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, int *, int *, int *)
pub fn stub_9891c() -> ! {
    todo!("0x9891c __ZN4FMOD9CodecMPEG12decodeHeaderEPvPiS2_S2_")
}

#[doc(alias = "FMOD::CodecMPEG::decodeFrame(unsigned char *,void *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj")]
// 0x98e9c — __ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, void *, unsigned int *)
pub fn stub_98e9c() -> ! {
    todo!("0x98e9c __ZN4FMOD9CodecMPEG11decodeFrameEPhPvPj")
}

#[doc(alias = "FMOD::CodecMPEG::getIIStuff(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10getIIStuffEv")]
// 0x99024 — __ZN4FMOD9CodecMPEG10getIIStuffEv
// type: int __fastcall(FMOD::CodecMPEG *this)
pub fn stub_99024() -> ! {
    todo!("0x99024 __ZN4FMOD9CodecMPEG10getIIStuffEv")
}

#[doc(alias = "FMOD::CodecMPEG::II_step_two(unsigned int *,float (*)[4][32],int *,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii")]
// 0x99118 — __ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, float (*)[4][32], int *, int)
pub fn stub_99118() -> ! {
    todo!("0x99118 __ZN4FMOD9CodecMPEG11II_step_twoEPjPA4_A32_fPii")
}

#[doc(alias = "FMOD::CodecMPEG::II_step_one(unsigned int *,int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG11II_step_oneEPjPi")]
// 0x99728 — __ZN4FMOD9CodecMPEG11II_step_oneEPjPi
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, unsigned int *)
pub fn stub_99728() -> ! {
    todo!("0x99728 __ZN4FMOD9CodecMPEG11II_step_oneEPjPi")
}

#[doc(alias = "FMOD::CodecMPEG::decodeLayer2(void *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12decodeLayer2EPvPj")]
// 0x99a10 — __ZN4FMOD9CodecMPEG12decodeLayer2EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
pub fn stub_99a10() -> ! {
    todo!("0x99a10 __ZN4FMOD9CodecMPEG12decodeLayer2EPvPj")
}

#[doc(alias = "FMOD::CodecMPEG::initLayer2(void)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10initLayer2Ev")]
// 0x99b08 — __ZN4FMOD9CodecMPEG10initLayer2Ev
// type: int __fastcall(FMOD::CodecMPEG *this)
pub fn stub_99b08() -> ! {
    todo!("0x99b08 __ZN4FMOD9CodecMPEG10initLayer2Ev")
}

#[doc(alias = "FMOD::CodecMPEG::III_i_stereo(float (*)[32][18],int *,FMOD::gr_info_s *,int,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii")]
// 0x99d7c — __ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii
// type: int __fastcall(int, int, int, _DWORD *, int, int, int)
pub fn stub_99d7c() -> ! {
    todo!("0x99d7c __ZN4FMOD9CodecMPEG12III_i_stereoEPA32_A18_fPiPNS_9gr_info_sEiii")
}

#[doc(alias = "FMOD::CodecMPEG::III_antialias(float (*)[18],FMOD::gr_info_s *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE")]
// 0x9a240 — __ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_9a240() -> ! {
    todo!("0x9a240 __ZN4FMOD9CodecMPEG13III_antialiasEPA18_fPNS_9gr_info_sE")
}

#[doc(alias = "FMOD::CodecMPEG::dct36(float *,float *,float *,float *,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_")]
// 0x9a308 — __ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_
// type: float *__fastcall(FMOD::CodecMPEG *this, float *, float *, float *, float *, float *)
pub fn stub_9a308() -> ! {
    todo!("0x9a308 __ZN4FMOD9CodecMPEG5dct36EPfS1_S1_S1_S1_")
}

#[doc(alias = "FMOD::CodecMPEG::dct12(float *,float *,float *,float *,float *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_")]
// 0x9a9e8 — __ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_
// type: __int32 *__fastcall(__int32 *this, float *, float *, float *, float *, float *)
pub fn stub_9a9e8() -> ! {
    todo!("0x9a9e8 __ZN4FMOD9CodecMPEG5dct12EPfS1_S1_S1_S1_")
}

#[doc(alias = "FMOD::CodecMPEG::III_hybrid(float (*)[18],float (*)[32],int,FMOD::gr_info_s *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE")]
// 0x9af14 — __ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE
// type: int __fastcall(int, int, float *, int, _DWORD *)
pub fn stub_9af14() -> ! {
    todo!("0x9af14 __ZN4FMOD9CodecMPEG10III_hybridEPA18_fPA32_fiPNS_9gr_info_sE")
}

#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample_ms(float (*)[32][18],int *,FMOD::gr_info_s *,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii")]
// 0x9b1f8 — __ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *this, _DWORD *, int *, _DWORD *, int, int)
pub fn stub_9b1f8() -> ! {
    todo!("0x9b1f8 __ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii")
}

#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample(float (*)[18],int *,FMOD::gr_info_s *,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii")]
// 0x9c668 — __ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *, _DWORD *, int *, _DWORD *, int, int)
pub fn stub_9c668() -> ! {
    todo!("0x9c668 __ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii")
}

#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_2(int *,FMOD::gr_info_s *,int,int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_")]
// 0x9d78c — __ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, _DWORD *, int, _DWORD *)
pub fn stub_9d78c() -> ! {
    todo!("0x9d78c __ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_")
}

#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_1(int *,FMOD::gr_info_s *,int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_")]
// 0x9d920 — __ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, int *, _DWORD *)
pub fn stub_9d920() -> ! {
    todo!("0x9d920 __ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_")
}

#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_2(FMOD::III_sideinfo *,int,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii")]
// 0x9dcbc — __ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
pub fn stub_9dcbc() -> ! {
    todo!("0x9dcbc __ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii")
}

#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_1(FMOD::III_sideinfo *,int,int,int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii")]
// 0x9e0e0 — __ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
pub fn stub_9e0e0() -> ! {
    todo!("0x9e0e0 __ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii")
}

#[doc(alias = "FMOD::CodecMPEG::decodeLayer3(void *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG12decodeLayer3EPvPj")]
// 0x9e5ac — __ZN4FMOD9CodecMPEG12decodeLayer3EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
pub fn stub_9e5ac() -> ! {
    todo!("0x9e5ac __ZN4FMOD9CodecMPEG12decodeLayer3EPvPj")
}

#[doc(alias = "FMOD::CodecMPEG::initLayer3(int)")]
#[doc(alias = "__ZN4FMOD9CodecMPEG10initLayer3Ei")]
// 0x9eb14 — __ZN4FMOD9CodecMPEG10initLayer3Ei
// type: int __fastcall(FMOD::CodecMPEG *this, int)
pub fn stub_9eb14() -> ! {
    todo!("0x9eb14 __ZN4FMOD9CodecMPEG10initLayer3Ei")
}

#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x9fa10 — __ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
pub fn stub_9fa10() -> ! {
    todo!("0x9fa10 __ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE")]
// 0x9fa34 — __ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
pub fn stub_9fa34() -> ! {
    todo!("0x9fa34 __ZN4FMOD14CodecOggVorbis21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::CodecOggVorbis::readVorbisComments(void)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv")]
// 0x9fa8c — __ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
pub fn stub_9fa8c() -> ! {
    todo!("0x9fa8c __ZN4FMOD14CodecOggVorbis18readVorbisCommentsEv")
}

#[doc(alias = "FMOD::CodecOggVorbis::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis19setPositionInternalEijj")]
// 0x9fb70 — __ZN4FMOD14CodecOggVorbis19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecOggVorbis *this, int, unsigned int, unsigned int)
pub fn stub_9fb70() -> ! {
    todo!("0x9fb70 __ZN4FMOD14CodecOggVorbis19setPositionInternalEijj")
}

#[doc(alias = "FMOD::CodecOggVorbis::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x9fba0 — __ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecOggVorbis *, int, unsigned int, unsigned int)
pub fn stub_9fba0() -> ! {
    todo!("0x9fba0 __ZN4FMOD14CodecOggVorbis19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecOggVorbis::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis12readInternalEPvjPj")]
// 0x9fbac — __ZN4FMOD14CodecOggVorbis12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *this, void *, unsigned int, unsigned int *)
pub fn stub_9fbac() -> ! {
    todo!("0x9fbac __ZN4FMOD14CodecOggVorbis12readInternalEPvjPj")
}

#[doc(alias = "FMOD::CodecOggVorbis::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x9fd24 — __ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecOggVorbis *, void *, unsigned int, unsigned int *)
pub fn stub_9fd24() -> ! {
    todo!("0x9fd24 __ZN4FMOD14CodecOggVorbis12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "FMOD::CodecOggVorbis::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis13closeInternalEv")]
// 0x9fd30 — __ZN4FMOD14CodecOggVorbis13closeInternalEv
// type: int __fastcall(FMOD::CodecOggVorbis *this)
pub fn stub_9fd30() -> ! {
    todo!("0x9fd30 __ZN4FMOD14CodecOggVorbis13closeInternalEv")
}

#[doc(alias = "FMOD::CodecOggVorbis::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x9fd50 — __ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecOggVorbis *)
pub fn stub_9fd50() -> ! {
    todo!("0x9fd50 __ZN4FMOD14CodecOggVorbis13closeCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::FMOD_OggVorbis_SeekCallback(void *,long long,int)")]
#[doc(alias = "__ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi")]
// 0x9fd5c — __ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi
// type: int __fastcall(FMOD *this, int, __int64, int)
pub fn stub_9fd5c() -> ! {
    todo!("0x9fd5c __ZN4FMOD27FMOD_OggVorbis_SeekCallbackEPvxi")
}

#[doc(alias = "FMOD::CodecOggVorbis::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis16getDescriptionExEv")]
// 0x9fd80 — __ZN4FMOD14CodecOggVorbis16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecOggVorbis *this)
pub fn stub_9fd80() -> ! {
    todo!("0x9fd80 __ZN4FMOD14CodecOggVorbis16getDescriptionExEv")
}

#[doc(alias = "FMOD::FMOD_OggVorbis_ReadCallback(void *,unsigned long,unsigned long,void *)")]
#[doc(alias = "__ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_")]
// 0x9fe30 — __ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_
// type: unsigned int __fastcall(FMOD *this, unsigned int, unsigned int, FMOD::File *, void *)
pub fn stub_9fe30() -> ! {
    todo!("0x9fe30 __ZN4FMOD27FMOD_OggVorbis_ReadCallbackEPvmmS0_")
}

#[doc(alias = "_FMOD_OggVorbis_Free")]
// 0x9fe7c — _FMOD_OggVorbis_Free
// type: int __fastcall(int, _DWORD *)
pub fn stub_9fe7c() -> ! {
    todo!("0x9fe7c _FMOD_OggVorbis_Free")
}

#[doc(alias = "FMOD::CodecOggVorbis::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x9fec8 — __ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
pub fn stub_9fec8() -> ! {
    todo!("0x9fec8 __ZN4FMOD14CodecOggVorbis12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecOggVorbis::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0xa0448 — __ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
pub fn stub_a0448() -> ! {
    todo!("0xa0448 __ZN4FMOD14CodecOggVorbis12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::FMOD_OggVorbis_TellCallback(void *)")]
#[doc(alias = "__ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv")]
// 0xa0454 — __ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv
// type: unsigned int __fastcall(FMOD *this, void *)
pub fn stub_a0454() -> ! {
    todo!("0xa0454 __ZN4FMOD27FMOD_OggVorbis_TellCallbackEPv")
}

#[doc(alias = "_FMOD_OggVorbis_ReAlloc")]
// 0xa0474 — _FMOD_OggVorbis_ReAlloc
// type: int __fastcall(int, _DWORD *, int, int)
pub fn stub_a0474() -> ! {
    todo!("0xa0474 _FMOD_OggVorbis_ReAlloc")
}

#[doc(alias = "_FMOD_OggVorbis_Calloc")]
// 0xa0500 — _FMOD_OggVorbis_Calloc
// type: int __fastcall(int, int, int)
pub fn stub_a0500() -> ! {
    todo!("0xa0500 _FMOD_OggVorbis_Calloc")
}

#[doc(alias = "_FMOD_OggVorbis_Malloc")]
// 0xa0564 — _FMOD_OggVorbis_Malloc
// type: int __fastcall(int, int)
pub fn stub_a0564() -> ! {
    todo!("0xa0564 _FMOD_OggVorbis_Malloc")
}

#[doc(alias = "global constructor keyed to_FMOD_OggVorbis_Malloc")]
#[doc(alias = "__GLOBAL__I_FMOD_OggVorbis_Malloc")]
// 0xa0614 — __GLOBAL__I_FMOD_OggVorbis_Malloc
// type: int()
pub fn stub_a0614() -> ! {
    todo!("0xa0614 __GLOBAL__I_FMOD_OggVorbis_Malloc")
}

#[doc(alias = "FMOD::CodecPlaylist::getQuoteData(char const*,char *,int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi")]
// 0xa0620 — __ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi
// type: int __fastcall(FMOD::CodecPlaylist *this, const char *, char *, int *)
pub fn stub_a0620() -> ! {
    todo!("0xa0620 __ZN4FMOD13CodecPlaylist12getQuoteDataEPKcPcPi")
}

#[doc(alias = "FMOD::CodecPlaylist::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist13closeInternalEv")]
// 0xa0684 — __ZN4FMOD13CodecPlaylist13closeInternalEv
// type: int __fastcall(FMOD::CodecPlaylist *this)
pub fn stub_a0684() -> ! {
    todo!("0xa0684 __ZN4FMOD13CodecPlaylist13closeInternalEv")
}

#[doc(alias = "FMOD::CodecPlaylist::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE")]
// 0xa068c — __ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecPlaylist *)
pub fn stub_a068c() -> ! {
    todo!("0xa068c __ZN4FMOD13CodecPlaylist13closeCallbackEP16FMOD_CODEC_STATE")
}

#[doc(alias = "FMOD::CodecPlaylist::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0xa0698 — __ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int()
pub fn stub_a0698() -> ! {
    todo!("0xa0698 __ZN4FMOD13CodecPlaylist12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

#[doc(alias = "FMOD::CodecPlaylist::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0xa06a0 — __ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int()
pub fn stub_a06a0() -> ! {
    todo!("0xa06a0 __ZN4FMOD13CodecPlaylist19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

#[doc(alias = "FMOD::CodecPlaylist::isNewLine(char)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist9isNewLineEc")]
// 0xa06a8 — __ZN4FMOD13CodecPlaylist9isNewLineEc
// type: bool __fastcall(FMOD::File **this, char)
pub fn stub_a06a8() -> ! {
    todo!("0xa06a8 __ZN4FMOD13CodecPlaylist9isNewLineEc")
}

#[doc(alias = "FMOD::CodecPlaylist::skipWhiteSpace(int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi")]
// 0xa0704 — __ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi
// type: int __fastcall(FMOD::File **this, int *)
pub fn stub_a0704() -> ! {
    todo!("0xa0704 __ZN4FMOD13CodecPlaylist14skipWhiteSpaceEPi")
}

#[doc(alias = "FMOD::CodecPlaylist::readLine(char *,int,int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist8readLineEPciPi")]
// 0xa0784 — __ZN4FMOD13CodecPlaylist8readLineEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
pub fn stub_a0784() -> ! {
    todo!("0xa0784 __ZN4FMOD13CodecPlaylist8readLineEPciPi")
}

#[doc(alias = "FMOD::CodecPlaylist::skipSimpleComments(void)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv")]
// 0xa0820 — __ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a0820() -> ! {
    todo!("0xa0820 __ZN4FMOD13CodecPlaylist18skipSimpleCommentsEv")
}

#[doc(alias = "FMOD::CodecPlaylist::getPLSToken(char *,int,int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi")]
// 0xa08b8 — __ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi
// type: int __fastcall(FMOD::File **this, char *, int, int *)
pub fn stub_a08b8() -> ! {
    todo!("0xa08b8 __ZN4FMOD13CodecPlaylist11getPLSTokenEPciPi")
}

#[doc(alias = "FMOD::CodecPlaylist::getNextXMLTag(char *,int *,char *,int *)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_")]
// 0xa0a54 — __ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_
// type: int __fastcall(FMOD::File **this, char *, int *, char *, int *)
pub fn stub_a0a54() -> ! {
    todo!("0xa0a54 __ZN4FMOD13CodecPlaylist13getNextXMLTagEPcPiS1_S2_")
}

#[doc(alias = "FMOD::CodecPlaylist::readSimple(void)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist10readSimpleEv")]
// 0xa0bb8 — __ZN4FMOD13CodecPlaylist10readSimpleEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a0bb8() -> ! {
    todo!("0xa0bb8 __ZN4FMOD13CodecPlaylist10readSimpleEv")
}

#[doc(alias = "FMOD::CodecPlaylist::readPLS(void)")]
#[doc(alias = "__ZN4FMOD13CodecPlaylist7readPLSEv")]
// 0xa0c58 — __ZN4FMOD13CodecPlaylist7readPLSEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a0c58() -> ! {
    todo!("0xa0c58 __ZN4FMOD13CodecPlaylist7readPLSEv")
}

#[doc(alias = "FMOD::CodecXM::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD7CodecXM12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0xac01c — __ZN4FMOD7CodecXM12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_ac01c() -> ! {
    todo!("0xac01c __ZN4FMOD7CodecXM12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "FMOD::CodecXM::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD7CodecXM12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0xad880 — __ZN4FMOD7CodecXM12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_ad880() -> ! {
    todo!("0xad880 __ZN4FMOD7CodecXM12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

#[doc(alias = "global constructor keyed toFMOD::xmcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD7xmcodecE")]
// 0xad8d8 — __GLOBAL__I__ZN4FMOD7xmcodecE
// type: int()
pub fn stub_ad8d8() -> ! {
    todo!("0xad8d8 __GLOBAL__I__ZN4FMOD7xmcodecE")
}

#[doc(alias = "FMOD::DSPChorus::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xad8e4 — __ZN4FMOD9DSPChorus17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPChorus *this, FMOD::MemoryTracker *)
pub fn stub_ad8e4() -> ! {
    todo!("0xad8e4 __ZN4FMOD9DSPChorus17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPChorus::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xad918 — __ZN4FMOD9DSPChorus21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPChorus *this, FMOD::MemoryTracker *)
pub fn stub_ad918() -> ! {
    todo!("0xad918 __ZN4FMOD9DSPChorus21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPChorus::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus20getParameterInternalEiPfPc")]
// 0xad970 — __ZN4FMOD9DSPChorus20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPChorus *this, int, float *, char *)
pub fn stub_ad970() -> ! {
    todo!("0xad970 __ZN4FMOD9DSPChorus20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPChorus::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xadb24 — __ZN4FMOD9DSPChorus20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPChorus *, int, float *, char *)
pub fn stub_adb24() -> ! {
    todo!("0xadb24 __ZN4FMOD9DSPChorus20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}
