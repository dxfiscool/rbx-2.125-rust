//! rendering wd3 1788337482 — 120 stubs 0xaefcc..0xb5fe4 EA-sorted asc gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler 52145->52265 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xaefcc — FMOD::DSPCodecPool::alloc(FMOD::DSPCodec **)
// type: int __fastcall(FMOD::DSPCodecPool *this, FMOD::DSPCodec **)
#[doc(alias = "FMOD::DSPCodecPool::alloc(FMOD::DSPCodec **)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool5allocEPPNS_8DSPCodecE")]
// IDA 0xaefcc: 33 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aefcc() {
}

// 0xaf050 — FMOD::DSPCodecPool::close(void)
// type: int __fastcall(FMOD::DSPCodecPool *this)
#[doc(alias = "FMOD::DSPCodecPool::close(void)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool5closeEv")]
// IDA 0xaf050: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af050() {
}

// 0xaf148 — FMOD::DSPCodecPool::init(FMOD::FMOD_DSP_CATEGORY,int,int)
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::DSPCodecPool::init(FMOD::FMOD_DSP_CATEGORY,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool4initENS_17FMOD_DSP_CATEGORYEii")]
// IDA 0xaf148: 243 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af148() {
}

// 0xaf528 — FMOD::Codec::defaultGetWaveFormat(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)
// type: int __fastcall(int *, int, void *__dst)
#[doc(alias = "FMOD::Codec::defaultGetWaveFormat(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
#[doc(alias = "__ZN4FMOD5Codec20defaultGetWaveFormatEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT")]
// IDA 0xaf528: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af528() {
}

// 0xaf5ac — FMOD::DSPCompressor::createInternal(void)
// type: int __fastcall(FMOD::DSPCompressor *this)
#[doc(alias = "FMOD::DSPCompressor::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor14createInternalEv")]
// IDA 0xaf5ac: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af5ac() {
}

// 0xaf648 — FMOD::DSPCompressor::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int()
#[doc(alias = "FMOD::DSPCompressor::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xaf648: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af648() {
}

// 0xaf650 — FMOD::DSPCompressor::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPCompressor *)
#[doc(alias = "FMOD::DSPCompressor::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xaf650: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af650() {
}

// 0xaf65c — FMOD::DSPCompressor::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPCompressor::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xaf65c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af65c() {
}

// 0xaf6b4 — FMOD::DSPCompressor::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPCompressor *this, int, float *, char *)
#[doc(alias = "FMOD::DSPCompressor::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20getParameterInternalEiPfPc")]
// IDA 0xaf6b4: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af6b4() {
}

// 0xaf798 — FMOD::DSPCompressor::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPCompressor *, int, float *, char *)
#[doc(alias = "FMOD::DSPCompressor::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xaf798: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af798() {
}

// 0xaf7a4 — FMOD::DSPCompressor::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPCompressor *this, int, float)
#[doc(alias = "FMOD::DSPCompressor::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20setParameterInternalEif")]
// IDA 0xaf7a4: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af7a4() {
}

// 0xaf8a4 — FMOD::DSPCompressor::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPCompressor *, int, float)
#[doc(alias = "FMOD::DSPCompressor::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xaf8a4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af8a4() {
}

// 0xaf8b0 — FMOD::DSPCompressor::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPCompressor *this, float *, float *__dst, unsigned int, int, int)
#[doc(alias = "FMOD::DSPCompressor::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor12readInternalEPfS1_jii")]
// IDA 0xaf8b0: 234 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_af8b0() {
}

// 0xafc5c — FMOD::DSPCompressor::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPCompressor *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPCompressor::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xafc5c: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afc5c() {
}

// 0xafc84 — FMOD::DSPCompressor::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPCompressor *this)
#[doc(alias = "FMOD::DSPCompressor::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor16getDescriptionExEv")]
// IDA 0xafc84: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afc84() {
}

// 0xafd50 — __Z41__static_initialization_and_destruction_0ii_18
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_18")]
// IDA 0xafd50: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afd50() {
}

