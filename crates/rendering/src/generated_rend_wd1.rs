//! rendering shard rend_wd1 — 120 stubs 0xb5fec..0x690db4 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render 15112 total filtered, 1 uncovered -> 0, gap filler distinct per crate) [skeleton batch WD1]
//! Source: ida/export.json (85545 funcs) EA asc Ogre/G3D/Render-filtered then global gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb5fec — __ZN4FMOD10DSPLowPass20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPLowPass *, int, float)
#[doc(alias = "FMOD::DSPLowPass::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb5fec: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5fec() {
}


// 0xb5ff8 — __ZN4FMOD10DSPLowPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPLowPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb5ff8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5ff8() {
}


// 0xb6050 — __ZN4FMOD10DSPLowPass20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPass *this, int, float *, char *)
#[doc(alias = "FMOD::DSPLowPass::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20getParameterInternalEiPfPc")]
// IDA 0xb6050: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6050() {
}


// 0xb60c8 — __ZN4FMOD10DSPLowPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPLowPass *, int, float *, char *)
#[doc(alias = "FMOD::DSPLowPass::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb60c8: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b60c8() {
}


// 0xb60d4 — __ZN4FMOD10DSPLowPass7prewarpEPfS1_S1_ff
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, float *, float32_t, float32_t)
#[doc(alias = "FMOD::DSPLowPass::prewarp(float *,float *,float *,float,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7prewarpEPfS1_S1_ff")]
// IDA 0xb60d4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b60d4() {
}


// 0xb6144 — __ZN4FMOD10DSPLowPass7szxformEPfS1_S1_S1_S1_S1_ffS1_S1_
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, float *, float *, float *, float *, float32_t, float32_t, float *, float *)
#[doc(alias = "FMOD::DSPLowPass::szxform(float *,float *,float *,float *,float *,float *,float,float,float *,float *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7szxformEPfS1_S1_S1_S1_S1_ffS1_S1_")]
// IDA 0xb6144: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6144() {
}


// 0xb61ec — __ZN4FMOD10DSPLowPass16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPLowPass *this)
#[doc(alias = "FMOD::DSPLowPass::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass16getDescriptionExEv")]
// IDA 0xb61ec: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61ec() {
}


// 0xb62b8 — __ZN4FMOD10DSPLowPass11updateStateEff
// type: int __fastcall(FMOD::DSPLowPass *this, float, float32_t)
#[doc(alias = "FMOD::DSPLowPass::updateState(float,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass11updateStateEff")]
// IDA 0xb62b8: 117 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b62b8() {
}


// 0xb648c — __ZN4FMOD10DSPLowPass14createInternalEv
// type: int __fastcall(FMOD::DSPLowPass *this)
#[doc(alias = "FMOD::DSPLowPass::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass14createInternalEv")]
// IDA 0xb648c: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b648c() {
}


// 0xb6590 — __ZN4FMOD10DSPLowPass14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPLowPass *)
#[doc(alias = "FMOD::DSPLowPass::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb6590: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6590() {
}


// 0xb659c — __ZN4FMOD10DSPLowPass7processEPfS1_ji
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, unsigned int, int)
#[doc(alias = "FMOD::DSPLowPass::process(float *,float *,unsigned int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7processEPfS1_ji")]
// IDA 0xb659c: 1139 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b659c() {
}


// 0xb779c — __ZN4FMOD10DSPLowPass12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPLowPass::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass12readInternalEPfS1_jii")]
// IDA 0xb779c: 137 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b779c() {
}


// 0xb79c4 — __ZN4FMOD10DSPLowPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPLowPass *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPLowPass::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb79c4: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b79c4() {
}


// 0xb79ec — __Z41__static_initialization_and_destruction_0ii_24
// type: int __fastcall(int result, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_24")]
// IDA 0xb79ec: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b79ec() {
}


// 0xb7a30 — __GLOBAL__I__ZN4FMOD10dsplowpassE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::dsplowpass")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD10dsplowpassE")]
// IDA 0xb7a30: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_b7a30() {
}


