//! core shard jt — 150 core stubs EA-sorted, 0x280d34..0x504a0c (EA-sorted asc next 150 core gap filler not yet in rbx_core after js 0x316364, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|FMOD|Audio, EA-sorted ascending, next 150 not yet in rbx_core (core gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "YieldFunctionStateObject::execute(void)")]
// 0x280d34 — __ZN24YieldFunctionStateObject7executeEv
pub fn stub_280d34() -> ! {
    todo!("0x280d34 __ZN24YieldFunctionStateObject7executeEv")
}

#[doc(alias = "YieldFunctionStateObject::onRaiseException(std::string)")]
// 0x281c0c — __ZN24YieldFunctionStateObject16onRaiseExceptionESs
pub fn stub_281c0c() -> ! {
    todo!("0x281c0c __ZN24YieldFunctionStateObject16onRaiseExceptionESs")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// 0x371b5c — __ZN3RBX10Soundscape12SoundServiceC1Ev
pub fn stub_371b5c() -> ! {
    todo!("0x371b5c __ZN3RBX10Soundscape12SoundServiceC1Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// 0x371b60 — __ZN3RBX10Soundscape12SoundServiceC2Ev
pub fn stub_371b60() -> ! {
    todo!("0x371b60 __ZN3RBX10Soundscape12SoundServiceC2Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::update3DSettings(void)")]
// 0x3723f4 — __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
pub fn stub_3723f4() -> ! {
    todo!("0x3723f4 __ZN3RBX10Soundscape12SoundService16update3DSettingsEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::updateAmbientReverb(void)")]
// 0x372414 — __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
pub fn stub_372414() -> ! {
    todo!("0x372414 __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// 0x372460 — __ZN3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_372460() -> ! {
    todo!("0x372460 __ZN3RBX10Soundscape12SoundServiceD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// 0x372500 — __ZN3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_372500() -> ! {
    todo!("0x372500 __ZN3RBX10Soundscape12SoundServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// 0x372504 — __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_372504() -> ! {
    todo!("0x372504 __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// 0x37250c — __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_37250c() -> ! {
    todo!("0x37250c __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// 0x372514 — __ZN3RBX10Soundscape12SoundServiceD2Ev
pub fn stub_372514() -> ! {
    todo!("0x372514 __ZN3RBX10Soundscape12SoundServiceD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// 0x3728b0 — __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_3728b0() -> ! {
    todo!("0x3728b0 __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// 0x3728b8 — __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_3728b8() -> ! {
    todo!("0x3728b8 __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::loadStockSounds(void)")]
// 0x372bb0 — __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
pub fn stub_372bb0() -> ! {
    todo!("0x372bb0 __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")]
// 0x373554 — __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
pub fn stub_373554() -> ! {
    todo!("0x373554 __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")]
// 0x37384c — __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
pub fn stub_37384c() -> ! {
    todo!("0x37384c __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE")
}

#[doc(alias = "RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")]
// 0x373894 — __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
pub fn stub_373894() -> ! {
    todo!("0x373894 __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE")
}

#[doc(alias = "RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")]
// 0x3738a8 — __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
pub fn stub_3738a8() -> ! {
    todo!("0x3738a8 __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE")
}

#[doc(alias = "RBX::Soundscape::SoundService::playSound(RBX::SoundType)")]
// 0x3738d8 — __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
pub fn stub_3738d8() -> ! {
    todo!("0x3738d8 __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::play(void)")]
// 0x373918 — __ZN3RBX10Soundscape12SoundChannel4playEv
pub fn stub_373918() -> ! {
    todo!("0x373918 __ZN3RBX10Soundscape12SoundChannel4playEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x373974 — __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
pub fn stub_373974() -> ! {
    todo!("0x373974 __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Soundscape::SoundService::step(void)")]
// 0x373cb8 — __ZN3RBX10Soundscape12SoundService4stepEv
pub fn stub_373cb8() -> ! {
    todo!("0x373cb8 __ZN3RBX10Soundscape12SoundService4stepEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::garbageCollectSounds(void)")]
// 0x373fd0 — __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
pub fn stub_373fd0() -> ! {
    todo!("0x373fd0 __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv")
}

#[doc(alias = "RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")]
// 0x374028 — __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
pub fn stub_374028() -> ! {
    todo!("0x374028 __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getSoundId(void)const")]
// 0x374a2c — __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
pub fn stub_374a2c() -> ! {
    todo!("0x374a2c __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getVolume(void)const")]
// 0x374a44 — __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
pub fn stub_374a44() -> ! {
    todo!("0x374a44 __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setVolume(float)")]
// 0x374a48 — __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
pub fn stub_374a48() -> ! {
    todo!("0x374a48 __ZN3RBX10Soundscape12SoundChannel9setVolumeEf")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getPitch(void)const")]
// 0x374aa4 — __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
pub fn stub_374aa4() -> ! {
    todo!("0x374aa4 __ZNK3RBX10Soundscape12SoundChannel8getPitchEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setPitch(float)")]
// 0x374aa8 — __ZN3RBX10Soundscape12SoundChannel8setPitchEf
pub fn stub_374aa8() -> ! {
    todo!("0x374aa8 __ZN3RBX10Soundscape12SoundChannel8setPitchEf")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setPlayCount(int)")]
// 0x374af8 — __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
pub fn stub_374af8() -> ! {
    todo!("0x374af8 __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getLooped(void)const")]
// 0x374b68 — __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
pub fn stub_374b68() -> ! {
    todo!("0x374b68 __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setLooped(bool)")]
// 0x374b74 — __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
pub fn stub_374b74() -> ! {
    todo!("0x374b74 __ZN3RBX10Soundscape12SoundChannel9setLoopedEb")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::isPlaying(void)const")]
// 0x374bb4 — __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
pub fn stub_374bb4() -> ! {
    todo!("0x374bb4 __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::isPaused(void)const")]
// 0x374bec — __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
pub fn stub_374bec() -> ! {
    todo!("0x374bec __ZNK3RBX10Soundscape12SoundChannel8isPausedEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::pause(void)")]
// 0x374c24 — __ZN3RBX10Soundscape12SoundChannel5pauseEv
pub fn stub_374c24() -> ! {
    todo!("0x374c24 __ZN3RBX10Soundscape12SoundChannel5pauseEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::stop(void)")]
// 0x374c68 — __ZN3RBX10Soundscape12SoundChannel4stopEv
pub fn stub_374c68() -> ! {
    todo!("0x374c68 __ZN3RBX10Soundscape12SoundChannel4stopEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::SoundChannel(void)")]
// 0x374cc4 — __ZN3RBX10Soundscape12SoundChannelC2Ev
pub fn stub_374cc4() -> ! {
    todo!("0x374cc4 __ZN3RBX10Soundscape12SoundChannelC2Ev")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x374ff4 — __ZN3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_374ff4() -> ! {
    todo!("0x374ff4 __ZN3RBX10Soundscape12SoundChannelD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x375094 — __ZN3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375094() -> ! {
    todo!("0x375094 __ZN3RBX10Soundscape12SoundChannelD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x375098 — __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_375098() -> ! {
    todo!("0x375098 __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x3750a0 — __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_3750a0() -> ! {
    todo!("0x3750a0 __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x3750a8 — __ZN3RBX10Soundscape12SoundChannelD2Ev
pub fn stub_3750a8() -> ! {
    todo!("0x3750a8 __ZN3RBX10Soundscape12SoundChannelD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x375330 — __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375330() -> ! {
    todo!("0x375330 __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// 0x375338 — __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375338() -> ! {
    todo!("0x375338 __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")]
// 0x375340 — __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
pub fn stub_375340() -> ! {
    todo!("0x375340 __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE")
}

#[doc(alias = "RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")]
// 0x375418 — __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
pub fn stub_375418() -> ! {
    todo!("0x375418 __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi")
}

#[doc(alias = "RBX::Soundscape::Sound::release(void)")]
// 0x3754c4 — __ZN3RBX10Soundscape5Sound7releaseEv
pub fn stub_3754c4() -> ! {
    todo!("0x3754c4 __ZN3RBX10Soundscape5Sound7releaseEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::releaseChannel(void)")]
// 0x3754e0 — __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
pub fn stub_3754e0() -> ! {
    todo!("0x3754e0 __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::updateListenState(void)")]
// 0x375520 — __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
pub fn stub_375520() -> ! {
    todo!("0x375520 __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")]
// 0x375660 — __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
pub fn stub_375660() -> ! {
    todo!("0x375660 __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x37567c — __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_37567c() -> ! {
    todo!("0x37567c __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x375b7c — __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
pub fn stub_375b7c() -> ! {
    todo!("0x375b7c __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::preloadSound(void)")]
// 0x375be0 — __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
pub fn stub_375be0() -> ! {
    todo!("0x375be0 __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::updateLooped(void)")]
// 0x375c8c — __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
pub fn stub_375c8c() -> ! {
    todo!("0x375c8c __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv")
}

#[doc(alias = "RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")]
// 0x375dd4 — __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
pub fn stub_375dd4() -> ! {
    todo!("0x375dd4 __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb")
}

#[doc(alias = "RBX::registerSound(void)")]
// 0x376198 — __ZN3RBX13registerSoundEv
pub fn stub_376198() -> ! {
    todo!("0x376198 __ZN3RBX13registerSoundEv")
}

#[doc(alias = "RBX::Soundscape::Sound::~Sound()")]
// 0x37619c — __ZN3RBX10Soundscape5SoundD2Ev
pub fn stub_37619c() -> ! {
    todo!("0x37619c __ZN3RBX10Soundscape5SoundD2Ev")
}

#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
pub fn stub_376ac4() -> ! {
    todo!("0x376ac4 __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
pub fn stub_376c84() -> ! {
    todo!("0x376c84 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_")
}

#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
pub fn stub_376fb8() -> ! {
    todo!("0x376fb8 __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getPlayCount(void)const")]
// 0x37706c — __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
pub fn stub_37706c() -> ! {
    todo!("0x37706c __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")]
// 0x377988 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
pub fn stub_377988() -> ! {
    todo!("0x377988 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
// 0x3779d8 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
pub fn stub_3779d8() -> ! {
    todo!("0x3779d8 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::construct_func(char const*,char *)")]
// 0x377a44 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc
pub fn stub_377a44() -> ! {
    todo!("0x377a44 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::destruct_func(char *)")]
// 0x377a50 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc
pub fn stub_377a50() -> ! {
    todo!("0x377a50 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x377b20 — __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_377b20() -> ! {
    todo!("0x377b20 __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")]
// 0x378478 — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv
pub fn stub_378478() -> ! {
    todo!("0x378478 __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
// 0x37847c — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
pub fn stub_37847c() -> ! {
    todo!("0x37847c __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")
}

#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37aacc — __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_37aacc() -> ! {
    todo!("0x37aacc __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x37bc94 — __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_37bc94() -> ! {
    todo!("0x37bc94 __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37bcec — __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_37bcec() -> ! {
    todo!("0x37bcec __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
// 0x37bddc — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
pub fn stub_37bddc() -> ! {
    todo!("0x37bddc __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")]
// 0x37be48 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc
pub fn stub_37be48() -> ! {
    todo!("0x37be48 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")]
// 0x37be64 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc
pub fn stub_37be64() -> ! {
    todo!("0x37be64 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")]
// 0x37c60c — __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv
pub fn stub_37c60c() -> ! {
    todo!("0x37c60c __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
// 0x37c610 — __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
pub fn stub_37c610() -> ! {
    todo!("0x37c610 __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")
}

#[doc(alias = "RBX::StockSound::~StockSound()")]
// 0x37c934 — __ZN3RBX10StockSoundD1Ev
pub fn stub_37c934() -> ! {
    todo!("0x37c934 __ZN3RBX10StockSoundD1Ev")
}

#[doc(alias = "RBX::StockSound::~StockSound()")]
// 0x37c938 — __ZN3RBX10StockSoundD0Ev
pub fn stub_37c938() -> ! {
    todo!("0x37c938 __ZN3RBX10StockSoundD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37c9e8 — __ZThn32_N3RBX10StockSoundD1Ev
pub fn stub_37c9e8() -> ! {
    todo!("0x37c9e8 __ZThn32_N3RBX10StockSoundD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37c9f0 — __ZThn32_N3RBX10StockSoundD0Ev
pub fn stub_37c9f0() -> ! {
    todo!("0x37c9f0 __ZThn32_N3RBX10StockSoundD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37caa4 — __ZThn36_N3RBX10StockSoundD1Ev
pub fn stub_37caa4() -> ! {
    todo!("0x37caa4 __ZThn36_N3RBX10StockSoundD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37caac — __ZThn36_N3RBX10StockSoundD0Ev
pub fn stub_37caac() -> ! {
    todo!("0x37caac __ZThn36_N3RBX10StockSoundD0Ev")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
// 0x37d49c — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
pub fn stub_37d49c() -> ! {
    todo!("0x37d49c __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
// 0x37d4d0 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
pub fn stub_37d4d0() -> ! {
    todo!("0x37d4d0 __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
// 0x37d4f8 — __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_37d4f8() -> ! {
    todo!("0x37d4f8 __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_37d550() -> ! {
    todo!("0x37d550 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_37d604() -> ! {
    todo!("0x37d604 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d65c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_37d65c() -> ! {
    todo!("0x37d65c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
// 0x37d6c4 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_37d6c4() -> ! {
    todo!("0x37d6c4 __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
// 0x37d7a8 — __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
pub fn stub_37d7a8() -> ! {
    todo!("0x37d7a8 __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
// 0x37d7c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
pub fn stub_37d7c0() -> ! {
    todo!("0x37d7c0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
// 0x37d7fc — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_37d7fc() -> ! {
    todo!("0x37d7fc __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
// 0x37de98 — __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
pub fn stub_37de98() -> ! {
    todo!("0x37de98 __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
pub fn stub_37e05c() -> ! {
    todo!("0x37e05c __ZN21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
pub fn stub_37e098() -> ! {
    todo!("0x37e098 __ZN21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "SoundServiceStatsItem::update(void)")]
// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
pub fn stub_37e16c() -> ! {
    todo!("0x37e16c __ZN21SoundServiceStatsItem6updateEv")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
pub fn stub_37e344() -> ! {
    todo!("0x37e344 __ZThn32_N21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
pub fn stub_37e384() -> ! {
    todo!("0x37e384 __ZThn32_N21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
pub fn stub_37e458() -> ! {
    todo!("0x37e458 __ZThn36_N21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
pub fn stub_37e498() -> ! {
    todo!("0x37e498 __ZThn36_N21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
pub fn stub_37e86c() -> ! {
    todo!("0x37e86c __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
pub fn stub_37e9c4() -> ! {
    todo!("0x37e9c4 __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
pub fn stub_37e9c8() -> ! {
    todo!("0x37e9c8 __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_37ea68() -> ! {
    todo!("0x37ea68 __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_37ea84() -> ! {
    todo!("0x37ea84 __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
// 0x37eab0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_37eab0() -> ! {
    todo!("0x37eab0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
// 0x37f7cc — __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
pub fn stub_37f7cc() -> ! {
    todo!("0x37f7cc __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x37fd64 — __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_37fd64() -> ! {
    todo!("0x37fd64 __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37fdbc — __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_37fdbc() -> ! {
    todo!("0x37fdbc __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
// 0x37feac — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
pub fn stub_37feac() -> ! {
    todo!("0x37feac __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
// 0x37fee0 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
pub fn stub_37fee0() -> ! {
    todo!("0x37fee0 __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
// 0x37ff08 — __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_37ff08() -> ! {
    todo!("0x37ff08 __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x37ff60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_37ff60() -> ! {
    todo!("0x37ff60 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x380014 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_380014() -> ! {
    todo!("0x380014 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x38006c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_38006c() -> ! {
    todo!("0x38006c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
// 0x3800d4 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_3800d4() -> ! {
    todo!("0x3800d4 __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
// 0x3801b8 — __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
pub fn stub_3801b8() -> ! {
    todo!("0x3801b8 __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
// 0x3801d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
pub fn stub_3801d0() -> ! {
    todo!("0x3801d0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
// 0x38020c — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_38020c() -> ! {
    todo!("0x38020c __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")]
// 0x434d00 — __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
pub fn stub_434d00() -> ! {
    todo!("0x434d00 __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v")
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
// 0x44558c — __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
pub fn stub_44558c() -> ! {
    todo!("0x44558c __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// 0x445848 — __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
pub fn stub_445848() -> ! {
    todo!("0x445848 __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")]
// 0x44588c — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv
pub fn stub_44588c() -> ! {
    todo!("0x44588c __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// 0x445890 — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
pub fn stub_445890() -> ! {
    todo!("0x445890 __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Soundscape::SoundService>(void)")]
// 0x445974 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10Soundscape12SoundServiceEEEvv
pub fn stub_445974() -> ! {
    todo!("0x445974 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10Soundscape12SoundServiceEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
// 0x445978 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv
pub fn stub_445978() -> ! {
    todo!("0x445978 __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv")
}

#[doc(alias = "DummyJob::DummyJob(bool,double)")]
// 0x48d9bc — __ZN8DummyJobC2Ebd
pub fn stub_48d9bc() -> ! {
    todo!("0x48d9bc __ZN8DummyJobC2Ebd")
}

#[doc(alias = "DummyJob::~DummyJob()")]
// 0x48db88 — __ZN8DummyJobD1Ev
pub fn stub_48db88() -> ! {
    todo!("0x48db88 __ZN8DummyJobD1Ev")
}

#[doc(alias = "DummyJob::~DummyJob()")]
// 0x48db8c — __ZN8DummyJobD0Ev
pub fn stub_48db8c() -> ! {
    todo!("0x48db8c __ZN8DummyJobD0Ev")
}

#[doc(alias = "DummyJob::getPriorityFactor(void)")]
// 0x48dc4c — __ZN8DummyJob17getPriorityFactorEv
pub fn stub_48dc4c() -> ! {
    todo!("0x48dc4c __ZN8DummyJob17getPriorityFactorEv")
}

#[doc(alias = "global constructor keyed to_a_182")]
// 0x48ddb0 — __GLOBAL__I_a_182
pub fn stub_48ddb0() -> ! {
    todo!("0x48ddb0 __GLOBAL__I_a_182")
}

#[doc(alias = "global constructor keyed to_a_183")]
// 0x493248 — __GLOBAL__I_a_183
pub fn stub_493248() -> ! {
    todo!("0x493248 __GLOBAL__I_a_183")
}

#[doc(alias = "global constructor keyed to_a_184")]
// 0x49519c — __GLOBAL__I_a_184
pub fn stub_49519c() -> ! {
    todo!("0x49519c __GLOBAL__I_a_184")
}

#[doc(alias = "global constructor keyed to_a_185")]
// 0x49aee0 — __GLOBAL__I_a_185
pub fn stub_49aee0() -> ! {
    todo!("0x49aee0 __GLOBAL__I_a_185")
}

#[doc(alias = "global constructor keyed to_a_186")]
// 0x49b3fc — __GLOBAL__I_a_186
pub fn stub_49b3fc() -> ! {
    todo!("0x49b3fc __GLOBAL__I_a_186")
}

#[doc(alias = "global constructor keyed to_a_187")]
// 0x49f33c — __GLOBAL__I_a_187
pub fn stub_49f33c() -> ! {
    todo!("0x49f33c __GLOBAL__I_a_187")
}

#[doc(alias = "global constructor keyed to_a_188")]
// 0x4a6898 — __GLOBAL__I_a_188
pub fn stub_4a6898() -> ! {
    todo!("0x4a6898 __GLOBAL__I_a_188")
}

#[doc(alias = "global constructor keyed to_a_189")]
// 0x4a9168 — __GLOBAL__I_a_189
pub fn stub_4a9168() -> ! {
    todo!("0x4a9168 __GLOBAL__I_a_189")
}

#[doc(alias = "global constructor keyed to_a_190")]
// 0x4aa5e8 — __GLOBAL__I_a_190
pub fn stub_4aa5e8() -> ! {
    todo!("0x4aa5e8 __GLOBAL__I_a_190")
}

#[doc(alias = "global constructor keyed to_a_191")]
// 0x4e1618 — __GLOBAL__I_a_191
pub fn stub_4e1618() -> ! {
    todo!("0x4e1618 __GLOBAL__I_a_191")
}

#[doc(alias = "global constructor keyed to_a_192")]
// 0x4ed6e0 — __GLOBAL__I_a_192
pub fn stub_4ed6e0() -> ! {
    todo!("0x4ed6e0 __GLOBAL__I_a_192")
}

#[doc(alias = "global constructor keyed to_a_193")]
// 0x4eecb8 — __GLOBAL__I_a_193
pub fn stub_4eecb8() -> ! {
    todo!("0x4eecb8 __GLOBAL__I_a_193")
}

#[doc(alias = "global constructor keyed to_a_194")]
// 0x4ef44c — __GLOBAL__I_a_194
pub fn stub_4ef44c() -> ! {
    todo!("0x4ef44c __GLOBAL__I_a_194")
}

#[doc(alias = "global constructor keyed to_a_195")]
// 0x4f1070 — __GLOBAL__I_a_195
pub fn stub_4f1070() -> ! {
    todo!("0x4f1070 __GLOBAL__I_a_195")
}

#[doc(alias = "global constructor keyed to_a_196")]
// 0x4f3080 — __GLOBAL__I_a_196
pub fn stub_4f3080() -> ! {
    todo!("0x4f3080 __GLOBAL__I_a_196")
}

#[doc(alias = "global constructor keyed to_a_197")]
// 0x4f7894 — __GLOBAL__I_a_197
pub fn stub_4f7894() -> ! {
    todo!("0x4f7894 __GLOBAL__I_a_197")
}

#[doc(alias = "global constructor keyed to_a_198")]
// 0x4f8e28 — __GLOBAL__I_a_198
pub fn stub_4f8e28() -> ! {
    todo!("0x4f8e28 __GLOBAL__I_a_198")
}

#[doc(alias = "global constructor keyed to_a_199")]
// 0x4fac70 — __GLOBAL__I_a_199
pub fn stub_4fac70() -> ! {
    todo!("0x4fac70 __GLOBAL__I_a_199")
}

#[doc(alias = "global constructor keyed to_a_200")]
// 0x500254 — __GLOBAL__I_a_200
pub fn stub_500254() -> ! {
    todo!("0x500254 __GLOBAL__I_a_200")
}

#[doc(alias = "global constructor keyed to_a_201")]
// 0x504a0c — __GLOBAL__I_a_201
pub fn stub_504a0c() -> ! {
    todo!("0x504a0c __GLOBAL__I_a_201")
}