// 0xafd94 — global constructor keyed toFMOD::dspcompressor
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dspcompressor")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dspcompressorE")]
// IDA 0xafd94: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afd94() {
}

// 0xafda0 — FMOD::DSPConnectionPool::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPConnectionPool::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xafda0: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afda0() {
}

// 0xafe90 — FMOD::DSPConnectionPool::close(void)
// type: int __fastcall(FMOD::DSPConnectionPool *this)
#[doc(alias = "FMOD::DSPConnectionPool::close(void)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool5closeEv")]
// IDA 0xafe90: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afe90() {
}

// 0xaff70 — FMOD::DSPConnectionPool::init(FMOD::SystemI *,int,int,int)
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::SystemI *, int, int, int)
#[doc(alias = "FMOD::DSPConnectionPool::init(FMOD::SystemI *,int,int,int)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool4initEPNS_7SystemIEiii")]
// IDA 0xaff70: 143 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aff70() {
}

// 0xb01bc — FMOD::DSPConnectionPool::free(FMOD::DSPConnectionI *,bool)
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::DSPConnectionI *, bool)
#[doc(alias = "FMOD::DSPConnectionPool::free(FMOD::DSPConnectionI *,bool)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool4freeEPNS_14DSPConnectionIEb")]
// IDA 0xb01bc: 70 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b01bc() {
}

// 0xb02d4 — FMOD::DSPConnectionPool::alloc(FMOD::DSPConnectionI **,bool)
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::DSPConnectionI **, bool)
#[doc(alias = "FMOD::DSPConnectionPool::alloc(FMOD::DSPConnectionI **,bool)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool5allocEPPNS_14DSPConnectionIEb")]
// IDA 0xb02d4: 201 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b02d4() {
}

// 0xb0608 — FMOD::DSPConnectionPool::getMemoryUsed(FMOD::MemoryTracker *)
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPConnectionPool::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool13getMemoryUsedEPNS_13MemoryTrackerE")]
// IDA 0xb0608: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0608() {
}

// 0xb0660 — FMOD::DSPDistortion::createInternal(void)
// type: int __fastcall(FMOD::DSPDistortion *this)
#[doc(alias = "FMOD::DSPDistortion::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion14createInternalEv")]
// IDA 0xb0660: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0660() {
}

// 0xb06d8 — FMOD::DSPDistortion::releaseInternal(void)
// type: int __fastcall(FMOD::DSPDistortion *this)
#[doc(alias = "FMOD::DSPDistortion::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion15releaseInternalEv")]
// IDA 0xb06d8: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b06d8() {
}

// 0xb06e0 — FMOD::DSPDistortion::resetInternal(void)
// type: int __fastcall(FMOD::DSPDistortion *this)
#[doc(alias = "FMOD::DSPDistortion::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion13resetInternalEv")]
// IDA 0xb06e0: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b06e0() {
}

// 0xb06e8 — FMOD::DSPDistortion::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPDistortion *this, int, float)
#[doc(alias = "FMOD::DSPDistortion::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20setParameterInternalEif")]
// IDA 0xb06e8: 3 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b06e8() {
}

// 0xb06f4 — FMOD::DSPDistortion::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPDistortion *)
#[doc(alias = "FMOD::DSPDistortion::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb06f4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b06f4() {
}

// 0xb0700 — FMOD::DSPDistortion::releaseCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPDistortion *)
#[doc(alias = "FMOD::DSPDistortion::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb0700: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0700() {
}

// 0xb070c — FMOD::DSPDistortion::resetCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPDistortion *)
#[doc(alias = "FMOD::DSPDistortion::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb070c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b070c() {
}

// 0xb0718 — FMOD::DSPDistortion::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPDistortion *, int, float)
#[doc(alias = "FMOD::DSPDistortion::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb0718: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0718() {
}

// 0xb0724 — FMOD::DSPDistortion::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPDistortion *this, int, float *, char *)
#[doc(alias = "FMOD::DSPDistortion::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20getParameterInternalEiPfPc")]
// IDA 0xb0724: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0724() {
}