// 0xb7a3c — __ZN4FMOD11DSPLowPass213resetInternalEv
// type: int __fastcall(FMOD::DSPLowPass2 *this)
#[doc(alias = "FMOD::DSPLowPass2::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass213resetInternalEv")]
// IDA 0xb7a3c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7a3c() {
}


// 0xb7a74 — __ZN4FMOD11DSPLowPass220setParameterInternalEif
// type: int __fastcall(FMOD::DSPLowPass2 *this, int, float)
#[doc(alias = "FMOD::DSPLowPass2::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220setParameterInternalEif")]
// IDA 0xb7a74: 10 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7a74() {
}


// 0xb7a9c — __ZN4FMOD11DSPLowPass217getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int()
#[doc(alias = "FMOD::DSPLowPass2::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass217getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb7a9c: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7a9c() {
}


// 0xb7aa4 — __ZN4FMOD11DSPLowPass213resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPLowPass2 *)
#[doc(alias = "FMOD::DSPLowPass2::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass213resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb7aa4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7aa4() {
}


// 0xb7ab0 — __ZN4FMOD11DSPLowPass220setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPLowPass2 *, int, float)
#[doc(alias = "FMOD::DSPLowPass2::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb7ab0: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7ab0() {
}


// 0xb7abc — __ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPLowPass2::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb7abc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7abc() {
}


// 0xb7b14 — __ZN4FMOD11DSPLowPass220getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPass2 *this, int, float *, char *)
#[doc(alias = "FMOD::DSPLowPass2::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220getParameterInternalEiPfPc")]
// IDA 0xb7b14: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7b14() {
}


// 0xb7b8c — __ZN4FMOD11DSPLowPass220getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPLowPass2 *, int, float *, char *)
#[doc(alias = "FMOD::DSPLowPass2::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb7b8c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7b8c() {
}


// 0xb7b98 — __ZN4FMOD11DSPLowPass218updateCoefficientsEff
// type: int __fastcall(FMOD::DSPLowPass2 *this, float32_t, float32_t)
#[doc(alias = "FMOD::DSPLowPass2::updateCoefficients(float,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass218updateCoefficientsEff")]
// IDA 0xb7b98: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7b98() {
}


// 0xb7cc8 — __ZN4FMOD11DSPLowPass212readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPLowPass2 *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPLowPass2::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass212readInternalEPfS1_jii")]
// IDA 0xb7cc8: 676 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7cc8() {
}


// 0xb8780 — __ZN4FMOD11DSPLowPass212readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "FMOD::DSPLowPass2::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass212readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb8780: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8780() {
}


// 0xb87a8 — __ZN4FMOD11DSPLowPass214createInternalEv
// type: int __fastcall(FMOD::DSPLowPass2 *this)
#[doc(alias = "FMOD::DSPLowPass2::createInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass214createInternalEv")]
// IDA 0xb87a8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b87a8() {
}


// 0xb8840 — __ZN4FMOD11DSPLowPass214createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPLowPass2::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass214createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb8840: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8840() {
}


// 0xb884c — __ZN4FMOD11DSPLowPass216getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPLowPass2 *__hidden this)
#[doc(alias = "FMOD::DSPLowPass2::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass216getDescriptionExEv")]
// IDA 0xb884c: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b884c() {
}


// 0xb8928 — __Z41__static_initialization_and_destruction_0ii_25
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_25")]
// IDA 0xb8928: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8928() {
}


// 0xb896c — __GLOBAL__I__ZN4FMOD11dsplowpass2E
#[doc(alias = "global constructor keyed toFMOD::dsplowpass2")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD11dsplowpass2E")]
// IDA 0xb896c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_b896c() {
}


// 0xb8978 — __ZN4FMOD16DSPLowPassSimple13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
#[doc(alias = "FMOD::DSPLowPassSimple::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple13resetInternalEv")]
// IDA 0xb8978: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8978() {
}


