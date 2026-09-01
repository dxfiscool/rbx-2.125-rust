//! core shard jy — 100 stubs EA-sorted 0xb3a90..0xb9884 (global EA-sorted, next 100 not yet in core after jx 0xb3a84, rbx_core::SharedPtr not boost).
//! Source: `ida/export.json` (85545 funcs) global EA-sorted ascending, next 100 not yet in rbx_core (34559 before -> 34659 after, gap 50986->50886).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::DSPFlange::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange13resetInternalEv")]
// 0xb3a90 — __ZN4FMOD9DSPFlange13resetInternalEv
// type: int __fastcall(FMOD::DSPFlange *this)
pub fn stub_b3a90() -> ! {
    todo!("0xb3a90 __ZN4FMOD9DSPFlange13resetInternalEv")
}

#[doc(alias = "FMOD::DSPFlange::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb3ac4 — __ZN4FMOD9DSPFlange13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPFlange *)
pub fn stub_b3ac4() -> ! {
    todo!("0xb3ac4 __ZN4FMOD9DSPFlange13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPFlange::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPFlange12readInternalEPfS1_jii")]
// 0xb3ad0 — __ZN4FMOD9DSPFlange12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPFlange *this, float *, float *, unsigned int, int, int)
pub fn stub_b3ad0() -> ! {
    todo!("0xb3ad0 __ZN4FMOD9DSPFlange12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPFlange::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPFlange12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb3ec0 — __ZN4FMOD9DSPFlange12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPFlange *, float *, float *, unsigned int, int, int)
pub fn stub_b3ec0() -> ! {
    todo!("0xb3ec0 __ZN4FMOD9DSPFlange12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPFlange::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange15releaseInternalEv")]
// 0xb3ee8 — __ZN4FMOD9DSPFlange15releaseInternalEv
// type: int __fastcall(FMOD::DSPFlange *this)
pub fn stub_b3ee8() -> ! {
    todo!("0xb3ee8 __ZN4FMOD9DSPFlange15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPFlange::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb3f38 — __ZN4FMOD9DSPFlange15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPFlange *)
pub fn stub_b3f38() -> ! {
    todo!("0xb3f38 __ZN4FMOD9DSPFlange15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPFlange::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange14createInternalEv")]
// 0xb3f44 — __ZN4FMOD9DSPFlange14createInternalEv
// type: int __fastcall(FMOD::DSPFlange *this)
pub fn stub_b3f44() -> ! {
    todo!("0xb3f44 __ZN4FMOD9DSPFlange14createInternalEv")
}

#[doc(alias = "FMOD::DSPFlange::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPFlange14createCallbackEP14FMOD_DSP_STATE")]
// 0xb4138 — __ZN4FMOD9DSPFlange14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPFlange *)
pub fn stub_b4138() -> ! {
    todo!("0xb4138 __ZN4FMOD9DSPFlange14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPFlange::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPFlange16getDescriptionExEv")]
// 0xb4144 — __ZN4FMOD9DSPFlange16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPFlange *this)
pub fn stub_b4144() -> ! {
    todo!("0xb4144 __ZN4FMOD9DSPFlange16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspflange")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspflangeE")]
// 0xb4274 — __GLOBAL__I__ZN4FMOD9dspflangeE
// type: int()
pub fn stub_b4274() -> ! {
    todo!("0xb4274 __GLOBAL__I__ZN4FMOD9dspflangeE")
}

#[doc(alias = "FMOD::DSPHighPass::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass13resetInternalEv")]
// 0xb4280 — __ZN4FMOD11DSPHighPass13resetInternalEv
// type: int __fastcall(FMOD::DSPHighPass *this)
pub fn stub_b4280() -> ! {
    todo!("0xb4280 __ZN4FMOD11DSPHighPass13resetInternalEv")
}

#[doc(alias = "FMOD::DSPHighPass::process(float *,float *,unsigned int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass7processEPfS1_ji")]
// 0xb42c8 — __ZN4FMOD11DSPHighPass7processEPfS1_ji
// type: int __fastcall(FMOD::DSPHighPass *this, float *, float *, unsigned int, int)
pub fn stub_b42c8() -> ! {
    todo!("0xb42c8 __ZN4FMOD11DSPHighPass7processEPfS1_ji")
}

#[doc(alias = "FMOD::DSPHighPass::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20setParameterInternalEif")]
// 0xb4dd0 — __ZN4FMOD11DSPHighPass20setParameterInternalEif
// type: int __fastcall(FMOD::DSPHighPass *this, int, float)
pub fn stub_b4dd0() -> ! {
    todo!("0xb4dd0 __ZN4FMOD11DSPHighPass20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPHighPass::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb4e0c — __ZN4FMOD11DSPHighPass17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int()
pub fn stub_b4e0c() -> ! {
    todo!("0xb4e0c __ZN4FMOD11DSPHighPass17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPHighPass::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb4e14 — __ZN4FMOD11DSPHighPass13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPHighPass *)
pub fn stub_b4e14() -> ! {
    todo!("0xb4e14 __ZN4FMOD11DSPHighPass13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPHighPass::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb4e20 — __ZN4FMOD11DSPHighPass20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPHighPass *, int, float)
pub fn stub_b4e20() -> ! {
    todo!("0xb4e20 __ZN4FMOD11DSPHighPass20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPHighPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb4e2c — __ZN4FMOD11DSPHighPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_b4e2c() -> ! {
    todo!("0xb4e2c __ZN4FMOD11DSPHighPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPHighPass::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20getParameterInternalEiPfPc")]
// 0xb4e84 — __ZN4FMOD11DSPHighPass20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPHighPass *this, int, float *, char *)
pub fn stub_b4e84() -> ! {
    todo!("0xb4e84 __ZN4FMOD11DSPHighPass20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPHighPass::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb4efc — __ZN4FMOD11DSPHighPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPHighPass *, int, float *, char *)
pub fn stub_b4efc() -> ! {
    todo!("0xb4efc __ZN4FMOD11DSPHighPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPHighPass::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass16getDescriptionExEv")]
// 0xb4f08 — __ZN4FMOD11DSPHighPass16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPHighPass *this)
pub fn stub_b4f08() -> ! {
    todo!("0xb4f08 __ZN4FMOD11DSPHighPass16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPHighPass::updateCoefficients(float,float)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass18updateCoefficientsEff")]
// 0xb4fe4 — __ZN4FMOD11DSPHighPass18updateCoefficientsEff
// type: int __fastcall(FMOD::DSPHighPass *this, float32_t, float32_t)
pub fn stub_b4fe4() -> ! {
    todo!("0xb4fe4 __ZN4FMOD11DSPHighPass18updateCoefficientsEff")
}

#[doc(alias = "FMOD::DSPHighPass::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass12readInternalEPfS1_jii")]
// 0xb5098 — __ZN4FMOD11DSPHighPass12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPHighPass *this, float *, float *, unsigned int, int, int)
pub fn stub_b5098() -> ! {
    todo!("0xb5098 __ZN4FMOD11DSPHighPass12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPHighPass::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb52c0 — __ZN4FMOD11DSPHighPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPHighPass *, float *, float *, unsigned int, int, int)
pub fn stub_b52c0() -> ! {
    todo!("0xb52c0 __ZN4FMOD11DSPHighPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPHighPass::createInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass14createInternalEv")]
// 0xb52e8 — __ZN4FMOD11DSPHighPass14createInternalEv
// type: int __fastcall(FMOD::DSPHighPass *this)
pub fn stub_b52e8() -> ! {
    todo!("0xb52e8 __ZN4FMOD11DSPHighPass14createInternalEv")
}

#[doc(alias = "FMOD::DSPHighPass::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPHighPass14createCallbackEP14FMOD_DSP_STATE")]
// 0xb53a4 — __ZN4FMOD11DSPHighPass14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPHighPass *)
pub fn stub_b53a4() -> ! {
    todo!("0xb53a4 __ZN4FMOD11DSPHighPass14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "global constructor keyed toFMOD::dsphighpass")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD11dsphighpassE")]
// 0xb53f4 — __GLOBAL__I__ZN4FMOD11dsphighpassE
// type: int()
pub fn stub_b53f4() -> ! {
    todo!("0xb53f4 __GLOBAL__I__ZN4FMOD11dsphighpassE")
}

#[doc(alias = "FMOD::DSPITEcho::createInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho14createInternalEv")]
// 0xb5400 — __ZN4FMOD9DSPITEcho14createInternalEv
// type: int __fastcall(FMOD::DSPITEcho *this)
pub fn stub_b5400() -> ! {
    todo!("0xb5400 __ZN4FMOD9DSPITEcho14createInternalEv")
}

#[doc(alias = "FMOD::DSPITEcho::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho14createCallbackEP14FMOD_DSP_STATE")]
// 0xb5484 — __ZN4FMOD9DSPITEcho14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPITEcho *)
pub fn stub_b5484() -> ! {
    todo!("0xb5484 __ZN4FMOD9DSPITEcho14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPITEcho::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb5490 — __ZN4FMOD9DSPITEcho17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPITEcho *this, FMOD::MemoryTracker *)
pub fn stub_b5490() -> ! {
    todo!("0xb5490 __ZN4FMOD9DSPITEcho17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPITEcho::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb54dc — __ZN4FMOD9DSPITEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPITEcho *this, FMOD::MemoryTracker *)
pub fn stub_b54dc() -> ! {
    todo!("0xb54dc __ZN4FMOD9DSPITEcho21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPITEcho::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20getParameterInternalEiPfPc")]
// 0xb5534 — __ZN4FMOD9DSPITEcho20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPITEcho *this, int, float *, char *__dst)
pub fn stub_b5534() -> ! {
    todo!("0xb5534 __ZN4FMOD9DSPITEcho20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPITEcho::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb568c — __ZN4FMOD9DSPITEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPITEcho *, int, float *, char *)
pub fn stub_b568c() -> ! {
    todo!("0xb568c __ZN4FMOD9DSPITEcho20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPITEcho::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho15releaseInternalEv")]
// 0xb5698 — __ZN4FMOD9DSPITEcho15releaseInternalEv
// type: int __fastcall(FMOD::DSPITEcho *this)
pub fn stub_b5698() -> ! {
    todo!("0xb5698 __ZN4FMOD9DSPITEcho15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPITEcho::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb5700 — __ZN4FMOD9DSPITEcho15releaseCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPITEcho *)
pub fn stub_b5700() -> ! {
    todo!("0xb5700 __ZN4FMOD9DSPITEcho15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPITEcho::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho13resetInternalEv")]
// 0xb570c — __ZN4FMOD9DSPITEcho13resetInternalEv
// type: int __fastcall(FMOD::DSPITEcho *this)
pub fn stub_b570c() -> ! {
    todo!("0xb570c __ZN4FMOD9DSPITEcho13resetInternalEv")
}

#[doc(alias = "FMOD::DSPITEcho::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb575c — __ZN4FMOD9DSPITEcho13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPITEcho *)
pub fn stub_b575c() -> ! {
    todo!("0xb575c __ZN4FMOD9DSPITEcho13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPITEcho::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20setParameterInternalEif")]
// 0xb5768 — __ZN4FMOD9DSPITEcho20setParameterInternalEif
// type: int __fastcall(FMOD::DSPITEcho *this, int, float)
pub fn stub_b5768() -> ! {
    todo!("0xb5768 __ZN4FMOD9DSPITEcho20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPITEcho::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb5960 — __ZN4FMOD9DSPITEcho20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPITEcho *, int, float)
pub fn stub_b5960() -> ! {
    todo!("0xb5960 __ZN4FMOD9DSPITEcho20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPITEcho::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho12readInternalEPfS1_jii")]
// 0xb596c — __ZN4FMOD9DSPITEcho12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPITEcho *this, float *, float *, unsigned int, int, int)
pub fn stub_b596c() -> ! {
    todo!("0xb596c __ZN4FMOD9DSPITEcho12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPITEcho::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb5d44 — __ZN4FMOD9DSPITEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPITEcho *, float *, float *, unsigned int, int, int)
pub fn stub_b5d44() -> ! {
    todo!("0xb5d44 __ZN4FMOD9DSPITEcho12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPITEcho::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD9DSPITEcho16getDescriptionExEv")]
// 0xb5d6c — __ZN4FMOD9DSPITEcho16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPITEcho *this)
pub fn stub_b5d6c() -> ! {
    todo!("0xb5d6c __ZN4FMOD9DSPITEcho16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dspitecho")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD9dspitechoE")]
// 0xb5e9c — __GLOBAL__I__ZN4FMOD9dspitechoE
// type: int()
pub fn stub_b5e9c() -> ! {
    todo!("0xb5e9c __GLOBAL__I__ZN4FMOD9dspitechoE")
}

#[doc(alias = "FMOD::DSPLowPass::bilinear(float,float,float,float,float,float,float *,float,float *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass8bilinearEffffffPffS1_")]
// 0xb5ea8 — __ZN4FMOD10DSPLowPass8bilinearEffffffPffS1_
// type: int __fastcall(FMOD::DSPLowPass *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float *, float32_t, float *)
pub fn stub_b5ea8() -> ! {
    todo!("0xb5ea8 __ZN4FMOD10DSPLowPass8bilinearEffffffPffS1_")
}

#[doc(alias = "FMOD::DSPLowPass::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20setParameterInternalEif")]
// 0xb5fac — __ZN4FMOD10DSPLowPass20setParameterInternalEif
// type: int __fastcall(FMOD::DSPLowPass *this, int, float)
pub fn stub_b5fac() -> ! {
    todo!("0xb5fac __ZN4FMOD10DSPLowPass20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPLowPass::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb5fe4 — __ZN4FMOD10DSPLowPass17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int()
pub fn stub_b5fe4() -> ! {
    todo!("0xb5fe4 __ZN4FMOD10DSPLowPass17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPass::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb5fec — __ZN4FMOD10DSPLowPass20setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPLowPass *, int, float)
pub fn stub_b5fec() -> ! {
    todo!("0xb5fec __ZN4FMOD10DSPLowPass20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPLowPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb5ff8 — __ZN4FMOD10DSPLowPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_b5ff8() -> ! {
    todo!("0xb5ff8 __ZN4FMOD10DSPLowPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPass::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20getParameterInternalEiPfPc")]
// 0xb6050 — __ZN4FMOD10DSPLowPass20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPass *this, int, float *, char *)
pub fn stub_b6050() -> ! {
    todo!("0xb6050 __ZN4FMOD10DSPLowPass20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPass::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb60c8 — __ZN4FMOD10DSPLowPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPLowPass *, int, float *, char *)
pub fn stub_b60c8() -> ! {
    todo!("0xb60c8 __ZN4FMOD10DSPLowPass20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPass::prewarp(float *,float *,float *,float,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7prewarpEPfS1_S1_ff")]
// 0xb60d4 — __ZN4FMOD10DSPLowPass7prewarpEPfS1_S1_ff
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, float *, float32_t, float32_t)
pub fn stub_b60d4() -> ! {
    todo!("0xb60d4 __ZN4FMOD10DSPLowPass7prewarpEPfS1_S1_ff")
}

#[doc(alias = "FMOD::DSPLowPass::szxform(float *,float *,float *,float *,float *,float *,float,float,float *,float *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7szxformEPfS1_S1_S1_S1_S1_ffS1_S1_")]
// 0xb6144 — __ZN4FMOD10DSPLowPass7szxformEPfS1_S1_S1_S1_S1_ffS1_S1_
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, float *, float *, float *, float *, float32_t, float32_t, float *, float *)
pub fn stub_b6144() -> ! {
    todo!("0xb6144 __ZN4FMOD10DSPLowPass7szxformEPfS1_S1_S1_S1_S1_ffS1_S1_")
}

#[doc(alias = "FMOD::DSPLowPass::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass16getDescriptionExEv")]
// 0xb61ec — __ZN4FMOD10DSPLowPass16getDescriptionExEv
// type: void *__fastcall(FMOD::DSPLowPass *this)
pub fn stub_b61ec() -> ! {
    todo!("0xb61ec __ZN4FMOD10DSPLowPass16getDescriptionExEv")
}

#[doc(alias = "FMOD::DSPLowPass::updateState(float,float)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass11updateStateEff")]
// 0xb62b8 — __ZN4FMOD10DSPLowPass11updateStateEff
// type: int __fastcall(FMOD::DSPLowPass *this, float, float32_t)
pub fn stub_b62b8() -> ! {
    todo!("0xb62b8 __ZN4FMOD10DSPLowPass11updateStateEff")
}

#[doc(alias = "FMOD::DSPLowPass::createInternal(void)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass14createInternalEv")]
// 0xb648c — __ZN4FMOD10DSPLowPass14createInternalEv
// type: int __fastcall(FMOD::DSPLowPass *this)
pub fn stub_b648c() -> ! {
    todo!("0xb648c __ZN4FMOD10DSPLowPass14createInternalEv")
}

#[doc(alias = "FMOD::DSPLowPass::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass14createCallbackEP14FMOD_DSP_STATE")]
// 0xb6590 — __ZN4FMOD10DSPLowPass14createCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPLowPass *)
pub fn stub_b6590() -> ! {
    todo!("0xb6590 __ZN4FMOD10DSPLowPass14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPLowPass::process(float *,float *,unsigned int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass7processEPfS1_ji")]
// 0xb659c — __ZN4FMOD10DSPLowPass7processEPfS1_ji
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, unsigned int, int)
pub fn stub_b659c() -> ! {
    todo!("0xb659c __ZN4FMOD10DSPLowPass7processEPfS1_ji")
}

#[doc(alias = "FMOD::DSPLowPass::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass12readInternalEPfS1_jii")]
// 0xb779c — __ZN4FMOD10DSPLowPass12readInternalEPfS1_jii
// type: int __fastcall(FMOD::DSPLowPass *this, float *, float *, unsigned int, int, int)
pub fn stub_b779c() -> ! {
    todo!("0xb779c __ZN4FMOD10DSPLowPass12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPLowPass::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD10DSPLowPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb79c4 — __ZN4FMOD10DSPLowPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii
// type: int __fastcall(FMOD::DSPLowPass *, float *, float *, unsigned int, int, int)
pub fn stub_b79c4() -> ! {
    todo!("0xb79c4 __ZN4FMOD10DSPLowPass12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "global constructor keyed toFMOD::dsplowpass")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD10dsplowpassE")]
// 0xb7a30 — __GLOBAL__I__ZN4FMOD10dsplowpassE
// type: int()
pub fn stub_b7a30() -> ! {
    todo!("0xb7a30 __GLOBAL__I__ZN4FMOD10dsplowpassE")
}

#[doc(alias = "FMOD::DSPLowPass2::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass213resetInternalEv")]
// 0xb7a3c — __ZN4FMOD11DSPLowPass213resetInternalEv
// type: int __fastcall(FMOD::DSPLowPass2 *this)
pub fn stub_b7a3c() -> ! {
    todo!("0xb7a3c __ZN4FMOD11DSPLowPass213resetInternalEv")
}

#[doc(alias = "FMOD::DSPLowPass2::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220setParameterInternalEif")]
// 0xb7a74 — __ZN4FMOD11DSPLowPass220setParameterInternalEif
// type: int __fastcall(FMOD::DSPLowPass2 *this, int, float)
pub fn stub_b7a74() -> ! {
    todo!("0xb7a74 __ZN4FMOD11DSPLowPass220setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPLowPass2::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass217getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb7a9c — __ZN4FMOD11DSPLowPass217getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int()
pub fn stub_b7a9c() -> ! {
    todo!("0xb7a9c __ZN4FMOD11DSPLowPass217getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPass2::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass213resetCallbackEP14FMOD_DSP_STATE")]
// 0xb7aa4 — __ZN4FMOD11DSPLowPass213resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPLowPass2 *)
pub fn stub_b7aa4() -> ! {
    todo!("0xb7aa4 __ZN4FMOD11DSPLowPass213resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPLowPass2::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb7ab0 — __ZN4FMOD11DSPLowPass220setParameterCallbackEP14FMOD_DSP_STATEif
// type: int __fastcall(FMOD::DSPLowPass2 *, int, float)
pub fn stub_b7ab0() -> ! {
    todo!("0xb7ab0 __ZN4FMOD11DSPLowPass220setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPLowPass2::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb7abc — __ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_b7abc() -> ! {
    todo!("0xb7abc __ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPass2::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220getParameterInternalEiPfPc")]
// 0xb7b14 — __ZN4FMOD11DSPLowPass220getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPass2 *this, int, float *, char *)
pub fn stub_b7b14() -> ! {
    todo!("0xb7b14 __ZN4FMOD11DSPLowPass220getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPass2::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass220getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb7b8c — __ZN4FMOD11DSPLowPass220getParameterCallbackEP14FMOD_DSP_STATEiPfPc
// type: int __fastcall(FMOD::DSPLowPass2 *, int, float *, char *)
pub fn stub_b7b8c() -> ! {
    todo!("0xb7b8c __ZN4FMOD11DSPLowPass220getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPass2::updateCoefficients(float,float)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass218updateCoefficientsEff")]
// 0xb7b98 — __ZN4FMOD11DSPLowPass218updateCoefficientsEff
// type: int __fastcall(FMOD::DSPLowPass2 *this, float32_t, float32_t)
pub fn stub_b7b98() -> ! {
    todo!("0xb7b98 __ZN4FMOD11DSPLowPass218updateCoefficientsEff")
}

#[doc(alias = "FMOD::DSPLowPass2::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass212readInternalEPfS1_jii")]
// 0xb7cc8 — __ZN4FMOD11DSPLowPass212readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPLowPass2 *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_b7cc8() -> ! {
    todo!("0xb7cc8 __ZN4FMOD11DSPLowPass212readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPLowPass2::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass212readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb8780 — __ZN4FMOD11DSPLowPass212readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_b8780() -> ! {
    todo!("0xb8780 __ZN4FMOD11DSPLowPass212readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPLowPass2::createInternal(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass214createInternalEv")]
// 0xb87a8 — __ZN4FMOD11DSPLowPass214createInternalEv
// type: int __fastcall(FMOD::DSPLowPass2 *this)
pub fn stub_b87a8() -> ! {
    todo!("0xb87a8 __ZN4FMOD11DSPLowPass214createInternalEv")
}

#[doc(alias = "FMOD::DSPLowPass2::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass214createCallbackEP14FMOD_DSP_STATE")]
// 0xb8840 — __ZN4FMOD11DSPLowPass214createCallbackEP14FMOD_DSP_STATE
pub fn stub_b8840() -> ! {
    todo!("0xb8840 __ZN4FMOD11DSPLowPass214createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPLowPass2::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD11DSPLowPass216getDescriptionExEv")]
// 0xb884c — __ZN4FMOD11DSPLowPass216getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPLowPass2 *__hidden this)
pub fn stub_b884c() -> ! {
    todo!("0xb884c __ZN4FMOD11DSPLowPass216getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dsplowpass2")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD11dsplowpass2E")]
// 0xb896c — __GLOBAL__I__ZN4FMOD11dsplowpass2E
pub fn stub_b896c() -> ! {
    todo!("0xb896c __GLOBAL__I__ZN4FMOD11dsplowpass2E")
}

#[doc(alias = "FMOD::DSPLowPassSimple::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple13resetInternalEv")]
// 0xb8978 — __ZN4FMOD16DSPLowPassSimple13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
pub fn stub_b8978() -> ! {
    todo!("0xb8978 __ZN4FMOD16DSPLowPassSimple13resetInternalEv")
}

#[doc(alias = "FMOD::DSPLowPassSimple::updateCoefficients(float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple18updateCoefficientsEf")]
// 0xb89b0 — __ZN4FMOD16DSPLowPassSimple18updateCoefficientsEf
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, float)
pub fn stub_b89b0() -> ! {
    todo!("0xb89b0 __ZN4FMOD16DSPLowPassSimple18updateCoefficientsEf")
}

#[doc(alias = "FMOD::DSPLowPassSimple::createInternal(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple14createInternalEv")]
// 0xb8a70 — __ZN4FMOD16DSPLowPassSimple14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
pub fn stub_b8a70() -> ! {
    todo!("0xb8a70 __ZN4FMOD16DSPLowPassSimple14createInternalEv")
}

#[doc(alias = "FMOD::DSPLowPassSimple::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20setParameterInternalEif")]
// 0xb8b00 — __ZN4FMOD16DSPLowPassSimple20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, int, float)
pub fn stub_b8b00() -> ! {
    todo!("0xb8b00 __ZN4FMOD16DSPLowPassSimple20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPLowPassSimple::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb8b10 — __ZN4FMOD16DSPLowPassSimple17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_b8b10() -> ! {
    todo!("0xb8b10 __ZN4FMOD16DSPLowPassSimple17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPassSimple::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple14createCallbackEP14FMOD_DSP_STATE")]
// 0xb8b18 — __ZN4FMOD16DSPLowPassSimple14createCallbackEP14FMOD_DSP_STATE
pub fn stub_b8b18() -> ! {
    todo!("0xb8b18 __ZN4FMOD16DSPLowPassSimple14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPLowPassSimple::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb8b24 — __ZN4FMOD16DSPLowPassSimple13resetCallbackEP14FMOD_DSP_STATE
pub fn stub_b8b24() -> ! {
    todo!("0xb8b24 __ZN4FMOD16DSPLowPassSimple13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPLowPassSimple::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb8b30 — __ZN4FMOD16DSPLowPassSimple20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_b8b30() -> ! {
    todo!("0xb8b30 __ZN4FMOD16DSPLowPassSimple20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPLowPassSimple::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb8b3c — __ZN4FMOD16DSPLowPassSimple21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_b8b3c() -> ! {
    todo!("0xb8b3c __ZN4FMOD16DSPLowPassSimple21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPLowPassSimple::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20getParameterInternalEiPfPc")]
// 0xb8b94 — __ZN4FMOD16DSPLowPassSimple20getParameterInternalEiPfPc
// type: int __fastcall(FMOD::DSPLowPassSimple *this, int, float *, char *)
pub fn stub_b8b94() -> ! {
    todo!("0xb8b94 __ZN4FMOD16DSPLowPassSimple20getParameterInternalEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPassSimple::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")]
// 0xb8bd4 — __ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
pub fn stub_b8bd4() -> ! {
    todo!("0xb8bd4 __ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc")
}

#[doc(alias = "FMOD::DSPLowPassSimple::readInternal(float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple12readInternalEPfS1_jii")]
// 0xb8be0 — __ZN4FMOD16DSPLowPassSimple12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this, float *, float *, unsigned int, int, int)
pub fn stub_b8be0() -> ! {
    todo!("0xb8be0 __ZN4FMOD16DSPLowPassSimple12readInternalEPfS1_jii")
}

#[doc(alias = "FMOD::DSPLowPassSimple::readCallback(FMOD_DSP_STATE *,float *,float *,unsigned int,int,int)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
// 0xb958c — __ZN4FMOD16DSPLowPassSimple12readCallbackEP14FMOD_DSP_STATEPfS3_jii
pub fn stub_b958c() -> ! {
    todo!("0xb958c __ZN4FMOD16DSPLowPassSimple12readCallbackEP14FMOD_DSP_STATEPfS3_jii")
}

#[doc(alias = "FMOD::DSPLowPassSimple::getDescriptionEx(void)")]
#[doc(alias = "__ZN4FMOD16DSPLowPassSimple16getDescriptionExEv")]
// 0xb95b4 — __ZN4FMOD16DSPLowPassSimple16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPLowPassSimple *__hidden this)
pub fn stub_b95b4() -> ! {
    todo!("0xb95b4 __ZN4FMOD16DSPLowPassSimple16getDescriptionExEv")
}

#[doc(alias = "global constructor keyed toFMOD::dsplowpass_simple")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD17dsplowpass_simpleE")]
// 0xb96d4 — __GLOBAL__I__ZN4FMOD17dsplowpass_simpleE
pub fn stub_b96d4() -> ! {
    todo!("0xb96d4 __GLOBAL__I__ZN4FMOD17dsplowpass_simpleE")
}

#[doc(alias = "FMOD::DSPNormalize::createInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize14createInternalEv")]
// 0xb96e0 — __ZN4FMOD12DSPNormalize14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
pub fn stub_b96e0() -> ! {
    todo!("0xb96e0 __ZN4FMOD12DSPNormalize14createInternalEv")
}

#[doc(alias = "FMOD::DSPNormalize::releaseInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize15releaseInternalEv")]
// 0xb9770 — __ZN4FMOD12DSPNormalize15releaseInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
pub fn stub_b9770() -> ! {
    todo!("0xb9770 __ZN4FMOD12DSPNormalize15releaseInternalEv")
}

#[doc(alias = "FMOD::DSPNormalize::resetInternal(void)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize13resetInternalEv")]
// 0xb9778 — __ZN4FMOD12DSPNormalize13resetInternalEv
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this)
pub fn stub_b9778() -> ! {
    todo!("0xb9778 __ZN4FMOD12DSPNormalize13resetInternalEv")
}

#[doc(alias = "FMOD::DSPNormalize::setParameterInternal(int,float)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20setParameterInternalEif")]
// 0xb978c — __ZN4FMOD12DSPNormalize20setParameterInternalEif
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, int, float)
pub fn stub_b978c() -> ! {
    todo!("0xb978c __ZN4FMOD12DSPNormalize20setParameterInternalEif")
}

#[doc(alias = "FMOD::DSPNormalize::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xb97f4 — __ZN4FMOD12DSPNormalize17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_b97f4() -> ! {
    todo!("0xb97f4 __ZN4FMOD12DSPNormalize17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPNormalize::createCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize14createCallbackEP14FMOD_DSP_STATE")]
// 0xb97fc — __ZN4FMOD12DSPNormalize14createCallbackEP14FMOD_DSP_STATE
pub fn stub_b97fc() -> ! {
    todo!("0xb97fc __ZN4FMOD12DSPNormalize14createCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPNormalize::releaseCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize15releaseCallbackEP14FMOD_DSP_STATE")]
// 0xb9808 — __ZN4FMOD12DSPNormalize15releaseCallbackEP14FMOD_DSP_STATE
pub fn stub_b9808() -> ! {
    todo!("0xb9808 __ZN4FMOD12DSPNormalize15releaseCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPNormalize::resetCallback(FMOD_DSP_STATE *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize13resetCallbackEP14FMOD_DSP_STATE")]
// 0xb9814 — __ZN4FMOD12DSPNormalize13resetCallbackEP14FMOD_DSP_STATE
// type: int __fastcall(FMOD::DSPNormalize *)
pub fn stub_b9814() -> ! {
    todo!("0xb9814 __ZN4FMOD12DSPNormalize13resetCallbackEP14FMOD_DSP_STATE")
}

#[doc(alias = "FMOD::DSPNormalize::setParameterCallback(FMOD_DSP_STATE *,int,float)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20setParameterCallbackEP14FMOD_DSP_STATEif")]
// 0xb9820 — __ZN4FMOD12DSPNormalize20setParameterCallbackEP14FMOD_DSP_STATEif
pub fn stub_b9820() -> ! {
    todo!("0xb9820 __ZN4FMOD12DSPNormalize20setParameterCallbackEP14FMOD_DSP_STATEif")
}

#[doc(alias = "FMOD::DSPNormalize::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")]
// 0xb982c — __ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
pub fn stub_b982c() -> ! {
    todo!("0xb982c __ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::DSPNormalize::getParameterInternal(int,float *,char *)")]
#[doc(alias = "__ZN4FMOD12DSPNormalize20getParameterInternalEiPfPc")]
// 0xb9884 — __ZN4FMOD12DSPNormalize20getParameterInternalEiPfPc
// type: _DWORD __fastcall(FMOD::DSPNormalize *__hidden this, int, float *, char *)
pub fn stub_b9884() -> ! {
    todo!("0xb9884 __ZN4FMOD12DSPNormalize20getParameterInternalEiPfPc")
}