// 0xb075c — FMOD::DSPDistortion::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPDistortion *, int, float *, char *)
#[doc(alias = "FMOD::DSPDistortion::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb075c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b075c() {
}

// 0xb0768 — FMOD::DSPDistortion::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPDistortion *this, float *, float *__dst, unsigned int, int, int)
#[doc(alias = "FMOD::DSPDistortion::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion12readInternalEPfS1_jii")]
// IDA 0xb0768: 431 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0768() {
}

// 0xb0e2c — FMOD::DSPDistortion::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPDistortion *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPDistortion::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb0e2c: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0e2c() {
}

// 0xb0e54 — FMOD::DSPDistortion::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPDistortion *this)
#[doc(alias = "FMOD::DSPDistortion::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion16getDescriptionExEv")]
// IDA 0xb0e54: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0e54() {
}

// 0xb0f30 — __Z41__static_initialization_and_destruction_0ii_19
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_19")]
// IDA 0xb0f30: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0f30() {
}

// 0xb0f74 — global constructor keyed toFMOD::dspdistortion
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dspdistortion")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dspdistortionE")]
// IDA 0xb0f74: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0f74() {
}

// 0xb0f80 — FMOD::DSPEcho::createInternal(void)
// type: int __fastcall(FMOD::DSPEcho *this)
#[doc(alias = "FMOD::DSPEcho::createInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14createInternalEv")]
// IDA 0xb0f80: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b0f80() {
}

// 0xb1034 — FMOD::DSPEcho::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPEcho *)
#[doc(alias = "FMOD::DSPEcho::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb1034: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1034() {
}

// 0xb1040 — FMOD::DSPEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPEcho *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb1040: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1040() {
}

// 0xb1074 — FMOD::DSPEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPEcho *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb1074: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1074() {
}

// 0xb10cc — FMOD::DSPEcho::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPEcho *this, int, float *, char *)
#[doc(alias = "FMOD::DSPEcho::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20getParameterInternalEiPfPc")]
// IDA 0xb10cc: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b10cc() {
}

// 0xb11f4 — FMOD::DSPEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPEcho *, int, float *, char *)
#[doc(alias = "FMOD::DSPEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb11f4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b11f4() {
}

// 0xb1200 — FMOD::DSPEcho::releaseInternal(void)
// type: int __fastcall(FMOD::DSPEcho *this)
#[doc(alias = "FMOD::DSPEcho::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho15releaseInternalEv")]
// IDA 0xb1200: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1200() {
}

// 0xb1254 — FMOD::DSPEcho::releaseCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPEcho *)
#[doc(alias = "FMOD::DSPEcho::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb1254: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1254() {
}

// 0xb1260 — FMOD::DSPEcho::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPEcho *this, float *, float *__dst, unsigned int, int, int)
#[doc(alias = "FMOD::DSPEcho::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD7DSPEcho12readInternalEPfS1_jii")]
// IDA 0xb1260: 863 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1260() {
}

// 0xb1fec — FMOD::DSPEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPEcho *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD7DSPEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb1fec: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1fec() {
}

// 0xb2014 — FMOD::DSPEcho::resetInternal(void)
// type: int __fastcall(FMOD::DSPEcho *this)
#[doc(alias = "FMOD::DSPEcho::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho13resetInternalEv")]
// IDA 0xb2014: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2014() {
}

// 0xb2048 — FMOD::DSPEcho::resetCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPEcho *)
#[doc(alias = "FMOD::DSPEcho::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb2048: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2048() {
}

// 0xb2054 — FMOD::DSPEcho::updateInternal(void)
// type: int __fastcall(FMOD::DSPEcho *this)
#[doc(alias = "FMOD::DSPEcho::updateInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14updateInternalEv")]
// IDA 0xb2054: 96 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2054() {
}

// 0xb21e8 — FMOD::DSPEcho::updateCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPEcho *)
#[doc(alias = "FMOD::DSPEcho::updateCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14updateCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb21e8: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b21e8() {
}