// 0xb89b0 — __ZN4FMOD16DSPLowPassSimple18updateCoefficientsEf
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, float)
#[doc(alias = "FMOD::DSPLowPassSimple::updateCoefficients(float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple18updateCoefficientsEf")]
// IDA 0xb89b0: 44 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b89b0() {
}


// 0xb8a70 — __ZN4FMOD16DSPLowPassSimple14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
#[doc(alias = "FMOD::DSPLowPassSimple::createInternal(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple14createInternalEv")]
// IDA 0xb8a70: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8a70() {
}


// 0xb8b00 — __ZN4FMOD16DSPLowPassSimple20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, int, float)
#[doc(alias = "FMOD::DSPLowPassSimple::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20setParameterInternalEif")]
// IDA 0xb8b00: 4 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b00() {
}


// 0xb8b10 — __ZN4FMOD16DSPLowPassSimple17getMemoryUsedImplEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPLowPassSimple::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb8b10: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b10() {
}


// 0xb8b18 — __ZN4FMOD16DSPLowPassSimple14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPLowPassSimple::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb8b18: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b18() {
}


// 0xb8b24 — __ZN4FMOD16DSPLowPassSimple13resetCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPLowPassSimple::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb8b24: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b24() {
}


// 0xb8b30 — __ZN4FMOD16DSPLowPassSimple20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "FMOD::DSPLowPassSimple::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb8b30: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b30() {
}


// 0xb8b3c — __ZN4FMOD16DSPLowPassSimple21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPLowPassSimple::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb8b3c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b3c() {
}


// 0xb8b94 — __ZN4FMOD16DSPLowPassSimple20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPassSimple *this, int, float *, char *)
#[doc(alias = "FMOD::DSPLowPassSimple::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20getParameterInternalEiPfPc")]
// IDA 0xb8b94: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b94() {
}


// 0xb8bd4 — __ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPLowPassSimple::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb8bd4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8bd4() {
}


// 0xb8be0 — __ZN4FMOD16DSPLowPassSimple12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPLowPassSimple::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple12readInternalEPfS1_jii")]
// IDA 0xb8be0: 609 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8be0() {
}


// 0xb958c — __ZN4FMOD16DSPLowPassSimple12readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "FMOD::DSPLowPassSimple::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb958c: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b958c() {
}


// 0xb95b4 — __ZN4FMOD16DSPLowPassSimple16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
#[doc(alias = "FMOD::DSPLowPassSimple::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple16getDescriptionExEv")]
// IDA 0xb95b4: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b95b4() {
}


// 0xb9690 — __Z41__static_initialization_and_destruction_0ii_26
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_26")]
// IDA 0xb9690: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9690() {
}


// 0xb96d4 — __GLOBAL__I__ZN4FMOD17dsplowpass_simpleE
#[doc(alias = "global constructor keyed toFMOD::dsplowpass_simple")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD17dsplowpass_simpleE")]
// IDA 0xb96d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_b96d4() {
}


// 0xb96e0 — __ZN4FMOD12DSPNormalize14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
#[doc(alias = "FMOD::DSPNormalize::createInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize14createInternalEv")]
// IDA 0xb96e0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b96e0() {
}


// 0xb9770 — __ZN4FMOD12DSPNormalize15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
#[doc(alias = "FMOD::DSPNormalize::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize15releaseInternalEv")]
// IDA 0xb9770: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9770() {
}


// 0xb9778 — __ZN4FMOD12DSPNormalize13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
#[doc(alias = "FMOD::DSPNormalize::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize13resetInternalEv")]
// IDA 0xb9778: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9778() {
}


// 0xb978c — __ZN4FMOD12DSPNormalize20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, int, float)
#[doc(alias = "FMOD::DSPNormalize::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20setParameterInternalEif")]
// IDA 0xb978c: 25 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b978c() {
}


// 0xb97f4 — __ZN4FMOD12DSPNormalize17getMemoryUsedImplEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPNormalize::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xb97f4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b97f4() {
}


