//! core shard jx — 100 stubs EA-sorted 0xadb30..0xb3a84 (global EA-sorted, next 100 not yet in core after jw 0xadb24, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) global EA-sorted ascending, next 100 not yet in rbx_core (34459 before -> 34559 after, gap 51086->50986).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::DSPChorus::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPChorus12readInternalEPfS1_jii")]
// 0xadb30 — __ZN4FMOD9DSPChorus12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPChorus *this, float *, float *, unsigned int, int, int)
pub fn stub_adb30() -> ! {
    todo!("0xadb30 __ZN4FMOD9DSPChorus12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPChorus::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPChorus12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xae24c — __ZN4FMOD9DSPChorus12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPChorus *, float *, float *, unsigned int, int, int)
pub fn stub_ae24c() -> ! {
    todo!("0xae24c __ZN4FMOD9DSPChorus12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPChorus::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPChorus13resetInternalEv")]
// 0xae274 — __ZN4FMOD9DSPChorus13resetInternalEv
// type: int __fastcall(FMOD::DSPChorus *this)
pub fn stub_ae274() -> ! {
    todo!("0xae274 __ZN4FMOD9DSPChorus13resetInternalEv")
}

#[doc(alias = "FMOD::DSPChorus::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus13resetCallbackEP14FMOD_DSP_STATE")]
// 0xae2a8 — __ZN4FMOD9DSPChorus13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPChorus *)
pub fn stub_ae2a8() -> ! {
    todo!("0xae2a8 __ZN4FMOD9DSPChorus13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPChorus::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPChorus20setParameterInternalEif")]
// 0xae2b4 — __ZN4FMOD9DSPChorus20setParameterInternalEif
// type: int __fastcall(FMOD::DSPChorus *this, int, float)
pub fn stub_ae2b4() -> ! {
    todo!("0xae2b4 __ZN4FMOD9DSPChorus20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPChorus::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPChorus20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xae3c4 — __ZN4FMOD9DSPChorus20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPChorus *, int, float)
pub fn stub_ae3c4() -> ! {
    todo!("0xae3c4 __ZN4FMOD9DSPChorus20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPChorus::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPChorus15releaseInternalEv")]
// 0xae3d0 — __ZN4FMOD9DSPChorus15releaseInternalEv
// type: int __fastcall(FMOD::DSPChorus *this)
pub fn stub_ae3d0() -> ! {
    todo!("0xae3d0 __ZN4FMOD9DSPChorus15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPChorus::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xae420 — __ZN4FMOD9DSPChorus15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPChorus *)
pub fn stub_ae420() -> ! {
    todo!("0xae420 __ZN4FMOD9DSPChorus15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPChorus::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPChorus14createInternalEv")]
// 0xae42c — __ZN4FMOD9DSPChorus14createInternalEv
// type: int __fastcall(FMOD::DSPChorus *this)
pub fn stub_ae42c() -> ! {
    todo!("0xae42c __ZN4FMOD9DSPChorus14createInternalEv")
}

#[doc(alias = "FMOD::DSPChorus::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPChorus14createCallbackEP14FMOD_DSP_STATE")]
// 0xae5ac — __ZN4FMOD9DSPChorus14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPChorus *)
pub fn stub_ae5ac() -> ! {
    todo!("0xae5ac __ZN4FMOD9DSPChorus14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPChorus::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPChorus16getDescriptionExEv")]
// 0xae5b8 — __ZN4FMOD9DSPChorus16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPChorus *this)
pub fn stub_ae5b8() -> ! {
    todo!("0xae5b8 __ZN4FMOD9DSPChorus16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspchorus")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspchorusE")]
// 0xae6e8 — __GLOBAL__I__ZN4FMOD9dspchorusE
// type: int()
pub fn stub_ae6e8() -> ! {
    todo!("0xae6e8 __GLOBAL__I__ZN4FMOD9dspchorusE")
}

#[doc(alias = "FMOD::DSPCodec::createInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPCodec14createInternalEv")]
// 0xae6f4 — __ZN4FMOD8DSPCodec14createInternalEv
// type: int __fastcall(FMOD::DSPCodec *this)
pub fn stub_ae6f4() -> ! {
    todo!("0xae6f4 __ZN4FMOD8DSPCodec14createInternalEv")
}

#[doc(alias = "FMOD::DSPCodec::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPCodec15releaseInternalEv")]
// 0xae73c — __ZN4FMOD8DSPCodec15releaseInternalEv
// type: int __fastcall(FMOD::DSPCodec *this)
pub fn stub_ae73c() -> ! {
    todo!("0xae73c __ZN4FMOD8DSPCodec15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPCodec::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD8DSPCodec13resetInternalEv")]
// 0xae744 — __ZN4FMOD8DSPCodec13resetInternalEv
// type: int __fastcall(FMOD::DSPCodec *this)
pub fn stub_ae744() -> ! {
    todo!("0xae744 __ZN4FMOD8DSPCodec13resetInternalEv")
}

#[doc(alias = "FMOD::DSPCodec::setPositionInternal(unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD8DSPCodec19setPositionInternalEjb")]
// 0xae74c — __ZN4FMOD8DSPCodec19setPositionInternalEjb
// type: int __fastcall(FMOD::DSPCodec *this, unsigned int, bool)
pub fn stub_ae74c() -> ! {
    todo!("0xae74c __ZN4FMOD8DSPCodec19setPositionInternalEjb")
}

#[doc(alias = "FMOD::DSPCodec::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD8DSPCodec20setParameterInternalEif")]
// 0xae76c — __ZN4FMOD8DSPCodec20setParameterInternalEif
// type: int __fastcall(FMOD::DSPCodec *this, int, float)
pub fn stub_ae76c() -> ! {
    todo!("0xae76c __ZN4FMOD8DSPCodec20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPCodec::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD8DSPCodec20getParameterInternalEiPfPc")]
// 0xae774 — __ZN4FMOD8DSPCodec20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPCodec *this, int, float *, char *)
pub fn stub_ae774() -> ! {
    todo!("0xae774 __ZN4FMOD8DSPCodec20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPCodec::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPCodec14createCallbackEP14FMOD_DSP_STATE")]
// 0xae77c — __ZN4FMOD8DSPCodec14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPCodec *)
pub fn stub_ae77c() -> ! {
    todo!("0xae77c __ZN4FMOD8DSPCodec14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPCodec::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPCodec15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xae788 — __ZN4FMOD8DSPCodec15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPCodec *)
pub fn stub_ae788() -> ! {
    todo!("0xae788 __ZN4FMOD8DSPCodec15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPCodec::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD8DSPCodec13resetCallbackEP14FMOD_DSP_STATE")]
// 0xae794 — __ZN4FMOD8DSPCodec13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPCodec *)
pub fn stub_ae794() -> ! {
    todo!("0xae794 __ZN4FMOD8DSPCodec13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPCodec::setPositionCallback(FMOD_DSP_STATE *,unsigned int)")]
#[doc(alias = "__ZN4FMOD8DSPCodec19setPositionCallbackEP14FMOD_DSP_STATEj")]
// 0xae7a0 — __ZN4FMOD8DSPCodec19setPositionCallbackEP14FMOD_DSP_STATEj
// type: int __fastcall(FMOD::DSPCodec *, unsigned int)
pub fn stub_ae7a0() -> ! {
    todo!("0xae7a0 __ZN4FMOD8DSPCodec19setPositionCallbackEP14FMOD_DSP_STATEj")
}

#[doc(alias = "FMOD::DSPCodec::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD8DSPCodec20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xae7b0 — __ZN4FMOD8DSPCodec20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPCodec *, int, float)
pub fn stub_ae7b0() -> ! {
    todo!("0xae7b0 __ZN4FMOD8DSPCodec20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPCodec::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD8DSPCodec20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xae7bc — __ZN4FMOD8DSPCodec20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPCodec *, int, float *, char *)
pub fn stub_ae7bc() -> ! {
    todo!("0xae7bc __ZN4FMOD8DSPCodec20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPCodec::updateDSPCodec(FMOD::SoundI *,int)")]
#[doc(alias = "__ZN4FMOD8DSPCodec14updateDSPCodecEPNS_6SoundIEi")]
// 0xae7c8 — __ZN4FMOD8DSPCodec14updateDSPCodecEPNS_6SoundIEi
// type: int __fastcall(FMOD::DSPCodec *this, FMOD::SoundI *, int)
pub fn stub_ae7c8() -> ! {
    todo!("0xae7c8 __ZN4FMOD8DSPCodec14updateDSPCodecEPNS_6SoundIEi")
}

#[doc(alias = "FMOD::DSPCodec::readInternal(short *,short *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD8DSPCodec12readInternalEPsS1_jii")]
// 0xae858 — __ZN4FMOD8DSPCodec12readInternalEPsS1_jii
// type: int __fastcall(FMOD::DSPCodec *this, __int16 *, __int16 *, unsigned int, int, int)
pub fn stub_ae858() -> ! {
    todo!("0xae858 __ZN4FMOD8DSPCodec12readInternalEPsS1_jii")
}

#[doc(alias = "FMOD::DSPCodec::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD8DSPCodec12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xaed98 — __ZN4FMOD8DSPCodec12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPCodec *, __int16 *, __int16 *, unsigned int, int, int)
pub fn stub_aed98() -> ! {
    todo!("0xaed98 __ZN4FMOD8DSPCodec12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPCodec::release(bool)")]
#[doc(alias = "__ZN4FMOD8DSPCodec7releaseEb")]
// 0xaedc0 — __ZN4FMOD8DSPCodec7releaseEb
// type: int __fastcall(FMOD::DSPCodec *this, bool)
pub fn stub_aedc0() -> ! {
    todo!("0xaedc0 __ZN4FMOD8DSPCodec7releaseEb")
}

#[doc(alias = "FMOD::DSPCodec::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD8DSPCodec16getDescriptionExEv")]
// 0xaee3c — __ZN4FMOD8DSPCodec16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPCodec *this)
pub fn stub_aee3c() -> ! {
    todo!("0xaee3c __ZN4FMOD8DSPCodec16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspcodec")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD8dspcodecE")]
// 0xaef58 — __GLOBAL__I__ZN4FMOD8dspcodecE
// type: int()
pub fn stub_aef58() -> ! {
    todo!("0xaef58 __GLOBAL__I__ZN4FMOD8dspcodecE")
}

#[doc(alias = "FMOD::DSPCodecPool::areAnyFree(void)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool10areAnyFreeEv")]
// 0xaef64 — __ZN4FMOD12DSPCodecPool10areAnyFreeEv
// type: int __fastcall(FMOD::DSPCodecPool *this)
pub fn stub_aef64() -> ! {
    todo!("0xaef64 __ZN4FMOD12DSPCodecPool10areAnyFreeEv")
}

#[doc(alias = "FMOD::DSPCodecPool::alloc(FMOD::DSPCodec **)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool5allocEPPNS_8DSPCodecE")]
// 0xaefcc — __ZN4FMOD12DSPCodecPool5allocEPPNS_8DSPCodecE
// type: int __fastcall(FMOD::DSPCodecPool *this, FMOD::DSPCodec **)
pub fn stub_aefcc() -> ! {
    todo!("0xaefcc __ZN4FMOD12DSPCodecPool5allocEPPNS_8DSPCodecE")
}

#[doc(alias = "FMOD::DSPCodecPool::close(void)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool5closeEv")]
// 0xaf050 — __ZN4FMOD12DSPCodecPool5closeEv
// type: int __fastcall(FMOD::DSPCodecPool *this)
pub fn stub_af050() -> ! {
    todo!("0xaf050 __ZN4FMOD12DSPCodecPool5closeEv")
}

#[doc(alias = "FMOD::DSPCodecPool::init(FMOD::FMOD_DSP_CATEGORY,int,int)")]
#[doc(alias = "__ZN4FMOD12DSPCodecPool4initENS_17FMOD_DSP_CATEGORYEii")]
// 0xaf148 — __ZN4FMOD12DSPCodecPool4initENS_17FMOD_DSP_CATEGORYEii
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_af148() -> ! {
    todo!("0xaf148 __ZN4FMOD12DSPCodecPool4initENS_17FMOD_DSP_CATEGORYEii")
}

#[doc(alias = "FMOD::Codec::defaultGetWaveFormat(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
#[doc(alias = "__ZN4FMOD5Codec20defaultGetWaveFormatEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT")]
// 0xaf528 — __ZN4FMOD5Codec20defaultGetWaveFormatEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int *, int, void *__dst)
pub fn stub_af528() -> ! {
    todo!("0xaf528 __ZN4FMOD5Codec20defaultGetWaveFormatEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT")
}

#[doc(alias = "FMOD::DSPCompressor::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor14createInternalEv")]
// 0xaf5ac — __ZN4FMOD13DSPCompressor14createInternalEv
// type: int __fastcall(FMOD::DSPCompressor *this)
pub fn stub_af5ac() -> ! {
    todo!("0xaf5ac __ZN4FMOD13DSPCompressor14createInternalEv")
}

#[doc(alias = "FMOD::DSPCompressor::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xaf648 — __ZN4FMOD13DSPCompressor17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int()
pub fn stub_af648() -> ! {
    todo!("0xaf648 __ZN4FMOD13DSPCompressor17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPCompressor::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor14createCallbackEP14FMOD_DSP_STATE")]
// 0xaf650 — __ZN4FMOD13DSPCompressor14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPCompressor *)
pub fn stub_af650() -> ! {
    todo!("0xaf650 __ZN4FMOD13DSPCompressor14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPCompressor::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xaf65c — __ZN4FMOD13DSPCompressor21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_af65c() -> ! {
    todo!("0xaf65c __ZN4FMOD13DSPCompressor21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPCompressor::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20getParameterInternalEiPfPc")]
// 0xaf6b4 — __ZN4FMOD13DSPCompressor20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPCompressor *this, int, float *, char *)
pub fn stub_af6b4() -> ! {
    todo!("0xaf6b4 __ZN4FMOD13DSPCompressor20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPCompressor::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xaf798 — __ZN4FMOD13DSPCompressor20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPCompressor *, int, float *, char *)
pub fn stub_af798() -> ! {
    todo!("0xaf798 __ZN4FMOD13DSPCompressor20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPCompressor::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20setParameterInternalEif")]
// 0xaf7a4 — __ZN4FMOD13DSPCompressor20setParameterInternalEif
// type: int __fastcall(FMOD::DSPCompressor *this, int, float)
pub fn stub_af7a4() -> ! {
    todo!("0xaf7a4 __ZN4FMOD13DSPCompressor20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPCompressor::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xaf8a4 — __ZN4FMOD13DSPCompressor20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPCompressor *, int, float)
pub fn stub_af8a4() -> ! {
    todo!("0xaf8a4 __ZN4FMOD13DSPCompressor20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPCompressor::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor12readInternalEPfS1_jii")]
// 0xaf8b0 — __ZN4FMOD13DSPCompressor12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPCompressor *this, float *, float *__dst, unsigned int, int, int)
pub fn stub_af8b0() -> ! {
    todo!("0xaf8b0 __ZN4FMOD13DSPCompressor12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPCompressor::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xafc5c — __ZN4FMOD13DSPCompressor12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPCompressor *, float *, float *, unsigned int, int, int)
pub fn stub_afc5c() -> ! {
    todo!("0xafc5c __ZN4FMOD13DSPCompressor12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPCompressor::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPCompressor16getDescriptionExEv")]
// 0xafc84 — __ZN4FMOD13DSPCompressor16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPCompressor *this)
pub fn stub_afc84() -> ! {
    todo!("0xafc84 __ZN4FMOD13DSPCompressor16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspcompressor")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dspcompressorE")]
// 0xafd94 — __GLOBAL__I__ZN4FMOD13dspcompressorE
// type: int()
pub fn stub_afd94() -> ! {
    todo!("0xafd94 __GLOBAL__I__ZN4FMOD13dspcompressorE")
}

#[doc(alias = "FMOD::DSPConnectionPool::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xafda0 — __ZN4FMOD17DSPConnectionPool17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::MemoryTracker *)
pub fn stub_afda0() -> ! {
    todo!("0xafda0 __ZN4FMOD17DSPConnectionPool17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPConnectionPool::close(void)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool5closeEv")]
// 0xafe90 — __ZN4FMOD17DSPConnectionPool5closeEv
// type: int __fastcall(FMOD::DSPConnectionPool *this)
pub fn stub_afe90() -> ! {
    todo!("0xafe90 __ZN4FMOD17DSPConnectionPool5closeEv")
}

#[doc(alias = "FMOD::DSPConnectionPool::init(FMOD::SystemI *,int,int,int)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool4initEPNS_7SystemIEiii")]
// 0xaff70 — __ZN4FMOD17DSPConnectionPool4initEPNS_7SystemIEiii
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::SystemI *, int, int, int)
pub fn stub_aff70() -> ! {
    todo!("0xaff70 __ZN4FMOD17DSPConnectionPool4initEPNS_7SystemIEiii")
}

#[doc(alias = "FMOD::DSPConnectionPool::free(FMOD::DSPConnectionI *,bool)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool4freeEPNS_14DSPConnectionIEb")]
// 0xb01bc — __ZN4FMOD17DSPConnectionPool4freeEPNS_14DSPConnectionIEb
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::DSPConnectionI *, bool)
pub fn stub_b01bc() -> ! {
    todo!("0xb01bc __ZN4FMOD17DSPConnectionPool4freeEPNS_14DSPConnectionIEb")
}

#[doc(alias = "FMOD::DSPConnectionPool::alloc(FMOD::DSPConnectionI **,bool)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool5allocEPPNS_14DSPConnectionIEb")]
// 0xb02d4 — __ZN4FMOD17DSPConnectionPool5allocEPPNS_14DSPConnectionIEb
// type: int __fastcall(FMOD::DSPConnectionPool *this, FMOD::DSPConnectionI **, bool)
pub fn stub_b02d4() -> ! {
    todo!("0xb02d4 __ZN4FMOD17DSPConnectionPool5allocEPPNS_14DSPConnectionIEb")
}

#[doc(alias = "FMOD::DSPConnectionPool::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD17DSPConnectionPool13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xb0608 — __ZN4FMOD17DSPConnectionPool13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_b0608() -> ! {
    todo!("0xb0608 __ZN4FMOD17DSPConnectionPool13getMemoryUsedEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPDistortion::createInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion14createInternalEv")]
// 0xb0660 — __ZN4FMOD13DSPDistortion14createInternalEv
// type: int __fastcall(FMOD::DSPDistortion *this)
pub fn stub_b0660() -> ! {
    todo!("0xb0660 __ZN4FMOD13DSPDistortion14createInternalEv")
}

#[doc(alias = "FMOD::DSPDistortion::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion15releaseInternalEv")]
// 0xb06d8 — __ZN4FMOD13DSPDistortion15releaseInternalEv
// type: int __fastcall(FMOD::DSPDistortion *this)
pub fn stub_b06d8() -> ! {
    todo!("0xb06d8 __ZN4FMOD13DSPDistortion15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPDistortion::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion13resetInternalEv")]
// 0xb06e0 — __ZN4FMOD13DSPDistortion13resetInternalEv
// type: int __fastcall(FMOD::DSPDistortion *this)
pub fn stub_b06e0() -> ! {
    todo!("0xb06e0 __ZN4FMOD13DSPDistortion13resetInternalEv")
}

#[doc(alias = "FMOD::DSPDistortion::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20setParameterInternalEif")]
// 0xb06e8 — __ZN4FMOD13DSPDistortion20setParameterInternalEif
// type: int __fastcall(FMOD::DSPDistortion *this, int, float)
pub fn stub_b06e8() -> ! {
    todo!("0xb06e8 __ZN4FMOD13DSPDistortion20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPDistortion::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion14createCallbackEP14FMOD_DSP_STATE")]
// 0xb06f4 — __ZN4FMOD13DSPDistortion14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPDistortion *)
pub fn stub_b06f4() -> ! {
    todo!("0xb06f4 __ZN4FMOD13DSPDistortion14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPDistortion::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb0700 — __ZN4FMOD13DSPDistortion15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPDistortion *)
pub fn stub_b0700() -> ! {
    todo!("0xb0700 __ZN4FMOD13DSPDistortion15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPDistortion::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb070c — __ZN4FMOD13DSPDistortion13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPDistortion *)
pub fn stub_b070c() -> ! {
    todo!("0xb070c __ZN4FMOD13DSPDistortion13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPDistortion::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb0718 — __ZN4FMOD13DSPDistortion20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPDistortion *, int, float)
pub fn stub_b0718() -> ! {
    todo!("0xb0718 __ZN4FMOD13DSPDistortion20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPDistortion::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20getParameterInternalEiPfPc")]
// 0xb0724 — __ZN4FMOD13DSPDistortion20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPDistortion *this, int, float *, char *)
pub fn stub_b0724() -> ! {
    todo!("0xb0724 __ZN4FMOD13DSPDistortion20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPDistortion::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb075c — __ZN4FMOD13DSPDistortion20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPDistortion *, int, float *, char *)
pub fn stub_b075c() -> ! {
    todo!("0xb075c __ZN4FMOD13DSPDistortion20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPDistortion::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion12readInternalEPfS1_jii")]
// 0xb0768 — __ZN4FMOD13DSPDistortion12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPDistortion *this, float *, float *__dst, unsigned int, int, int)
pub fn stub_b0768() -> ! {
    todo!("0xb0768 __ZN4FMOD13DSPDistortion12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPDistortion::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb0e2c — __ZN4FMOD13DSPDistortion12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPDistortion *, float *, float *, unsigned int, int, int)
pub fn stub_b0e2c() -> ! {
    todo!("0xb0e2c __ZN4FMOD13DSPDistortion12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPDistortion::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD13DSPDistortion16getDescriptionExEv")]
// 0xb0e54 — __ZN4FMOD13DSPDistortion16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPDistortion *this)
pub fn stub_b0e54() -> ! {
    todo!("0xb0e54 __ZN4FMOD13DSPDistortion16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspdistortion")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD13dspdistortionE")]
// 0xb0f74 — __GLOBAL__I__ZN4FMOD13dspdistortionE
// type: int()
pub fn stub_b0f74() -> ! {
    todo!("0xb0f74 __GLOBAL__I__ZN4FMOD13dspdistortionE")
}

#[doc(alias = "FMOD::DSPEcho::createInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14createInternalEv")]
// 0xb0f80 — __ZN4FMOD7DSPEcho14createInternalEv
// type: int __fastcall(FMOD::DSPEcho *this)
pub fn stub_b0f80() -> ! {
    todo!("0xb0f80 __ZN4FMOD7DSPEcho14createInternalEv")
}

#[doc(alias = "FMOD::DSPEcho::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14createCallbackEP14FMOD_DSP_STATE")]
// 0xb1034 — __ZN4FMOD7DSPEcho14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPEcho *)
pub fn stub_b1034() -> ! {
    todo!("0xb1034 __ZN4FMOD7DSPEcho14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb1040 — __ZN4FMOD7DSPEcho17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPEcho *this, FMOD::MemoryTracker *)
pub fn stub_b1040() -> ! {
    todo!("0xb1040 __ZN4FMOD7DSPEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb1074 — __ZN4FMOD7DSPEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPEcho *this, FMOD::MemoryTracker *)
pub fn stub_b1074() -> ! {
    todo!("0xb1074 __ZN4FMOD7DSPEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPEcho::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20getParameterInternalEiPfPc")]
// 0xb10cc — __ZN4FMOD7DSPEcho20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPEcho *this, int, float *, char *)
pub fn stub_b10cc() -> ! {
    todo!("0xb10cc __ZN4FMOD7DSPEcho20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb11f4 — __ZN4FMOD7DSPEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPEcho *, int, float *, char *)
pub fn stub_b11f4() -> ! {
    todo!("0xb11f4 __ZN4FMOD7DSPEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPEcho::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho15releaseInternalEv")]
// 0xb1200 — __ZN4FMOD7DSPEcho15releaseInternalEv
// type: int __fastcall(FMOD::DSPEcho *this)
pub fn stub_b1200() -> ! {
    todo!("0xb1200 __ZN4FMOD7DSPEcho15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPEcho::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb1254 — __ZN4FMOD7DSPEcho15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPEcho *)
pub fn stub_b1254() -> ! {
    todo!("0xb1254 __ZN4FMOD7DSPEcho15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPEcho::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD7DSPEcho12readInternalEPfS1_jii")]
// 0xb1260 — __ZN4FMOD7DSPEcho12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPEcho *this, float *, float *__dst, unsigned int, int, int)
pub fn stub_b1260() -> ! {
    todo!("0xb1260 __ZN4FMOD7DSPEcho12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD7DSPEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb1fec — __ZN4FMOD7DSPEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPEcho *, float *, float *, unsigned int, int, int)
pub fn stub_b1fec() -> ! {
    todo!("0xb1fec __ZN4FMOD7DSPEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPEcho::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho13resetInternalEv")]
// 0xb2014 — __ZN4FMOD7DSPEcho13resetInternalEv
// type: int __fastcall(FMOD::DSPEcho *this)
pub fn stub_b2014() -> ! {
    todo!("0xb2014 __ZN4FMOD7DSPEcho13resetInternalEv")
}

#[doc(alias = "FMOD::DSPEcho::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb2048 — __ZN4FMOD7DSPEcho13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPEcho *)
pub fn stub_b2048() -> ! {
    todo!("0xb2048 __ZN4FMOD7DSPEcho13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPEcho::updateInternal(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14updateInternalEv")]
// 0xb2054 — __ZN4FMOD7DSPEcho14updateInternalEv
// type: int __fastcall(FMOD::DSPEcho *this)
pub fn stub_b2054() -> ! {
    todo!("0xb2054 __ZN4FMOD7DSPEcho14updateInternalEv")
}

#[doc(alias = "FMOD::DSPEcho::updateCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD7DSPEcho14updateCallbackEP14FMOD_DSP_STATE")]
// 0xb21e8 — __ZN4FMOD7DSPEcho14updateCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPEcho *)
pub fn stub_b21e8() -> ! {
    todo!("0xb21e8 __ZN4FMOD7DSPEcho14updateCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPEcho::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD7DSPEcho16getDescriptionExEv")]
// 0xb21f4 — __ZN4FMOD7DSPEcho16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPEcho *this)
pub fn stub_b21f4() -> ! {
    todo!("0xb21f4 __ZN4FMOD7DSPEcho16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPEcho::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20setParameterInternalEif")]
// 0xb22f0 — __ZN4FMOD7DSPEcho20setParameterInternalEif
// type: int __fastcall(FMOD::DSPEcho *this, int, float)
pub fn stub_b22f0() -> ! {
    todo!("0xb22f0 __ZN4FMOD7DSPEcho20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD7DSPEcho20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb2424 — __ZN4FMOD7DSPEcho20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPEcho *, int, float)
pub fn stub_b2424() -> ! {
    todo!("0xb2424 __ZN4FMOD7DSPEcho20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "global constructor keyed toFMOD::dspecho_desc")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD12dspecho_descE")]
// 0xb2474 — __GLOBAL__I__ZN4FMOD12dspecho_descE
// type: int()
pub fn stub_b2474() -> ! {
    todo!("0xb2474 __GLOBAL__I__ZN4FMOD12dspecho_descE")
}

#[doc(alias = "FMOD::DSPFFT::process(int)")]
#[doc(alias = "__ZN4FMOD6DSPFFT7processEi")]
// 0xb2480 — __ZN4FMOD6DSPFFT7processEi
// type: int __fastcall(FMOD::DSPFFT *this, int)
pub fn stub_b2480() -> ! {
    todo!("0xb2480 __ZN4FMOD6DSPFFT7processEi")
}

#[doc(alias = "FMOD::DSPFFT::DSPFFT(void)")]
#[doc(alias = "__ZN4FMOD6DSPFFTC2Ev")]
// 0xb2764 — __ZN4FMOD6DSPFFTC2Ev
// type: float __fastcall(FMOD::DSPFFT *this)
pub fn stub_b2764() -> ! {
    todo!("0xb2764 __ZN4FMOD6DSPFFTC2Ev")
}

#[doc(alias = "FMOD::DSPFFT::DSPFFT(void)")]
#[doc(alias = "__ZN4FMOD6DSPFFTC1Ev")]
// 0xb27bc — __ZN4FMOD6DSPFFTC1Ev
// type: float __fastcall(FMOD::DSPFFT *this)
pub fn stub_b27bc() -> ! {
    todo!("0xb27bc __ZN4FMOD6DSPFFTC1Ev")
}

#[doc(alias = "FMOD::DSPFFT::getSpectrum(float *,unsigned int,unsigned int,float *,int,int,int,FMOD_DSP_FFT_WINDOW)")]
#[doc(alias = "__ZN4FMOD6DSPFFT11getSpectrumEPfjjS1_iii19FMOD_DSP_FFT_WINDOW")]
// 0xb27c0 — __ZN4FMOD6DSPFFT11getSpectrumEPfjjS1_iii19FMOD_DSP_FFT_WINDOW
// type: int __fastcall(FMOD::DSPFFT *this, int, unsigned int, unsigned int, int, int, int, int, int)
pub fn stub_b27c0() -> ! {
    todo!("0xb27c0 __ZN4FMOD6DSPFFT11getSpectrumEPfjjS1_iii19FMOD_DSP_FFT_WINDOW")
}

#[doc(alias = "FMOD::DSPFilter::getHistoryBuffer(float **,unsigned int *,unsigned int *)")]
#[doc(alias = "__ZN4FMOD9DSPFilter16getHistoryBufferEPPfPjS3_")]
// 0xb308c — __ZN4FMOD9DSPFilter16getHistoryBufferEPPfPjS3_
// type: int __fastcall(FMOD::DSPFilter *this, float **, unsigned int *, unsigned int *)
pub fn stub_b308c() -> ! {
    todo!("0xb308c __ZN4FMOD9DSPFilter16getHistoryBufferEPPfPjS3_")
}

#[doc(alias = "FMOD::DSPFilter::release(bool)")]
#[doc(alias = "__ZN4FMOD9DSPFilter7releaseEb")]
// 0xb30b8 — __ZN4FMOD9DSPFilter7releaseEb
// type: int __fastcall(FMOD::DSPFilter *this, bool)
pub fn stub_b30b8() -> ! {
    todo!("0xb30b8 __ZN4FMOD9DSPFilter7releaseEb")
}

#[doc(alias = "FMOD::DSPFilter::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD9DSPFilter4readEPPfPiPj16FMOD_SPEAKERMODEij")]
// 0xb30e4 — __ZN4FMOD9DSPFilter4readEPPfPiPj16FMOD_SPEAKERMODEij
// type: int __fastcall(int, float **, int *, unsigned int *, int, int, int)
pub fn stub_b30e4() -> ! {
    todo!("0xb30e4 __ZN4FMOD9DSPFilter4readEPPfPiPj16FMOD_SPEAKERMODEij")
}

#[doc(alias = "FMOD::DSPFilter::stopBuffering(void)")]
#[doc(alias = "__ZN4FMOD9DSPFilter13stopBufferingEv")]
// 0xb37c0 — __ZN4FMOD9DSPFilter13stopBufferingEv
// type: int __fastcall(FMOD::DSPFilter *this)
pub fn stub_b37c0() -> ! {
    todo!("0xb37c0 __ZN4FMOD9DSPFilter13stopBufferingEv")
}

#[doc(alias = "FMOD::DSPFilter::startBuffering(void)")]
#[doc(alias = "__ZN4FMOD9DSPFilter14startBufferingEv")]
// 0xb3810 — __ZN4FMOD9DSPFilter14startBufferingEv
// type: int __fastcall(FMOD::DSPFilter *this)
pub fn stub_b3810() -> ! {
    todo!("0xb3810 __ZN4FMOD9DSPFilter14startBufferingEv")
}

#[doc(alias = "FMOD::DSPFlange::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20setParameterInternalEif")]
// 0xb38a8 — __ZN4FMOD9DSPFlange20setParameterInternalEif
// type: int __fastcall(FMOD::DSPFlange *this, int, float)
pub fn stub_b38a8() -> ! {
    todo!("0xb38a8 __ZN4FMOD9DSPFlange20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPFlange::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb38f4 — __ZN4FMOD9DSPFlange20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPFlange *, int, float)
pub fn stub_b38f4() -> ! {
    todo!("0xb38f4 __ZN4FMOD9DSPFlange20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPFlange::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb3900 — __ZN4FMOD9DSPFlange17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPFlange *this, FMOD::MemoryTracker *)
pub fn stub_b3900() -> ! {
    todo!("0xb3900 __ZN4FMOD9DSPFlange17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPFlange::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb3934 — __ZN4FMOD9DSPFlange21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPFlange *this, FMOD::MemoryTracker *)
pub fn stub_b3934() -> ! {
    todo!("0xb3934 __ZN4FMOD9DSPFlange21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPFlange::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20getParameterInternalEiPfPc")]
// 0xb398c — __ZN4FMOD9DSPFlange20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPFlange *this, int, float *, char *)
pub fn stub_b398c() -> ! {
    todo!("0xb398c __ZN4FMOD9DSPFlange20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPFlange::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb3a84 — __ZN4FMOD9DSPFlange20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPFlange *, int, float *, char *)
pub fn stub_b3a84() -> ! {
    todo!("0xb3a84 __ZN4FMOD9DSPFlange20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