// 0xb21f4 — FMOD::DSPEcho::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPEcho *this)
#[doc(alias = "FMOD::DSPEcho::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho16getDescriptionExEv")]
// IDA 0xb21f4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b21f4() {
}

// 0xb22f0 — FMOD::DSPEcho::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPEcho *this, int, float)
#[doc(alias = "FMOD::DSPEcho::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20setParameterInternalEif")]
// IDA 0xb22f0: 77 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b22f0() {
}

// 0xb2424 — FMOD::DSPEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPEcho *, int, float)
#[doc(alias = "FMOD::DSPEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb2424: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2424() {
}

// 0xb2430 — __Z41__static_initialization_and_destruction_0ii_20
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_20")]
// IDA 0xb2430: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2430() {
}

// 0xb2474 — global constructor keyed toFMOD::dspecho_desc
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dspecho_desc")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspecho_descE")]
// IDA 0xb2474: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2474() {
}

// 0xb2480 — FMOD::DSPFFT::process(int)
// type: int __fastcall(FMOD::DSPFFT *this, int)
#[doc(alias = "FMOD::DSPFFT::process(int)")]
#[doc(alias = "__ZN4FMOD6DSPFFT7processEi")]
// IDA 0xb2480: 182 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2480() {
}

// 0xb2764 — FMOD::DSPFFT::DSPFFT(void)
// type: float __fastcall(FMOD::DSPFFT *this)
#[doc(alias = "FMOD::DSPFFT::DSPFFT(void)")]
#[doc(alias = "__ZN4FMOD6DSPFFTC2Ev")]
// IDA 0xb2764: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b2764() {
}

// 0xb27bc — FMOD::DSPFFT::DSPFFT(void)
// type: float __fastcall(FMOD::DSPFFT *this)
#[doc(alias = "FMOD::DSPFFT::DSPFFT(void)")]
#[doc(alias = "__ZN4FMOD6DSPFFTC1Ev")]
// IDA 0xb27bc: 1 insn (B) — branch/return thunk, no state change.
pub fn stub_b27bc() {
}

// 0xb27c0 — FMOD::DSPFFT::getSpectrum(float *,unsigned int,unsigned int,float *,int,int,int,FMOD_DSP_FFT_WINDOW)
// type: int __fastcall(FMOD::DSPFFT *this, int, unsigned int, unsigned int, int, int, int, int, int)
#[doc(alias = "FMOD::DSPFFT::getSpectrum(float *,unsigned int,unsigned int,float *,int,int,int,FMOD_DSP_FFT_WINDOW)")]
#[doc(alias = "__ZN4FMOD6DSPFFT11getSpectrumEPfjjS1_iii19FMOD_DSP_FFT_WINDOW")]
// IDA 0xb27c0: 551 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b27c0() {
}

// 0xb308c — FMOD::DSPFilter::getHistoryBuffer(float **,unsigned int *,unsigned int *)
// type: int __fastcall(FMOD::DSPFilter *this, float **, unsigned int *, unsigned int *)
#[doc(alias = "FMOD::DSPFilter::getHistoryBuffer(float **,unsigned int *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9DSPFilter16getHistoryBufferEPPfPjS3_")]
// IDA 0xb308c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b308c() {
}

// 0xb30b8 — FMOD::DSPFilter::release(bool)
// type: int __fastcall(FMOD::DSPFilter *this, bool)
#[doc(alias = "FMOD::DSPFilter::release(bool)")]
#[doc(alias = "__ZN4FMOD9DSPFilter7releaseEb")]
// IDA 0xb30b8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b30b8() {
}

// 0xb30e4 — FMOD::DSPFilter::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)
// type: int __fastcall(int, float **, int *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::DSPFilter::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9DSPFilter4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// IDA 0xb30e4: 439 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b30e4() {
}

// 0xb37c0 — FMOD::DSPFilter::stopBuffering(void)
// type: int __fastcall(FMOD::DSPFilter *this)
#[doc(alias = "FMOD::DSPFilter::stopBuffering(void)")]
#[doc(alias = "__ZN4FMOD9DSPFilter13stopBufferingEv")]
// IDA 0xb37c0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b37c0() {
}

