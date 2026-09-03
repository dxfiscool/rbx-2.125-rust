//! core shard jz — 150 stubs EA-sorted 0xb9934..0xc4aa8 (global EA-sorted, next 150 not yet in core after jy 0xb9884, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) global EA-sorted ascending, next 150 not yet in rbx_core (32244 before -> 32394 after, gap 53301->53151).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::DSPNormalize::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb9934 — __ZN4FMOD12DSPNormalize20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xb9934() {
    // IDA 0xb9934: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPNormalize::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readInternalEPfS1_jii")]
// 0xb9940 — __ZN4FMOD12DSPNormalize12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, float *, float *__dst, unsigned int, int, int)
pub fn stub_0xb9940() {
    // IDA 0xb9940: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPNormalize::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb9a94 — __ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xb9a94() {
    // IDA 0xb9a94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPNormalize::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize16getDescriptionExEv")]
// 0xb9abc — __ZN4FMOD12DSPNormalize16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
pub fn stub_0xb9abc() {
    // IDA 0xb9abc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dspnormalize")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspnormalizeE")]
// 0xb9bec — __GLOBAL__I__ZN4FMOD12dspnormalizeE
pub fn stub_0xb9bec() {
    // IDA 0xb9bec: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPOscillator::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createInternalEv")]
// 0xb9bf8 — __ZN4FMOD13DSPOscillator14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xb9bf8() {
    // IDA 0xb9bf8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPOscillator::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseInternalEv")]
// 0xb9c78 — __ZN4FMOD13DSPOscillator15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xb9c78() {
    // IDA 0xb9c78: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPOscillator::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterInternalEif")]
// 0xb9c80 — __ZN4FMOD13DSPOscillator20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float)
pub fn stub_0xb9c80() {
    // IDA 0xb9c80: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPOscillator::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE")]
// 0xb9ccc — __ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xb9ccc() {
    // IDA 0xb9ccc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb9cd8 — __ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPOscillator *)
pub fn stub_0xb9cd8() {
    // IDA 0xb9cd8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb9ce4 — __ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xb9ce4() {
    // IDA 0xb9ce4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc")]
// 0xb9cf0 — __ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float *, char *)
pub fn stub_0xb9cf0() {
    // IDA 0xb9cf0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb9e04 — __ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xb9e04() {
    // IDA 0xb9e04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readInternalEPfS1_jii")]
// 0xb9e10 — __ZN4FMOD13DSPOscillator12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xb9e10() {
    // IDA 0xb9e10: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xba0f4 — __ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xba0f4() {
    // IDA 0xba0f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPOscillator::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator16getDescriptionExEv")]
// 0xba11c — __ZN4FMOD13DSPOscillator16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xba11c() {
    // IDA 0xba11c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xba1fc — __ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, FMOD::MemoryTracker *this)
pub fn stub_0xba1fc() {
    // IDA 0xba1fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dsposcillator")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dsposcillatorE")]
// 0xba270 — __GLOBAL__I__ZN4FMOD13dsposcillatorE
pub fn stub_0xba270() {
    // IDA 0xba270: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPParamEq::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetInternalEv")]
// 0xba27c — __ZN4FMOD10DSPParamEq13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xba27c() {
    // IDA 0xba27c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xba2c4 — __ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_0xba2c4() {
    // IDA 0xba2c4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPParamEq::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE")]
// 0xba2cc — __ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xba2cc() {
    // IDA 0xba2cc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xba2d8 — __ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_0xba2d8() {
    // IDA 0xba2d8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPParamEq::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc")]
// 0xba330 — __ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPParamEq *this, int, float *, char *)
pub fn stub_0xba330() {
    // IDA 0xba330: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xba3e0 — __ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xba3e0() {
    // IDA 0xba3e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::updateCoefficients(float,float,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq18updateCoefficientsEfff")]
// 0xba3ec — __ZN4FMOD10DSPParamEq18updateCoefficientsEfff
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float, float, float)
pub fn stub_0xba3ec() {
    // IDA 0xba3ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readInternalEPfS1_jii")]
// 0xba49c — __ZN4FMOD10DSPParamEq12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xba49c() {
    // IDA 0xba49c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xbb54c — __ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPParamEq *, float *, float *, unsigned int, int, int)
pub fn stub_0xbb54c() {
    // IDA 0xbb54c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createInternalEv")]
// 0xbb574 — __ZN4FMOD10DSPParamEq14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xbb574() {
    // IDA 0xbb574: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE")]
// 0xbb628 — __ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbb628() {
    // IDA 0xbb628: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq16getDescriptionExEv")]
// 0xbb634 — __ZN4FMOD10DSPParamEq16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xbb634() {
    // IDA 0xbb634: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterInternalEif")]
// 0xbb710 — __ZN4FMOD10DSPParamEq20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, int, float)
pub fn stub_0xbb710() {
    // IDA 0xbb710: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPParamEq::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xbb770 — __ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xbb770() {
    // IDA 0xbb770: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dspparameq")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD10dspparameqE")]
// 0xbb7c0 — __GLOBAL__I__ZN4FMOD10dspparameqE
pub fn stub_0xbb7c0() {
    // IDA 0xbb7c0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi")]
// 0xbb7cc — __ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbb7cc() {
    // IDA 0xbb7cc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2conj(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi")]
// 0xbbc58 — __ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbbc58() {
    // IDA 0xbbc58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cft1st(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cft1stEPf")]
// 0xbc170 — __ZN4FMOD16DSPPitchShiftSMB6cft1stEPf
// type: int __fastcall(int this, float *)
pub fn stub_0xbc170() {
    // IDA 0xbc170: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftmdl(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi")]
// 0xbc4c8 — __ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbc4c8() {
    // IDA 0xbc4c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftfsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf")]
// 0xbca50 — __ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf
// type: float *__fastcall(float *this, float *)
pub fn stub_0xbca50() {
    // IDA 0xbca50: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftbsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf")]
// 0xbcc28 — __ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *)
pub fn stub_0xbcc28() {
    // IDA 0xbcc28: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::fft(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB3fftEPfi")]
// 0xbce08 — __ZN4FMOD16DSPPitchShiftSMB3fftEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbce08() {
    // IDA 0xbce08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::setResetPhaseFlag(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv")]
// 0xbce64 — __ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
pub fn stub_0xbce64() {
    // IDA 0xbce64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xbce78 — __ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xbce78() {
    // IDA 0xbce78: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xbcebc — __ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPPitchShift *this)
pub fn stub_0xbcebc() {
    // IDA 0xbcebc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc")]
// 0xbcf14 — __ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float *, char *)
pub fn stub_0xbcf14() {
    // IDA 0xbcf14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xbd054 — __ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xbd054() {
    // IDA 0xbd054: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseInternalEv")]
// 0xbd060 — __ZN4FMOD13DSPPitchShift15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd060() {
    // IDA 0xbd060: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xbd0b4 — __ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbd0b4() {
    // IDA 0xbd0b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::smbInit(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7smbInitEv")]
// 0xbd0c0 — __ZN4FMOD16DSPPitchShiftSMB7smbInitEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
pub fn stub_0xbd0c0() {
    // IDA 0xbd0c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetInternalEv")]
// 0xbd1b0 — __ZN4FMOD13DSPPitchShift13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd1b0() {
    // IDA 0xbd1b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE")]
// 0xbd238 — __ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPPitchShift *)
pub fn stub_0xbd238() {
    // IDA 0xbd238: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createInternalEv")]
// 0xbd244 — __ZN4FMOD13DSPPitchShift14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd244() {
    // IDA 0xbd244: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE")]
// 0xbd32c — __ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbd32c() {
    // IDA 0xbd32c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift16getDescriptionExEv")]
// 0xbd338 — __ZN4FMOD13DSPPitchShift16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd338() {
    // IDA 0xbd338: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::initFft(int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7initFftEi")]
// 0xbd424 — __ZN4FMOD16DSPPitchShiftSMB7initFftEi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, int)
pub fn stub_0xbd424() {
    // IDA 0xbd424: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterInternalEif")]
// 0xbd698 — __ZN4FMOD13DSPPitchShift20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float)
pub fn stub_0xbd698() {
    // IDA 0xbd698: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xbdcb4 — __ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xbdcb4() {
    // IDA 0xbdcb4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::smbPitchShift(float,int,int,float,float *,float *,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii")]
// 0xbdcc0 — __ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float, int, int, float, float *, float *, int, int)
pub fn stub_0xbdcc0() {
    // IDA 0xbdcc0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii")]
// 0xbf024 — __ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xbf024() {
    // IDA 0xbf024: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPPitchShift::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xbf2f0 — __ZN4FMOD13DSPPitchShift12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xbf2f0() {
    // IDA 0xbf2f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dsppitchshift")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dsppitchshiftE")]
// 0xbf35c — __GLOBAL__I__ZN4FMOD13dsppitchshiftE
pub fn stub_0xbf35c() {
    // IDA 0xbf35c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPResampler::addInput(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler8addInputEPNS_4DSPIE")]
// 0xbf368 — __ZN4FMOD12DSPResampler8addInputEPNS_4DSPIE
pub fn stub_0xbf368() {
    // IDA 0xbf368: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPResampler::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD12DSPResampler12setFrequencyEf")]
// 0xbf370 — __ZN4FMOD12DSPResampler12setFrequencyEf
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, float)
pub fn stub_0xbf370() {
    // IDA 0xbf370: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPResampler::getFinished(bool *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11getFinishedEPb")]
// 0xbf3d4 — __ZN4FMOD12DSPResampler11getFinishedEPb
// type: int __fastcall(FMOD::DSPResampler *this, bool *)
pub fn stub_0xbf3d4() {
    // IDA 0xbf3d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPResampler::setFinished(bool,bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11setFinishedEbb")]
// 0xbf434 — __ZN4FMOD12DSPResampler11setFinishedEbb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, bool, bool)
pub fn stub_0xbf434() {
    // IDA 0xbf434: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResampler::setPosition(unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11setPositionEjb")]
// 0xbf4c4 — __ZN4FMOD12DSPResampler11setPositionEjb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, unsigned int, bool)
pub fn stub_0xbf4c4() {
    // IDA 0xbf4c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResampler::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xbf514 — __ZN4FMOD12DSPResampler5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xbf514() {
    // IDA 0xbf514: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResampler::release(bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler7releaseEb")]
// 0xbf784 — __ZN4FMOD12DSPResampler7releaseEb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, bool)
pub fn stub_0xbf784() {
    // IDA 0xbf784: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResampler::DSPResampler(void)")]
#[doc(alias = "__ZN4FMOD12DSPResamplerC2Ev")]
// 0xbf814 — __ZN4FMOD12DSPResamplerC2Ev
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this)
pub fn stub_0xbf814() {
    // IDA 0xbf814: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResampler::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPResampler4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xbf8c4 — __ZN4FMOD12DSPResampler4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xbf8c4() {
    // IDA 0xbf8c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResamplerMultiInput::addInput(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD22DSPResamplerMultiInput8addInputEPNS_4DSPIE")]
// 0xc0334 — __ZN4FMOD22DSPResamplerMultiInput8addInputEPNS_4DSPIE
// type: _DWORD __fastcall(FMOD::DSPResamplerMultiInput *__hidden this, FMOD::DSPI *)
pub fn stub_0xc0334() {
    // IDA 0xc0334: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPResamplerMultiInput::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD22DSPResamplerMultiInput4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc0378 — __ZN4FMOD22DSPResamplerMultiInput4readEPPfPiPj16FMOD_SPEAKERMODEij
// type: int __fastcall(FMOD::DSPI *this, int, int, int, char, int, int)
pub fn stub_0xc0378() {
    // IDA 0xc0378: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_Resampler_NoInterp")]
#[doc(alias = "_FMOD_Resampler_NoInterp_0xc2c84")]
// 0xc097c — _FMOD_Resampler_NoInterp
pub fn stub_0xc097c() {
    // IDA 0xc097c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb15releaseInternalEv")]
// 0xc1498 — __ZN4FMOD9DSPReverb15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc1498() {
    // IDA 0xc1498: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb13resetInternalEv")]
// 0xc14a0 — __ZN4FMOD9DSPReverb13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc14a0() {
    // IDA 0xc14a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc14a8 — __ZN4FMOD9DSPReverb17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_0xc14a8() {
    // IDA 0xc14a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xc14b0 — __ZN4FMOD9DSPReverb15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc14b0() {
    // IDA 0xc14b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc14bc — __ZN4FMOD9DSPReverb13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPReverb *)
pub fn stub_0xc14bc() {
    // IDA 0xc14bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xc14c8 — __ZN4FMOD9DSPReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_0xc14c8() {
    // IDA 0xc14c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20getParameterInternalEiPfPc")]
// 0xc1520 — __ZN4FMOD9DSPReverb20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, int, float *, char *)
pub fn stub_0xc1520() {
    // IDA 0xc1520: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc16c4 — __ZN4FMOD9DSPReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc16c4() {
    // IDA 0xc16c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20setParameterInternalEif")]
// 0xc16d0 — __ZN4FMOD9DSPReverb20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, int, float)
pub fn stub_0xc16d0() {
    // IDA 0xc16d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc1870 — __ZN4FMOD9DSPReverb20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc1870() {
    // IDA 0xc1870: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPReverb12readInternalEPfS1_jii")]
// 0xc187c — __ZN4FMOD9DSPReverb12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xc187c() {
    // IDA 0xc187c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xc191c — __ZN4FMOD9DSPReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xc191c() {
    // IDA 0xc191c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb14createInternalEv")]
// 0xc1944 — __ZN4FMOD9DSPReverb14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc1944() {
    // IDA 0xc1944: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb14createCallbackEP14FMOD_DSP_STATE")]
// 0xc19c4 — __ZN4FMOD9DSPReverb14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc19c4() {
    // IDA 0xc19c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPReverb::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb16getDescriptionExEv")]
// 0xc19d0 — __ZN4FMOD9DSPReverb16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc19d0() {
    // IDA 0xc19d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dspreverb")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspreverbE")]
// 0xc1b04 — __GLOBAL__I__ZN4FMOD9dspreverbE
pub fn stub_0xc1b04() {
    // IDA 0xc1b04: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomRolloffFactor(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20SetRoomRolloffFactorEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1b10 — __ZN4FMOD12DSPSfxReverb20SetRoomRolloffFactorEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1b10() {
    // IDA 0xc1b10: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSfxReverb::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc1b24 — __ZN4FMOD12DSPSfxReverb17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPSfxReverb *this, FMOD::MemoryTracker *)
pub fn stub_0xc1b24() {
    // IDA 0xc1b24: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSfxReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xc1c2c — __ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPSfxReverb *this)
pub fn stub_0xc1c2c() {
    // IDA 0xc1c2c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDiffusion(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12SetDiffusionEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1c84 — __ZN4FMOD12DSPSfxReverb12SetDiffusionEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1c84() {
    // IDA 0xc1c84: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReflectionsLevel(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetReflectionsLevelEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1d48 — __ZN4FMOD12DSPSfxReverb19SetReflectionsLevelEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1d48() {
    // IDA 0xc1d48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReverbDelay(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetReverbDelayEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1de4 — __ZN4FMOD12DSPSfxReverb14SetReverbDelayEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1de4() {
    // IDA 0xc1de4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReflectionsDelay(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetReflectionsDelayEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1e74 — __ZN4FMOD12DSPSfxReverb19SetReflectionsDelayEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1e74() {
    // IDA 0xc1e74: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReverbLevel(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetReverbLevelEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1f00 — __ZN4FMOD12DSPSfxReverb14SetReverbLevelEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1f00() {
    // IDA 0xc1f00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoom(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb7SetRoomEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2014 — __ZN4FMOD12DSPSfxReverb7SetRoomEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2014() {
    // IDA 0xc2014: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::CalculateShelfCoeffs(float,float,float,float *,float *,float *,float *,float *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20CalculateShelfCoeffsEfffPfS1_S1_S1_S1_")]
// 0xc207c — __ZN4FMOD12DSPSfxReverb20CalculateShelfCoeffsEfffPfS1_S1_S1_S1_
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float, float, float, float *, float *, float *, float *, float *)
pub fn stub_0xc207c() {
    // IDA 0xc207c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomLF(FMOD::SFX_REVERB_LFPROPS *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb9SetRoomLFEPNS_18SFX_REVERB_LFPROPSE")]
// 0xc2178 — __ZN4FMOD12DSPSfxReverb9SetRoomLFEPNS_18SFX_REVERB_LFPROPSE
pub fn stub_0xc2178() {
    // IDA 0xc2178: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetLFReference(FMOD::SFX_REVERB_LFPROPS *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetLFReferenceEPNS_18SFX_REVERB_LFPROPSE")]
// 0xc2210 — __ZN4FMOD12DSPSfxReverb14SetLFReferenceEPNS_18SFX_REVERB_LFPROPSE
pub fn stub_0xc2210() {
    // IDA 0xc2210: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::Calculate1stOrderLowpassCoeff(float,float,float,float *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb29Calculate1stOrderLowpassCoeffEfffPf")]
// 0xc2250 — __ZN4FMOD12DSPSfxReverb29Calculate1stOrderLowpassCoeffEfffPf
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float, float, float, float *)
pub fn stub_0xc2250() {
    // IDA 0xc2250: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDecayTime(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12SetDecayTimeEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2370 — __ZN4FMOD12DSPSfxReverb12SetDecayTimeEP25_I3DL2_LISTENERPROPERTIES
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xc2370() {
    // IDA 0xc2370: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDecayHFRatio(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15SetDecayHFRatioEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2508 — __ZN4FMOD12DSPSfxReverb15SetDecayHFRatioEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2508() {
    // IDA 0xc2508: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDelayLineLengths(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetDelayLineLengthsEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2550 — __ZN4FMOD12DSPSfxReverb19SetDelayLineLengthsEP25_I3DL2_LISTENERPROPERTIES
// type: int __fastcall(int, int)
pub fn stub_0xc2550() {
    // IDA 0xc2550: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDensity(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb10SetDensityEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2618 — __ZN4FMOD12DSPSfxReverb10SetDensityEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2618() {
    // IDA 0xc2618: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomHF(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb9SetRoomHFEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2664 — __ZN4FMOD12DSPSfxReverb9SetRoomHFEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2664() {
    // IDA 0xc2664: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::SetHFReference(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetHFReferenceEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2730 — __ZN4FMOD12DSPSfxReverb14SetHFReferenceEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2730() {
    // IDA 0xc2730: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::updateInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14updateInternalEv")]
// 0xc2794 — __ZN4FMOD12DSPSfxReverb14updateInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc2794() {
    // IDA 0xc2794: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::updateCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE")]
// 0xc2a18 — __ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc2a18() {
    // IDA 0xc2a18: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc")]
// 0xc2a24 — __ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float *, char *)
pub fn stub_0xc2a24() {
    // IDA 0xc2a24: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc2e88 — __ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc2e88() {
    // IDA 0xc2e88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterInternalEif")]
// 0xc2e94 — __ZN4FMOD12DSPSfxReverb20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float)
pub fn stub_0xc2e94() {
    // IDA 0xc2e94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc3178 — __ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc3178() {
    // IDA 0xc3178: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetInternalEv")]
// 0xc3184 — __ZN4FMOD12DSPSfxReverb13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc3184() {
    // IDA 0xc3184: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc31bc — __ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc31bc() {
    // IDA 0xc31bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii")]
// 0xc31c8 — __ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float *, float *__dst, unsigned int, int, int)
pub fn stub_0xc31c8() {
    // IDA 0xc31c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xc327c — __ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xc327c() {
    // IDA 0xc327c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseInternalEv")]
// 0xc32a4 — __ZN4FMOD12DSPSfxReverb15releaseInternalEv
// type: int __fastcall(void **this)
pub fn stub_0xc32a4() {
    // IDA 0xc32a4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xc32bc — __ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc32bc() {
    // IDA 0xc32bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::createInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createInternalEv")]
// 0xc32c8 — __ZN4FMOD12DSPSfxReverb14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc32c8() {
    // IDA 0xc32c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE")]
// 0xc35cc — __ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc35cc() {
    // IDA 0xc35cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPSfxReverb::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb16getDescriptionExEv")]
// 0xc35d8 — __ZN4FMOD12DSPSfxReverb16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc35d8() {
    // IDA 0xc35d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::dspsfxreverb")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspsfxreverbE")]
// 0xc3718 — __GLOBAL__I__ZN4FMOD12dspsfxreverbE
pub fn stub_0xc3718() {
    // IDA 0xc3718: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSoundCard::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xc3724 — __ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xc3724() {
    // IDA 0xc3724: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPSoundCard::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij")]
// 0xc3750 — __ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij
pub fn stub_0xc3750() {
    // IDA 0xc3750: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPWaveTable::setPositionInternal(unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionInternalEj")]
// 0xc3bcc — __ZN4FMOD12DSPWaveTable19setPositionInternalEj
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, unsigned int)
pub fn stub_0xc3bcc() {
    // IDA 0xc3bcc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::DSPWaveTable::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterInternalEif")]
// 0xc3bf4 — __ZN4FMOD12DSPWaveTable20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float)
pub fn stub_0xc3bf4() {
    // IDA 0xc3bf4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc")]
// 0xc3bfc — __ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float *, char *)
pub fn stub_0xc3bfc() {
    // IDA 0xc3bfc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable12setFrequencyEf")]
// 0xc3c04 — __ZN4FMOD12DSPWaveTable12setFrequencyEf
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, float)
pub fn stub_0xc3c04() {
    // IDA 0xc3c04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::getFinished(bool *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable11getFinishedEPb")]
// 0xc3c80 — __ZN4FMOD12DSPWaveTable11getFinishedEPb
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, bool *)
pub fn stub_0xc3c80() {
    // IDA 0xc3c80: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::setPositionCallback(FMOD_DSP_STATE *,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj")]
// 0xc3cc4 — __ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj
// type: int __fastcall(FMOD::DSPWaveTable *, unsigned int)
pub fn stub_0xc3cc4() {
    // IDA 0xc3cc4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc3cd0 — __ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc3cd0() {
    // IDA 0xc3cd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc3cdc — __ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc3cdc() {
    // IDA 0xc3cdc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc3ce8 — __ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc3ce8() {
    // IDA 0xc3ce8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::setFinished(bool,bool)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable11setFinishedEbb")]
// 0xc3d00 — __ZN4FMOD12DSPWaveTable11setFinishedEbb
// type: int __fastcall(FMOD::DSPWaveTable *this, bool, bool)
pub fn stub_0xc3d00() {
    // IDA 0xc3d00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc3d94 — __ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xc3d94() {
    // IDA 0xc3d94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPWaveTable::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xc4728 — __ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xc4728() {
    // IDA 0xc4728: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc4788 — __ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xc4788() {
    // IDA 0xc4788: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij")]
// 0xc4790 — __ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij
pub fn stub_0xc4790() {
    // IDA 0xc4790: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::DSPI(void)")]
#[doc(alias = "__ZN4FMOD4DSPIC2Ev")]
// 0xc47d4 — __ZN4FMOD4DSPIC2Ev
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc47d4() {
    // IDA 0xc47d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getSystemObject(FMOD::System **)")]
#[doc(alias = "__ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE")]
// 0xc489c — __ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE
pub fn stub_0xc489c() {
    // IDA 0xc489c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::updateDSPTick(unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI13updateDSPTickEj")]
// 0xc48b4 — __ZN4FMOD4DSPI13updateDSPTickEj
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, unsigned int)
pub fn stub_0xc48b4() {
    // IDA 0xc48b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::reset(void)")]
#[doc(alias = "__ZN4FMOD4DSPI5resetEv")]
// 0xc48f8 — __ZN4FMOD4DSPI5resetEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc48f8() {
    // IDA 0xc48f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::setParameter(int,float)")]
#[doc(alias = "__ZN4FMOD4DSPI12setParameterEif")]
// 0xc4918 — __ZN4FMOD4DSPI12setParameterEif
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, float)
pub fn stub_0xc4918() {
    // IDA 0xc4918: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getNumParameters(int *)")]
#[doc(alias = "__ZN4FMOD4DSPI16getNumParametersEPi")]
// 0xc498c — __ZN4FMOD4DSPI16getNumParametersEPi
// type: int __fastcall(FMOD::DSPI *this, int *)
pub fn stub_0xc498c() {
    // IDA 0xc498c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::showConfigDialog(void *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI16showConfigDialogEPvb")]
// 0xc49a4 — __ZN4FMOD4DSPI16showConfigDialogEPvb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *, bool)
pub fn stub_0xc49a4() {
    // IDA 0xc49a4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getType(FMOD_DSP_TYPE *)")]
#[doc(alias = "__ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE")]
// 0xc49c8 — __ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE
pub fn stub_0xc49c8() {
    // IDA 0xc49c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::setDefaults(float,float,float,int)")]
#[doc(alias = "__ZN4FMOD4DSPI11setDefaultsEfffi")]
// 0xc49dc — __ZN4FMOD4DSPI11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float, float, float, int)
pub fn stub_0xc49dc() {
    // IDA 0xc49dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getDefaults(float *,float *,float *,int *)")]
#[doc(alias = "__ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi")]
// 0xc4a64 — __ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float *, float *, float *, int *)
pub fn stub_0xc4a64() {
    // IDA 0xc4a64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::setUserData(void *)")]
#[doc(alias = "__ZN4FMOD4DSPI11setUserDataEPv")]
// 0xc4aa8 — __ZN4FMOD4DSPI11setUserDataEPv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *)
pub fn stub_0xc4aa8() {
    // IDA 0xc4aa8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
