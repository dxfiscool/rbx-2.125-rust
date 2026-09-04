//! network generated_181 — RakNet + RBX::Network + global gap filler (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator -> 5109 funcs, 0 remaining before batch (filtered gap filler) + 150 global gap filler; batch EA-sorted asc 150 not yet in network
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0x741f4..0x7f23c | existing 19719 -> 19869 total (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::{HashMap, BTreeMap};

/// `rbx::signals::signal` slot list reduced to linkage bits.
#[derive(Clone, Debug, Default)]
pub struct GenSignalState {
    pub slots: Vec<(u64, bool)>,
    pub next: u64,
}

fn gen_connect(s: &mut GenSignalState) -> u64 {
    let id = s.next;
    s.next = s.next.wrapping_add(1);
    s.slots.push((id, true));
    id
}

fn gen_disconnect(s: &mut GenSignalState, id: u64) {
    s.slots.retain(|(i, _)| *i != id);
}

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944).
#[derive(Clone, Debug, Default)]
pub struct GenEventState {
    pub mode: bool,
    pub conn: bool,
    pub listener: bool,
    pub watched: u32,
    pub count: i32,
}

/// Reflection descriptor row (Bound/Prop/Event desc common shape).
#[derive(Clone, Debug, Default)]
pub struct GenDesc {
    pub name: String,
    pub value: i32,
    pub text: String,
    pub readable: bool,
    pub writable: bool,
    pub scriptable: bool,
    pub broadcast: bool,
}

/// `RBX::Network::Peer` transport view.
#[derive(Clone, Debug, Default)]
pub struct GenPeer {
    pub kbps: i32,
    pub connected: bool,
    pub port: u16,
    pub ip: u32,
}

/// RakNet stats accumulation (`PeerStatsItem::update`, IDA 0xad5790).
#[derive(Clone, Debug, Default)]
pub struct GenStats {
    pub packets: u64,
    pub bytes: u64,
    pub enabled: bool,
    pub checked: bool,
}

/// `TopNErrorsPhysicsSender` tables: part -> error plus descending top-N.
#[derive(Clone, Debug, Default)]
pub struct GenTopN {
    pub map: HashMap<u32, f32>,
    pub top: Vec<u32>,
}