// 0xb3810 — FMOD::DSPFilter::startBuffering(void)
// type: int __fastcall(FMOD::DSPFilter *this)
#[doc(alias = "FMOD::DSPFilter::startBuffering(void)")]
#[doc(alias = "__ZN4FMOD9DSPFilter14startBufferingEv")]
// IDA 0xb3810: 38 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3810() {
}

// 0xb38a8 — FMOD::DSPFlange::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPFlange *this, int, float)
#[doc(alias = "FMOD::DSPFlange::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20setParameterInternalEif")]
// IDA 0xb38a8: 19 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b38a8() {
}

// 0xb38f4 — FMOD::DSPFlange::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPFlange *, int, float)
#[doc(alias = "FMOD::DSPFlange::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb38f4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b38f4() {
}

// 0xb3900 — FMOD::DSPFlange::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPFlange *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPFlange::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb3900: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3900() {
}

// 0xb3934 — FMOD::DSPFlange::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPFlange *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPFlange::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb3934: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3934() {
}

// 0xb398c — FMOD::DSPFlange::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPFlange *this, int, float *, char *)
#[doc(alias = "FMOD::DSPFlange::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20getParameterInternalEiPfPc")]
// IDA 0xb398c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b398c() {
}

// 0xb3a84 — FMOD::DSPFlange::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPFlange *, int, float *, char *)
#[doc(alias = "FMOD::DSPFlange::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb3a84: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3a84() {
}

// 0xb3a90 — FMOD::DSPFlange::resetInternal(void)
// type: int __fastcall(FMOD::DSPFlange *this)
#[doc(alias = "FMOD::DSPFlange::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange13resetInternalEv")]
// IDA 0xb3a90: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3a90() {
}

// 0xb3ac4 — FMOD::DSPFlange::resetCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPFlange *)
#[doc(alias = "FMOD::DSPFlange::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb3ac4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3ac4() {
}

// 0xb3ad0 — FMOD::DSPFlange::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPFlange *this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPFlange::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPFlange12readInternalEPfS1_jii")]
// IDA 0xb3ad0: 248 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3ad0() {
}

// 0xb3ec0 — FMOD::DSPFlange::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPFlange *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPFlange::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPFlange12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb3ec0: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3ec0() {
}

// 0xb3ee8 — FMOD::DSPFlange::releaseInternal(void)
// type: int __fastcall(FMOD::DSPFlange *this)
#[doc(alias = "FMOD::DSPFlange::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange15releaseInternalEv")]
// IDA 0xb3ee8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3ee8() {
}

// 0xb3f38 — FMOD::DSPFlange::releaseCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPFlange *)
#[doc(alias = "FMOD::DSPFlange::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb3f38: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3f38() {
}

// 0xb3f44 — FMOD::DSPFlange::createInternal(void)
// type: int __fastcall(FMOD::DSPFlange *this)
#[doc(alias = "FMOD::DSPFlange::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange14createInternalEv")]
// IDA 0xb3f44: 117 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b3f44() {
}

// 0xb4138 — FMOD::DSPFlange::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPFlange *)
#[doc(alias = "FMOD::DSPFlange::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb4138: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4138() {
}

// 0xb4144 — FMOD::DSPFlange::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPFlange *this)
#[doc(alias = "FMOD::DSPFlange::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange16getDescriptionExEv")]
// IDA 0xb4144: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4144() {
}

// 0xb4230 — __Z41__static_initialization_and_destruction_0ii_21
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_21")]
// IDA 0xb4230: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4230() {
}

// 0xb4274 — global constructor keyed toFMOD::dspflange
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dspflange")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspflangeE")]
// IDA 0xb4274: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4274() {
}

// 0xb4280 — FMOD::DSPHighPass::resetInternal(void)
// type: int __fastcall(FMOD::DSPHighPass *this)
#[doc(alias = "FMOD::DSPHighPass::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass13resetInternalEv")]
// IDA 0xb4280: 18 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4280() {
}

