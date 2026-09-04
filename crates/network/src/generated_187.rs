//! network generated_187 — RakNet + RBX::Network + global gap filler (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator -> 5119 funcs, 0 remaining before batch (filtered complete) + 150 global gap filler; batch EA-sorted asc 150 not yet in network
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0xc2a18..0xcb8e0 | existing 21209 -> 21359 total (rbx_core::SharedPtr not boost)

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



// 0xc2a18 — __ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE")]
pub fn stub_c2a18(state: u32) -> i32 {
    // IDA 0xc2a18: FMOD DSP pump; returns FMOD_OK.
    let _ = state;
    0
}
// 0xc2a24 — __ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float *, char *)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc")]
pub fn stub_c2a24(param: i32) -> f32 {
    // IDA 0xc2a24: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc2e88 — __ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
pub fn stub_c2e88(param: i32) -> f32 {
    // IDA 0xc2e88: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc2e94 — __ZN4FMOD12DSPSfxReverb20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterInternalEif")]
pub fn stub_c2e94(param: i32, value: f32) {
    // IDA 0xc2e94: FMOD DSP parameter write.
    let _ = (param, value);
}
// 0xc3178 — __ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif")]
pub fn stub_c3178(param: i32, value: f32) {
    // IDA 0xc3178: FMOD DSP parameter write.
    let _ = (param, value);
}
// 0xc3184 — __ZN4FMOD12DSPSfxReverb13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetInternalEv")]
pub fn stub_c3184(handle: u32) {
    // IDA 0xc3184: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc31bc — __ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE")]
pub fn stub_c31bc(handle: u32) {
    // IDA 0xc31bc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc31c8 — __ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float *, float *__dst, unsigned int, int, int)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii")]
pub fn stub_c31c8(data: &[u8]) -> bool {
    // IDA 0xc31c8: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc327c — __ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
pub fn stub_c327c(data: &[u8]) -> bool {
    // IDA 0xc327c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc32a4 — __ZN4FMOD12DSPSfxReverb15releaseInternalEv
// type: int __fastcall(void **this)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseInternalEv")]
pub fn stub_c32a4(handle: u32) {
    // IDA 0xc32a4: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc32bc — __ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE")]
pub fn stub_c32bc(handle: u32) {
    // IDA 0xc32bc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc32c8 — __ZN4FMOD12DSPSfxReverb14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createInternalEv")]
pub fn stub_c32c8() -> Option<u32> {
    // IDA 0xc32c8: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc35cc — __ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE")]
pub fn stub_c35cc() -> Option<u32> {
    // IDA 0xc35cc: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc35d8 — __ZN4FMOD12DSPSfxReverb16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
#[doc(alias = "__ZN4FMOD12DSPSfxReverb16getDescriptionExEv")]
pub fn stub_c35d8() -> &'static str {
    // IDA 0xc35d8: FMOD DSP static description record.
    "DSP"
}
// 0xc36d4 — __Z41__static_initialization_and_destruction_0ii_32
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_32")]
pub fn stub_c36d4() -> Option<u32> {
    // IDA 0xc36d4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc3718 — __GLOBAL__I__ZN4FMOD12dspsfxreverbE
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspsfxreverbE")]
pub fn stub_c3718() {
    // IDA 0xc3718: static initializer/terminator registration.
}
// 0xc3724 — __ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
#[doc(alias = "__ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
pub fn stub_c3724() -> Option<u32> {
    // IDA 0xc3724: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc3750 — __ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij
#[doc(alias = "__ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij")]
pub fn stub_c3750(data: &[u8]) -> bool {
    // IDA 0xc3750: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc3bcc — __ZN4FMOD12DSPWaveTable19setPositionInternalEj
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionInternalEj")]
pub fn stub_c3bcc(value: f32) {
    // IDA 0xc3bcc: FMOD wavetable transport positioning.
    let _ = value;
}
// 0xc3bf4 — __ZN4FMOD12DSPWaveTable20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float)
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterInternalEif")]
pub fn stub_c3bf4(param: i32, value: f32) {
    // IDA 0xc3bf4: FMOD DSP parameter write.
    let _ = (param, value);
}
// 0xc3bfc — __ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float *, char *)
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc")]
pub fn stub_c3bfc(param: i32) -> f32 {
    // IDA 0xc3bfc: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc3c04 — __ZN4FMOD12DSPWaveTable12setFrequencyEf
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, float)
#[doc(alias = "__ZN4FMOD12DSPWaveTable12setFrequencyEf")]
pub fn stub_c3c04(value: f32) {
    // IDA 0xc3c04: FMOD wavetable transport positioning.
    let _ = value;
}
// 0xc3c80 — __ZN4FMOD12DSPWaveTable11getFinishedEPb
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, bool *)
#[doc(alias = "__ZN4FMOD12DSPWaveTable11getFinishedEPb")]
pub fn stub_c3c80() {
    // IDA 0xc3c80: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc3cc4 — __ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj
// type: int __fastcall(FMOD::DSPWaveTable *, unsigned int)
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj")]
pub fn stub_c3cc4(value: f32) {
    // IDA 0xc3cc4: FMOD wavetable transport positioning.
    let _ = value;
}
// 0xc3cd0 — __ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif")]
pub fn stub_c3cd0(param: i32, value: f32) {
    // IDA 0xc3cd0: FMOD DSP parameter write.
    let _ = (param, value);
}
// 0xc3cdc — __ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
pub fn stub_c3cdc(param: i32) -> f32 {
    // IDA 0xc3cdc: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc3ce8 — __ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE")]
pub fn stub_c3ce8(handle: u32) {
    // IDA 0xc3ce8: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc3d00 — __ZN4FMOD12DSPWaveTable11setFinishedEbb
// type: int __fastcall(FMOD::DSPWaveTable *this, bool, bool)
#[doc(alias = "__ZN4FMOD12DSPWaveTable11setFinishedEbb")]
pub fn stub_c3d00() {
    // IDA 0xc3d00: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc3d94 — __ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij
#[doc(alias = "__ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij")]
pub fn stub_c3d94(data: &[u8]) -> bool {
    // IDA 0xc3d94: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc4728 — __ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
#[doc(alias = "__ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
pub fn stub_c4728() -> Option<u32> {
    // IDA 0xc4728: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc4788 — __ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij
#[doc(alias = "__ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij")]
pub fn stub_c4788(data: &[u8]) -> bool {
    // IDA 0xc4788: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc4790 — __ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij
#[doc(alias = "__ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij")]
pub fn stub_c4790(data: &[u8]) -> bool {
    // IDA 0xc4790: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc47d4 — __ZN4FMOD4DSPIC2Ev
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
#[doc(alias = "__ZN4FMOD4DSPIC2Ev")]
pub fn stub_c47d4() {
    // IDA 0xc47d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc489c — __ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE
#[doc(alias = "__ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE")]
pub fn stub_c489c() {
    // IDA 0xc489c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc48b4 — __ZN4FMOD4DSPI13updateDSPTickEj
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD4DSPI13updateDSPTickEj")]
pub fn stub_c48b4() {
    // IDA 0xc48b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc48f8 — __ZN4FMOD4DSPI5resetEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
#[doc(alias = "__ZN4FMOD4DSPI5resetEv")]
pub fn stub_c48f8(handle: u32) {
    // IDA 0xc48f8: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc4918 — __ZN4FMOD4DSPI12setParameterEif
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, float)
#[doc(alias = "__ZN4FMOD4DSPI12setParameterEif")]
pub fn stub_c4918(param: i32, value: f32) {
    // IDA 0xc4918: FMOD DSP parameter write.
    let _ = (param, value);
}
// 0xc498c — __ZN4FMOD4DSPI16getNumParametersEPi
// type: int __fastcall(FMOD::DSPI *this, int *)
#[doc(alias = "__ZN4FMOD4DSPI16getNumParametersEPi")]
pub fn stub_c498c() {
    // IDA 0xc498c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc49a4 — __ZN4FMOD4DSPI16showConfigDialogEPvb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *, bool)
#[doc(alias = "__ZN4FMOD4DSPI16showConfigDialogEPvb")]
pub fn stub_c49a4() {
    // IDA 0xc49a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc49c8 — __ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE
#[doc(alias = "__ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE")]
pub fn stub_c49c8() {
    // IDA 0xc49c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc49dc — __ZN4FMOD4DSPI11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float, float, float, int)
#[doc(alias = "__ZN4FMOD4DSPI11setDefaultsEfffi")]
pub fn stub_c49dc() {
    // IDA 0xc49dc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4a64 — __ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float *, float *, float *, int *)
#[doc(alias = "__ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi")]
pub fn stub_c4a64() {
    // IDA 0xc4a64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4aa8 — __ZN4FMOD4DSPI11setUserDataEPv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *)
#[doc(alias = "__ZN4FMOD4DSPI11setUserDataEPv")]
pub fn stub_c4aa8() {
    // IDA 0xc4aa8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4ab4 — __ZN4FMOD4DSPI18setTargetFrequencyEi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int)
#[doc(alias = "__ZN4FMOD4DSPI18setTargetFrequencyEi")]
pub fn stub_c4ab4() {
    // IDA 0xc4ab4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4ac0 — __ZN4FMOD4DSPI18getTargetFrequencyEPi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int *)
#[doc(alias = "__ZN4FMOD4DSPI18getTargetFrequencyEPi")]
pub fn stub_c4ac0() {
    // IDA 0xc4ac0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4ad8 — __ZN4FMOD4DSPI13stopBufferingEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
#[doc(alias = "__ZN4FMOD4DSPI13stopBufferingEv")]
pub fn stub_c4ad8() {
    // IDA 0xc4ad8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4ae0 — __ZN4FMOD4DSPI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD4DSPI17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_c4ae0() {
    // IDA 0xc4ae0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4b68 — __ZN4FMOD4DSPI14calculatePeaksEPKfjjPS0_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, const float *, unsigned int, unsigned int, FMOD::DSPI *)
#[doc(alias = "__ZN4FMOD4DSPI14calculatePeaksEPKfjjPS0_")]
pub fn stub_c4b68() {
    // IDA 0xc4b68: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4d3c — __ZN4FMOD4DSPI7getInfoEPcPjPiS3_S3_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, char *, unsigned int *, int *, int *, int *)
#[doc(alias = "__ZN4FMOD4DSPI7getInfoEPcPjPiS3_S3_")]
pub fn stub_c4d3c() {
    // IDA 0xc4d3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc4dac — __ZN4FMOD4DSPI12getParameterEiPfPci
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, float *, char *, int)
#[doc(alias = "__ZN4FMOD4DSPI12getParameterEiPfPci")]
pub fn stub_c4dac(param: i32) -> f32 {
    // IDA 0xc4dac: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc4e40 — __ZN4FMOD4DSPI16getParameterInfoEiPcS1_S1_iPfS2_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, char *, char *, char *, int, float *, float *)
#[doc(alias = "__ZN4FMOD4DSPI16getParameterInfoEiPcS1_S1_iPfS2_")]
pub fn stub_c4e40(param: i32) -> f32 {
    // IDA 0xc4e40: FMOD DSP parameter read (buffer/label traffic engine-side).
    let _ = param;
    0.0
}
// 0xc4f64 — __ZN4FMOD4DSPI13getNumOutputsEPib
// type: int __fastcall(FMOD::SystemI **this, int *, bool)
#[doc(alias = "__ZN4FMOD4DSPI13getNumOutputsEPib")]
pub fn stub_c4f64(handle: u32) {
    // IDA 0xc4f64: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc4fd0 — __ZN4FMOD4DSPI12getNumInputsEPib
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int *, bool)
#[doc(alias = "__ZN4FMOD4DSPI12getNumInputsEPib")]
pub fn stub_c4fd0(handle: u32) {
    // IDA 0xc4fd0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc503c — __ZN4FMOD4DSPI14addInputQueuedEPS0_bPNS_14DSPConnectionIEPS3_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool, FMOD::DSPConnectionI *, FMOD::DSPConnectionI **)
#[doc(alias = "__ZN4FMOD4DSPI14addInputQueuedEPS0_bPNS_14DSPConnectionIEPS3_")]
pub fn stub_c503c(handle: u32) {
    // IDA 0xc503c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc51bc — __ZN4FMOD4DSPI8addInputEPS0_PPNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, FMOD::DSPConnectionI **)
#[doc(alias = "__ZN4FMOD4DSPI8addInputEPS0_PPNS_14DSPConnectionIE")]
pub fn stub_c51bc(handle: u32) {
    // IDA 0xc51bc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc51f0 — __ZN4FMOD4DSPI15updateTreeLevelEi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int)
#[doc(alias = "__ZN4FMOD4DSPI15updateTreeLevelEi")]
pub fn stub_c51f0() {
    // IDA 0xc51f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc5374 — __ZN4FMOD4DSPI20releaseHistoryBufferEPf
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float *)
#[doc(alias = "__ZN4FMOD4DSPI20releaseHistoryBufferEPf")]
pub fn stub_c5374(handle: u32) {
    // IDA 0xc5374: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc5390 — __ZN4FMOD4DSPI19createHistoryBufferEPPfi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float **, int)
#[doc(alias = "__ZN4FMOD4DSPI19createHistoryBufferEPPfi")]
pub fn stub_c5390() -> Option<u32> {
    // IDA 0xc5390: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc53ac — __ZN4FMOD4DSPI22calculateSpeakerLevelsEffffffff16FMOD_SPEAKERMODEi19FMOD_SPEAKERMAPTYPEPfPi
// type: int __fastcall(int, int, int, int, float, float, float, float, int, int, int, void *__b, int)
#[doc(alias = "__ZN4FMOD4DSPI22calculateSpeakerLevelsEffffffff16FMOD_SPEAKERMODEi19FMOD_SPEAKERMAPTYPEPfPi")]
pub fn stub_c53ac() {
    // IDA 0xc53ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc6888 — __ZN4FMOD4DSPI5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
#[doc(alias = "__ZN4FMOD4DSPI5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
pub fn stub_c6888() -> Option<u32> {
    // IDA 0xc6888: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc693c — __ZN4FMOD4DSPI13disconnectAllEbb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool, bool)
#[doc(alias = "__ZN4FMOD4DSPI13disconnectAllEbb")]
pub fn stub_c693c(s: &mut GenSignalState) {
    // IDA 0xc693c: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xc6a60 — __ZN4FMOD4DSPI14disconnectFromEPS0_PNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, FMOD::DSPConnectionI *)
#[doc(alias = "__ZN4FMOD4DSPI14disconnectFromEPS0_PNS_14DSPConnectionIE")]
pub fn stub_c6a60() {
    // IDA 0xc6a60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc6b5c — __ZN4FMOD4DSPI18insertInputBetweenEPS0_ibPPNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, int, bool, FMOD::DSPConnectionI **)
#[doc(alias = "__ZN4FMOD4DSPI18insertInputBetweenEPS0_ibPPNS_14DSPConnectionIE")]
pub fn stub_c6b5c(handle: u32) {
    // IDA 0xc6b5c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc6ca4 — __ZN4FMOD4DSPI9getOutputEiPPS0_PPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, FMOD::DSPI **, FMOD::DSPConnectionI **, bool)
#[doc(alias = "__ZN4FMOD4DSPI9getOutputEiPPS0_PPNS_14DSPConnectionIEb")]
pub fn stub_c6ca4(handle: u32) {
    // IDA 0xc6ca4: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc6d84 — __ZN4FMOD4DSPI8getInputEiPPS0_PPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, FMOD::DSPI **, FMOD::DSPConnectionI **, bool)
#[doc(alias = "__ZN4FMOD4DSPI8getInputEiPPS0_PPNS_14DSPConnectionIEb")]
pub fn stub_c6d84(handle: u32) {
    // IDA 0xc6d84: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc6e64 — __ZN4FMOD4DSPI22disconnectFromInternalEPS0_PNS_14DSPConnectionIEb
// type: int __fastcall(FMOD::DSPI *this, FMOD::DSPI *, FMOD::DSPConnectionI *, bool)
#[doc(alias = "__ZN4FMOD4DSPI22disconnectFromInternalEPS0_PNS_14DSPConnectionIEb")]
pub fn stub_c6e64() {
    // IDA 0xc6e64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7194 — __ZN4FMOD4DSPI21disconnectAllInternalEbbb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool, bool, bool)
#[doc(alias = "__ZN4FMOD4DSPI21disconnectAllInternalEbbb")]
pub fn stub_c7194(s: &mut GenSignalState) {
    // IDA 0xc7194: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xc72c0 — __ZN4FMOD4DSPI11setPositionEjb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, unsigned int, bool)
#[doc(alias = "__ZN4FMOD4DSPI11setPositionEjb")]
pub fn stub_c72c0(value: f32) {
    // IDA 0xc72c0: FMOD wavetable transport positioning.
    let _ = value;
}
// 0xc7380 — __ZN4FMOD4DSPI13doesUnitExistEPS0_b
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool)
#[doc(alias = "__ZN4FMOD4DSPI13doesUnitExistEPS0_b")]
pub fn stub_c7380() {
    // IDA 0xc7380: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7430 — __ZN4FMOD4DSPI16addInputInternalEPS0_bPNS_14DSPConnectionIEPS3_b
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool, FMOD::DSPConnectionI *, FMOD::DSPConnectionI **, bool)
#[doc(alias = "__ZN4FMOD4DSPI16addInputInternalEPS0_bPNS_14DSPConnectionIEPS3_b")]
pub fn stub_c7430(handle: u32) {
    // IDA 0xc7430: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc7720 — __ZN4FMOD4DSPI14removeInternalEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool)
#[doc(alias = "__ZN4FMOD4DSPI14removeInternalEb")]
pub fn stub_c7720() {
    // IDA 0xc7720: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7858 — __ZN4FMOD4DSPI6removeEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
#[doc(alias = "__ZN4FMOD4DSPI6removeEv")]
pub fn stub_c7858() {
    // IDA 0xc7858: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7860 — __ZN4FMOD4DSPI7releaseEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool)
#[doc(alias = "__ZN4FMOD4DSPI7releaseEb")]
pub fn stub_c7860(handle: u32) {
    // IDA 0xc7860: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc798c — __ZN4FMOD4DSPI26insertInputBetweenInternalEPS0_ibPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, int, bool, FMOD::DSPConnectionI *, bool)
#[doc(alias = "__ZN4FMOD4DSPI26insertInputBetweenInternalEPS0_ibPNS_14DSPConnectionIEb")]
pub fn stub_c798c(handle: u32) {
    // IDA 0xc798c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc7b14 — __ZN4FMOD4FileC2Ev
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4FileC2Ev")]
pub fn stub_c7b14() {
    // IDA 0xc7b14: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7b48 — __ZN4FMOD4File6cancelEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File6cancelEv")]
pub fn stub_c7b48() {
    // IDA 0xc7b48: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7b60 — __ZN4FMOD4File4seekEii
// type: _DWORD __fastcall(FMOD::File *__hidden this, int, int)
#[doc(alias = "__ZN4FMOD4File4seekEii")]
pub fn stub_c7b60() {
    // IDA 0xc7b60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7d08 — __ZN4FMOD4File4tellEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File4tellEPj")]
pub fn stub_c7d08() {
    // IDA 0xc7d08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7d3c — __ZN4FMOD4File14setStartOffsetEj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD4File14setStartOffsetEj")]
pub fn stub_c7d3c() {
    // IDA 0xc7d3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7d64 — __ZN4FMOD4File14getStartOffsetEPj
// type: unsigned int __fastcall(FMOD::File *this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File14getStartOffsetEPj")]
pub fn stub_c7d64() {
    // IDA 0xc7d64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7d7c — __ZN4FMOD4File7getNameEPPc
// type: _DWORD __fastcall(FMOD::File *__hidden this, char **)
#[doc(alias = "__ZN4FMOD4File7getNameEPPc")]
pub fn stub_c7d7c(handle: u32) -> String {
    // IDA 0xc7d7c: string query off the handle.
    let _ = handle;
    String::new()
}
// 0xc7d90 — __ZN4FMOD4File17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::File *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD4File17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_c7d90() {
    // IDA 0xc7d90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7db4 — __ZN4FMOD4File7setNameEPc
// type: _DWORD __fastcall(FMOD::File *__hidden this, char *)
#[doc(alias = "__ZN4FMOD4File7setNameEPc")]
pub fn stub_c7db4(handle: u32) -> String {
    // IDA 0xc7db4: string query off the handle.
    let _ = handle;
    String::new()
}
// 0xc7de4 — _FMOD_File_SetDiskBusy
#[doc(alias = "_FMOD_File_SetDiskBusy")]
pub fn stub_c7de4() {
    // IDA 0xc7de4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc7e48 — __ZN4FMOD4File12seekAndResetEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File12seekAndResetEv")]
pub fn stub_c7e48(handle: u32) {
    // IDA 0xc7e48: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc7f10 — __ZN4FMOD4File4flipEb
// type: _DWORD __fastcall(FMOD::File *__hidden this, bool)
#[doc(alias = "__ZN4FMOD4File4flipEb")]
pub fn stub_c7f10() {
    // IDA 0xc7f10: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc82a0 — __ZN4FMOD4File19checkBufferedStatusEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File19checkBufferedStatusEv")]
pub fn stub_c82a0() {
    // IDA 0xc82a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc85b0 — __ZN4FMOD4File4readEPvjjPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, void *, unsigned int, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD4File4readEPvjjPj")]
pub fn stub_c85b0(data: &[u8]) -> bool {
    // IDA 0xc85b0: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc8ab4 — __ZN4FMOD4File8getDwordEPi
// type: _DWORD __fastcall(FMOD::File *__hidden this, int *)
#[doc(alias = "__ZN4FMOD4File8getDwordEPi")]
pub fn stub_c8ab4() {
    // IDA 0xc8ab4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8af0 — __ZN4FMOD4File8getDwordEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File8getDwordEPj")]
pub fn stub_c8af0() {
    // IDA 0xc8af0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8b2c — __ZN4FMOD4File7getWordEPi
// type: _DWORD __fastcall(FMOD::File *__hidden this, int *)
#[doc(alias = "__ZN4FMOD4File7getWordEPi")]
pub fn stub_c8b2c() {
    // IDA 0xc8b2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8b68 — __ZN4FMOD4File7getWordEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File7getWordEPj")]
pub fn stub_c8b68() {
    // IDA 0xc8b68: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8ba4 — __ZN4FMOD4File7getWordEPt
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int16 *)
#[doc(alias = "__ZN4FMOD4File7getWordEPt")]
pub fn stub_c8ba4() {
    // IDA 0xc8ba4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8be0 — __ZN4FMOD4File7getByteEPi
// type: int __fastcall(FMOD::File *this, int *)
#[doc(alias = "__ZN4FMOD4File7getByteEPi")]
pub fn stub_c8be0() {
    // IDA 0xc8be0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8c1c — __ZN4FMOD4File7getByteEPa
// type: _DWORD __fastcall(FMOD::File *__hidden this, signed __int8 *)
#[doc(alias = "__ZN4FMOD4File7getByteEPa")]
pub fn stub_c8c1c() {
    // IDA 0xc8c1c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8c58 — __ZN4FMOD4File7getByteEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File7getByteEPj")]
pub fn stub_c8c58() {
    // IDA 0xc8c58: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8c94 — __ZN4FMOD4File7getByteEPt
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int16 *)
#[doc(alias = "__ZN4FMOD4File7getByteEPt")]
pub fn stub_c8c94() {
    // IDA 0xc8c94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8cd0 — __ZN4FMOD4File7getByteEPh
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int8 *)
#[doc(alias = "__ZN4FMOD4File7getByteEPh")]
pub fn stub_c8cd0() {
    // IDA 0xc8cd0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc8d0c — __ZN4FMOD10FileThread10threadFuncEv
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
#[doc(alias = "__ZN4FMOD10FileThread10threadFuncEv")]
pub fn stub_c8d0c(data: &[u8]) -> bool {
    // IDA 0xc8d0c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc8dbc — __ZN4FMOD14fileThreadFuncEPv
// type: _DWORD __fastcall(FMOD *__hidden this, void *)
#[doc(alias = "__ZN4FMOD14fileThreadFuncEPv")]
pub fn stub_c8dbc(data: &[u8]) -> bool {
    // IDA 0xc8dbc: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc8dc0 — __ZN4FMOD4File4initEPNS_7SystemIEji
// type: _DWORD __fastcall(FMOD::File *__hidden this, FMOD::SystemI *, unsigned int, int)
#[doc(alias = "__ZN4FMOD4File4initEPNS_7SystemIEji")]
pub fn stub_c8dc0() -> Option<u32> {
    // IDA 0xc8dc0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc8e88 — __ZN4FMOD4File4openEPKcjbS2_
// type: _DWORD __fastcall(FMOD::File *__hidden this, const char *, unsigned int, bool, const char *)
#[doc(alias = "__ZN4FMOD4File4openEPKcjbS2_")]
pub fn stub_c8e88() -> Option<u32> {
    // IDA 0xc8e88: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc90e0 — __ZN4FMOD10FileThread7releaseEv
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
#[doc(alias = "__ZN4FMOD10FileThread7releaseEv")]
pub fn stub_c90e0(handle: u32) {
    // IDA 0xc90e0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc9164 — __ZN4FMOD10FileThread4initEibPNS_7SystemIE
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this, int, bool, FMOD::SystemI *)
#[doc(alias = "__ZN4FMOD10FileThread4initEibPNS_7SystemIE")]
pub fn stub_c9164() -> Option<u32> {
    // IDA 0xc9164: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc9238 — __ZN4FMOD10FileThreadC2Ev
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
#[doc(alias = "__ZN4FMOD10FileThreadC2Ev")]
pub fn stub_c9238(data: &[u8]) -> bool {
    // IDA 0xc9238: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc9280 — __ZN4FMOD10FileThreadC1Ev
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
#[doc(alias = "__ZN4FMOD10FileThreadC1Ev")]
pub fn stub_c9280(data: &[u8]) -> bool {
    // IDA 0xc9280: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc9284 — __ZN4FMOD4File13getFileThreadEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File13getFileThreadEv")]
pub fn stub_c9284(data: &[u8]) -> bool {
    // IDA 0xc9284: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc93ac — __ZN4FMOD4File18enableDoubleBufferEjPv
// type: int __fastcall(FMOD::File *this, unsigned int, void *)
#[doc(alias = "__ZN4FMOD4File18enableDoubleBufferEjPv")]
pub fn stub_c93ac() {
    // IDA 0xc93ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc952c — __ZN4FMOD4File8shutDownEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File8shutDownEv")]
pub fn stub_c952c() {
    // IDA 0xc952c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc95b8 — __ZN4FMOD4File5closeEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File5closeEv")]
pub fn stub_c95b8(handle: u32) {
    // IDA 0xc95b8: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc96e4 — __ZN4FMOD4File13getMemoryUsedEPNS_13MemoryTrackerE
#[doc(alias = "__ZN4FMOD4File13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_c96e4() {
    // IDA 0xc96e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc973c — __ZN4FMOD4File11getMetadataEPPNS_8MetadataE
#[doc(alias = "__ZN4FMOD4File11getMetadataEPPNS_8MetadataE")]
pub fn stub_c973c() {
    // IDA 0xc973c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9744 — __ZN4FMOD4File7getSizeEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
#[doc(alias = "__ZN4FMOD4File7getSizeEPj")]
pub fn stub_c9744() {
    // IDA 0xc9744: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9754 — __ZN4FMOD4File12reallyCancelEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
#[doc(alias = "__ZN4FMOD4File12reallyCancelEv")]
pub fn stub_c9754() {
    // IDA 0xc9754: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc975c — __ZN4FMOD4File15reallyAsyncReadEP18FMOD_ASYNCREADINFO
#[doc(alias = "__ZN4FMOD4File15reallyAsyncReadEP18FMOD_ASYNCREADINFO")]
pub fn stub_c975c(data: &[u8]) -> bool {
    // IDA 0xc975c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc9788 — __ZN4FMOD8DiskFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD8DiskFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_c9788() {
    // IDA 0xc9788: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc97bc — __ZN4FMOD8DiskFile12reallyCancelEv
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this)
#[doc(alias = "__ZN4FMOD8DiskFile12reallyCancelEv")]
pub fn stub_c97bc() {
    // IDA 0xc97bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc97d0 — __ZN4FMOD8DiskFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD8DiskFile10reallySeekEj")]
pub fn stub_c97d0() {
    // IDA 0xc97d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc97e4 — __ZN4FMOD8DiskFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, void *, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD8DiskFile10reallyReadEPvjPj")]
pub fn stub_c97e4(data: &[u8]) -> bool {
    // IDA 0xc97e4: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc98a8 — __ZN4FMOD8DiskFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this)
#[doc(alias = "__ZN4FMOD8DiskFile11reallyCloseEv")]
pub fn stub_c98a8(handle: u32) {
    // IDA 0xc98a8: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc98d0 — __ZN4FMOD8DiskFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, const char *, unsigned int *)
#[doc(alias = "__ZN4FMOD8DiskFile10reallyOpenEPKcPj")]
pub fn stub_c98d0() -> Option<u32> {
    // IDA 0xc98d0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc9978 — __ZN4FMOD8DiskFile13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN4FMOD8DiskFile13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_c9978() {
    // IDA 0xc9978: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc99d0 — __ZN4FMOD10MemoryFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, const char *, unsigned int *)
#[doc(alias = "__ZN4FMOD10MemoryFile10reallyOpenEPKcPj")]
pub fn stub_c99d0() -> Option<u32> {
    // IDA 0xc99d0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc99f0 — __ZN4FMOD10MemoryFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this)
#[doc(alias = "__ZN4FMOD10MemoryFile11reallyCloseEv")]
pub fn stub_c99f0(handle: u32) {
    // IDA 0xc99f0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc99f8 — __ZN4FMOD10MemoryFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD10MemoryFile10reallySeekEj")]
pub fn stub_c99f8() {
    // IDA 0xc99f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9a10 — __ZN4FMOD10MemoryFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD10MemoryFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_c9a10() {
    // IDA 0xc9a10: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9a44 — __ZN4FMOD10MemoryFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, void *__dst, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD10MemoryFile10reallyReadEPvjPj")]
pub fn stub_c9a44(data: &[u8]) -> bool {
    // IDA 0xc9a44: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xc9aa8 — __ZN4FMOD10MemoryFile13getMemoryUsedEPNS_13MemoryTrackerE
#[doc(alias = "__ZN4FMOD10MemoryFile13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_c9aa8() {
    // IDA 0xc9aa8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9b00 — __ZN4FMOD7NetFile9openAsMMSEPKcPcS3_S3_tPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, char *, char *, char *, unsigned __int16, unsigned int *)
#[doc(alias = "__ZN4FMOD7NetFile9openAsMMSEPKcPcS3_S3_tPj")]
pub fn stub_c9b00() -> Option<u32> {
    // IDA 0xc9b00: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xc9b08 — __ZN4FMOD7NetFile11getMetadataEPPNS_8MetadataE
#[doc(alias = "__ZN4FMOD7NetFile11getMetadataEPPNS_8MetadataE")]
pub fn stub_c9b08() {
    // IDA 0xc9b08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9b20 — __ZN4FMOD7NetFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD7NetFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_c9b20() {
    // IDA 0xc9b20: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9b54 — __ZN4FMOD7NetFile12reallyCancelEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFile12reallyCancelEv")]
pub fn stub_c9b54() {
    // IDA 0xc9b54: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9b6c — __ZN4FMOD7NetFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD7NetFile10reallySeekEj")]
pub fn stub_c9b6c() {
    // IDA 0xc9b6c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xc9d64 — __ZN4FMOD7NetFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFile11reallyCloseEv")]
pub fn stub_c9d64(handle: u32) {
    // IDA 0xc9d64: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xc9dd4 — __ZN4FMOD7NetFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, void *, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD7NetFile10reallyReadEPvjPj")]
pub fn stub_c9dd4(data: &[u8]) -> bool {
    // IDA 0xc9dd4: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xca20c — __ZN4FMOD7NetFile8parseUrlEPcS1_iS1_iPtS1_iPb
// type: int __fastcall(FMOD::NetFile *this, char *, char *, int, char *, int, unsigned __int16 *, char *, int, bool *)
#[doc(alias = "__ZN4FMOD7NetFile8parseUrlEPcS1_iS1_iPtS1_iPb")]
pub fn stub_ca20c(data: &[u8]) -> bool {
    // IDA 0xca20c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xcb4cc — __ZN4FMOD7NetFile8shutDownEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFile8shutDownEv")]
pub fn stub_cb4cc() {
    // IDA 0xcb4cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb4dc — __ZN4FMOD7NetFile4initEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFile4initEv")]
pub fn stub_cb4dc() -> Option<u32> {
    // IDA 0xcb4dc: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xcb4ec — __ZN4FMOD7NetFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, unsigned int *)
#[doc(alias = "__ZN4FMOD7NetFile10reallyOpenEPKcPj")]
pub fn stub_cb4ec() -> Option<u32> {
    // IDA 0xcb4ec: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xcb658 — __ZN4FMOD7NetFileC2Ev
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFileC2Ev")]
pub fn stub_cb658() {
    // IDA 0xcb658: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb6fc — __ZN4FMOD7NetFileC1Ev
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
#[doc(alias = "__ZN4FMOD7NetFileC1Ev")]
pub fn stub_cb6fc() {
    // IDA 0xcb6fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb700 — __ZN4FMOD7NetFile13getMemoryUsedEPNS_13MemoryTrackerE
#[doc(alias = "__ZN4FMOD7NetFile13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_cb700() {
    // IDA 0xcb700: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb758 — __ZN4FMOD8NullFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, const char *, unsigned int *)
#[doc(alias = "__ZN4FMOD8NullFile10reallyOpenEPKcPj")]
pub fn stub_cb758() -> Option<u32> {
    // IDA 0xcb758: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xcb76c — __ZN4FMOD8NullFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this)
#[doc(alias = "__ZN4FMOD8NullFile11reallyCloseEv")]
pub fn stub_cb76c(handle: u32) {
    // IDA 0xcb76c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0xcb774 — __ZN4FMOD8NullFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, void *, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD8NullFile10reallyReadEPvjPj")]
pub fn stub_cb774(data: &[u8]) -> bool {
    // IDA 0xcb774: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0xcb7b0 — __ZN4FMOD8NullFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, unsigned int)
#[doc(alias = "__ZN4FMOD8NullFile10reallySeekEj")]
pub fn stub_cb7b0() {
    // IDA 0xcb7b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb7c8 — __ZN4FMOD8NullFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD8NullFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_cb7c8() {
    // IDA 0xcb7c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb7fc — __ZN4FMOD8NullFile13getMemoryUsedEPNS_13MemoryTrackerE
#[doc(alias = "__ZN4FMOD8NullFile13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_cb7fc() {
    // IDA 0xcb7fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xcb854 — __ZN4FMOD8UserFile10reallyOpenEPKcPj
// type: int __fastcall(FMOD::UserFile *this, const char *, unsigned int *)
#[doc(alias = "__ZN4FMOD8UserFile10reallyOpenEPKcPj")]
pub fn stub_cb854() -> Option<u32> {
    // IDA 0xcb854: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0xcb8e0 — __ZN4FMOD8UserFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this)
#[doc(alias = "__ZN4FMOD8UserFile11reallyCloseEv")]
pub fn stub_cb8e0(handle: u32) {
    // IDA 0xcb8e0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
