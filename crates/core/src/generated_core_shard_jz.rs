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
pub fn stub_0xb9934() -> ! {
    todo!("0xb9934 __ZN4FMOD12DSPNormalize20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPNormalize::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readInternalEPfS1_jii")]
// 0xb9940 — __ZN4FMOD12DSPNormalize12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, float *, float *__dst, unsigned int, int, int)
pub fn stub_0xb9940() -> ! {
    todo!("0xb9940 __ZN4FMOD12DSPNormalize12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPNormalize::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb9a94 — __ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xb9a94() -> ! {
    todo!("0xb9a94 __ZN4FMOD12DSPNormalize12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPNormalize::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize16getDescriptionExEv")]
// 0xb9abc — __ZN4FMOD12DSPNormalize16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
pub fn stub_0xb9abc() -> ! {
    todo!("0xb9abc __ZN4FMOD12DSPNormalize16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspnormalize")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspnormalizeE")]
// 0xb9bec — __GLOBAL__I__ZN4FMOD12dspnormalizeE
pub fn stub_0xb9bec() -> ! {
    todo!("0xb9bec __GLOBAL__I__ZN4FMOD12dspnormalizeE")
}

#[doc(alias = "FMOD::DSPOscillator::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createInternalEv")]
// 0xb9bf8 — __ZN4FMOD13DSPOscillator14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xb9bf8() -> ! {
    todo!("0xb9bf8 __ZN4FMOD13DSPOscillator14createInternalEv")
}

#[doc(alias = "FMOD::DSPOscillator::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseInternalEv")]
// 0xb9c78 — __ZN4FMOD13DSPOscillator15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xb9c78() -> ! {
    todo!("0xb9c78 __ZN4FMOD13DSPOscillator15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPOscillator::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterInternalEif")]
// 0xb9c80 — __ZN4FMOD13DSPOscillator20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float)
pub fn stub_0xb9c80() -> ! {
    todo!("0xb9c80 __ZN4FMOD13DSPOscillator20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPOscillator::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE")]