// 0xb42c8 — FMOD::DSPHighPass::process(float *,float *,unsigned int,int)
// type: int __fastcall(FMOD::DSPHighPass *this, float *, float *, unsigned int, int)
#[doc(alias = "FMOD::DSPHighPass::process(float *,float *,unsigned int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass7processEPfS1_ji")]
// IDA 0xb42c8: 696 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b42c8() {
}

// 0xb4dd0 — FMOD::DSPHighPass::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPHighPass *this, int, float)
#[doc(alias = "FMOD::DSPHighPass::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20setParameterInternalEif")]
// IDA 0xb4dd0: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4dd0() {
}

// 0xb4e0c — FMOD::DSPHighPass::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int()
#[doc(alias = "FMOD::DSPHighPass::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb4e0c: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4e0c() {
}

// 0xb4e14 — FMOD::DSPHighPass::resetCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPHighPass *)
#[doc(alias = "FMOD::DSPHighPass::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb4e14: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4e14() {
}

// 0xb4e20 — FMOD::DSPHighPass::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPHighPass *, int, float)
#[doc(alias = "FMOD::DSPHighPass::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb4e20: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4e20() {
}

// 0xb4e2c — FMOD::DSPHighPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPHighPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb4e2c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4e2c() {
}

// 0xb4e84 — FMOD::DSPHighPass::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPHighPass *this, int, float *, char *)
#[doc(alias = "FMOD::DSPHighPass::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20getParameterInternalEiPfPc")]
// IDA 0xb4e84: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4e84() {
}

// 0xb4efc — FMOD::DSPHighPass::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPHighPass *, int, float *, char *)
#[doc(alias = "FMOD::DSPHighPass::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb4efc: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4efc() {
}

// 0xb4f08 — FMOD::DSPHighPass::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPHighPass *this)
#[doc(alias = "FMOD::DSPHighPass::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass16getDescriptionExEv")]
// IDA 0xb4f08: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4f08() {
}

// 0xb4fe4 — FMOD::DSPHighPass::updateCoefficients(float,float)
// type: int __fastcall(FMOD::DSPHighPass *this, float32_t, float32_t)
#[doc(alias = "FMOD::DSPHighPass::updateCoefficients(float,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass18updateCoefficientsEff")]
// IDA 0xb4fe4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4fe4() {
}

// 0xb5098 — FMOD::DSPHighPass::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPHighPass *this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPHighPass::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass12readInternalEPfS1_jii")]
// IDA 0xb5098: 137 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5098() {
}

// 0xb52c0 — FMOD::DSPHighPass::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPHighPass *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPHighPass::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb52c0: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b52c0() {
}

// 0xb52e8 — FMOD::DSPHighPass::createInternal(void)
// type: int __fastcall(FMOD::DSPHighPass *this)
#[doc(alias = "FMOD::DSPHighPass::createInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass14createInternalEv")]
// IDA 0xb52e8: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b52e8() {
}

// 0xb53a4 — FMOD::DSPHighPass::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPHighPass *)
#[doc(alias = "FMOD::DSPHighPass::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb53a4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b53a4() {
}

// 0xb53b0 — __Z41__static_initialization_and_destruction_0ii_22
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_22")]
// IDA 0xb53b0: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b53b0() {
}

// 0xb53f4 — global constructor keyed toFMOD::dsphighpass
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dsphighpass")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD11dsphighpassE")]
// IDA 0xb53f4: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b53f4() {
}

// 0xb5400 — FMOD::DSPITEcho::createInternal(void)
// type: int __fastcall(FMOD::DSPITEcho *this)
#[doc(alias = "FMOD::DSPITEcho::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho14createInternalEv")]
// IDA 0xb5400: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5400() {
}

// 0xb5484 — FMOD::DSPITEcho::createCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPITEcho *)
#[doc(alias = "FMOD::DSPITEcho::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb5484: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5484() {
}

// 0xb5490 — FMOD::DSPITEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPITEcho *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPITEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb5490: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5490() {
}

// 0xb54dc — FMOD::DSPITEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)
// type: int __fastcall(FMOD::DSPITEcho *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPITEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb54dc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b54dc() {
}