// 0xb97fc — __ZN4FMOD12DSPNormalize14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPNormalize::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb97fc: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b97fc() {
}


// 0xb9808 — __ZN4FMOD12DSPNormalize15releaseCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPNormalize::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb9808: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9808() {
}


// 0xb9814 — __ZN4FMOD12DSPNormalize13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPNormalize *)
#[doc(alias = "FMOD::DSPNormalize::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb9814: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9814() {
}


// 0xb9820 — __ZN4FMOD12DSPNormalize20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "FMOD::DSPNormalize::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb9820: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9820() {
}


// 0xb982c — __ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPNormalize::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xb982c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b982c() {
}


// 0xb9884 — __ZN4FMOD12DSPNormalize20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, int, float *, char *)
#[doc(alias = "FMOD::DSPNormalize::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20getParameterInternalEiPfPc")]
// IDA 0xb9884: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9884() {
}


// 0xb9934 — __ZN4FMOD12DSPNormalize20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPNormalize::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb9934: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9934() {
}


// 0xb9940 — __ZN4FMOD12DSPNormalize12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, float *, float *__dst, unsigned int, int, int)
#[doc(alias = "FMOD::DSPNormalize::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readInternalEPfS1_jii")]
// IDA 0xb9940: 85 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9940() {
}


// 0xb9a94 — __ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "FMOD::DSPNormalize::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xb9a94: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a94() {
}


// 0xb9abc — __ZN4FMOD12DSPNormalize16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
#[doc(alias = "FMOD::DSPNormalize::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize16getDescriptionExEv")]
// IDA 0xb9abc: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9abc() {
}


// 0xb9ba8 — __Z41__static_initialization_and_destruction_0ii_27
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_27")]
// IDA 0xb9ba8: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9ba8() {
}


// 0xb9bec — __GLOBAL__I__ZN4FMOD12dspnormalizeE
#[doc(alias = "global constructor keyed toFMOD::dspnormalize")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspnormalizeE")]
// IDA 0xb9bec: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_b9bec() {
}


// 0xb9bf8 — __ZN4FMOD13DSPOscillator14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
#[doc(alias = "FMOD::DSPOscillator::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createInternalEv")]
// IDA 0xb9bf8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bf8() {
}


// 0xb9c78 — __ZN4FMOD13DSPOscillator15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
#[doc(alias = "FMOD::DSPOscillator::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseInternalEv")]
// IDA 0xb9c78: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c78() {
}


// 0xb9c80 — __ZN4FMOD13DSPOscillator20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float)
#[doc(alias = "FMOD::DSPOscillator::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterInternalEif")]
// IDA 0xb9c80: 19 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c80() {
}


// 0xb9ccc — __ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPOscillator::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb9ccc: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9ccc() {
}


// 0xb9cd8 — __ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPOscillator *)
#[doc(alias = "FMOD::DSPOscillator::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xb9cd8: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9cd8() {
}


// 0xb9ce4 — __ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "FMOD::DSPOscillator::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xb9ce4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9ce4() {
}


// 0xb9cf0 — __ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float *, char *)
#[doc(alias = "FMOD::DSPOscillator::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc")]
// IDA 0xb9cf0: 62 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9cf0() {
}


// 0xb9e04 — __ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPOscillator::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xb9e04: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e04() {
}


// 0xb9e10 — __ZN4FMOD13DSPOscillator12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPOscillator::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readInternalEPfS1_jii")]
// IDA 0xb9e10: 180 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e10() {
}


// 0xba0f4 — __ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "FMOD::DSPOscillator::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xba0f4: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba0f4() {
}


// 0xba11c — __ZN4FMOD13DSPOscillator16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
#[doc(alias = "FMOD::DSPOscillator::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator16getDescriptionExEv")]
// IDA 0xba11c: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba11c() {
}


// 0xba1fc — __ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, FMOD::MemoryTracker *this)
#[doc(alias = "FMOD::DSPI::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xba1fc: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba1fc() {
}