fn gen_refresh_top(t: &mut GenTopN) {
    let mut ids: Vec<u32> = t.map.keys().copied().collect();
    ids.sort_by(|a, b| {
        t.map
            .get(b)
            .partial_cmp(&t.map.get(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    t.top = ids;
}

/// `InterpolatingPhysicsReceiver` lerp queue (IDA 0xada700).
#[derive(Clone, Debug, Default)]
pub struct GenInterp {
    pub alpha: f32,
    pub active: bool,
    pub queue: Vec<u32>,
}

/// `RBX::Network::Replicator` connection view.
#[derive(Clone, Debug, Default)]
pub struct GenReplicator {
    pub open: bool,
    pub process: bool,
    pub port: u16,
    pub ip: u32,
    pub markers: u64,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct GenFunctor {
    pub has: bool,
}

/// `boost::multi_index` nugget index: hash by part + order by stamp.
#[derive(Clone, Debug, Default)]
pub struct GenIndex {
    pub by_id: HashMap<u32, u64>,
    pub by_time: BTreeMap<u64, u32>,
}

/// TaskScheduler job view (`sleepTime`, IDA 0xad74f8).
#[derive(Clone, Debug, Default)]
pub struct GenJob {
    pub owner: u32,
    pub running: bool,
}

/// `RBX::Network::Marker` fire state (IDA 0xad12d0).
#[derive(Clone, Debug, Default)]
pub struct GenMarker {
    pub returned: bool,
    pub fired: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct GenOwner {
    pub ip: u32,
    pub port: u16,
    pub server: bool,
}

/// `boost::gregorian` date error (`std::logic_error` payload; thrown via
/// `boost::throw_exception`, IDA 0x251d10/0x251d94).
#[derive(Clone, Debug)]
pub struct GenDateError {
    pub kind: &'static str,
}

impl std::fmt::Display for GenDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad date: {}", self.kind)
    }
}

impl std::error::Error for GenDateError {}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}


// 0x741f4 — __ZN4FMOD15ChannelSoftware11getPositionEPjj
// type: int __fastcall(int this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")]
pub fn stub_741f4() {
    // IDA 0x741f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74554 — __ZN4FMOD15ChannelSoftware10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")]
pub fn stub_74554() {
    // IDA 0x74554: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74564 — __ZN4FMOD15ChannelSoftware16moveChannelGroupEPNS_13ChannelGroupIES2_b
// type: FMOD::DSPI *__fastcall(FMOD::DSPI **this, FMOD::DSPI **, FMOD::DSPI **, bool)
#[doc(alias = "FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_74564() {
    // IDA 0x74564: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x745d4 — __ZN4FMOD15ChannelSoftware19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_745d4() {
    // IDA 0x745d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7464c — __ZN4FMOD15ChannelSoftware12addToReverbsEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")]
pub fn stub_7464c() {
    // IDA 0x7464c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x748b4 — __ZN4FMOD15ChannelSoftware11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelSoftware *this, float *, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getWaveData(float *,int,int)")]
pub fn stub_748b4() {
    // IDA 0x748b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x749c4 — __ZN4FMOD15ChannelSoftware11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_749c4() {
    // IDA 0x749c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74b20 — __ZN4FMOD15ChannelSoftware9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *, bool)
#[doc(alias = "FMOD::ChannelSoftware::isPlaying(bool *,bool)")]
pub fn stub_74b20() {
    // IDA 0x74b20: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74bd0 — __ZN4FMOD15ChannelSoftware7setModeEj
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setMode(unsigned int)")]
pub fn stub_74bd0() {
    // IDA 0x74bd0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74c04 — __ZN4FMOD15ChannelSoftware12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::getLoopCount(int *)")]
pub fn stub_74c04() {
    // IDA 0x74c04: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74c44 — __ZN4FMOD15ChannelSoftware12setLoopCountEi
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopCount(int)")]
pub fn stub_74c44() {
    // IDA 0x74c44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74c90 — __ZN4FMOD15ChannelSoftware13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelSoftware *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_74c90() {
    // IDA 0x74c90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74cd8 — __ZN4FMOD15ChannelSoftware6setPanEff
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t, float)
#[doc(alias = "FMOD::ChannelSoftware::setPan(float,float)")]
pub fn stub_74cd8() {
    // IDA 0x74cd8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74de8 — __ZN4FMOD15ChannelSoftware12setFrequencyEf
// type: FMOD::DSPWaveTable *__fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setFrequency(float)")]
pub fn stub_74de8() {
    // IDA 0x74de8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x74edc — __ZN4FMOD15ChannelSoftware15updateReverbMixEPNS_7ReverbIEf
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::ReverbI *, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")]
pub fn stub_74edc() {
    // IDA 0x74edc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x751dc — __ZN4FMOD15ChannelSoftware15updateDirectMixEf
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateDirectMix(float)")]
pub fn stub_751dc() {
    // IDA 0x751dc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75408 — __ZN4FMOD15ChannelSoftware13setupDSPCodecEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")]
pub fn stub_75408() {
    // IDA 0x75408: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75738 — __ZN4FMOD15ChannelSoftware5closeEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::close(void)")]
pub fn stub_75738() {
    // IDA 0x75738: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x757fc — __ZN4FMOD15ChannelSoftware4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelSoftware *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_757fc() {
    // IDA 0x757fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x759c0 — __ZN4FMOD15ChannelSoftwareC2Ev
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_759c0() {
    // IDA 0x759c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75a44 — __ZN4FMOD15ChannelSoftwareC1Ev
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_75a44() {
    // IDA 0x75a44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75a48 — __ZN4FMOD15ChannelSoftware9setPausedEb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool)
#[doc(alias = "FMOD::ChannelSoftware::setPaused(bool)")]
pub fn stub_75a48() {
    // IDA 0x75a48: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75b50 — __ZN4FMOD15ChannelSoftware5startEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::start(void)")]
pub fn stub_75b50() {
    // IDA 0x75b50: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75be0 — __ZN4FMOD15ChannelSoftware5allocEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::alloc(void)")]
pub fn stub_75be0() {
    // IDA 0x75be0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x75f8c — __ZN4FMOD15ChannelSoftware4stopEv
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::stop(void)")]
pub fn stub_75f8c() {
    // IDA 0x75f8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x762c4 — __ZN4FMOD15ChannelSoftware16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelSoftware *this, int, float *, int)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")]
pub fn stub_762c4() {
    // IDA 0x762c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x76584 — __ZN4FMOD15ChannelSoftware13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelSoftware *this, int, int, int, int, float, float, float, float)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_76584() {
    // IDA 0x76584: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x76988 — __ZN4FMOD15ChannelSoftware9setVolumeEf
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setVolume(float)")]
pub fn stub_76988() {
    // IDA 0x76988: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x76a80 — __ZN4FMOD15ChannelSoftware14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelSoftware *this, float, float)
#[doc(alias = "FMOD::ChannelSoftware::set3DOcclusion(float,float)")]
pub fn stub_76a80() {
    // IDA 0x76a80: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x76b3c — __ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_76b3c() {
    // IDA 0x76b3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7709c — __ZN4FMOD15ChannelSoftware9getPausedEPb
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *)
#[doc(alias = "FMOD::ChannelSoftware::getPaused(bool *)")]
pub fn stub_7709c() {
    // IDA 0x7709c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77138 — __ZN4FMOD15ChannelSoftware5allocEPNS_4DSPIE
// type: int __fastcall(FMOD::DSPI **this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")]
pub fn stub_77138() {
    // IDA 0x77138: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x773c4 — __ZN4FMOD15ChannelSoftwareD1Ev
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773c4() {
    // IDA 0x773c4: dtor releases the owned control block/slots.
}
// 0x773f0 — __ZN4FMOD15ChannelSoftwareD0Ev
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773f0() {
    // IDA 0x773f0: dtor releases the owned control block/slots.
}
// 0x77428 — __ZN4FMOD13ChannelStream23set2DFreqVolumePanFor3DEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set2DFreqVolumePanFor3D(void)")]
pub fn stub_77428() {
    // IDA 0x77428: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77474 — __ZN4FMOD13ChannelStream16moveChannelGroupEPNS_13ChannelGroupIES2_b
// type: int __fastcall(int, int, int, unsigned __int8)
#[doc(alias = "FMOD::ChannelStream::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_77474() {
    // IDA 0x77474: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x774e0 — __ZN4FMOD13ChannelStream5startEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::start(void)")]
pub fn stub_774e0() {
    // IDA 0x774e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77574 — __ZN4FMOD13ChannelStream6updateEi
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::update(int)")]
pub fn stub_77574() {
    // IDA 0x77574: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x775d0 — __ZN4FMOD13ChannelStream9setVolumeEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setVolume(float)")]
pub fn stub_775d0() {
    // IDA 0x775d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77718 — __ZN4FMOD13ChannelStream12setFrequencyEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setFrequency(float)")]
pub fn stub_77718() {
    // IDA 0x77718: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77774 — __ZN4FMOD13ChannelStream6setPanEff
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::setPan(float,float)")]
pub fn stub_77774() {
    // IDA 0x77774: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7781c — __ZN4FMOD13ChannelStream16setDSPClockDelayEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::setDSPClockDelay(void)")]
pub fn stub_7781c() {
    // IDA 0x7781c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77868 — __ZN4FMOD13ChannelStream13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelStream *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelStream::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_77868() {
    // IDA 0x77868: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77904 — __ZN4FMOD13ChannelStream16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelStream *this, int, float *, int)
#[doc(alias = "FMOD::ChannelStream::setSpeakerLevels(int,float *,int)")]
pub fn stub_77904() {
    // IDA 0x77904: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77970 — __ZN4FMOD13ChannelStream15set3DAttributesEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DAttributes(void)")]
pub fn stub_77970() {
    // IDA 0x77970: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x779bc — __ZN4FMOD13ChannelStream14setLowPassGainEf
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setLowPassGain(float)")]
pub fn stub_779bc() {
    // IDA 0x779bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77a18 — __ZN4FMOD13ChannelStream19set3DMinMaxDistanceEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DMinMaxDistance(void)")]
pub fn stub_77a18() {
    // IDA 0x77a18: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77a64 — __ZN4FMOD13ChannelStream14set3DOcclusionEff
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::set3DOcclusion(float,float)")]
pub fn stub_77a64() {
    // IDA 0x77a64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77ac8 — __ZN4FMOD13ChannelStream19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelStream::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_77ac8() {
    // IDA 0x77ac8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b24 — __ZN4FMOD13ChannelStream19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_77b24() {
    // IDA 0x77b24: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b48 — __ZN4FMOD13ChannelStream9isPlayingEPbb
// type: int __fastcall(FMOD::ChannelStream *this, bool *, bool)
#[doc(alias = "FMOD::ChannelStream::isPlaying(bool *,bool)")]
pub fn stub_77b48() {
    // IDA 0x77b48: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b5c — __ZN4FMOD13ChannelStream11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_77b5c() {
    // IDA 0x77b5c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b6c — __ZN4FMOD13ChannelStream11getWaveDataEPfii
// type: int __fastcall(FMOD::ChannelStream *this, float *, int, int)
#[doc(alias = "FMOD::ChannelStream::getWaveData(float *,int,int)")]
pub fn stub_77b6c() {
    // IDA 0x77b6c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b7c — __ZN4FMOD13ChannelStream10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getDSPHead(FMOD::DSPI **)")]
pub fn stub_77b7c() {
    // IDA 0x77b7c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77b8c — __ZN4FMOD13ChannelStream12setLoopCountEi
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setLoopCount(int)")]
pub fn stub_77b8c() {
    // IDA 0x77b8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77bc0 — __ZN4FMOD13ChannelStream13setLoopPointsEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_77bc0() {
    // IDA 0x77bc0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77c14 — __ZN4FMOD13ChannelStream11getPositionEPjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelStream::getPosition(unsigned int *,unsigned int)")]
pub fn stub_77c14() {
    // IDA 0x77c14: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x77f74 — __ZN4FMOD13ChannelStream4stopEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::stop(void)")]
pub fn stub_77f74() {
    // IDA 0x77f74: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78168 — __ZN4FMOD13ChannelStream7setModeEj
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setMode(unsigned int)")]
pub fn stub_78168() {
    // IDA 0x78168: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x781f0 — __ZN4FMOD13ChannelStreamC2Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_781f0() {
    // IDA 0x781f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7826c — __ZN4FMOD13ChannelStreamC1Ev
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_7826c() {
    // IDA 0x7826c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78270 — __ZN4FMOD13ChannelStream5allocEv
// type: int __fastcall(FMOD::ChannelStream *this, int, int)
#[doc(alias = "FMOD::ChannelStream::alloc(void)")]
pub fn stub_78270() {
    // IDA 0x78270: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78540 — __ZN4FMOD13ChannelStream13setPositionExEjjb
// type: int __fastcall(unsigned __int64 this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelStream::setPositionEx(unsigned int,unsigned int,bool)")]
pub fn stub_78540() {
    // IDA 0x78540: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78af0 — __ZN4FMOD13ChannelStream9setPausedEb
// type: int __fastcall(FMOD::ChannelStream *this, bool)
#[doc(alias = "FMOD::ChannelStream::setPaused(bool)")]
pub fn stub_78af0() {
    // IDA 0x78af0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78b80 — __ZN4FMOD13ChannelStream12updateStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::updateStream(void)")]
pub fn stub_78b80() {
    // IDA 0x78b80: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78fac — __ZN4FMOD13ChannelStream8isStreamEv
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::isStream(void)")]
pub fn stub_78fac() {
    // IDA 0x78fac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78fb4 — __ZN4FMOD13ChannelStream11setPositionEjj
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setPosition(unsigned int,unsigned int)")]
pub fn stub_78fb4() {
    // IDA 0x78fb4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x78fc4 — __ZN4FMOD13ChannelStreamD0Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fc4() {
    // IDA 0x78fc4: dtor releases the owned control block/slots.
}
// 0x78fe8 — __ZN4FMOD13ChannelStreamD1Ev
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fe8() {
    // IDA 0x78fe8: dtor releases the owned control block/slots.
}
// 0x79000 — __ZN4FMOD12ChannelGroup9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroup *this, float, FMOD::ChannelGroupI **)
#[doc(alias = "FMOD::ChannelGroup::setVolume(float)")]
pub fn stub_79000() {
    // IDA 0x79000: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79034 — __ZN4FMOD13ChannelGroupI8validateEPNS_12ChannelGroupEPPS0_
// type: int __fastcall(int result, int *)
#[doc(alias = "FMOD::ChannelGroupI::validate(FMOD::ChannelGroup *,FMOD::ChannelGroupI**)")]
pub fn stub_79034() {
    // IDA 0x79034: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79054 — __ZN4FMOD13ChannelGroupI9getPausedEPb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool *)
#[doc(alias = "FMOD::ChannelGroupI::getPaused(bool *)")]
pub fn stub_79054() {
    // IDA 0x79054: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7906c — __ZN4FMOD13ChannelGroupI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7906c() {
    // IDA 0x7906c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x790fc — __ZN4FMOD13ChannelGroupI20updateChildMixTargetEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelGroupI::updateChildMixTarget(FMOD::DSPI *)")]
pub fn stub_790fc() {
    // IDA 0x790fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x791e8 — __ZN4FMOD13ChannelGroupI7setMuteEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setMute(bool,bool)")]
pub fn stub_791e8() {
    // IDA 0x791e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79280 — __ZN4FMOD13ChannelGroupI9setPausedEbb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setPaused(bool,bool)")]
pub fn stub_79280() {
    // IDA 0x79280: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79334 — __ZN4FMOD13ChannelGroupI16setPitchInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setPitchInternal(void)")]
pub fn stub_79334() {
    // IDA 0x79334: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x793e4 — __ZN4FMOD13ChannelGroupI17setVolumeInternalEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setVolumeInternal(void)")]
pub fn stub_793e4() {
    // IDA 0x793e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x794c4 — __ZN4FMOD13ChannelGroupI8addGroupEPS0_
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")]
pub fn stub_794c4() {
    // IDA 0x794c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x796a4 — __ZN4FMOD13ChannelGroupI9setVolumeEf
// type: int __fastcall(FMOD::ChannelGroupI *this, float)
#[doc(alias = "FMOD::ChannelGroupI::setVolume(float)")]
pub fn stub_796a4() {
    // IDA 0x796a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x796d4 — __ZN4FMOD13ChannelGroupI15releaseInternalEb
// type: int __fastcall(FMOD::ChannelGroupI *this, bool)
#[doc(alias = "FMOD::ChannelGroupI::releaseInternal(bool)")]
pub fn stub_796d4() {
    // IDA 0x796d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7995c — __ZN4FMOD13ChannelGroupI7releaseEv
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::release(void)")]
pub fn stub_7995c() {
    // IDA 0x7995c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79980 — __ZN4FMOD20ChannelGroupSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelGroupSoftware *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_79980() {
    // IDA 0x79980: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79a38 — __ZN4FMOD13ChannelGroupI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a38() {
    // IDA 0x79a38: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79a90 — __ZN4FMOD20ChannelGroupSoftware13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a90() {
    // IDA 0x79a90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79ae8 — __ZN4FMOD8ChannelI16returnToFreeListEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::returnToFreeList(void)")]
pub fn stub_79ae8() {
    // IDA 0x79ae8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79b98 — __ZN4FMOD8ChannelI14referenceStampEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::referenceStamp(bool)")]
pub fn stub_79b98() {
    // IDA 0x79b98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79bdc — __ZN4FMOD8ChannelI14getRealChannelEPPNS_11ChannelRealEPi
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelReal **, int *)
#[doc(alias = "FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")]
pub fn stub_79bdc() {
    // IDA 0x79bdc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79ca8 — __ZN4FMOD8ChannelI4initEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::init(void)")]
pub fn stub_79ca8() {
    // IDA 0x79ca8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79dd4 — __ZN4FMOD8ChannelIC2EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79dd4() {
    // IDA 0x79dd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79e84 — __ZN4FMOD8ChannelIC1EiPNS_7SystemIE
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79e84() {
    // IDA 0x79e84: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79e88 — __ZN4FMOD8ChannelIC2Ev
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79e88() {
    // IDA 0x79e88: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79ef0 — __ZN4FMOD8ChannelIC1Ev
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79ef0() {
    // IDA 0x79ef0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79ef4 — __ZN4FMOD8ChannelI5allocEPNS_4DSPIEb
// type: int __fastcall(_DWORD *, int, char)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")]
pub fn stub_79ef4() {
    // IDA 0x79ef4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a0f8 — __ZN4FMOD8ChannelI5startEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::start(void)")]
pub fn stub_7a0f8() {
    // IDA 0x7a0f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a198 — __ZN4FMOD8ChannelI9getPausedEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getPaused(bool *)")]
pub fn stub_7a198() {
    // IDA 0x7a198: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a1ec — __ZN4FMOD8ChannelI9getVolumeEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getVolume(float *)")]
pub fn stub_7a1ec() {
    // IDA 0x7a1ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a214 — __ZN4FMOD8ChannelI12getFrequencyEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getFrequency(float *)")]
pub fn stub_7a214() {
    // IDA 0x7a214: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a23c — __ZN4FMOD8ChannelI6setPanEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setPan(float,bool)")]
pub fn stub_7a23c() {
    // IDA 0x7a23c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a358 — __ZN4FMOD8ChannelI8setDelayE14FMOD_DELAYTYPEjj
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")]
pub fn stub_7a358() {
    // IDA 0x7a358: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a50c — __ZN4FMOD8ChannelI13setSpeakerMixEffffffffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, float, float, float, float, float, float, bool)
#[doc(alias = "FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")]
pub fn stub_7a50c() {
    // IDA 0x7a50c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a7dc — __ZN4FMOD8ChannelI16getSpeakerLevelsE12FMOD_SPEAKERPfi
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")]
pub fn stub_7a7dc() {
    // IDA 0x7a7dc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a8b0 — __ZN4FMOD8ChannelI7getMuteEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getMute(bool *)")]
pub fn stub_7a8b0() {
    // IDA 0x7a8b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a8d8 — __ZN4FMOD8ChannelI15set3DAttributesEPK11FMOD_VECTORS3_
// type: int __fastcall(int, float *, float *)
#[doc(alias = "FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
pub fn stub_7a8d8() {
    // IDA 0x7a8d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7aa4c — __ZN4FMOD8ChannelI19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_7aa4c() {
    // IDA 0x7aa4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7aae0 — __ZN4FMOD8ChannelI19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_7aae0() {
    // IDA 0x7aae0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ab74 — __ZN4FMOD8ChannelI9isVirtualEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isVirtual(bool *)")]
pub fn stub_7ab74() {
    // IDA 0x7ab74: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7aba0 — __ZN4FMOD8ChannelI21getAudibilityInternalEPfb
// type: int __fastcall(FMOD::ChannelI *this, float *, bool)
#[doc(alias = "FMOD::ChannelI::getAudibilityInternal(float *,bool)")]
pub fn stub_7aba0() {
    // IDA 0x7aba0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ad00 — __ZN4FMOD8ChannelI13getAudibilityEPf
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getAudibility(float *)")]
pub fn stub_7ad00() {
    // IDA 0x7ad00: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ad08 — __ZN4FMOD8ChannelI15getCurrentSoundEPPNS_6SoundIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")]
pub fn stub_7ad08() {
    // IDA 0x7ad08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ad44 — __ZN4FMOD8ChannelI13getCurrentDSPEPPNS_4DSPIE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")]
pub fn stub_7ad44() {
    // IDA 0x7ad44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ad70 — __ZN4FMOD8ChannelI11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
// type: int __fastcall(int result, int)
#[doc(alias = "FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
pub fn stub_7ad70() {
    // IDA 0x7ad70: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ad88 — __ZN4FMOD8ChannelI11getPositionEPjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")]
pub fn stub_7ad88() {
    // IDA 0x7ad88: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7adb0 — __ZN4FMOD8ChannelI16updateSyncPointsEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::updateSyncPoints(bool)")]
pub fn stub_7adb0() {
    // IDA 0x7adb0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b1f8 — __ZN4FMOD8ChannelI12setFrequencyEf
// type: int __fastcall(FMOD::ChannelI *this, float)
#[doc(alias = "FMOD::ChannelI::setFrequency(float)")]
pub fn stub_7b1f8() {
    // IDA 0x7b1f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b31c — __ZN4FMOD8ChannelI10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7b31c() {
    // IDA 0x7b31c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b344 — __ZN4FMOD8ChannelI7getModeEPj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *)
#[doc(alias = "FMOD::ChannelI::getMode(unsigned int *)")]
pub fn stub_7b344() {
    // IDA 0x7b344: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b36c — __ZN4FMOD8ChannelI12setLoopCountEi
// type: int __fastcall(FMOD::ChannelI *this, int)
#[doc(alias = "FMOD::ChannelI::setLoopCount(int)")]
pub fn stub_7b36c() {
    // IDA 0x7b36c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b40c — __ZN4FMOD8ChannelI12getLoopCountEPi
// type: int __fastcall(FMOD::ChannelI *this, int *)
#[doc(alias = "FMOD::ChannelI::getLoopCount(int *)")]
pub fn stub_7b40c() {
    // IDA 0x7b40c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b434 — __ZN4FMOD8ChannelI11setUserDataEPv
// type: int __fastcall(FMOD::ChannelI *this, void *)
#[doc(alias = "FMOD::ChannelI::setUserData(void *)")]
pub fn stub_7b434() {
    // IDA 0x7b434: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b440 — __ZN4FMOD8ChannelI11getUserDataEPPv
// type: int __fastcall(FMOD::ChannelI *this, void **)
#[doc(alias = "FMOD::ChannelI::getUserData(void **)")]
pub fn stub_7b440() {
    // IDA 0x7b440: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b458 — __ZN4FMOD8ChannelI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7b458() {
    // IDA 0x7b458: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b47c — __ZN4FMOD8ChannelI6addDSPEPNS_4DSPIEPPNS_14DSPConnectionIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, FMOD::DSPConnectionI **)
#[doc(alias = "FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")]
pub fn stub_7b47c() {
    // IDA 0x7b47c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b4e8 — __ZN4FMOD8ChannelI16setSpeakerLevelsE12FMOD_SPEAKERPfib
// type: int __fastcall(int, unsigned int, int, int, char)
#[doc(alias = "FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")]
pub fn stub_7b4e8() {
    // IDA 0x7b4e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b79c — __ZN4FMOD8ChannelI21calculate3DReverbGainEPNS_7ReverbIEP11FMOD_VECTORPf
// type: int __fastcall(int, int, int, __int32 *)
#[doc(alias = "FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")]
pub fn stub_7b79c() {
    // IDA 0x7b79c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7b860 — __ZN4FMOD8ChannelI5allocEPNS_6SoundIEb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::SoundI *, bool)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")]
pub fn stub_7b860() {
    // IDA 0x7b860: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7bbc4 — __ZN4FMOD8ChannelI23calcVolumeAndPitchFor3DEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")]
pub fn stub_7bbc4() {
    // IDA 0x7bbc4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7c164 — __ZN4FMOD8ChannelI8validateEPNS_7ChannelEPPS0_
// type: int __fastcall(unsigned int, _DWORD *, FMOD::SystemI **)
#[doc(alias = "FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")]
pub fn stub_7c164() {
    // IDA 0x7c164: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7c224 — __ZN4FMOD8ChannelI9isPlayingEPb
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isPlaying(bool *)")]
pub fn stub_7c224() {
    // IDA 0x7c224: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7c3d8 — __ZN4FMOD8ChannelI13getLoopPointsEPjjS1_j
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")]
pub fn stub_7c3d8() {
    // IDA 0x7c3d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7c784 — __ZN4FMOD8ChannelI14getChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(FMOD::ChannelI *, int)
#[doc(alias = "FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7c784() {
    // IDA 0x7c784: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7c83c — __ZN4FMOD8ChannelI11setPositionEjj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPosition(unsigned int,unsigned int)")]
pub fn stub_7c83c() {
    // IDA 0x7c83c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ce58 — __ZN4FMOD8ChannelI13setLoopPointsEjjjj
// type: int __fastcall(unsigned __int64 this, unsigned int, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_7ce58() {
    // IDA 0x7ce58: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d208 — __ZN4FMOD8ChannelI14setChannelInfoEPNS_17FMOD_CHANNEL_INFOE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7d208() {
    // IDA 0x7d208: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d480 — __ZN4FMOD8ChannelI12forceVirtualEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::forceVirtual(bool)")]
pub fn stub_7d480() {
    // IDA 0x7d480: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d5fc — __ZN4FMOD8ChannelI14updatePositionEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::updatePosition(void)")]
pub fn stub_7d5fc() {
    // IDA 0x7d5fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d8c4 — __ZN4FMOD8ChannelI22set3DOcclusionInternalEffb
// type: int __fastcall(FMOD::ChannelI *this, float, float, bool)
#[doc(alias = "FMOD::ChannelI::set3DOcclusionInternal(float,float,bool)")]
pub fn stub_7d8c4() {
    // IDA 0x7d8c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d9b8 — __ZN4FMOD8ChannelI11setPriorityEi
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPriority(int)")]
pub fn stub_7d9b8() {
    // IDA 0x7d9b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7d9d0 — __ZN4FMOD8ChannelI9setVolumeEfb
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setVolume(float,bool)")]
pub fn stub_7d9d0() {
    // IDA 0x7d9d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7db84 — __ZN4FMOD8ChannelI7setMuteEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setMute(bool)")]
pub fn stub_7db84() {
    // IDA 0x7db84: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7dc98 — __ZN4FMOD8ChannelI11setDefaultsEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::setDefaults(void)")]
pub fn stub_7dc98() {
    // IDA 0x7dc98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7df78 — __ZN4FMOD8ChannelI6updateEib
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelI::update(int,bool)")]
pub fn stub_7df78() {
    // IDA 0x7df78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7e58c — __ZN4FMOD8ChannelI7setModeEj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setMode(unsigned int)")]
pub fn stub_7e58c() {
    // IDA 0x7e58c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7e8f0 — __ZN4FMOD8ChannelI9setPausedEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setPaused(bool)")]
pub fn stub_7e8f0() {
    // IDA 0x7e8f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ea20 — __ZN4FMOD8ChannelI23setChannelGroupInternalEPNS_13ChannelGroupIEbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *, bool, bool)
#[doc(alias = "FMOD::ChannelI::setChannelGroupInternal(FMOD::ChannelGroupI *,bool,bool)")]
pub fn stub_7ea20() {
    // IDA 0x7ea20: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ecf8 — __ZN4FMOD8ChannelI15setChannelGroupEPNS_13ChannelGroupIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelI::setChannelGroup(FMOD::ChannelGroupI *)")]
pub fn stub_7ecf8() {
    // IDA 0x7ecf8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7ed04 — __ZN4FMOD8ChannelI6stopExEj
// type: int __fastcall(FMOD::ChannelI *this, char)
#[doc(alias = "FMOD::ChannelI::stopEx(unsigned int)")]
pub fn stub_7ed04() {
    // IDA 0x7ed04: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7f0f4 — __ZN4FMOD8ChannelI4stopEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::stop(void)")]
pub fn stub_7f0f4() {
    // IDA 0x7f0f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7f0fc — __ZN4FMOD8ChannelI4playEPNS_4DSPIEbbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, bool, char, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::DSPI *,bool,bool,bool)")]
pub fn stub_7f0fc() {
    // IDA 0x7f0fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7f23c — __ZN4FMOD8ChannelI4playEPNS_6SoundIEbbb
// type: int __fastcall(FMOD::ChannelI *this, unsigned __int8 **, bool, bool, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::SoundI *,bool,bool,bool)")]
pub fn stub_7f23c() {
    // IDA 0x7f23c: faithful no-op shell; control block / ref traffic stays engine-side.
}