// 0xb9ccc — __ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xb9ccc() -> ! {
    todo!("0xb9ccc __ZN4FMOD13DSPOscillator14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPOscillator::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb9cd8 — __ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPOscillator *)
pub fn stub_0xb9cd8() -> ! {
    todo!("0xb9cd8 __ZN4FMOD13DSPOscillator15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPOscillator::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb9ce4 — __ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xb9ce4() -> ! {
    todo!("0xb9ce4 __ZN4FMOD13DSPOscillator20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPOscillator::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc")]
// 0xb9cf0 — __ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, int, float *, char *)
pub fn stub_0xb9cf0() -> ! {
    todo!("0xb9cf0 __ZN4FMOD13DSPOscillator20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPOscillator::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb9e04 — __ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xb9e04() -> ! {
    todo!("0xb9e04 __ZN4FMOD13DSPOscillator20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPOscillator::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readInternalEPfS1_jii")]
// 0xb9e10 — __ZN4FMOD13DSPOscillator12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xb9e10() -> ! {
    todo!("0xb9e10 __ZN4FMOD13DSPOscillator12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPOscillator::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xba0f4 — __ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xba0f4() -> ! {
    todo!("0xba0f4 __ZN4FMOD13DSPOscillator12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPOscillator::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPOscillator16getDescriptionExEv")]
// 0xba11c — __ZN4FMOD13DSPOscillator16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPOscillator *__hidden this)
pub fn stub_0xba11c() -> ! {
    todo!("0xba11c __ZN4FMOD13DSPOscillator16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPI::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xba1fc — __ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, FMOD::MemoryTracker *this)
pub fn stub_0xba1fc() -> ! {
    todo!("0xba1fc __ZN4FMOD4DSPI21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "global constructor keyed toFMOD::dsposcillator")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dsposcillatorE")]
// 0xba270 — __GLOBAL__I__ZN4FMOD13dsposcillatorE
pub fn stub_0xba270() -> ! {
    todo!("0xba270 __GLOBAL__I__ZN4FMOD13dsposcillatorE")
}

#[doc(alias = "FMOD::DSPParamEq::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetInternalEv")]
// 0xba27c — __ZN4FMOD10DSPParamEq13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xba27c() -> ! {
    todo!("0xba27c __ZN4FMOD10DSPParamEq13resetInternalEv")
}

#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xba2c4 — __ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_0xba2c4() -> ! {
    todo!("0xba2c4 __ZN4FMOD10DSPParamEq17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPParamEq::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE")]
// 0xba2cc — __ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xba2cc() -> ! {
    todo!("0xba2cc __ZN4FMOD10DSPParamEq13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPParamEq::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xba2d8 — __ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_0xba2d8() -> ! {
    todo!("0xba2d8 __ZN4FMOD10DSPParamEq21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPParamEq::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc")]
// 0xba330 — __ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPParamEq *this, int, float *, char *)
pub fn stub_0xba330() -> ! {
    todo!("0xba330 __ZN4FMOD10DSPParamEq20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPParamEq::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xba3e0 — __ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xba3e0() -> ! {
    todo!("0xba3e0 __ZN4FMOD10DSPParamEq20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPParamEq::updateCoefficients(float,float,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq18updateCoefficientsEfff")]
// 0xba3ec — __ZN4FMOD10DSPParamEq18updateCoefficientsEfff
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float, float, float)
pub fn stub_0xba3ec() -> ! {
    todo!("0xba3ec __ZN4FMOD10DSPParamEq18updateCoefficientsEfff")
}

#[doc(alias = "FMOD::DSPParamEq::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readInternalEPfS1_jii")]
// 0xba49c — __ZN4FMOD10DSPParamEq12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xba49c() -> ! {
    todo!("0xba49c __ZN4FMOD10DSPParamEq12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPParamEq::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xbb54c — __ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPParamEq *, float *, float *, unsigned int, int, int)
pub fn stub_0xbb54c() -> ! {
    todo!("0xbb54c __ZN4FMOD10DSPParamEq12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPParamEq::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createInternalEv")]
// 0xbb574 — __ZN4FMOD10DSPParamEq14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xbb574() -> ! {
    todo!("0xbb574 __ZN4FMOD10DSPParamEq14createInternalEv")
}

#[doc(alias = "FMOD::DSPParamEq::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE")]
// 0xbb628 — __ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbb628() -> ! {
    todo!("0xbb628 __ZN4FMOD10DSPParamEq14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPParamEq::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq16getDescriptionExEv")]
// 0xbb634 — __ZN4FMOD10DSPParamEq16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this)
pub fn stub_0xbb634() -> ! {
    todo!("0xbb634 __ZN4FMOD10DSPParamEq16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPParamEq::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterInternalEif")]
// 0xbb710 — __ZN4FMOD10DSPParamEq20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPParamEq *__hidden this, int, float)
pub fn stub_0xbb710() -> ! {
    todo!("0xbb710 __ZN4FMOD10DSPParamEq20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPParamEq::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xbb770 — __ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xbb770() -> ! {
    todo!("0xbb770 __ZN4FMOD10DSPParamEq20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "global constructor keyed toFMOD::dspparameq")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD10dspparameqE")]
// 0xbb7c0 — __GLOBAL__I__ZN4FMOD10dspparameqE
pub fn stub_0xbb7c0() -> ! {
    todo!("0xbb7c0 __GLOBAL__I__ZN4FMOD10dspparameqE")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi")]
// 0xbb7cc — __ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbb7cc() -> ! {
    todo!("0xbb7cc __ZN4FMOD16DSPPitchShiftSMB6bitrv2EPfi")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::bitrv2conj(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi")]
// 0xbbc58 — __ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbbc58() -> ! {
    todo!("0xbbc58 __ZN4FMOD16DSPPitchShiftSMB10bitrv2conjEPfi")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cft1st(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cft1stEPf")]
// 0xbc170 — __ZN4FMOD16DSPPitchShiftSMB6cft1stEPf
// type: int __fastcall(int this, float *)
pub fn stub_0xbc170() -> ! {
    todo!("0xbc170 __ZN4FMOD16DSPPitchShiftSMB6cft1stEPf")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftmdl(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi")]
// 0xbc4c8 — __ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbc4c8() -> ! {
    todo!("0xbc4c8 __ZN4FMOD16DSPPitchShiftSMB6cftmdlEPfi")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftfsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf")]
// 0xbca50 — __ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf
// type: float *__fastcall(float *this, float *)
pub fn stub_0xbca50() -> ! {
    todo!("0xbca50 __ZN4FMOD16DSPPitchShiftSMB7cftfsubEPf")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::cftbsub(float *)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf")]
// 0xbcc28 — __ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *)
pub fn stub_0xbcc28() -> ! {
    todo!("0xbcc28 __ZN4FMOD16DSPPitchShiftSMB7cftbsubEPf")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::fft(float *,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB3fftEPfi")]
// 0xbce08 — __ZN4FMOD16DSPPitchShiftSMB3fftEPfi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float *, int)
pub fn stub_0xbce08() -> ! {
    todo!("0xbce08 __ZN4FMOD16DSPPitchShiftSMB3fftEPfi")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::setResetPhaseFlag(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv")]
// 0xbce64 — __ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
pub fn stub_0xbce64() -> ! {
    todo!("0xbce64 __ZN4FMOD16DSPPitchShiftSMB17setResetPhaseFlagEv")
}

#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xbce78 — __ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xbce78() -> ! {
    todo!("0xbce78 __ZN4FMOD13DSPPitchShift17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPPitchShift::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xbcebc — __ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPPitchShift *this)
pub fn stub_0xbcebc() -> ! {
    todo!("0xbcebc __ZN4FMOD13DSPPitchShift21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPPitchShift::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc")]
// 0xbcf14 — __ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float *, char *)
pub fn stub_0xbcf14() -> ! {
    todo!("0xbcf14 __ZN4FMOD13DSPPitchShift20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPPitchShift::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xbd054 — __ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xbd054() -> ! {
    todo!("0xbd054 __ZN4FMOD13DSPPitchShift20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPPitchShift::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseInternalEv")]
// 0xbd060 — __ZN4FMOD13DSPPitchShift15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd060() -> ! {
    todo!("0xbd060 __ZN4FMOD13DSPPitchShift15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPPitchShift::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xbd0b4 — __ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbd0b4() -> ! {
    todo!("0xbd0b4 __ZN4FMOD13DSPPitchShift15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::smbInit(void)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7smbInitEv")]
// 0xbd0c0 — __ZN4FMOD16DSPPitchShiftSMB7smbInitEv
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this)
pub fn stub_0xbd0c0() -> ! {
    todo!("0xbd0c0 __ZN4FMOD16DSPPitchShiftSMB7smbInitEv")
}

#[doc(alias = "FMOD::DSPPitchShift::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetInternalEv")]
// 0xbd1b0 — __ZN4FMOD13DSPPitchShift13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd1b0() -> ! {
    todo!("0xbd1b0 __ZN4FMOD13DSPPitchShift13resetInternalEv")
}

#[doc(alias = "FMOD::DSPPitchShift::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE")]
// 0xbd238 — __ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPPitchShift *)
pub fn stub_0xbd238() -> ! {
    todo!("0xbd238 __ZN4FMOD13DSPPitchShift13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPPitchShift::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createInternalEv")]
// 0xbd244 — __ZN4FMOD13DSPPitchShift14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd244() -> ! {
    todo!("0xbd244 __ZN4FMOD13DSPPitchShift14createInternalEv")
}

#[doc(alias = "FMOD::DSPPitchShift::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE")]
// 0xbd32c — __ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xbd32c() -> ! {
    todo!("0xbd32c __ZN4FMOD13DSPPitchShift14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPPitchShift::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift16getDescriptionExEv")]
// 0xbd338 — __ZN4FMOD13DSPPitchShift16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this)
pub fn stub_0xbd338() -> ! {
    todo!("0xbd338 __ZN4FMOD13DSPPitchShift16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::initFft(int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB7initFftEi")]
// 0xbd424 — __ZN4FMOD16DSPPitchShiftSMB7initFftEi
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, int)
pub fn stub_0xbd424() -> ! {
    todo!("0xbd424 __ZN4FMOD16DSPPitchShiftSMB7initFftEi")
}

#[doc(alias = "FMOD::DSPPitchShift::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterInternalEif")]
// 0xbd698 — __ZN4FMOD13DSPPitchShift20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, int, float)
pub fn stub_0xbd698() -> ! {
    todo!("0xbd698 __ZN4FMOD13DSPPitchShift20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPPitchShift::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xbdcb4 — __ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xbdcb4() -> ! {
    todo!("0xbdcb4 __ZN4FMOD13DSPPitchShift20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPPitchShiftSMB::smbPitchShift(float,int,int,float,float *,float *,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii")]
// 0xbdcc0 — __ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii
// type: _DWORD __fastcall(FMOD::DSPPitchShiftSMB *__hidden this, float, int, int, float, float *, float *, int, int)
pub fn stub_0xbdcc0() -> ! {
    todo!("0xbdcc0 __ZN4FMOD16DSPPitchShiftSMB13smbPitchShiftEfiifPfS1_ii")
}

#[doc(alias = "FMOD::DSPPitchShift::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii")]
// 0xbf024 — __ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPPitchShift *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xbf024() -> ! {
    todo!("0xbf024 __ZN4FMOD13DSPPitchShift12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPPitchShift::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPPitchShift12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xbf2f0 — __ZN4FMOD13DSPPitchShift12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xbf2f0() -> ! {
    todo!("0xbf2f0 __ZN4FMOD13DSPPitchShift12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "global constructor keyed toFMOD::dsppitchshift")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dsppitchshiftE")]
// 0xbf35c — __GLOBAL__I__ZN4FMOD13dsppitchshiftE
pub fn stub_0xbf35c() -> ! {
    todo!("0xbf35c __GLOBAL__I__ZN4FMOD13dsppitchshiftE")
}

#[doc(alias = "FMOD::DSPResampler::addInput(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler8addInputEPNS_4DSPIE")]
// 0xbf368 — __ZN4FMOD12DSPResampler8addInputEPNS_4DSPIE
pub fn stub_0xbf368() -> ! {
    todo!("0xbf368 __ZN4FMOD12DSPResampler8addInputEPNS_4DSPIE")
}

#[doc(alias = "FMOD::DSPResampler::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD12DSPResampler12setFrequencyEf")]
// 0xbf370 — __ZN4FMOD12DSPResampler12setFrequencyEf
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, float)
pub fn stub_0xbf370() -> ! {
    todo!("0xbf370 __ZN4FMOD12DSPResampler12setFrequencyEf")
}

#[doc(alias = "FMOD::DSPResampler::getFinished(bool *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11getFinishedEPb")]
// 0xbf3d4 — __ZN4FMOD12DSPResampler11getFinishedEPb
// type: int __fastcall(FMOD::DSPResampler *this, bool *)
pub fn stub_0xbf3d4() -> ! {
    todo!("0xbf3d4 __ZN4FMOD12DSPResampler11getFinishedEPb")
}

#[doc(alias = "FMOD::DSPResampler::setFinished(bool,bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11setFinishedEbb")]
// 0xbf434 — __ZN4FMOD12DSPResampler11setFinishedEbb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, bool, bool)
pub fn stub_0xbf434() -> ! {
    todo!("0xbf434 __ZN4FMOD12DSPResampler11setFinishedEbb")
}

#[doc(alias = "FMOD::DSPResampler::setPosition(unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler11setPositionEjb")]
// 0xbf4c4 — __ZN4FMOD12DSPResampler11setPositionEjb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, unsigned int, bool)
pub fn stub_0xbf4c4() -> ! {
    todo!("0xbf4c4 __ZN4FMOD12DSPResampler11setPositionEjb")
}

#[doc(alias = "FMOD::DSPResampler::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPResampler5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xbf514 — __ZN4FMOD12DSPResampler5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xbf514() -> ! {
    todo!("0xbf514 __ZN4FMOD12DSPResampler5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")
}

#[doc(alias = "FMOD::DSPResampler::release(bool)")]
#[doc(alias = "__ZN4FMOD12DSPResampler7releaseEb")]
// 0xbf784 — __ZN4FMOD12DSPResampler7releaseEb
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this, bool)
pub fn stub_0xbf784() -> ! {
    todo!("0xbf784 __ZN4FMOD12DSPResampler7releaseEb")
}

#[doc(alias = "FMOD::DSPResampler::DSPResampler(void)")]
#[doc(alias = "__ZN4FMOD12DSPResamplerC2Ev")]
// 0xbf814 — __ZN4FMOD12DSPResamplerC2Ev
// type: _DWORD __fastcall(FMOD::DSPResampler *__hidden this)
pub fn stub_0xbf814() -> ! {
    todo!("0xbf814 __ZN4FMOD12DSPResamplerC2Ev")
}

#[doc(alias = "FMOD::DSPResampler::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPResampler4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xbf8c4 — __ZN4FMOD12DSPResampler4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xbf8c4() -> ! {
    todo!("0xbf8c4 __ZN4FMOD12DSPResampler4readEPPfPiPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPResamplerMultiInput::addInput(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD22DSPResamplerMultiInput8addInputEPNS_4DSPIE")]
// 0xc0334 — __ZN4FMOD22DSPResamplerMultiInput8addInputEPNS_4DSPIE
// type: _DWORD __fastcall(FMOD::DSPResamplerMultiInput *__hidden this, FMOD::DSPI *)
pub fn stub_0xc0334() -> ! {
    todo!("0xc0334 __ZN4FMOD22DSPResamplerMultiInput8addInputEPNS_4DSPIE")
}

#[doc(alias = "FMOD::DSPResamplerMultiInput::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD22DSPResamplerMultiInput4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc0378 — __ZN4FMOD22DSPResamplerMultiInput4readEPPfPiPj16FMOD_SPEAKERMODEij
// type: int __fastcall(FMOD::DSPI *this, int, int, int, char, int, int)
pub fn stub_0xc0378() -> ! {
    todo!("0xc0378 __ZN4FMOD22DSPResamplerMultiInput4readEPPfPiPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "_FMOD_Resampler_NoInterp")]
#[doc(alias = "_FMOD_Resampler_NoInterp_0xc2c84")]
// 0xc097c — _FMOD_Resampler_NoInterp
pub fn stub_0xc097c() -> ! {
    todo!("0xc097c _FMOD_Resampler_NoInterp")
}

#[doc(alias = "FMOD::DSPReverb::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb15releaseInternalEv")]
// 0xc1498 — __ZN4FMOD9DSPReverb15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc1498() -> ! {
    todo!("0xc1498 __ZN4FMOD9DSPReverb15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPReverb::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb13resetInternalEv")]
// 0xc14a0 — __ZN4FMOD9DSPReverb13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc14a0() -> ! {
    todo!("0xc14a0 __ZN4FMOD9DSPReverb13resetInternalEv")
}

#[doc(alias = "FMOD::DSPReverb::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc14a8 — __ZN4FMOD9DSPReverb17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_0xc14a8() -> ! {
    todo!("0xc14a8 __ZN4FMOD9DSPReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPReverb::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xc14b0 — __ZN4FMOD9DSPReverb15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc14b0() -> ! {
    todo!("0xc14b0 __ZN4FMOD9DSPReverb15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPReverb::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc14bc — __ZN4FMOD9DSPReverb13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPReverb *)
pub fn stub_0xc14bc() -> ! {
    todo!("0xc14bc __ZN4FMOD9DSPReverb13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xc14c8 — __ZN4FMOD9DSPReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_0xc14c8() -> ! {
    todo!("0xc14c8 __ZN4FMOD9DSPReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPReverb::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20getParameterInternalEiPfPc")]
// 0xc1520 — __ZN4FMOD9DSPReverb20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, int, float *, char *)
pub fn stub_0xc1520() -> ! {
    todo!("0xc1520 __ZN4FMOD9DSPReverb20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPReverb::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc16c4 — __ZN4FMOD9DSPReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc16c4() -> ! {
    todo!("0xc16c4 __ZN4FMOD9DSPReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPReverb::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20setParameterInternalEif")]
// 0xc16d0 — __ZN4FMOD9DSPReverb20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, int, float)
pub fn stub_0xc16d0() -> ! {
    todo!("0xc16d0 __ZN4FMOD9DSPReverb20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPReverb::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPReverb20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc1870 — __ZN4FMOD9DSPReverb20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc1870() -> ! {
    todo!("0xc1870 __ZN4FMOD9DSPReverb20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPReverb::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPReverb12readInternalEPfS1_jii")]
// 0xc187c — __ZN4FMOD9DSPReverb12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_0xc187c() -> ! {
    todo!("0xc187c __ZN4FMOD9DSPReverb12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPReverb::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xc191c — __ZN4FMOD9DSPReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xc191c() -> ! {
    todo!("0xc191c __ZN4FMOD9DSPReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPReverb::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb14createInternalEv")]
// 0xc1944 — __ZN4FMOD9DSPReverb14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc1944() -> ! {
    todo!("0xc1944 __ZN4FMOD9DSPReverb14createInternalEv")
}

#[doc(alias = "FMOD::DSPReverb::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPReverb14createCallbackEP14FMOD_DSP_STATE")]
// 0xc19c4 — __ZN4FMOD9DSPReverb14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc19c4() -> ! {
    todo!("0xc19c4 __ZN4FMOD9DSPReverb14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPReverb::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPReverb16getDescriptionExEv")]
// 0xc19d0 — __ZN4FMOD9DSPReverb16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPReverb *__hidden this)
pub fn stub_0xc19d0() -> ! {
    todo!("0xc19d0 __ZN4FMOD9DSPReverb16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspreverb")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspreverbE")]
// 0xc1b04 — __GLOBAL__I__ZN4FMOD9dspreverbE
pub fn stub_0xc1b04() -> ! {
    todo!("0xc1b04 __GLOBAL__I__ZN4FMOD9dspreverbE")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomRolloffFactor(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20SetRoomRolloffFactorEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1b10 — __ZN4FMOD12DSPSfxReverb20SetRoomRolloffFactorEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1b10() -> ! {
    todo!("0xc1b10 __ZN4FMOD12DSPSfxReverb20SetRoomRolloffFactorEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc1b24 — __ZN4FMOD12DSPSfxReverb17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPSfxReverb *this, FMOD::MemoryTracker *)
pub fn stub_0xc1b24() -> ! {
    todo!("0xc1b24 __ZN4FMOD12DSPSfxReverb17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPSfxReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xc1c2c — __ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPSfxReverb *this)
pub fn stub_0xc1c2c() -> ! {
    todo!("0xc1c2c __ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDiffusion(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12SetDiffusionEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1c84 — __ZN4FMOD12DSPSfxReverb12SetDiffusionEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1c84() -> ! {
    todo!("0xc1c84 __ZN4FMOD12DSPSfxReverb12SetDiffusionEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReflectionsLevel(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetReflectionsLevelEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1d48 — __ZN4FMOD12DSPSfxReverb19SetReflectionsLevelEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1d48() -> ! {
    todo!("0xc1d48 __ZN4FMOD12DSPSfxReverb19SetReflectionsLevelEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReverbDelay(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetReverbDelayEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1de4 — __ZN4FMOD12DSPSfxReverb14SetReverbDelayEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1de4() -> ! {
    todo!("0xc1de4 __ZN4FMOD12DSPSfxReverb14SetReverbDelayEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReflectionsDelay(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetReflectionsDelayEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1e74 — __ZN4FMOD12DSPSfxReverb19SetReflectionsDelayEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1e74() -> ! {
    todo!("0xc1e74 __ZN4FMOD12DSPSfxReverb19SetReflectionsDelayEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetReverbLevel(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetReverbLevelEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc1f00 — __ZN4FMOD12DSPSfxReverb14SetReverbLevelEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc1f00() -> ! {
    todo!("0xc1f00 __ZN4FMOD12DSPSfxReverb14SetReverbLevelEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoom(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb7SetRoomEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2014 — __ZN4FMOD12DSPSfxReverb7SetRoomEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2014() -> ! {
    todo!("0xc2014 __ZN4FMOD12DSPSfxReverb7SetRoomEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::CalculateShelfCoeffs(float,float,float,float *,float *,float *,float *,float *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20CalculateShelfCoeffsEfffPfS1_S1_S1_S1_")]
// 0xc207c — __ZN4FMOD12DSPSfxReverb20CalculateShelfCoeffsEfffPfS1_S1_S1_S1_
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float, float, float, float *, float *, float *, float *, float *)
pub fn stub_0xc207c() -> ! {
    todo!("0xc207c __ZN4FMOD12DSPSfxReverb20CalculateShelfCoeffsEfffPfS1_S1_S1_S1_")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomLF(FMOD::SFX_REVERB_LFPROPS *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb9SetRoomLFEPNS_18SFX_REVERB_LFPROPSE")]
// 0xc2178 — __ZN4FMOD12DSPSfxReverb9SetRoomLFEPNS_18SFX_REVERB_LFPROPSE
pub fn stub_0xc2178() -> ! {
    todo!("0xc2178 __ZN4FMOD12DSPSfxReverb9SetRoomLFEPNS_18SFX_REVERB_LFPROPSE")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetLFReference(FMOD::SFX_REVERB_LFPROPS *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetLFReferenceEPNS_18SFX_REVERB_LFPROPSE")]
// 0xc2210 — __ZN4FMOD12DSPSfxReverb14SetLFReferenceEPNS_18SFX_REVERB_LFPROPSE
pub fn stub_0xc2210() -> ! {
    todo!("0xc2210 __ZN4FMOD12DSPSfxReverb14SetLFReferenceEPNS_18SFX_REVERB_LFPROPSE")
}

#[doc(alias = "FMOD::DSPSfxReverb::Calculate1stOrderLowpassCoeff(float,float,float,float *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb29Calculate1stOrderLowpassCoeffEfffPf")]
// 0xc2250 — __ZN4FMOD12DSPSfxReverb29Calculate1stOrderLowpassCoeffEfffPf
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float, float, float, float *)
pub fn stub_0xc2250() -> ! {
    todo!("0xc2250 __ZN4FMOD12DSPSfxReverb29Calculate1stOrderLowpassCoeffEfffPf")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDecayTime(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12SetDecayTimeEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2370 — __ZN4FMOD12DSPSfxReverb12SetDecayTimeEP25_I3DL2_LISTENERPROPERTIES
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xc2370() -> ! {
    todo!("0xc2370 __ZN4FMOD12DSPSfxReverb12SetDecayTimeEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDecayHFRatio(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15SetDecayHFRatioEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2508 — __ZN4FMOD12DSPSfxReverb15SetDecayHFRatioEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2508() -> ! {
    todo!("0xc2508 __ZN4FMOD12DSPSfxReverb15SetDecayHFRatioEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDelayLineLengths(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb19SetDelayLineLengthsEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2550 — __ZN4FMOD12DSPSfxReverb19SetDelayLineLengthsEP25_I3DL2_LISTENERPROPERTIES
// type: int __fastcall(int, int)
pub fn stub_0xc2550() -> ! {
    todo!("0xc2550 __ZN4FMOD12DSPSfxReverb19SetDelayLineLengthsEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetDensity(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb10SetDensityEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2618 — __ZN4FMOD12DSPSfxReverb10SetDensityEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2618() -> ! {
    todo!("0xc2618 __ZN4FMOD12DSPSfxReverb10SetDensityEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetRoomHF(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb9SetRoomHFEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2664 — __ZN4FMOD12DSPSfxReverb9SetRoomHFEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2664() -> ! {
    todo!("0xc2664 __ZN4FMOD12DSPSfxReverb9SetRoomHFEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::SetHFReference(_I3DL2_LISTENERPROPERTIES *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14SetHFReferenceEP25_I3DL2_LISTENERPROPERTIES")]
// 0xc2730 — __ZN4FMOD12DSPSfxReverb14SetHFReferenceEP25_I3DL2_LISTENERPROPERTIES
pub fn stub_0xc2730() -> ! {
    todo!("0xc2730 __ZN4FMOD12DSPSfxReverb14SetHFReferenceEP25_I3DL2_LISTENERPROPERTIES")
}

#[doc(alias = "FMOD::DSPSfxReverb::updateInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14updateInternalEv")]
// 0xc2794 — __ZN4FMOD12DSPSfxReverb14updateInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc2794() -> ! {
    todo!("0xc2794 __ZN4FMOD12DSPSfxReverb14updateInternalEv")
}

#[doc(alias = "FMOD::DSPSfxReverb::updateCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE")]
// 0xc2a18 — __ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc2a18() -> ! {
    todo!("0xc2a18 __ZN4FMOD12DSPSfxReverb14updateCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPSfxReverb::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc")]
// 0xc2a24 — __ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float *, char *)
pub fn stub_0xc2a24() -> ! {
    todo!("0xc2a24 __ZN4FMOD12DSPSfxReverb20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPSfxReverb::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc2e88 — __ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc2e88() -> ! {
    todo!("0xc2e88 __ZN4FMOD12DSPSfxReverb20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPSfxReverb::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterInternalEif")]
// 0xc2e94 — __ZN4FMOD12DSPSfxReverb20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, int, float)
pub fn stub_0xc2e94() -> ! {
    todo!("0xc2e94 __ZN4FMOD12DSPSfxReverb20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPSfxReverb::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc3178 — __ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc3178() -> ! {
    todo!("0xc3178 __ZN4FMOD12DSPSfxReverb20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPSfxReverb::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetInternalEv")]
// 0xc3184 — __ZN4FMOD12DSPSfxReverb13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc3184() -> ! {
    todo!("0xc3184 __ZN4FMOD12DSPSfxReverb13resetInternalEv")
}

#[doc(alias = "FMOD::DSPSfxReverb::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc31bc — __ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc31bc() -> ! {
    todo!("0xc31bc __ZN4FMOD12DSPSfxReverb13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPSfxReverb::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii")]
// 0xc31c8 — __ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this, float *, float *__dst, unsigned int, int, int)
pub fn stub_0xc31c8() -> ! {
    todo!("0xc31c8 __ZN4FMOD12DSPSfxReverb12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPSfxReverb::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xc327c — __ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_0xc327c() -> ! {
    todo!("0xc327c __ZN4FMOD12DSPSfxReverb12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPSfxReverb::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseInternalEv")]
// 0xc32a4 — __ZN4FMOD12DSPSfxReverb15releaseInternalEv
// type: int __fastcall(void **this)
pub fn stub_0xc32a4() -> ! {
    todo!("0xc32a4 __ZN4FMOD12DSPSfxReverb15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPSfxReverb::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xc32bc — __ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc32bc() -> ! {
    todo!("0xc32bc __ZN4FMOD12DSPSfxReverb15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPSfxReverb::createInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createInternalEv")]
// 0xc32c8 — __ZN4FMOD12DSPSfxReverb14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc32c8() -> ! {
    todo!("0xc32c8 __ZN4FMOD12DSPSfxReverb14createInternalEv")
}

#[doc(alias = "FMOD::DSPSfxReverb::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE")]
// 0xc35cc — __ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc35cc() -> ! {
    todo!("0xc35cc __ZN4FMOD12DSPSfxReverb14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPSfxReverb::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD12DSPSfxReverb16getDescriptionExEv")]
// 0xc35d8 — __ZN4FMOD12DSPSfxReverb16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPSfxReverb *__hidden this)
pub fn stub_0xc35d8() -> ! {
    todo!("0xc35d8 __ZN4FMOD12DSPSfxReverb16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspsfxreverb")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspsfxreverbE")]
// 0xc3718 — __GLOBAL__I__ZN4FMOD12dspsfxreverbE
pub fn stub_0xc3718() -> ! {
    todo!("0xc3718 __GLOBAL__I__ZN4FMOD12dspsfxreverbE")
}

#[doc(alias = "FMOD::DSPSoundCard::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xc3724 — __ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xc3724() -> ! {
    todo!("0xc3724 __ZN4FMOD12DSPSoundCard5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")
}

#[doc(alias = "FMOD::DSPSoundCard::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij")]
// 0xc3750 — __ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij
pub fn stub_0xc3750() -> ! {
    todo!("0xc3750 __ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPWaveTable::setPositionInternal(unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionInternalEj")]
// 0xc3bcc — __ZN4FMOD12DSPWaveTable19setPositionInternalEj
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, unsigned int)
pub fn stub_0xc3bcc() -> ! {
    todo!("0xc3bcc __ZN4FMOD12DSPWaveTable19setPositionInternalEj")
}

#[doc(alias = "FMOD::DSPWaveTable::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterInternalEif")]
// 0xc3bf4 — __ZN4FMOD12DSPWaveTable20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float)
pub fn stub_0xc3bf4() -> ! {
    todo!("0xc3bf4 __ZN4FMOD12DSPWaveTable20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPWaveTable::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc")]
// 0xc3bfc — __ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, int, float *, char *)
pub fn stub_0xc3bfc() -> ! {
    todo!("0xc3bfc __ZN4FMOD12DSPWaveTable20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPWaveTable::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable12setFrequencyEf")]
// 0xc3c04 — __ZN4FMOD12DSPWaveTable12setFrequencyEf
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, float)
pub fn stub_0xc3c04() -> ! {
    todo!("0xc3c04 __ZN4FMOD12DSPWaveTable12setFrequencyEf")
}

#[doc(alias = "FMOD::DSPWaveTable::getFinished(bool *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable11getFinishedEPb")]
// 0xc3c80 — __ZN4FMOD12DSPWaveTable11getFinishedEPb
// type: _DWORD __fastcall(FMOD::DSPWaveTable *__hidden this, bool *)
pub fn stub_0xc3c80() -> ! {
    todo!("0xc3c80 __ZN4FMOD12DSPWaveTable11getFinishedEPb")
}

#[doc(alias = "FMOD::DSPWaveTable::setPositionCallback(FMOD_DSP_STATE *,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj")]
// 0xc3cc4 — __ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj
// type: int __fastcall(FMOD::DSPWaveTable *, unsigned int)
pub fn stub_0xc3cc4() -> ! {
    todo!("0xc3cc4 __ZN4FMOD12DSPWaveTable19setPositionCallbackEP14FMOD_DSP_STATEj")
}

#[doc(alias = "FMOD::DSPWaveTable::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xc3cd0 — __ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_0xc3cd0() -> ! {
    todo!("0xc3cd0 __ZN4FMOD12DSPWaveTable20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPWaveTable::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xc3cdc — __ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_0xc3cdc() -> ! {
    todo!("0xc3cdc __ZN4FMOD12DSPWaveTable20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPWaveTable::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE")]
// 0xc3ce8 — __ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_0xc3ce8() -> ! {
    todo!("0xc3ce8 __ZN4FMOD12DSPWaveTable13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPWaveTable::setFinished(bool,bool)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable11setFinishedEbb")]
// 0xc3d00 — __ZN4FMOD12DSPWaveTable11setFinishedEbb
// type: int __fastcall(FMOD::DSPWaveTable *this, bool, bool)
pub fn stub_0xc3d00() -> ! {
    todo!("0xc3d00 __ZN4FMOD12DSPWaveTable11setFinishedEbb")
}

#[doc(alias = "FMOD::DSPWaveTable::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc3d94 — __ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xc3d94() -> ! {
    todo!("0xc3d94 __ZN4FMOD12DSPWaveTable4readEPPfPiPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPWaveTable::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xc4728 — __ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xc4728() -> ! {
    todo!("0xc4728 __ZN4FMOD12DSPWaveTable5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")
}

#[doc(alias = "FMOD::DSPI::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xc4788 — __ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij
pub fn stub_0xc4788() -> ! {
    todo!("0xc4788 __ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPI::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij")]
// 0xc4790 — __ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij
pub fn stub_0xc4790() -> ! {
    todo!("0xc4790 __ZN4FMOD4DSPI4readEPvPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPI::DSPI(void)")]
#[doc(alias = "__ZN4FMOD4DSPIC2Ev")]
// 0xc47d4 — __ZN4FMOD4DSPIC2Ev
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc47d4() -> ! {
    todo!("0xc47d4 __ZN4FMOD4DSPIC2Ev")
}

#[doc(alias = "FMOD::DSPI::getSystemObject(FMOD::System **)")]
#[doc(alias = "__ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE")]
// 0xc489c — __ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE
pub fn stub_0xc489c() -> ! {
    todo!("0xc489c __ZN4FMOD4DSPI15getSystemObjectEPPNS_6SystemE")
}

#[doc(alias = "FMOD::DSPI::updateDSPTick(unsigned int)")]
#[doc(alias = "__ZN4FMOD4DSPI13updateDSPTickEj")]
// 0xc48b4 — __ZN4FMOD4DSPI13updateDSPTickEj
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, unsigned int)
pub fn stub_0xc48b4() -> ! {
    todo!("0xc48b4 __ZN4FMOD4DSPI13updateDSPTickEj")
}

#[doc(alias = "FMOD::DSPI::reset(void)")]
#[doc(alias = "__ZN4FMOD4DSPI5resetEv")]
// 0xc48f8 — __ZN4FMOD4DSPI5resetEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc48f8() -> ! {
    todo!("0xc48f8 __ZN4FMOD4DSPI5resetEv")
}

#[doc(alias = "FMOD::DSPI::setParameter(int,float)")]
#[doc(alias = "__ZN4FMOD4DSPI12setParameterEif")]
// 0xc4918 — __ZN4FMOD4DSPI12setParameterEif
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, float)
pub fn stub_0xc4918() -> ! {
    todo!("0xc4918 __ZN4FMOD4DSPI12setParameterEif")
}

#[doc(alias = "FMOD::DSPI::getNumParameters(int *)")]
#[doc(alias = "__ZN4FMOD4DSPI16getNumParametersEPi")]
// 0xc498c — __ZN4FMOD4DSPI16getNumParametersEPi
// type: int __fastcall(FMOD::DSPI *this, int *)
pub fn stub_0xc498c() -> ! {
    todo!("0xc498c __ZN4FMOD4DSPI16getNumParametersEPi")
}

#[doc(alias = "FMOD::DSPI::showConfigDialog(void *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI16showConfigDialogEPvb")]
// 0xc49a4 — __ZN4FMOD4DSPI16showConfigDialogEPvb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *, bool)
pub fn stub_0xc49a4() -> ! {
    todo!("0xc49a4 __ZN4FMOD4DSPI16showConfigDialogEPvb")
}

#[doc(alias = "FMOD::DSPI::getType(FMOD_DSP_TYPE *)")]
#[doc(alias = "__ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE")]
// 0xc49c8 — __ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE
pub fn stub_0xc49c8() -> ! {
    todo!("0xc49c8 __ZN4FMOD4DSPI7getTypeEP13FMOD_DSP_TYPE")
}

#[doc(alias = "FMOD::DSPI::setDefaults(float,float,float,int)")]
#[doc(alias = "__ZN4FMOD4DSPI11setDefaultsEfffi")]
// 0xc49dc — __ZN4FMOD4DSPI11setDefaultsEfffi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float, float, float, int)
pub fn stub_0xc49dc() -> ! {
    todo!("0xc49dc __ZN4FMOD4DSPI11setDefaultsEfffi")
}

#[doc(alias = "FMOD::DSPI::getDefaults(float *,float *,float *,int *)")]
#[doc(alias = "__ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi")]
// 0xc4a64 — __ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float *, float *, float *, int *)
pub fn stub_0xc4a64() -> ! {
    todo!("0xc4a64 __ZN4FMOD4DSPI11getDefaultsEPfS1_S1_Pi")
}

#[doc(alias = "FMOD::DSPI::setUserData(void *)")]
#[doc(alias = "__ZN4FMOD4DSPI11setUserDataEPv")]
// 0xc4aa8 — __ZN4FMOD4DSPI11setUserDataEPv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, void *)
pub fn stub_0xc4aa8() -> ! {
    todo!("0xc4aa8 __ZN4FMOD4DSPI11setUserDataEPv")
}