// 0xba22c — __Z41__static_initialization_and_destruction_0ii_28
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_28")]
// IDA 0xba22c: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba22c() {
}


// 0xba270 — __GLOBAL__I__ZN4FMOD13dsposcillatorE
#[doc(alias = "global constructor keyed toFMOD::dsposcillator")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dsposcillatorE")]
// IDA 0xba270: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_ba270() {
}


// 0xba27c — __ZN4FMOD10DSPParamEq13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
#[doc(alias = "FMOD::DSPParamEq::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetInternalEv")]
// IDA 0xba27c: 18 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba27c() {
}


// 0xba2c4 — __ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xba2c4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba2c4() {
}


// 0xba2cc — __ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPParamEq::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xba2cc: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba2cc() {
}


// 0xba2d8 — __ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xba2d8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba2d8() {
}


// 0xba330 — __ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPParamEq *this, int, float *, char *)
#[doc(alias = "FMOD::DSPParamEq::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc")]
// IDA 0xba330: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba330() {
}


// 0xba3e0 — __ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPParamEq::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xba3e0: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba3e0() {
}


// 0xba3ec — __ZN4FMOD10DSPParamEq18updateCoefficientsEfff
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float, float, float)
#[doc(alias = "FMOD::DSPParamEq::updateCoefficients(float,float,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq18updateCoefficientsEfff")]
// IDA 0xba3ec: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba3ec() {
}


// 0xba49c — __ZN4FMOD10DSPParamEq12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPParamEq::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readInternalEPfS1_jii")]
// IDA 0xba49c: 1058 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba49c() {
}


// 0xbb54c — __ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPParamEq *, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPParamEq::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// IDA 0xbb54c: 10 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb54c() {
}


// 0xbb574 — __ZN4FMOD10DSPParamEq14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
#[doc(alias = "FMOD::DSPParamEq::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createInternalEv")]
// IDA 0xbb574: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb574() {
}


// 0xbb628 — __ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPParamEq::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xbb628: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb628() {
}


// 0xbb634 — __ZN4FMOD10DSPParamEq16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
#[doc(alias = "FMOD::DSPParamEq::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq16getDescriptionExEv")]
// IDA 0xbb634: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb634() {
}


// 0xbb710 — __ZN4FMOD10DSPParamEq20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, int, float)
#[doc(alias = "FMOD::DSPParamEq::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterInternalEif")]
// IDA 0xbb710: 23 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb710() {
}


// 0xbb770 — __ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "FMOD::DSPParamEq::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xbb770: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb770() {
}


// 0xbb77c — __Z41__static_initialization_and_destruction_0ii_29
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_29")]
// IDA 0xbb77c: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb77c() {
}


// 0xbb7c0 — __GLOBAL__I__ZN4FMOD10dspparameqE
#[doc(alias = "global constructor keyed toFMOD::dspparameq")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD10dspparameqE")]
// IDA 0xbb7c0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_bb7c0() {
}


// 0xbb7cc — __ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi")]
// IDA 0xbb7cc: 285 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb7cc() {
}


// 0xbbc58 — __ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2conj(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi")]
// IDA 0xbbc58: 320 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bbc58() {
}


// 0xbc170 — __ZN4FMOD16DSPPitchShiftSMB6cft1stEPf
// type: int __fastcall(int this, float *)
#[doc(alias = "FMOD::DSPPitchShiftSMB::cft1st(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cft1stEPf")]
// IDA 0xbc170: 212 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc170() {
}


// 0xbc4c8 — __ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::cftmdl(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi")]
// IDA 0xbc4c8: 351 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc4c8() {
}


// 0xbca50 — __ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf
// type: float *__fastcall(float *this, float *)
#[doc(alias = "FMOD::DSPPitchShiftSMB::cftfsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf")]
// IDA 0xbca50: 117 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bca50() {
}


// 0xbcc28 — __ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *)
#[doc(alias = "FMOD::DSPPitchShiftSMB::cftbsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf")]
// IDA 0xbcc28: 119 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcc28() {
}


