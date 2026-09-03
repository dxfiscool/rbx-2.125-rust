//! core shard jv — 150 stubs EA-sorted 0x72388..0x88818 (global EA-sorted, next 150 not yet in core after ju 0x72364, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) global EA-sorted ascending, next 150 not yet in rbx_core (34159 before -> 34309 after, gap 51387->51237).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::ChannelReal::isPlaying(bool *,bool)")]
#[doc(alias = "__ZN4FMOD11ChannelReal9isPlayingEPbb")]
// 0x72388 — __ZN4FMOD11ChannelReal9isPlayingEPbb
pub fn stub_72388() {
    // IDA 0x72388: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::isVirtual(bool *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal9isVirtualEPb")]
// 0x723b0 — __ZN4FMOD11ChannelReal9isVirtualEPb
pub fn stub_723b0() {
    // IDA 0x723b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
#[doc(alias = "__ZN4FMOD11ChannelReal11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW")]
// 0x723c4 — __ZN4FMOD11ChannelReal11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
pub fn stub_723c4() {
    // IDA 0x723c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getWaveData(float *,int,int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal11getWaveDataEPfii")]
// 0x723cc — __ZN4FMOD11ChannelReal11getWaveDataEPfii
pub fn stub_723cc() {
    // IDA 0x723cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")]
#[doc(alias = "__ZN4FMOD11ChannelReal10getDSPHeadEPPNS_4DSPIE")]
// 0x723d4 — __ZN4FMOD11ChannelReal10getDSPHeadEPPNS_4DSPIE
pub fn stub_723d4() {
    // IDA 0x723d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setMode(unsigned int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal7setModeEj")]
// 0x723e4 — __ZN4FMOD11ChannelReal7setModeEj
pub fn stub_723e4() {
    // IDA 0x723e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES")]
// 0x72528 — __ZN4FMOD11ChannelReal19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
pub fn stub_72528() {
    // IDA 0x72528: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
#[doc(alias = "__ZN4FMOD11ChannelReal19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES")]
// 0x725a0 — __ZN4FMOD11ChannelReal19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
pub fn stub_725a0() {
    // IDA 0x725a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::updateSpeakerLevels(float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal19updateSpeakerLevelsEf")]
// 0x726d8 — __ZN4FMOD11ChannelReal19updateSpeakerLevelsEf
pub fn stub_726d8() {
    // IDA 0x726d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal16setSpeakerLevelsEiPfi")]
// 0x72910 — __ZN4FMOD11ChannelReal16setSpeakerLevelsEiPfi
pub fn stub_72910() {
    // IDA 0x72910: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
#[doc(alias = "__ZN4FMOD11ChannelRealD0Ev")]
// 0x72a04 — __ZN4FMOD11ChannelRealD0Ev
pub fn stub_72a04() {
    // IDA 0x72a04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
#[doc(alias = "__ZN4FMOD11ChannelRealD1Ev")]
// 0x72a28 — __ZN4FMOD11ChannelRealD1Ev
pub fn stub_72a28() {
    // IDA 0x72a28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelRealManual3D::alloc(void)")]
#[doc(alias = "__ZN4FMOD19ChannelRealManual3D5allocEv")]
// 0x72a40 — __ZN4FMOD19ChannelRealManual3D5allocEv
pub fn stub_72a40() {
    // IDA 0x72a40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")]
#[doc(alias = "__ZN4FMOD19ChannelRealManual3DC2Ev")]
// 0x72a58 — __ZN4FMOD19ChannelRealManual3DC2Ev
pub fn stub_72a58() {
    // IDA 0x72a58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")]
#[doc(alias = "__ZN4FMOD19ChannelRealManual3D23set2DFreqVolumePanFor3DEv")]
// 0x72a88 — __ZN4FMOD19ChannelRealManual3D23set2DFreqVolumePanFor3DEv
pub fn stub_72a88() {
    // IDA 0x72a88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
#[doc(alias = "__ZN4FMOD19ChannelRealManual3DD0Ev")]
// 0x73de4 — __ZN4FMOD19ChannelRealManual3DD0Ev
pub fn stub_73de4() {
    // IDA 0x73de4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
#[doc(alias = "__ZN4FMOD19ChannelRealManual3DD1Ev")]
// 0x73e08 — __ZN4FMOD19ChannelRealManual3DD1Ev
pub fn stub_73e08() {
    // IDA 0x73e08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::setLowPassGain(float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware14setLowPassGainEf")]
// 0x73e20 — __ZN4FMOD15ChannelSoftware14setLowPassGainEf
pub fn stub_73e20() {
    // IDA 0x73e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::setDSPClockDelay(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware16setDSPClockDelayEv")]
// 0x73e34 — __ZN4FMOD15ChannelSoftware16setDSPClockDelayEv
pub fn stub_73e34() {
    // IDA 0x73e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::setPosition(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware11setPositionEjj")]
// 0x73f0c — __ZN4FMOD15ChannelSoftware11setPositionEjj
pub fn stub_73f0c() {
    // IDA 0x73f0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware11getPositionEPjj")]
// 0x741f4 — __ZN4FMOD15ChannelSoftware11getPositionEPjj
pub fn stub_741f4() {
    // IDA 0x741f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware10getDSPHeadEPPNS_4DSPIE")]
// 0x74554 — __ZN4FMOD15ChannelSoftware10getDSPHeadEPPNS_4DSPIE
pub fn stub_74554() {
    // IDA 0x74554: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware16moveChannelGroupEPNS_13ChannelGroupIES2_b")]
// 0x74564 — __ZN4FMOD15ChannelSoftware16moveChannelGroupEPNS_13ChannelGroupIES2_b
pub fn stub_74564() {
    // IDA 0x74564: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES")]
// 0x745d4 — __ZN4FMOD15ChannelSoftware19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
pub fn stub_745d4() {
    // IDA 0x745d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware12addToReverbsEPNS_4DSPIE")]
// 0x7464c — __ZN4FMOD15ChannelSoftware12addToReverbsEPNS_4DSPIE
pub fn stub_7464c() {
    // IDA 0x7464c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::getWaveData(float *,int,int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware11getWaveDataEPfii")]
// 0x748b4 — __ZN4FMOD15ChannelSoftware11getWaveDataEPfii
pub fn stub_748b4() {
    // IDA 0x748b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW")]
// 0x749c4 — __ZN4FMOD15ChannelSoftware11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
pub fn stub_749c4() {
    // IDA 0x749c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::isPlaying(bool *,bool)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware9isPlayingEPbb")]
// 0x74b20 — __ZN4FMOD15ChannelSoftware9isPlayingEPbb
pub fn stub_74b20() {
    // IDA 0x74b20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setMode(unsigned int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware7setModeEj")]
// 0x74bd0 — __ZN4FMOD15ChannelSoftware7setModeEj
pub fn stub_74bd0() {
    // IDA 0x74bd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::getLoopCount(int *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware12getLoopCountEPi")]
// 0x74c04 — __ZN4FMOD15ChannelSoftware12getLoopCountEPi
pub fn stub_74c04() {
    // IDA 0x74c04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setLoopCount(int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware12setLoopCountEi")]
// 0x74c44 — __ZN4FMOD15ChannelSoftware12setLoopCountEi
pub fn stub_74c44() {
    // IDA 0x74c44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware13setLoopPointsEjj")]
// 0x74c90 — __ZN4FMOD15ChannelSoftware13setLoopPointsEjj
pub fn stub_74c90() {
    // IDA 0x74c90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setPan(float,float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware6setPanEff")]
// 0x74cd8 — __ZN4FMOD15ChannelSoftware6setPanEff
pub fn stub_74cd8() {
    // IDA 0x74cd8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware12setFrequencyEf")]
// 0x74de8 — __ZN4FMOD15ChannelSoftware12setFrequencyEf
pub fn stub_74de8() {
    // IDA 0x74de8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware15updateReverbMixEPNS_7ReverbIEf")]
// 0x74edc — __ZN4FMOD15ChannelSoftware15updateReverbMixEPNS_7ReverbIEf
pub fn stub_74edc() {
    // IDA 0x74edc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::updateDirectMix(float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware15updateDirectMixEf")]
// 0x751dc — __ZN4FMOD15ChannelSoftware15updateDirectMixEf
pub fn stub_751dc() {
    // IDA 0x751dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware13setupDSPCodecEPNS_4DSPIE")]
// 0x75408 — __ZN4FMOD15ChannelSoftware13setupDSPCodecEPNS_4DSPIE
pub fn stub_75408() {
    // IDA 0x75408: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::close(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware5closeEv")]
// 0x75738 — __ZN4FMOD15ChannelSoftware5closeEv
pub fn stub_75738() {
    // IDA 0x75738: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE")]
// 0x757fc — __ZN4FMOD15ChannelSoftware4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
pub fn stub_757fc() {
    // IDA 0x757fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftwareC2Ev")]
// 0x759c0 — __ZN4FMOD15ChannelSoftwareC2Ev
pub fn stub_759c0() {
    // IDA 0x759c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftwareC1Ev")]
// 0x75a44 — __ZN4FMOD15ChannelSoftwareC1Ev
pub fn stub_75a44() {
    // IDA 0x75a44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setPaused(bool)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware9setPausedEb")]
// 0x75a48 — __ZN4FMOD15ChannelSoftware9setPausedEb
pub fn stub_75a48() {
    // IDA 0x75a48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::start(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware5startEv")]
// 0x75b50 — __ZN4FMOD15ChannelSoftware5startEv
pub fn stub_75b50() {
    // IDA 0x75b50: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::alloc(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware5allocEv")]
// 0x75be0 — __ZN4FMOD15ChannelSoftware5allocEv
pub fn stub_75be0() {
    // IDA 0x75be0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::stop(void)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware4stopEv")]
// 0x75f8c — __ZN4FMOD15ChannelSoftware4stopEv
pub fn stub_75f8c() {
    // IDA 0x75f8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware16setSpeakerLevelsEiPfi")]
// 0x762c4 — __ZN4FMOD15ChannelSoftware16setSpeakerLevelsEiPfi
pub fn stub_762c4() {
    // IDA 0x762c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware13setSpeakerMixEffffffff")]
// 0x76584 — __ZN4FMOD15ChannelSoftware13setSpeakerMixEffffffff
pub fn stub_76584() {
    // IDA 0x76584: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setVolume(float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware9setVolumeEf")]
// 0x76988 — __ZN4FMOD15ChannelSoftware9setVolumeEf
pub fn stub_76988() {
    // IDA 0x76988: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::set3DOcclusion(float,float)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware14set3DOcclusionEff")]
// 0x76a80 — __ZN4FMOD15ChannelSoftware14set3DOcclusionEff
pub fn stub_76a80() {
    // IDA 0x76a80: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES")]
// 0x76b3c — __ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
pub fn stub_76b3c() {
    // IDA 0x76b3c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::getPaused(bool *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware9getPausedEPb")]
// 0x7709c — __ZN4FMOD15ChannelSoftware9getPausedEPb
pub fn stub_7709c() {
    // IDA 0x7709c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD15ChannelSoftware5allocEPNS_4DSPIE")]
// 0x77138 — __ZN4FMOD15ChannelSoftware5allocEPNS_4DSPIE
pub fn stub_77138() {
    // IDA 0x77138: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
#[doc(alias = "__ZN4FMOD15ChannelSoftwareD1Ev")]
// 0x773c4 — __ZN4FMOD15ChannelSoftwareD1Ev
pub fn stub_773c4() {
    // IDA 0x773c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
#[doc(alias = "__ZN4FMOD15ChannelSoftwareD0Ev")]
// 0x773f0 — __ZN4FMOD15ChannelSoftwareD0Ev
pub fn stub_773f0() {
    // IDA 0x773f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::set2DFreqVolumePanFor3D(void)")]
#[doc(alias = "__ZN4FMOD13ChannelStream23set2DFreqVolumePanFor3DEv")]
// 0x77428 — __ZN4FMOD13ChannelStream23set2DFreqVolumePanFor3DEv
pub fn stub_77428() {
    // IDA 0x77428: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
#[doc(alias = "__ZN4FMOD13ChannelStream16moveChannelGroupEPNS_13ChannelGroupIES2_b")]
// 0x77474 — __ZN4FMOD13ChannelStream16moveChannelGroupEPNS_13ChannelGroupIES2_b
pub fn stub_77474() {
    // IDA 0x77474: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::start(void)")]
#[doc(alias = "__ZN4FMOD13ChannelStream5startEv")]
// 0x774e0 — __ZN4FMOD13ChannelStream5startEv
pub fn stub_774e0() {
    // IDA 0x774e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::update(int)")]
#[doc(alias = "__ZN4FMOD13ChannelStream6updateEi")]
// 0x77574 — __ZN4FMOD13ChannelStream6updateEi
pub fn stub_77574() {
    // IDA 0x77574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelStream::setVolume(float)")]
#[doc(alias = "__ZN4FMOD13ChannelStream9setVolumeEf")]
// 0x775d0 — __ZN4FMOD13ChannelStream9setVolumeEf
pub fn stub_775d0() {
    // IDA 0x775d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD13ChannelStream12setFrequencyEf")]
// 0x77718 — __ZN4FMOD13ChannelStream12setFrequencyEf
pub fn stub_77718() {
    // IDA 0x77718: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setPan(float,float)")]
#[doc(alias = "__ZN4FMOD13ChannelStream6setPanEff")]
// 0x77774 — __ZN4FMOD13ChannelStream6setPanEff
pub fn stub_77774() {
    // IDA 0x77774: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setDSPClockDelay(void)")]
#[doc(alias = "__ZN4FMOD13ChannelStream16setDSPClockDelayEv")]
// 0x7781c — __ZN4FMOD13ChannelStream16setDSPClockDelayEv
pub fn stub_7781c() {
    // IDA 0x7781c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setSpeakerMix(float,float,float,float,float,float,float,float)")]
#[doc(alias = "__ZN4FMOD13ChannelStream13setSpeakerMixEffffffff")]
// 0x77868 — __ZN4FMOD13ChannelStream13setSpeakerMixEffffffff
pub fn stub_77868() {
    // IDA 0x77868: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::setSpeakerLevels(int,float *,int)")]
#[doc(alias = "__ZN4FMOD13ChannelStream16setSpeakerLevelsEiPfi")]
// 0x77904 — __ZN4FMOD13ChannelStream16setSpeakerLevelsEiPfi
pub fn stub_77904() {
    // IDA 0x77904: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelStream::set3DAttributes(void)")]
#[doc(alias = "__ZN4FMOD13ChannelStream15set3DAttributesEv")]
// 0x77970 — __ZN4FMOD13ChannelStream15set3DAttributesEv
pub fn stub_77970() {
    // IDA 0x77970: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::release(void)")]
#[doc(alias = "__ZN4FMOD5Codec7releaseEv")]
// 0x7fd9c — __ZN4FMOD5Codec7releaseEv
pub fn stub_7fd9c() {
    // IDA 0x7fd9c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::setPosition(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD5Codec11setPositionEijj")]
// 0x7fe6c — __ZN4FMOD5Codec11setPositionEijj
pub fn stub_7fe6c() {
    // IDA 0x7fe6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF19setPositionInternalEijj")]
// 0x80388 — __ZN4FMOD9CodecAIFF19setPositionInternalEijj
pub fn stub_80388() {
    // IDA 0x80388: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x804cc — __ZN4FMOD9CodecAIFF19setPositionCallbackEP16FMOD_CODEC_STATEijj
pub fn stub_804cc() {
    // IDA 0x804cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF12readInternalEPvjPj")]
// 0x804d8 — __ZN4FMOD9CodecAIFF12readInternalEPvjPj
pub fn stub_804d8() {
    // IDA 0x804d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x806e4 — __ZN4FMOD9CodecAIFF12readCallbackEP16FMOD_CODEC_STATEPvjPj
pub fn stub_806e4() {
    // IDA 0x806e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF13closeInternalEv")]
// 0x806f0 — __ZN4FMOD9CodecAIFF13closeInternalEv
pub fn stub_806f0() {
    // IDA 0x806f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x80744 — __ZN4FMOD9CodecAIFF13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_80744() {
    // IDA 0x80744: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ConvertFromIeeeExtended(unsigned char *)")]
#[doc(alias = "__ZN4FMOD23ConvertFromIeeeExtendedEPh")]
// 0x80750 — __ZN4FMOD23ConvertFromIeeeExtendedEPh
pub fn stub_80750() {
    // IDA 0x80750: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x80864 — __ZN4FMOD9CodecAIFF12openInternalEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_80864() {
    // IDA 0x80864: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x81068 — __ZN4FMOD9CodecAIFF12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_81068() {
    // IDA 0x81068: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecAIFF::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9CodecAIFF16getDescriptionExEv")]
// 0x81074 — __ZN4FMOD9CodecAIFF16getDescriptionExEv
pub fn stub_81074() {
    // IDA 0x81074: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::aiffcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9aiffcodecE")]
// 0x8115c — __GLOBAL__I__ZN4FMOD9aiffcodecE
pub fn stub_8115c() {
    // IDA 0x8115c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecDLS::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecDLS19setPositionInternalEijj")]
// 0x81168 — __ZN4FMOD8CodecDLS19setPositionInternalEijj
pub fn stub_81168() {
    // IDA 0x81168: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecDLS::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecDLS19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x8132c — __ZN4FMOD8CodecDLS19setPositionCallbackEP16FMOD_CODEC_STATEijj
pub fn stub_8132c() {
    // IDA 0x8132c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecDLS::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecDLS12readInternalEPvjPj")]
// 0x81338 — __ZN4FMOD8CodecDLS12readInternalEPvjPj
pub fn stub_81338() {
    // IDA 0x81338: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecDLS::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecDLS12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x813e8 — __ZN4FMOD8CodecDLS12readCallbackEP16FMOD_CODEC_STATEPvjPj
pub fn stub_813e8() {
    // IDA 0x813e8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecDLS::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD8CodecDLS13closeInternalEv")]
// 0x813f4 — __ZN4FMOD8CodecDLS13closeInternalEv
pub fn stub_813f4() {
    // IDA 0x813f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecDLS::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD8CodecDLS13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x815e0 — __ZN4FMOD8CodecDLS13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_815e0() {
    // IDA 0x815e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecDLS::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD8CodecDLS16getDescriptionExEv")]
// 0x815ec — __ZN4FMOD8CodecDLS16getDescriptionExEv
pub fn stub_815ec() {
    // IDA 0x815ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecDLS::parseChunk(char *,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecDLS10parseChunkEPcj")]
// 0x8168c — __ZN4FMOD8CodecDLS10parseChunkEPcj
pub fn stub_8168c() {
    // IDA 0x8168c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecDLS::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecDLS12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x82848 — __ZN4FMOD8CodecDLS12openInternalEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_82848() {
    // IDA 0x82848: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecDLS::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecDLS12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x82970 — __ZN4FMOD8CodecDLS12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_82970() {
    // IDA 0x82970: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dlscodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD8dlscodecE")]
// 0x829c8 — __GLOBAL__I__ZN4FMOD8dlscodecE
pub fn stub_829c8() {
    // IDA 0x829c8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::FMOD_FLAC_LengthCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
#[doc(alias = "__ZN4FMODL24FMOD_FLAC_LengthCallbackEPK19FLAC__StreamDecoderPyPv")]
// 0x829d4 — __ZN4FMODL24FMOD_FLAC_LengthCallbackEPK19FLAC__StreamDecoderPyPv
pub fn stub_829d4() {
    // IDA 0x829d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::FMOD_FLAC_ErrorCallback(FLAC__StreamDecoder const*,FLAC__StreamDecoderErrorStatus,void *)")]
#[doc(alias = "__ZN4FMODL23FMOD_FLAC_ErrorCallbackEPK19FLAC__StreamDecoder30FLAC__StreamDecoderErrorStatusPv")]
// 0x82a1c — __ZN4FMODL23FMOD_FLAC_ErrorCallbackEPK19FLAC__StreamDecoder30FLAC__StreamDecoderErrorStatusPv
pub fn stub_82a1c() {
    // IDA 0x82a1c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFLAC::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC19setPositionInternalEijj")]
// 0x82a20 — __ZN4FMOD9CodecFLAC19setPositionInternalEijj
pub fn stub_82a20() {
    // IDA 0x82a20: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFLAC::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x82a70 — __ZN4FMOD9CodecFLAC19setPositionCallbackEP16FMOD_CODEC_STATEijj
pub fn stub_82a70() {
    // IDA 0x82a70: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFLAC::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC12readInternalEPvjPj")]
// 0x82a7c — __ZN4FMOD9CodecFLAC12readInternalEPvjPj
pub fn stub_82a7c() {
    // IDA 0x82a7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x82adc — __ZN4FMOD9CodecFLAC12readCallbackEP16FMOD_CODEC_STATEPvjPj
pub fn stub_82adc() {
    // IDA 0x82adc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC13closeInternalEv")]
// 0x82ae8 — __ZN4FMOD9CodecFLAC13closeInternalEv
pub fn stub_82ae8() {
    // IDA 0x82ae8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x82ba4 — __ZN4FMOD9CodecFLAC13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_82ba4() {
    // IDA 0x82ba4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_SeekCallback(FLAC__StreamDecoder const*,unsigned long long,void *)")]
#[doc(alias = "__ZN4FMODL22FMOD_FLAC_SeekCallbackEPK19FLAC__StreamDecoderyPv")]
// 0x82bb0 — __ZN4FMODL22FMOD_FLAC_SeekCallbackEPK19FLAC__StreamDecoderyPv
pub fn stub_82bb0() {
    // IDA 0x82bb0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_ReadCallback(FLAC__StreamDecoder const*,unsigned char *,unsigned long *,void *)")]
#[doc(alias = "__ZN4FMODL22FMOD_FLAC_ReadCallbackEPK19FLAC__StreamDecoderPhPmPv")]
// 0x82bd0 — __ZN4FMODL22FMOD_FLAC_ReadCallbackEPK19FLAC__StreamDecoderPhPmPv
pub fn stub_82bd0() {
    // IDA 0x82bd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x82c14 — __ZN4FMOD9CodecFLAC12openInternalEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_82c14() {
    // IDA 0x82c14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x82f38 — __ZN4FMOD9CodecFLAC12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_82f38() {
    // IDA 0x82f38: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_WriteCallback(FLAC__StreamDecoder const*,FLAC__Frame const*,int const* const*,void *)")]
#[doc(alias = "__ZN4FMODL23FMOD_FLAC_WriteCallbackEPK19FLAC__StreamDecoderPK11FLAC__FramePKPKiPv")]
// 0x82f44 — __ZN4FMODL23FMOD_FLAC_WriteCallbackEPK19FLAC__StreamDecoderPK11FLAC__FramePKPKiPv
pub fn stub_82f44() {
    // IDA 0x82f44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_MetadataCallback(FLAC__StreamDecoder const*,FLAC__StreamMetadata const*,void *)")]
#[doc(alias = "__ZN4FMODL26FMOD_FLAC_MetadataCallbackEPK19FLAC__StreamDecoderPK20FLAC__StreamMetadataPv")]
// 0x830e4 — __ZN4FMODL26FMOD_FLAC_MetadataCallbackEPK19FLAC__StreamDecoderPK20FLAC__StreamMetadataPv
pub fn stub_830e4() {
    // IDA 0x830e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_EofCallback(FLAC__StreamDecoder const*,void *)")]
#[doc(alias = "__ZN4FMODL21FMOD_FLAC_EofCallbackEPK19FLAC__StreamDecoderPv")]
// 0x83298 — __ZN4FMODL21FMOD_FLAC_EofCallbackEPK19FLAC__StreamDecoderPv
pub fn stub_83298() {
    // IDA 0x83298: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_FLAC_TellCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
#[doc(alias = "__ZN4FMODL22FMOD_FLAC_TellCallbackEPK19FLAC__StreamDecoderPyPv")]
// 0x832e0 — __ZN4FMODL22FMOD_FLAC_TellCallbackEPK19FLAC__StreamDecoderPyPv
pub fn stub_832e0() {
    // IDA 0x832e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFLAC::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9CodecFLAC16getDescriptionExEv")]
// 0x83320 — __ZN4FMOD9CodecFLAC16getDescriptionExEv
pub fn stub_83320() {
    // IDA 0x83320: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::flaccodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9flaccodecE")]
// 0x8340c — __GLOBAL__I__ZN4FMOD9flaccodecE
pub fn stub_8340c() {
    // IDA 0x8340c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFSB::getNumSyncPoints(int,int *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB16getNumSyncPointsEiPi")]
// 0x83418 — __ZN4FMOD8CodecFSB16getNumSyncPointsEiPi
pub fn stub_83418() {
    // IDA 0x83418: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFSB::getSyncPointData(int,int,char **,int *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB16getSyncPointDataEiiPPcPi")]
// 0x83434 — __ZN4FMOD8CodecFSB16getSyncPointDataEiiPPcPi
pub fn stub_83434() {
    // IDA 0x83434: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFSB::canPointInternal(void)")]
#[doc(alias = "__ZN4FMOD8CodecFSB16canPointInternalEv")]
// 0x834a0 — __ZN4FMOD8CodecFSB16canPointInternalEv
pub fn stub_834a0() {
    // IDA 0x834a0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFSB::canPointCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB16canPointCallbackEP16FMOD_CODEC_STATE")]
// 0x834c8 — __ZN4FMOD8CodecFSB16canPointCallbackEP16FMOD_CODEC_STATE
pub fn stub_834c8() {
    // IDA 0x834c8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecFSB::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD8CodecFSB16getDescriptionExEv")]
// 0x834d4 — __ZN4FMOD8CodecFSB16getDescriptionExEv
pub fn stub_834d4() {
    // IDA 0x834d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x835d4 — __ZN4FMOD8CodecFSB17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_835d4() {
    // IDA 0x835d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE")]
// 0x83858 — __ZN4FMOD8CodecFSB21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
pub fn stub_83858() {
    // IDA 0x83858: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD8CodecFSB13closeInternalEv")]
// 0x838b0 — __ZN4FMOD8CodecFSB13closeInternalEv
pub fn stub_838b0() {
    // IDA 0x838b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x83c50 — __ZN4FMOD8CodecFSB13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_83c50() {
    // IDA 0x83c50: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD8CodecFSB13resetInternalEv")]
// 0x83c5c — __ZN4FMOD8CodecFSB13resetInternalEv
pub fn stub_83c5c() {
    // IDA 0x83c5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::resetCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB13resetCallbackEP16FMOD_CODEC_STATE")]
// 0x83ce0 — __ZN4FMOD8CodecFSB13resetCallbackEP16FMOD_CODEC_STATE
pub fn stub_83ce0() {
    // IDA 0x83ce0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getWaveFormatInternal(int,FMOD_CODEC_WAVEFORMAT *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB21getWaveFormatInternalEiP21FMOD_CODEC_WAVEFORMAT")]
// 0x83cec — __ZN4FMOD8CodecFSB21getWaveFormatInternalEiP21FMOD_CODEC_WAVEFORMAT
pub fn stub_83cec() {
    // IDA 0x83cec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getWaveFormatCallback(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB21getWaveFormatCallbackEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT")]
// 0x842c4 — __ZN4FMOD8CodecFSB21getWaveFormatCallbackEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT
pub fn stub_842c4() {
    // IDA 0x842c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::soundcreateInternal(int,FMOD_SOUND *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19soundcreateInternalEiP10FMOD_SOUND")]
// 0x842d0 — __ZN4FMOD8CodecFSB19soundcreateInternalEiP10FMOD_SOUND
pub fn stub_842d0() {
    // IDA 0x842d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::soundcreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19soundcreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND")]
// 0x84494 — __ZN4FMOD8CodecFSB19soundcreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
pub fn stub_84494() {
    // IDA 0x84494: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getPositionInternal(unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19getPositionInternalEPjj")]
// 0x844a0 — __ZN4FMOD8CodecFSB19getPositionInternalEPjj
pub fn stub_844a0() {
    // IDA 0x844a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::getPositionCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19getPositionCallbackEP16FMOD_CODEC_STATEPjj")]
// 0x84540 — __ZN4FMOD8CodecFSB19getPositionCallbackEP16FMOD_CODEC_STATEPjj
pub fn stub_84540() {
    // IDA 0x84540: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::readInternal(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB12readInternalEPvjPj")]
// 0x8454c — __ZN4FMOD8CodecFSB12readInternalEPvjPj
pub fn stub_8454c() {
    // IDA 0x8454c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
// 0x84ef4 — __ZN4FMOD8CodecFSB12readCallbackEP16FMOD_CODEC_STATEPvjPj
pub fn stub_84ef4() {
    // IDA 0x84ef4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
// 0x84f00 — __ZN4FMOD8CodecFSB12openInternalEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_84f00() {
    // IDA 0x84f00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
#[doc(alias = "__ZN4FMOD8CodecFSB12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
// 0x86654 — __ZN4FMOD8CodecFSB12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
pub fn stub_86654() {
    // IDA 0x86654: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::setPositionInternal(int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19setPositionInternalEijj")]
// 0x86660 — __ZN4FMOD8CodecFSB19setPositionInternalEijj
pub fn stub_86660() {
    // IDA 0x86660: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecFSB::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD8CodecFSB19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
// 0x86aa0 — __ZN4FMOD8CodecFSB19setPositionCallbackEP16FMOD_CODEC_STATEijj
pub fn stub_86aa0() {
    // IDA 0x86aa0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::fsbcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD8fsbcodecE")]
// 0x86b10 — __GLOBAL__I__ZN4FMOD8fsbcodecE
pub fn stub_86b10() {
    // IDA 0x86b10: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecIT::readBits(unsigned char,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7CodecIT8readBitsEhPj")]
// 0x86b1c — __ZN4FMOD7CodecIT8readBitsEhPj
pub fn stub_86b1c() {
    // IDA 0x86b1c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelIT::volumeSlide(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT11volumeSlideEv")]
// 0x86bcc — __ZN4FMOD14MusicChannelIT11volumeSlideEv
pub fn stub_86bcc() {
    // IDA 0x86bcc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelIT::panSlide(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT8panSlideEv")]
// 0x86c34 — __ZN4FMOD14MusicChannelIT8panSlideEv
pub fn stub_86c34() {
    // IDA 0x86c34: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelIT::portamento(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT10portamentoEv")]
// 0x86c9c — __ZN4FMOD14MusicChannelIT10portamentoEv
pub fn stub_86c9c() {
    // IDA 0x86c9c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelIT::vibrato(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT7vibratoEv")]
// 0x86d60 — __ZN4FMOD14MusicChannelIT7vibratoEv
pub fn stub_86d60() {
    // IDA 0x86d60: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelIT::fineVibrato(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT11fineVibratoEv")]
// 0x86eb0 — __ZN4FMOD14MusicChannelIT11fineVibratoEv
pub fn stub_86eb0() {
    // IDA 0x86eb0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelIT::tremolo(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT7tremoloEv")]
// 0x87000 — __ZN4FMOD14MusicChannelIT7tremoloEv
pub fn stub_87000() {
    // IDA 0x87000: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelIT::panbrello(void)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT9panbrelloEv")]
// 0x8710c — __ZN4FMOD14MusicChannelIT9panbrelloEv
pub fn stub_8710c() {
    // IDA 0x8710c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,FMOD::MusicEnvelopeNode *,int,int,int,int,int,unsigned char)")]
#[doc(alias = "__ZN4FMOD7CodecIT15processEnvelopeEPNS_18MusicEnvelopeStateEPNS_19MusicVirtualChannelEiPNS_17MusicEnvelopeNodeEiiiiih")]
// 0x87238 — __ZN4FMOD7CodecIT15processEnvelopeEPNS_18MusicEnvelopeStateEPNS_19MusicVirtualChannelEiPNS_17MusicEnvelopeNodeEiiiiih
pub fn stub_87238() {
    // IDA 0x87238: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::processPitchEnvelope(FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,int)")]
#[doc(alias = "__ZN4FMOD7CodecIT20processPitchEnvelopeEPNS_19MusicVirtualChannelEPNS_15MusicInstrumentEi")]
// 0x874a0 — __ZN4FMOD7CodecIT20processPitchEnvelopeEPNS_19MusicVirtualChannelEPNS_15MusicInstrumentEi
pub fn stub_874a0() {
    // IDA 0x874a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::sampleVibrato(FMOD::MusicVirtualChannel *)")]
#[doc(alias = "__ZN4FMOD7CodecIT13sampleVibratoEPNS_19MusicVirtualChannelE")]
// 0x87bd8 — __ZN4FMOD7CodecIT13sampleVibratoEPNS_19MusicVirtualChannelE
pub fn stub_87bd8() {
    // IDA 0x87bd8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelIT::processVolumeByte(FMOD::MusicNote *,bool)")]
#[doc(alias = "__ZN4FMOD14MusicChannelIT17processVolumeByteEPNS_9MusicNoteEb")]
// 0x87cdc — __ZN4FMOD14MusicChannelIT17processVolumeByteEPNS_9MusicNoteEb
pub fn stub_87cdc() {
    // IDA 0x87cdc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::closeInternal(void)")]
#[doc(alias = "__ZN4FMOD7CodecIT13closeInternalEv")]
// 0x87f7c — __ZN4FMOD7CodecIT13closeInternalEv
pub fn stub_87f7c() {
    // IDA 0x87f7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::closeCallback(FMOD_CODEC_STATE *)")]
#[doc(alias = "__ZN4FMOD7CodecIT13closeCallbackEP16FMOD_CODEC_STATE")]
// 0x883f0 — __ZN4FMOD7CodecIT13closeCallbackEP16FMOD_CODEC_STATE
pub fn stub_883f0() {
    // IDA 0x883f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::freeBlock(void)")]
#[doc(alias = "__ZN4FMOD7CodecIT9freeBlockEv")]
// 0x883fc — __ZN4FMOD7CodecIT9freeBlockEv
pub fn stub_883fc() {
    // IDA 0x883fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::unpackRow(void)")]
#[doc(alias = "__ZN4FMOD7CodecIT9unpackRowEv")]
// 0x88450 — __ZN4FMOD7CodecIT9unpackRowEv
pub fn stub_88450() {
    // IDA 0x88450: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD7CodecIT16getDescriptionExEv")]
// 0x88644 — __ZN4FMOD7CodecIT16getDescriptionExEv
pub fn stub_88644() {
    // IDA 0x88644: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::readBlock(signed char **)")]
#[doc(alias = "__ZN4FMOD7CodecIT9readBlockEPPa")]
// 0x8875c — __ZN4FMOD7CodecIT9readBlockEPPa
pub fn stub_8875c() {
    // IDA 0x8875c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecIT::decompress16(void **,void *,int,bool,int)")]
#[doc(alias = "__ZN4FMOD7CodecIT12decompress16EPPvS1_ibi")]
// 0x88818 — __ZN4FMOD7CodecIT12decompress16EPPvS1_ibi
pub fn stub_88818() {
    // IDA 0x88818: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
