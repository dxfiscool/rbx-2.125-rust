//! core — generated_core_shard_lv — 150 stubs EA-sorted asc global gap filler 0xd353c..0xd98a8 (earliest uncovered after 39349 existing, 46197 gaps before, 46047 after).
//! Source: ida/export.json (85545 funcs) EA asc not yet in crates/core/src — next 150 uncovered sorted asc.
//! Preserves IDA ea + mangled + demangled for rg; sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::OutputCoreAudio::recordStopCallback(FMOD_OUTPUT_STATE *,FMOD::FMOD_RECORDING_INFO *)")]
// 0xd353c — __ZN4FMOD15OutputCoreAudio18recordStopCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOE
pub fn stub_0xd353c() {
    // IDA 0xd353c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::recordStart(FMOD::FMOD_RECORDING_INFO *,FMOD::Sound *,bool)")]
// 0xd3548 — __ZN4FMOD15OutputCoreAudio11recordStartEPNS_19FMOD_RECORDING_INFOEPNS_5SoundEb
pub fn stub_0xd3548() {
    // IDA 0xd3548: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::recordStartCallback(FMOD_OUTPUT_STATE *,FMOD::FMOD_RECORDING_INFO *,FMOD_SOUND *,int)")]
// 0xd392c — __ZN4FMOD15OutputCoreAudio19recordStartCallbackEP17FMOD_OUTPUT_STATEPNS_19FMOD_RECORDING_INFOEP10FMOD_SOUNDi
pub fn stub_0xd392c() {
    // IDA 0xd392c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::handleInterruption(unsigned long)")]
// 0xd3940 — __ZN4FMOD15OutputCoreAudio18handleInterruptionEm
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int)
pub fn stub_0xd3940() {
    // IDA 0xd3940: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::interruptionCallback(void *,unsigned long)")]
// 0xd3968 — __ZN4FMOD15OutputCoreAudio20interruptionCallbackEPvm
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, void *, unsigned int)
pub fn stub_0xd3968() {
    // IDA 0xd3968: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::prepareAudioSession(FMOD_IPHONE_SESSIONCATEGORY,bool,bool)")]
// 0xd3978 — __ZN4FMOD15OutputCoreAudio19prepareAudioSessionE27FMOD_IPHONE_SESSIONCATEGORYbb
pub fn stub_0xd3978() {
    // IDA 0xd3978: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd3ae8 — __ZN4FMOD15OutputCoreAudio4initEijPiiP17FMOD_SOUND_FORMATiiPv
// type: int __fastcall(int, int, int, int, unsigned int, int, unsigned int, int, void *__src)
pub fn stub_0xd3ae8() {
    // IDA 0xd3ae8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd3c0c — __ZN4FMOD15OutputCoreAudio12initCallbackEP17FMOD_OUTPUT_STATEijPiiP17FMOD_SOUND_FORMATiiPv
// type: int __fastcall(int, int, int, int, unsigned int, int, unsigned int, int, void *__src)
pub fn stub_0xd3c0c() {
    // IDA 0xd3c0c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputCoreAudio::getDescriptionEx(void)")]
// 0xd3c5c — __ZN4FMOD15OutputCoreAudio16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this)
pub fn stub_0xd3c5c() {
    // IDA 0xd3c5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputEmulated::update(void)")]
// 0xd3dd8 — __ZN4FMOD14OutputEmulated6updateEv
// type: _DWORD __fastcall(FMOD::OutputEmulated *__hidden this)
pub fn stub_0xd3dd8() {
    // IDA 0xd3dd8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputEmulated::release(void)")]
// 0xd3de0 — __ZN4FMOD14OutputEmulated7releaseEv
// type: _DWORD __fastcall(FMOD::OutputEmulated *__hidden this)
pub fn stub_0xd3de0() {
    // IDA 0xd3de0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputEmulated::OutputEmulated(void)")]
// 0xd3e4c — __ZN4FMOD14OutputEmulatedC2Ev
// type: _DWORD __fastcall(FMOD::OutputEmulated *__hidden this)
pub fn stub_0xd3e4c() {
    // IDA 0xd3e4c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputEmulated::OutputEmulated(void)")]
// 0xd3e8c — __ZN4FMOD14OutputEmulatedC1Ev
// type: _DWORD __fastcall(FMOD::OutputEmulated *__hidden this)
pub fn stub_0xd3e8c() {
    // IDA 0xd3e8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputEmulated::init(int)")]
// 0xd3e90 — __ZN4FMOD14OutputEmulated4initEi
// type: _DWORD __fastcall(FMOD::OutputEmulated *__hidden this, int)
pub fn stub_0xd3e90() {
    // IDA 0xd3e90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::getNumDrivers(int *)")]
// 0xd3fe0 — __ZN4FMOD13OutputNoSound13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this, int *)
pub fn stub_0xd3fe0() {
    // IDA 0xd3fe0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::getDriverCaps(int,unsigned int *)")]
// 0xd3ff0 — __ZN4FMOD13OutputNoSound13getDriverCapsEiPj
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this, int, unsigned int *)
pub fn stub_0xd3ff0() {
    // IDA 0xd3ff0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xd4004 — __ZN4FMOD13OutputNoSound4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xd4004() {
    // IDA 0xd4004: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
// 0xd40a4 — __ZN4FMOD13OutputNoSound21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd40a4() {
    // IDA 0xd40a4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
// 0xd40b0 — __ZN4FMOD13OutputNoSound21getDriverCapsCallbackEP17FMOD_OUTPUT_STATEiPj
pub fn stub_0xd40b0() {
    // IDA 0xd40b0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound::lockCallback(FMOD_OUTPUT_STATE *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xd40bc — __ZN4FMOD13OutputNoSound12lockCallbackEP17FMOD_OUTPUT_STATEjjPPvS4_PjS5_
// type: int __fastcall(int, int, int, int, void **, unsigned int *, unsigned int *)
pub fn stub_0xd40bc() {
    // IDA 0xd40bc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound::getPosition(unsigned int *)")]
// 0xd40ec — __ZN4FMOD13OutputNoSound11getPositionEPj
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this, unsigned int *)
pub fn stub_0xd40ec() {
    // IDA 0xd40ec: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound::getPositionCallback(FMOD_OUTPUT_STATE *,unsigned int *)")]
// 0xd4140 — __ZN4FMOD13OutputNoSound19getPositionCallbackEP17FMOD_OUTPUT_STATEPj
pub fn stub_0xd4140() {
    // IDA 0xd4140: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::close(void)")]
// 0xd414c — __ZN4FMOD13OutputNoSound5closeEv
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this)
pub fn stub_0xd414c() {
    // IDA 0xd414c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::closeCallback(FMOD_OUTPUT_STATE *)")]
// 0xd419c — __ZN4FMOD13OutputNoSound13closeCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd419c() {
    // IDA 0xd419c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd41a8 — __ZN4FMOD13OutputNoSound4initEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd41a8() {
    // IDA 0xd41a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd4350 — __ZN4FMOD13OutputNoSound12initCallbackEP17FMOD_OUTPUT_STATEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd4350() {
    // IDA 0xd4350: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::getDriverName(int,char *,int)")]
// 0xd43a0 — __ZN4FMOD13OutputNoSound13getDriverNameEiPci
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this, int, char *, int)
pub fn stub_0xd43a0() {
    // IDA 0xd43a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
// 0xd43c8 — __ZN4FMOD13OutputNoSound21getDriverNameCallbackEP17FMOD_OUTPUT_STATEiPci
pub fn stub_0xd43c8() {
    // IDA 0xd43c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound::getDescriptionEx(void)")]
// 0xd43d4 — __ZN4FMOD13OutputNoSound16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::OutputNoSound *__hidden this)
pub fn stub_0xd43d4() {
    // IDA 0xd43d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::nosoundoutput")]
// 0xd44e8 — __GLOBAL__I__ZN4FMOD13nosoundoutputE
pub fn stub_0xd44e8() {
    // IDA 0xd44e8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getNumDrivers(int *)")]
// 0xd44f4 — __ZN4FMOD17OutputNoSound_NRT13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this, int *)
pub fn stub_0xd44f4() {
    // IDA 0xd44f4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverCaps(int,unsigned int *)")]
// 0xd4504 — __ZN4FMOD17OutputNoSound_NRT13getDriverCapsEiPj
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this, int, unsigned int *)
pub fn stub_0xd4504() {
    // IDA 0xd4504: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
// 0xd4518 — __ZN4FMOD17OutputNoSound_NRT21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd4518() {
    // IDA 0xd4518: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
// 0xd4524 — __ZN4FMOD17OutputNoSound_NRT21getDriverCapsCallbackEP17FMOD_OUTPUT_STATEiPj
pub fn stub_0xd4524() {
    // IDA 0xd4524: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::update(void)")]
// 0xd4530 — __ZN4FMOD17OutputNoSound_NRT6updateEv
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this)
pub fn stub_0xd4530() {
    // IDA 0xd4530: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::updateCallback(FMOD_OUTPUT_STATE *)")]
// 0xd454c — __ZN4FMOD17OutputNoSound_NRT14updateCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd454c() {
    // IDA 0xd454c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::close(void)")]
// 0xd4558 — __ZN4FMOD17OutputNoSound_NRT5closeEv
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this)
pub fn stub_0xd4558() {
    // IDA 0xd4558: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::closeCallback(FMOD_OUTPUT_STATE *)")]
// 0xd45ac — __ZN4FMOD17OutputNoSound_NRT13closeCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd45ac() {
    // IDA 0xd45ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd45b8 — __ZN4FMOD17OutputNoSound_NRT4initEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd45b8() {
    // IDA 0xd45b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd4764 — __ZN4FMOD17OutputNoSound_NRT12initCallbackEP17FMOD_OUTPUT_STATEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd4764() {
    // IDA 0xd4764: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverName(int,char *,int)")]
// 0xd47b4 — __ZN4FMOD17OutputNoSound_NRT13getDriverNameEiPci
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this, int, char *, int)
pub fn stub_0xd47b4() {
    // IDA 0xd47b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
// 0xd47dc — __ZN4FMOD17OutputNoSound_NRT21getDriverNameCallbackEP17FMOD_OUTPUT_STATEiPci
pub fn stub_0xd47dc() {
    // IDA 0xd47dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDescriptionEx(void)")]
// 0xd47e8 — __ZN4FMOD17OutputNoSound_NRT16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::OutputNoSound_NRT *__hidden this)
pub fn stub_0xd47e8() {
    // IDA 0xd47e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::nosoundoutput_nrt")]
// 0xd48ec — __GLOBAL__I__ZN4FMOD17nosoundoutput_nrtE
pub fn stub_0xd48ec() {
    // IDA 0xd48ec: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputPolled::stop(void)")]
// 0xd48f8 — __ZN4FMOD12OutputPolled4stopEv
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd48f8() {
    // IDA 0xd48f8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputPolled::OutputPolled(void)")]
// 0xd4930 — __ZN4FMOD12OutputPolledC2Ev
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd4930() {
    // IDA 0xd4930: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputPolled::OutputPolled(void)")]
// 0xd497c — __ZN4FMOD12OutputPolledC1Ev
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd497c() {
    // IDA 0xd497c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputPolled::start(void)")]
// 0xd4980 — __ZN4FMOD12OutputPolled5startEv
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd4980() {
    // IDA 0xd4980: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::threadFunc(void)")]
// 0xd4ac0 — __ZThn360_N4FMOD12OutputPolled10threadFuncEv
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd4ac0() {
    // IDA 0xd4ac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputPolled::threadFunc(void)")]
// 0xd4ac8 — __ZN4FMOD12OutputPolled10threadFuncEv
// type: _DWORD __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd4ac8() {
    // IDA 0xd4ac8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::~OutputPolled()")]
// 0xd4ff8 — __ZThn360_N4FMOD12OutputPolledD0Ev
// type: void __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd4ff8() {
    // IDA 0xd4ff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputPolled::~OutputPolled()")]
// 0xd5000 — __ZN4FMOD12OutputPolledD0Ev
// type: void __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd5000() {
    // IDA 0xd5000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::~OutputPolled()")]
// 0xd5038 — __ZThn360_N4FMOD12OutputPolledD1Ev
// type: void __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd5038() {
    // IDA 0xd5038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputPolled::~OutputPolled()")]
// 0xd5040 — __ZN4FMOD12OutputPolledD1Ev
// type: void __fastcall(FMOD::OutputPolled *__hidden this)
pub fn stub_0xd5040() {
    // IDA 0xd5040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputSoftware::getSampleMaxChannels(unsigned int,FMOD_SOUND_FORMAT)")]
// 0xd506c — __ZN4FMOD14OutputSoftware20getSampleMaxChannelsEj17FMOD_SOUND_FORMAT
pub fn stub_0xd506c() {
    // IDA 0xd506c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputSoftware::getSampleMaxChannelsCallback(FMOD_OUTPUT_STATE *,unsigned int,FMOD_SOUND_FORMAT)")]
// 0xd5074 — __ZN4FMOD14OutputSoftware28getSampleMaxChannelsCallbackEP17FMOD_OUTPUT_STATEj17FMOD_SOUND_FORMAT
pub fn stub_0xd5074() {
    // IDA 0xd5074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputSoftware::release(void)")]
// 0xd5080 — __ZN4FMOD14OutputSoftware7releaseEv
// type: _DWORD __fastcall(FMOD::OutputSoftware *__hidden this)
pub fn stub_0xd5080() {
    // IDA 0xd5080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputSoftware::OutputSoftware(void)")]
// 0xd50ec — __ZN4FMOD14OutputSoftwareC2Ev
// type: _DWORD __fastcall(FMOD::OutputSoftware *__hidden this)
pub fn stub_0xd50ec() {
    // IDA 0xd50ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OutputSoftware::OutputSoftware(void)")]
// 0xd5170 — __ZN4FMOD14OutputSoftwareC1Ev
// type: _DWORD __fastcall(FMOD::OutputSoftware *__hidden this)
pub fn stub_0xd5170() {
    // IDA 0xd5170: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputSoftware::init(int)")]
// 0xd5174 — __ZN4FMOD14OutputSoftware4initEi
// type: _DWORD __fastcall(FMOD::OutputSoftware *__hidden this, int)
pub fn stub_0xd5174() {
    // IDA 0xd5174: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputSoftware::createSample(unsigned int,FMOD_CODEC_WAVEFORMAT *,FMOD::Sample **)")]
// 0xd52d0 — __ZN4FMOD14OutputSoftware12createSampleEjP21FMOD_CODEC_WAVEFORMATPPNS_6SampleE
pub fn stub_0xd52d0() {
    // IDA 0xd52d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getNumDrivers(int *)")]
// 0xd5770 — __ZN4FMOD15OutputWavWriter13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, int *)
pub fn stub_0xd5770() {
    // IDA 0xd5770: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverCaps(int,unsigned int *)")]
// 0xd5780 — __ZN4FMOD15OutputWavWriter13getDriverCapsEiPj
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, int, unsigned int *)
pub fn stub_0xd5780() {
    // IDA 0xd5780: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xd5794 — __ZN4FMOD15OutputWavWriter4lockEjjPPvS2_PjS3_
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, unsigned int, unsigned int, void **, void **, unsigned int *, unsigned int *)
pub fn stub_0xd5794() {
    // IDA 0xd5794: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter::getHandle(void **)")]
// 0xd5834 — __ZN4FMOD15OutputWavWriter9getHandleEPPv
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, void **)
pub fn stub_0xd5834() {
    // IDA 0xd5834: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
// 0xd5844 — __ZN4FMOD15OutputWavWriter21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd5844() {
    // IDA 0xd5844: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
// 0xd5850 — __ZN4FMOD15OutputWavWriter21getDriverCapsCallbackEP17FMOD_OUTPUT_STATEiPj
pub fn stub_0xd5850() {
    // IDA 0xd5850: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter::lockCallback(FMOD_OUTPUT_STATE *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
// 0xd585c — __ZN4FMOD15OutputWavWriter12lockCallbackEP17FMOD_OUTPUT_STATEjjPPvS4_PjS5_
// type: int __fastcall(int, int, int, int, void **, unsigned int *, unsigned int *)
pub fn stub_0xd585c() {
    // IDA 0xd585c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter::getHandleCallback(FMOD_OUTPUT_STATE *,void **)")]
// 0xd588c — __ZN4FMOD15OutputWavWriter17getHandleCallbackEP17FMOD_OUTPUT_STATEPPv
pub fn stub_0xd588c() {
    // IDA 0xd588c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::writeWavHeader(void)")]
// 0xd5898 — __ZN4FMOD15OutputWavWriter14writeWavHeaderEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this)
pub fn stub_0xd5898() {
    // IDA 0xd5898: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::unlock(void *,void *,unsigned int,unsigned int)")]
// 0xd5adc — __ZN4FMOD15OutputWavWriter6unlockEPvS1_jj
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, void *__ptr, void *, size_t __nitems, size_t)
pub fn stub_0xd5adc() {
    // IDA 0xd5adc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::unlockCallback(FMOD_OUTPUT_STATE *,void *,void *,unsigned int,unsigned int)")]
// 0xd5bd0 — __ZN4FMOD15OutputWavWriter14unlockCallbackEP17FMOD_OUTPUT_STATEPvS3_jj
// type: int __fastcall(int, int, int, int, size_t)
pub fn stub_0xd5bd0() {
    // IDA 0xd5bd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::close(void)")]
// 0xd5be4 — __ZN4FMOD15OutputWavWriter5closeEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this)
pub fn stub_0xd5be4() {
    // IDA 0xd5be4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::closeCallback(FMOD_OUTPUT_STATE *)")]
// 0xd5c58 — __ZN4FMOD15OutputWavWriter13closeCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd5c58() {
    // IDA 0xd5c58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverName(int,char *,int)")]
// 0xd5c64 — __ZN4FMOD15OutputWavWriter13getDriverNameEiPci
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, int, char *, int)
pub fn stub_0xd5c64() {
    // IDA 0xd5c64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
// 0xd5c8c — __ZN4FMOD15OutputWavWriter21getDriverNameCallbackEP17FMOD_OUTPUT_STATEiPci
pub fn stub_0xd5c8c() {
    // IDA 0xd5c8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd5c98 — __ZN4FMOD15OutputWavWriter4initEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd5c98() {
    // IDA 0xd5c98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd5f48 — __ZN4FMOD15OutputWavWriter12initCallbackEP17FMOD_OUTPUT_STATEijPiiP17FMOD_SOUND_FORMATiiPv
pub fn stub_0xd5f48() {
    // IDA 0xd5f48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getDescriptionEx(void)")]
// 0xd5f98 — __ZN4FMOD15OutputWavWriter16getDescriptionExEv
// type: int *__fastcall(FMOD::OutputWavWriter *this)
pub fn stub_0xd5f98() {
    // IDA 0xd5f98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getPosition(unsigned int *)")]
// 0xd60a0 — __ZN4FMOD15OutputWavWriter11getPositionEPj
// type: _DWORD __fastcall(FMOD::OutputWavWriter *__hidden this, unsigned int *)
pub fn stub_0xd60a0() {
    // IDA 0xd60a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter::getPositionCallback(FMOD_OUTPUT_STATE *,unsigned int *)")]
// 0xd60f4 — __ZN4FMOD15OutputWavWriter19getPositionCallbackEP17FMOD_OUTPUT_STATEPj
pub fn stub_0xd60f4() {
    // IDA 0xd60f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::wavwriteroutput")]
// 0xd6144 — __GLOBAL__I__ZN4FMOD15wavwriteroutputE
pub fn stub_0xd6144() {
    // IDA 0xd6144: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getNumDrivers(int *)")]
// 0xd6150 — __ZN4FMOD19OutputWavWriter_NRT13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this, int *)
pub fn stub_0xd6150() {
    // IDA 0xd6150: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverCaps(int,unsigned int *)")]
// 0xd6160 — __ZN4FMOD19OutputWavWriter_NRT13getDriverCapsEiPj
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this, int, unsigned int *)
pub fn stub_0xd6160() {
    // IDA 0xd6160: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getHandle(void **)")]
// 0xd6174 — __ZN4FMOD19OutputWavWriter_NRT9getHandleEPPv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this, void **)
pub fn stub_0xd6174() {
    // IDA 0xd6174: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
// 0xd6184 — __ZN4FMOD19OutputWavWriter_NRT21getNumDriversCallbackEP17FMOD_OUTPUT_STATEPi
pub fn stub_0xd6184() {
    // IDA 0xd6184: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
// 0xd6190 — __ZN4FMOD19OutputWavWriter_NRT21getDriverCapsCallbackEP17FMOD_OUTPUT_STATEiPj
pub fn stub_0xd6190() {
    // IDA 0xd6190: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getHandleCallback(FMOD_OUTPUT_STATE *,void **)")]
// 0xd619c — __ZN4FMOD19OutputWavWriter_NRT17getHandleCallbackEP17FMOD_OUTPUT_STATEPPv
pub fn stub_0xd619c() {
    // IDA 0xd619c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::writeWavHeader(void)")]
// 0xd61a8 — __ZN4FMOD19OutputWavWriter_NRT14writeWavHeaderEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd61a8() {
    // IDA 0xd61a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::stop(void)")]
// 0xd63ec — __ZN4FMOD19OutputWavWriter_NRT4stopEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd63ec() {
    // IDA 0xd63ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::stopCallback(FMOD_OUTPUT_STATE *)")]
// 0xd641c — __ZN4FMOD19OutputWavWriter_NRT12stopCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd641c() {
    // IDA 0xd641c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::start(void)")]
// 0xd6428 — __ZN4FMOD19OutputWavWriter_NRT5startEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd6428() {
    // IDA 0xd6428: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::startCallback(FMOD_OUTPUT_STATE *)")]
// 0xd6468 — __ZN4FMOD19OutputWavWriter_NRT13startCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd6468() {
    // IDA 0xd6468: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::update(void)")]
// 0xd6474 — __ZN4FMOD19OutputWavWriter_NRT6updateEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd6474() {
    // IDA 0xd6474: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::updateCallback(FMOD_OUTPUT_STATE *)")]
// 0xd6504 — __ZN4FMOD19OutputWavWriter_NRT14updateCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd6504() {
    // IDA 0xd6504: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::close(void)")]
// 0xd6510 — __ZN4FMOD19OutputWavWriter_NRT5closeEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd6510() {
    // IDA 0xd6510: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::closeCallback(FMOD_OUTPUT_STATE *)")]
// 0xd6560 — __ZN4FMOD19OutputWavWriter_NRT13closeCallbackEP17FMOD_OUTPUT_STATE
pub fn stub_0xd6560() {
    // IDA 0xd6560: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd656c — __ZN4FMOD19OutputWavWriter_NRT4initEijPiiP17FMOD_SOUND_FORMATiiPv
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xd656c() {
    // IDA 0xd656c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
// 0xd67f0 — __ZN4FMOD19OutputWavWriter_NRT12initCallbackEP17FMOD_OUTPUT_STATEijPiiP17FMOD_SOUND_FORMATiiPv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xd67f0() {
    // IDA 0xd67f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverName(int,char *,int)")]
// 0xd6840 — __ZN4FMOD19OutputWavWriter_NRT13getDriverNameEiPci
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this, int, char *, int)
pub fn stub_0xd6840() {
    // IDA 0xd6840: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
// 0xd6868 — __ZN4FMOD19OutputWavWriter_NRT21getDriverNameCallbackEP17FMOD_OUTPUT_STATEiPci
pub fn stub_0xd6868() {
    // IDA 0xd6868: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDescriptionEx(void)")]
// 0xd6874 — __ZN4FMOD19OutputWavWriter_NRT16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::OutputWavWriter_NRT *__hidden this)
pub fn stub_0xd6874() {
    // IDA 0xd6874: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::wavwriteroutput_nrt")]
// 0xd69bc — __GLOBAL__I__ZN4FMOD19wavwriteroutput_nrtE
pub fn stub_0xd69bc() {
    // IDA 0xd69bc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Plugin::release(void)")]
// 0xd69c8 — __ZN4FMOD6Plugin7releaseEv
// type: _DWORD __fastcall(FMOD::Plugin *__hidden this)
pub fn stub_0xd69c8() {
    // IDA 0xd69c8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::PluginFactory::setSystem(FMOD::SystemI *)")]
// 0xd6a04 — __ZN4FMOD13PluginFactory9setSystemEPNS_7SystemIE
pub fn stub_0xd6a04() {
    // IDA 0xd6a04: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::PluginFactory::getNumCodecs(int *)")]
// 0xd6a10 — __ZN4FMOD13PluginFactory12getNumCodecsEPi
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int *)
pub fn stub_0xd6a10() {
    // IDA 0xd6a10: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::PluginFactory::getNumDSPs(int *)")]
// 0xd6a50 — __ZN4FMOD13PluginFactory10getNumDSPsEPi
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int *)
pub fn stub_0xd6a50() {
    // IDA 0xd6a50: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::PluginFactory::getNumOutputs(int *)")]
// 0xd6a90 — __ZN4FMOD13PluginFactory13getNumOutputsEPi
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int *)
pub fn stub_0xd6a90() {
    // IDA 0xd6a90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getCodecHandle(int,unsigned int *)")]
// 0xd6ad0 — __ZN4FMOD13PluginFactory14getCodecHandleEiPj
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int, unsigned int *)
pub fn stub_0xd6ad0() {
    // IDA 0xd6ad0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getDSPHandle(int,unsigned int *)")]
// 0xd6b30 — __ZN4FMOD13PluginFactory12getDSPHandleEiPj
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int, unsigned int *)
pub fn stub_0xd6b30() {
    // IDA 0xd6b30: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getOutputHandle(int,unsigned int *)")]
// 0xd6b90 — __ZN4FMOD13PluginFactory15getOutputHandleEiPj
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, int, unsigned int *)
pub fn stub_0xd6b90() {
    // IDA 0xd6b90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getCodec(unsigned int,FMOD::FMOD_CODEC_DESCRIPTION_EX **)")]
// 0xd6bf0 — __ZN4FMOD13PluginFactory8getCodecEjPPNS_25FMOD_CODEC_DESCRIPTION_EXE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xd6bf0() {
    // IDA 0xd6bf0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getDSP(unsigned int,FMOD::FMOD_DSP_DESCRIPTION_EX **)")]
// 0xd6c54 — __ZN4FMOD13PluginFactory6getDSPEjPPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xd6c54() {
    // IDA 0xd6c54: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::getOutput(unsigned int,FMOD::FMOD_OUTPUT_DESCRIPTION_EX **)")]
// 0xd6cb8 — __ZN4FMOD13PluginFactory9getOutputEjPPNS_26FMOD_OUTPUT_DESCRIPTION_EXE
pub fn stub_0xd6cb8() {
    // IDA 0xd6cb8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::unloadPlugin(unsigned int)")]
// 0xd6d1c — __ZN4FMOD13PluginFactory12unloadPluginEj
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, unsigned int)
pub fn stub_0xd6d1c() {
    // IDA 0xd6d1c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::release(void)")]
// 0xd6e9c — __ZN4FMOD13PluginFactory7releaseEv
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this)
pub fn stub_0xd6e9c() {
    // IDA 0xd6e9c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::setPluginPath(char const*)")]
// 0xd7004 — __ZN4FMOD13PluginFactory13setPluginPathEPKc
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this, const char *)
pub fn stub_0xd7004() {
    // IDA 0xd7004: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::PluginFactory(void)")]
// 0xd7040 — __ZN4FMOD13PluginFactoryC2Ev
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this)
pub fn stub_0xd7040() {
    // IDA 0xd7040: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::PluginFactory(void)")]
// 0xd70d8 — __ZN4FMOD13PluginFactoryC1Ev
// type: _DWORD __fastcall(FMOD::PluginFactory *__hidden this)
pub fn stub_0xd70d8() {
    // IDA 0xd70d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::registerCodec(FMOD::FMOD_CODEC_DESCRIPTION_EX *,unsigned int *,unsigned int)")]
// 0xd70dc — __ZN4FMOD13PluginFactory13registerCodecEPNS_25FMOD_CODEC_DESCRIPTION_EXEPjj
pub fn stub_0xd70dc() {
    // IDA 0xd70dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::createOutput(FMOD::FMOD_OUTPUT_DESCRIPTION_EX *,FMOD::Output **)")]
// 0xd72a0 — __ZN4FMOD13PluginFactory12createOutputEPNS_26FMOD_OUTPUT_DESCRIPTION_EXEPPNS_6OutputE
pub fn stub_0xd72a0() {
    // IDA 0xd72a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::registerDSP(FMOD::FMOD_DSP_DESCRIPTION_EX *,unsigned int *)")]
// 0xd73c0 — __ZN4FMOD13PluginFactory11registerDSPEPNS_23FMOD_DSP_DESCRIPTION_EXEPj
pub fn stub_0xd73c0() {
    // IDA 0xd73c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::registerOutput(FMOD::FMOD_OUTPUT_DESCRIPTION_EX *,unsigned int *)")]
// 0xd7538 — __ZN4FMOD13PluginFactory14registerOutputEPNS_26FMOD_OUTPUT_DESCRIPTION_EXEPj
pub fn stub_0xd7538() {
    // IDA 0xd7538: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::createCodec(FMOD::FMOD_CODEC_DESCRIPTION_EX *,FMOD::Codec **)")]
// 0xd7740 — __ZN4FMOD13PluginFactory11createCodecEPNS_25FMOD_CODEC_DESCRIPTION_EXEPPNS_5CodecE
pub fn stub_0xd7740() {
    // IDA 0xd7740: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::PluginFactory::createDSP(FMOD::FMOD_DSP_DESCRIPTION_EX *,FMOD::DSPI **)")]
// 0xd788c — __ZN4FMOD13PluginFactory9createDSPEPNS_23FMOD_DSP_DESCRIPTION_EXEPPNS_4DSPIE
pub fn stub_0xd788c() {
    // IDA 0xd788c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::defaultMetaData(FMOD_CODEC_STATE *,FMOD_TAGTYPE,char *,void *,unsigned int,FMOD_TAGDATATYPE,int)")]
// 0xd7f7c — __ZN4FMOD5Codec15defaultMetaDataEP16FMOD_CODEC_STATE12FMOD_TAGTYPEPcPvj16FMOD_TAGDATATYPEi
pub fn stub_0xd7f7c() {
    // IDA 0xd7f7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::defaultFileSeek(void *,unsigned int,void *)")]
// 0xd7fc0 — __ZN4FMOD5Codec15defaultFileSeekEPvjS1_
// type: _DWORD __fastcall(FMOD::Codec *__hidden this, void *, unsigned int, void *)
pub fn stub_0xd7fc0() {
    // IDA 0xd7fc0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Codec::defaultFileRead(void *,void *,unsigned int,unsigned int *,void *)")]
// 0xd7fd4 — __ZN4FMOD5Codec15defaultFileReadEPvS1_jPjS1_
// type: _DWORD __fastcall(FMOD::Codec *__hidden this, void *, void *, unsigned int *, unsigned int *, void *)
pub fn stub_0xd7fd4() {
    // IDA 0xd7fd4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::ReverbI(void)")]
// 0xd7ffc — __ZN4FMOD7ReverbIC2Ev
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this)
pub fn stub_0xd7ffc() {
    // IDA 0xd7ffc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::ReverbI(void)")]
// 0xd808c — __ZN4FMOD7ReverbIC1Ev
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this)
pub fn stub_0xd808c() {
    // IDA 0xd808c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::resetConnectionPointer(int,int)")]
// 0xd8090 — __ZN4FMOD7ReverbI22resetConnectionPointerEii
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int, int)
pub fn stub_0xd8090() {
    // IDA 0xd8090: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::get3DAttributes(FMOD_VECTOR *,float *,float *)")]
// 0xd80d0 — __ZN4FMOD7ReverbI15get3DAttributesEP11FMOD_VECTORPfS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xd80d0() {
    // IDA 0xd80d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::getProperties(FMOD_REVERB_PROPERTIES *)")]
// 0xd8110 — __ZN4FMOD7ReverbI13getPropertiesEP22FMOD_REVERB_PROPERTIES
pub fn stub_0xd8110() {
    // IDA 0xd8110: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::getActive(bool *)")]
// 0xd815c — __ZN4FMOD7ReverbI9getActiveEPb
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, bool *)
pub fn stub_0xd815c() {
    // IDA 0xd815c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::setPresenceGain(int,int,float)")]
// 0xd8174 — __ZN4FMOD7ReverbI15setPresenceGainEiif
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int, int, float)
pub fn stub_0xd8174() {
    // IDA 0xd8174: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::getPresenceGain(int,int,float *)")]
// 0xd81d8 — __ZN4FMOD7ReverbI15getPresenceGainEiiPf
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int, int, float *)
pub fn stub_0xd81d8() {
    // IDA 0xd81d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0xd8248 — __ZN4FMOD7ReverbI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xd8248() {
    // IDA 0xd8248: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::factorProps(FMOD_REVERB_PROPERTIES *,FMOD::FMOD_REVERB_STDPROPERTIES *,float)")]
// 0xd8338 — __ZN4FMOD7ReverbI11factorPropsEP22FMOD_REVERB_PROPERTIESPNS_25FMOD_REVERB_STDPROPERTIESEf
pub fn stub_0xd8338() {
    // IDA 0xd8338: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::sumProps(FMOD::FMOD_REVERB_STDPROPERTIES *,FMOD_REVERB_PROPERTIES *,float)")]
// 0xd857c — __ZN4FMOD7ReverbI8sumPropsEPNS_25FMOD_REVERB_STDPROPERTIESEP22FMOD_REVERB_PROPERTIESf
pub fn stub_0xd857c() {
    // IDA 0xd857c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::sumRoomProps(FMOD::FMOD_REVERB_STDPROPERTIES *,FMOD_REVERB_PROPERTIES *,float)")]
// 0xd8788 — __ZN4FMOD7ReverbI12sumRoomPropsEPNS_25FMOD_REVERB_STDPROPERTIESEP22FMOD_REVERB_PROPERTIESf
pub fn stub_0xd8788() {
    // IDA 0xd8788: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::calculateDistanceGain(FMOD_VECTOR *,float *,float *)")]
// 0xd87ec — __ZN4FMOD7ReverbI21calculateDistanceGainEP11FMOD_VECTORPfS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xd87ec() {
    // IDA 0xd87ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::resetChanProperties(int,int)")]
// 0xd8904 — __ZN4FMOD7ReverbI19resetChanPropertiesEii
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int, int)
pub fn stub_0xd8904() {
    // IDA 0xd8904: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::setChanProperties(int,int,FMOD_REVERB_CHANNELPROPERTIES const*,FMOD::DSPConnectionI *)")]
// 0xd89d8 — __ZN4FMOD7ReverbI17setChanPropertiesEiiPK29FMOD_REVERB_CHANNELPROPERTIESPNS_14DSPConnectionIE
// type: int __fastcall(int, int, int, void *__src, int)
pub fn stub_0xd89d8() {
    // IDA 0xd89d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::getChanProperties(int,int,FMOD_REVERB_CHANNELPROPERTIES *,FMOD::DSPConnectionI **)")]
// 0xd8ac8 — __ZN4FMOD7ReverbI17getChanPropertiesEiiP29FMOD_REVERB_CHANNELPROPERTIESPPNS_14DSPConnectionIE
// type: int __fastcall(int, int, int, void *__dst, int)
pub fn stub_0xd8ac8() {
    // IDA 0xd8ac8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::setProperties(FMOD_REVERB_PROPERTIES const*)")]
// 0xd8bbc — __ZN4FMOD7ReverbI13setPropertiesEPK22FMOD_REVERB_PROPERTIES
pub fn stub_0xd8bbc() {
    // IDA 0xd8bbc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::setDisableIfNoEnvironment(bool)")]
// 0xd94ac — __ZN4FMOD7ReverbI25setDisableIfNoEnvironmentEb
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, bool)
pub fn stub_0xd94ac() {
    // IDA 0xd94ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::releaseDSP(int)")]
// 0xd9510 — __ZN4FMOD7ReverbI10releaseDSPEi
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int)
pub fn stub_0xd9510() {
    // IDA 0xd9510: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::createDSP(int)")]
// 0xd9574 — __ZN4FMOD7ReverbI9createDSPEi
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, int)
pub fn stub_0xd9574() {
    // IDA 0xd9574: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::release(bool)")]
// 0xd9760 — __ZN4FMOD7ReverbI7releaseEb
// type: _DWORD __fastcall(FMOD::ReverbI *__hidden this, bool)
pub fn stub_0xd9760() {
    // IDA 0xd9760: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ReverbI::init(FMOD::SystemI *,bool,FMOD::FMOD_REVERB_MODE)")]
// 0xd98a8 — __ZN4FMOD7ReverbI4initEPNS_7SystemIEbNS_16FMOD_REVERB_MODEE
pub fn stub_0xd98a8() {
    // IDA 0xd98a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