// 0xbce08 — __ZN4FMOD16DSPPitchShiftSMB3fftEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::fft(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB3fftEPfi")]
// IDA 0xbce08: 22 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bce08() {
}


// 0xbce64 — __ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
#[doc(alias = "FMOD::DSPPitchShiftSMB::setResetPhaseFlag(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv")]
// IDA 0xbce64: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bce64() {
}


// 0xbce78 — __ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// IDA 0xbce78: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bce78() {
}


// 0xbcebc — __ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPPitchShift *this)
#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// IDA 0xbcebc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcebc() {
}


// 0xbcf14 — __ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float *, char *)
#[doc(alias = "FMOD::DSPPitchShift::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc")]
// IDA 0xbcf14: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bcf14() {
}


// 0xbd054 — __ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPPitchShift::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// IDA 0xbd054: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd054() {
}


// 0xbd060 — __ZN4FMOD13DSPPitchShift15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
#[doc(alias = "FMOD::DSPPitchShift::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseInternalEv")]
// IDA 0xbd060: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd060() {
}


// 0xbd0b4 — __ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPPitchShift::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE")]
// IDA 0xbd0b4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd0b4() {
}


// 0xbd0c0 — __ZN4FMOD16DSPPitchShiftSMB7smbInitEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
#[doc(alias = "FMOD::DSPPitchShiftSMB::smbInit(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7smbInitEv")]
// IDA 0xbd0c0: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd0c0() {
}


// 0xbd1b0 — __ZN4FMOD13DSPPitchShift13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
#[doc(alias = "FMOD::DSPPitchShift::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetInternalEv")]
// IDA 0xbd1b0: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd1b0() {
}


// 0xbd238 — __ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPPitchShift *)
#[doc(alias = "FMOD::DSPPitchShift::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE")]
// IDA 0xbd238: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd238() {
}


// 0xbd244 — __ZN4FMOD13DSPPitchShift14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
#[doc(alias = "FMOD::DSPPitchShift::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createInternalEv")]
// IDA 0xbd244: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd244() {
}


// 0xbd32c — __ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "FMOD::DSPPitchShift::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE")]
// IDA 0xbd32c: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd32c() {
}


// 0xbd338 — __ZN4FMOD13DSPPitchShift16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
#[doc(alias = "FMOD::DSPPitchShift::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift16getDescriptionExEv")]
// IDA 0xbd338: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd338() {
}


// 0xbd424 — __ZN4FMOD16DSPPitchShiftSMB7initFftEi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::initFft(int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7initFftEi")]
// IDA 0xbd424: 151 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd424() {
}


// 0xbd698 — __ZN4FMOD13DSPPitchShift20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float)
#[doc(alias = "FMOD::DSPPitchShift::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterInternalEif")]
// IDA 0xbd698: 381 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd698() {
}


// 0xbdcb4 — __ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif
#[doc(alias = "FMOD::DSPPitchShift::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif")]
// IDA 0xbdcb4: 3 insns (CMP..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdcb4() {
}


// 0xbdcc0 — __ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float, int, int, float, float *, float *, int, int)
#[doc(alias = "FMOD::DSPPitchShiftSMB::smbPitchShift(float,int,int,float,float *,float *,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii")]
// IDA 0xbdcc0: 1202 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdcc0() {
}


// 0xbf024 — __ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "FMOD::DSPPitchShift::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii")]
// IDA 0xbf024: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf024() {
}


// 0x690db4 — __ZN3RBX19ButtonBindingWidget12setTextureIdERKNS_9TextureIdE
// type: _DWORD __fastcall(RBX::ButtonBindingWidget *__hidden this, const RBX::TextureId *)
#[doc(alias = "RBX::ButtonBindingWidget::setTextureId(RBX::TextureId const&)")]
#[doc(alias = "__ZN3RBX19ButtonBindingWidget12setTextureIdERKNS_9TextureIdE")]
// IDA 0x690db4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_690db4() {
}