// 0xb5534 — FMOD::DSPITEcho::getParameterInternal(int,float *,char *)
// type: int __fastcall(FMOD::DSPITEcho *this, int, float *, char *__dst)
#[doc(alias = "FMOD::DSPITEcho::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20getParameterInternalEiPfPc")]
// IDA 0xb5534: 78 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5534() {
}

// 0xb568c — FMOD::DSPITEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)
// type: int __fastcall(FMOD::DSPITEcho *, int, float *, char *)
#[doc(alias = "FMOD::DSPITEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb568c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b568c() {
}

// 0xb5698 — FMOD::DSPITEcho::releaseInternal(void)
// type: int __fastcall(FMOD::DSPITEcho *this)
#[doc(alias = "FMOD::DSPITEcho::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho15releaseInternalEv")]
// IDA 0xb5698: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5698() {
}

// 0xb5700 — FMOD::DSPITEcho::releaseCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPITEcho *)
#[doc(alias = "FMOD::DSPITEcho::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb5700: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5700() {
}

// 0xb570c — FMOD::DSPITEcho::resetInternal(void)
// type: int __fastcall(FMOD::DSPITEcho *this)
#[doc(alias = "FMOD::DSPITEcho::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho13resetInternalEv")]
// IDA 0xb570c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b570c() {
}

// 0xb575c — FMOD::DSPITEcho::resetCallback(FMOD_DSP_STATE *)
// type: int __fastcall(FMOD::DSPITEcho *)
#[doc(alias = "FMOD::DSPITEcho::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb575c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b575c() {
}

// 0xb5768 — FMOD::DSPITEcho::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPITEcho *this, int, float)
#[doc(alias = "FMOD::DSPITEcho::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20setParameterInternalEif")]
// IDA 0xb5768: 120 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5768() {
}

// 0xb5960 — FMOD::DSPITEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)
// type: int __fastcall(FMOD::DSPITEcho *, int, float)
#[doc(alias = "FMOD::DSPITEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb5960: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5960() {
}

// 0xb596c — FMOD::DSPITEcho::readInternal(float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPITEcho *this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPITEcho::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho12readInternalEPfS1_jii")]
// IDA 0xb596c: 246 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b596c() {
}

// 0xb5d44 — FMOD::DSPITEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)
// type: int __fastcall(FMOD::DSPITEcho *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPITEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb5d44: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5d44() {
}

// 0xb5d6c — FMOD::DSPITEcho::getDescriptionEx(void)
// type: void *__fastcall(FMOD::DSPITEcho *this)
#[doc(alias = "FMOD::DSPITEcho::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho16getDescriptionExEv")]
// IDA 0xb5d6c: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5d6c() {
}

// 0xb5e58 — __Z41__static_initialization_and_destruction_0ii_23
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_23")]
// IDA 0xb5e58: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5e58() {
}

// 0xb5e9c — global constructor keyed toFMOD::dspitecho
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dspitecho")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspitechoE")]
// IDA 0xb5e9c: 3 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5e9c() {
}

// 0xb5ea8 — FMOD::DSPLowPass::bilinear(float,float,float,float,float,float,float *,float,float *)
// type: int __fastcall(FMOD::DSPLowPass *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float *, float32_t, float *)
#[doc(alias = "FMOD::DSPLowPass::bilinear(float,float,float,float,float,float,float *,float,float *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass8bilinearEffffffPffS1_")]
// IDA 0xb5ea8: 65 insns (VPUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5ea8() {
}

// 0xb5fac — FMOD::DSPLowPass::setParameterInternal(int,float)
// type: int __fastcall(FMOD::DSPLowPass *this, int, float)
#[doc(alias = "FMOD::DSPLowPass::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20setParameterInternalEif")]
// IDA 0xb5fac: 14 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5fac() {
}

// 0xb5fe4 — FMOD::DSPLowPass::getMemoryUsedImpl(FMOD::MemoryTracker *)
// type: int()
#[doc(alias = "FMOD::DSPLowPass::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb5fe4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5fe4() {
}
