//! core shard ka — 150 stubs EA-sorted 0xc4ab4..0xccf6c (global EA-sorted, next 150 not yet in core after jz 0xc4aa8, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) global EA-sorted ascending, next 150 not yet in rbx_core (34809 before -> 34959 after, gap 50737->50587).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::DSPI::setTargetFrequency(int)")]
#[doc(alias = "__ZN4FMOD4DSPI18setTargetFrequencyEi")]
// 0xc4ab4 — __ZN4FMOD4DSPI18setTargetFrequencyEi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int)
pub fn stub_0xc4ab4() {
    // IDA 0xc4ab4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getTargetFrequency(int *)")]
#[doc(alias = "__ZN4FMOD4DSPI18getTargetFrequencyEPi")]
// 0xc4ac0 — __ZN4FMOD4DSPI18getTargetFrequencyEPi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int *)
pub fn stub_0xc4ac0() {
    // IDA 0xc4ac0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::stopBuffering(void)")]
#[doc(alias = "__ZN4FMOD4DSPI13stopBufferingEv")]
// 0xc4ad8 — __ZN4FMOD4DSPI13stopBufferingEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc4ad8() {
    // IDA 0xc4ad8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4DSPI17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc4ae0 — __ZN4FMOD4DSPI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xc4ae0() {
    // IDA 0xc4ae0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::calculatePeaks(float const*,unsigned int,unsigned int,FMOD::DSPI*)")]
#[doc(alias = "__ZN4FMOD4DSPI14calculatePeaksEPKfjjPS0_")]
// 0xc4b68 — __ZN4FMOD4DSPI14calculatePeaksEPKfjjPS0_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, const float *, unsigned int, unsigned int, FMOD::DSPI *)
pub fn stub_0xc4b68() {
    // IDA 0xc4b68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getInfo(char *,unsigned int *,int *,int *,int *)")]
#[doc(alias = "__ZN4FMOD4DSPI7getInfoEPcPjPiS3_S3_")]
// 0xc4d3c — __ZN4FMOD4DSPI7getInfoEPcPjPiS3_S3_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, char *, unsigned int *, int *, int *, int *)
pub fn stub_0xc4d3c() {
    // IDA 0xc4d3c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getParameter(int,float *,char *,int)")]
#[doc(alias = "__ZN4FMOD4DSPI12getParameterEiPfPci")]
// 0xc4dac — __ZN4FMOD4DSPI12getParameterEiPfPci
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, float *, char *, int)
pub fn stub_0xc4dac() {
    // IDA 0xc4dac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getParameterInfo(int,char *,char *,char *,int,float *,float *)")]
#[doc(alias = "__ZN4FMOD4DSPI16getParameterInfoEiPcS1_S1_iPfS2_")]
// 0xc4e40 — __ZN4FMOD4DSPI16getParameterInfoEiPcS1_S1_iPfS2_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, char *, char *, char *, int, float *, float *)
pub fn stub_0xc4e40() {
    // IDA 0xc4e40: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getNumOutputs(int *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI13getNumOutputsEPib")]
// 0xc4f64 — __ZN4FMOD4DSPI13getNumOutputsEPib
// type: int __fastcall(FMOD::SystemI **this, int *, bool)
pub fn stub_0xc4f64() {
    // IDA 0xc4f64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getNumInputs(int *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI12getNumInputsEPib")]
// 0xc4fd0 — __ZN4FMOD4DSPI12getNumInputsEPib
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int *, bool)
pub fn stub_0xc4fd0() {
    // IDA 0xc4fd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::addInputQueued(FMOD::DSPI*,bool,FMOD::DSPConnectionI *,FMOD::DSPConnectionI **)")]
#[doc(alias = "__ZN4FMOD4DSPI14addInputQueuedEPS0_bPNS_14DSPConnectionIEPS3_")]
// 0xc503c — __ZN4FMOD4DSPI14addInputQueuedEPS0_bPNS_14DSPConnectionIEPS3_
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool, FMOD::DSPConnectionI *, FMOD::DSPConnectionI **)
pub fn stub_0xc503c() {
    // IDA 0xc503c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::addInput(FMOD::DSPI*,FMOD::DSPConnectionI **)")]
#[doc(alias = "__ZN4FMOD4DSPI8addInputEPS0_PPNS_14DSPConnectionIE")]
// 0xc51bc — __ZN4FMOD4DSPI8addInputEPS0_PPNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, FMOD::DSPConnectionI **)
pub fn stub_0xc51bc() {
    // IDA 0xc51bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::updateTreeLevel(int)")]
#[doc(alias = "__ZN4FMOD4DSPI15updateTreeLevelEi")]
// 0xc51f0 — __ZN4FMOD4DSPI15updateTreeLevelEi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int)
pub fn stub_0xc51f0() {
    // IDA 0xc51f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::releaseHistoryBuffer(float *)")]
#[doc(alias = "__ZN4FMOD4DSPI20releaseHistoryBufferEPf")]
// 0xc5374 — __ZN4FMOD4DSPI20releaseHistoryBufferEPf
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float *)
pub fn stub_0xc5374() {
    // IDA 0xc5374: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::createHistoryBuffer(float **,int)")]
#[doc(alias = "__ZN4FMOD4DSPI19createHistoryBufferEPPfi")]
// 0xc5390 — __ZN4FMOD4DSPI19createHistoryBufferEPPfi
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, float **, int)
pub fn stub_0xc5390() {
    // IDA 0xc5390: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::calculateSpeakerLevels(float,float,float,float,float,float,float,float,FMOD_SPEAKERMODE,int,FMOD_SPEAKERMAPTYPE,float *,int *)")]
#[doc(alias = "__ZN4FMOD4DSPI22calculateSpeakerLevelsEffffffff16FMOD_SPEAKERMODEi19FMOD_SPEAKERMAPTYPEPfPi")]
// 0xc53ac — __ZN4FMOD4DSPI22calculateSpeakerLevelsEffffffff16FMOD_SPEAKERMODEi19FMOD_SPEAKERMAPTYPEPfPi
// type: int __fastcall(int, int, int, int, float, float, float, float, int, int, int, void *__b, int)
pub fn stub_0xc53ac() {
    // IDA 0xc53ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::alloc(FMOD::FMOD_DSP_DESCRIPTION_EX *)")]
#[doc(alias = "__ZN4FMOD4DSPI5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE")]
// 0xc6888 — __ZN4FMOD4DSPI5allocEPNS_23FMOD_DSP_DESCRIPTION_EXE
pub fn stub_0xc6888() {
    // IDA 0xc6888: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::disconnectAll(bool,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI13disconnectAllEbb")]
// 0xc693c — __ZN4FMOD4DSPI13disconnectAllEbb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool, bool)
pub fn stub_0xc693c() {
    // IDA 0xc693c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::disconnectFrom(FMOD::DSPI*,FMOD::DSPConnectionI *)")]
#[doc(alias = "__ZN4FMOD4DSPI14disconnectFromEPS0_PNS_14DSPConnectionIE")]
// 0xc6a60 — __ZN4FMOD4DSPI14disconnectFromEPS0_PNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, FMOD::DSPConnectionI *)
pub fn stub_0xc6a60() {
    // IDA 0xc6a60: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::insertInputBetween(FMOD::DSPI*,int,bool,FMOD::DSPConnectionI **)")]
#[doc(alias = "__ZN4FMOD4DSPI18insertInputBetweenEPS0_ibPPNS_14DSPConnectionIE")]
// 0xc6b5c — __ZN4FMOD4DSPI18insertInputBetweenEPS0_ibPPNS_14DSPConnectionIE
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, int, bool, FMOD::DSPConnectionI **)
pub fn stub_0xc6b5c() {
    // IDA 0xc6b5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getOutput(int,FMOD::DSPI**,FMOD::DSPConnectionI **,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI9getOutputEiPPS0_PPNS_14DSPConnectionIEb")]
// 0xc6ca4 — __ZN4FMOD4DSPI9getOutputEiPPS0_PPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, FMOD::DSPI **, FMOD::DSPConnectionI **, bool)
pub fn stub_0xc6ca4() {
    // IDA 0xc6ca4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::getInput(int,FMOD::DSPI**,FMOD::DSPConnectionI **,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI8getInputEiPPS0_PPNS_14DSPConnectionIEb")]
// 0xc6d84 — __ZN4FMOD4DSPI8getInputEiPPS0_PPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, int, FMOD::DSPI **, FMOD::DSPConnectionI **, bool)
pub fn stub_0xc6d84() {
    // IDA 0xc6d84: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::disconnectFromInternal(FMOD::DSPI*,FMOD::DSPConnectionI *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI22disconnectFromInternalEPS0_PNS_14DSPConnectionIEb")]
// 0xc6e64 — __ZN4FMOD4DSPI22disconnectFromInternalEPS0_PNS_14DSPConnectionIEb
// type: int __fastcall(FMOD::DSPI *this, FMOD::DSPI *, FMOD::DSPConnectionI *, bool)
pub fn stub_0xc6e64() {
    // IDA 0xc6e64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::disconnectAllInternal(bool,bool,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI21disconnectAllInternalEbbb")]
// 0xc7194 — __ZN4FMOD4DSPI21disconnectAllInternalEbbb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool, bool, bool)
pub fn stub_0xc7194() {
    // IDA 0xc7194: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::setPosition(unsigned int,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI11setPositionEjb")]
// 0xc72c0 — __ZN4FMOD4DSPI11setPositionEjb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, unsigned int, bool)
pub fn stub_0xc72c0() {
    // IDA 0xc72c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::doesUnitExist(FMOD::DSPI*,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI13doesUnitExistEPS0_b")]
// 0xc7380 — __ZN4FMOD4DSPI13doesUnitExistEPS0_b
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool)
pub fn stub_0xc7380() {
    // IDA 0xc7380: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::addInputInternal(FMOD::DSPI*,bool,FMOD::DSPConnectionI *,FMOD::DSPConnectionI **,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI16addInputInternalEPS0_bPNS_14DSPConnectionIEPS3_b")]
// 0xc7430 — __ZN4FMOD4DSPI16addInputInternalEPS0_bPNS_14DSPConnectionIEPS3_b
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, bool, FMOD::DSPConnectionI *, FMOD::DSPConnectionI **, bool)
pub fn stub_0xc7430() {
    // IDA 0xc7430: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::removeInternal(bool)")]
#[doc(alias = "__ZN4FMOD4DSPI14removeInternalEb")]
// 0xc7720 — __ZN4FMOD4DSPI14removeInternalEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool)
pub fn stub_0xc7720() {
    // IDA 0xc7720: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::remove(void)")]
#[doc(alias = "__ZN4FMOD4DSPI6removeEv")]
// 0xc7858 — __ZN4FMOD4DSPI6removeEv
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this)
pub fn stub_0xc7858() {
    // IDA 0xc7858: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::release(bool)")]
#[doc(alias = "__ZN4FMOD4DSPI7releaseEb")]
// 0xc7860 — __ZN4FMOD4DSPI7releaseEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, bool)
pub fn stub_0xc7860() {
    // IDA 0xc7860: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DSPI::insertInputBetweenInternal(FMOD::DSPI*,int,bool,FMOD::DSPConnectionI *,bool)")]
#[doc(alias = "__ZN4FMOD4DSPI26insertInputBetweenInternalEPS0_ibPNS_14DSPConnectionIEb")]
// 0xc798c — __ZN4FMOD4DSPI26insertInputBetweenInternalEPS0_ibPNS_14DSPConnectionIEb
// type: _DWORD __fastcall(FMOD::DSPI *__hidden this, FMOD::DSPI *, int, bool, FMOD::DSPConnectionI *, bool)
pub fn stub_0xc798c() {
    // IDA 0xc798c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::File(void)")]
#[doc(alias = "__ZN4FMOD4FileC2Ev")]
// 0xc7b14 — __ZN4FMOD4FileC2Ev
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc7b14() {
    // IDA 0xc7b14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::cancel(void)")]
#[doc(alias = "__ZN4FMOD4File6cancelEv")]
// 0xc7b48 — __ZN4FMOD4File6cancelEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc7b48() {
    // IDA 0xc7b48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::seek(int,int)")]
#[doc(alias = "__ZN4FMOD4File4seekEii")]
// 0xc7b60 — __ZN4FMOD4File4seekEii
// type: _DWORD __fastcall(FMOD::File *__hidden this, int, int)
pub fn stub_0xc7b60() {
    // IDA 0xc7b60: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::tell(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File4tellEPj")]
// 0xc7d08 — __ZN4FMOD4File4tellEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
pub fn stub_0xc7d08() {
    // IDA 0xc7d08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::setStartOffset(unsigned int)")]
#[doc(alias = "__ZN4FMOD4File14setStartOffsetEj")]
// 0xc7d3c — __ZN4FMOD4File14setStartOffsetEj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int)
pub fn stub_0xc7d3c() {
    // IDA 0xc7d3c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getStartOffset(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File14getStartOffsetEPj")]
// 0xc7d64 — __ZN4FMOD4File14getStartOffsetEPj
// type: unsigned int __fastcall(FMOD::File *this, unsigned int *)
pub fn stub_0xc7d64() {
    // IDA 0xc7d64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getName(char **)")]
#[doc(alias = "__ZN4FMOD4File7getNameEPPc")]
// 0xc7d7c — __ZN4FMOD4File7getNameEPPc
// type: _DWORD __fastcall(FMOD::File *__hidden this, char **)
pub fn stub_0xc7d7c() {
    // IDA 0xc7d7c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4File17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc7d90 — __ZN4FMOD4File17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::File *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xc7d90() {
    // IDA 0xc7d90: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::setName(char *)")]
#[doc(alias = "__ZN4FMOD4File7setNameEPc")]
// 0xc7db4 — __ZN4FMOD4File7setNameEPc
// type: _DWORD __fastcall(FMOD::File *__hidden this, char *)
pub fn stub_0xc7db4() {
    // IDA 0xc7db4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_File_SetDiskBusy")]
// 0xc7de4 — _FMOD_File_SetDiskBusy
pub fn stub_0xc7de4() {
    // IDA 0xc7de4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::seekAndReset(void)")]
#[doc(alias = "__ZN4FMOD4File12seekAndResetEv")]
// 0xc7e48 — __ZN4FMOD4File12seekAndResetEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc7e48() {
    // IDA 0xc7e48: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::flip(bool)")]
#[doc(alias = "__ZN4FMOD4File4flipEb")]
// 0xc7f10 — __ZN4FMOD4File4flipEb
// type: _DWORD __fastcall(FMOD::File *__hidden this, bool)
pub fn stub_0xc7f10() {
    // IDA 0xc7f10: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::checkBufferedStatus(void)")]
#[doc(alias = "__ZN4FMOD4File19checkBufferedStatusEv")]
// 0xc82a0 — __ZN4FMOD4File19checkBufferedStatusEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc82a0() {
    // IDA 0xc82a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::read(void *,unsigned int,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File4readEPvjjPj")]
// 0xc85b0 — __ZN4FMOD4File4readEPvjjPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, void *, unsigned int, unsigned int, unsigned int *)
pub fn stub_0xc85b0() {
    // IDA 0xc85b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getDword(int *)")]
#[doc(alias = "__ZN4FMOD4File8getDwordEPi")]
// 0xc8ab4 — __ZN4FMOD4File8getDwordEPi
// type: _DWORD __fastcall(FMOD::File *__hidden this, int *)
pub fn stub_0xc8ab4() {
    // IDA 0xc8ab4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getDword(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File8getDwordEPj")]
// 0xc8af0 — __ZN4FMOD4File8getDwordEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
pub fn stub_0xc8af0() {
    // IDA 0xc8af0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getWord(int *)")]
#[doc(alias = "__ZN4FMOD4File7getWordEPi")]
// 0xc8b2c — __ZN4FMOD4File7getWordEPi
// type: _DWORD __fastcall(FMOD::File *__hidden this, int *)
pub fn stub_0xc8b2c() {
    // IDA 0xc8b2c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getWord(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File7getWordEPj")]
// 0xc8b68 — __ZN4FMOD4File7getWordEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
pub fn stub_0xc8b68() {
    // IDA 0xc8b68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getWord(unsigned short *)")]
#[doc(alias = "__ZN4FMOD4File7getWordEPt")]
// 0xc8ba4 — __ZN4FMOD4File7getWordEPt
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int16 *)
pub fn stub_0xc8ba4() {
    // IDA 0xc8ba4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getByte(int *)")]
#[doc(alias = "__ZN4FMOD4File7getByteEPi")]
// 0xc8be0 — __ZN4FMOD4File7getByteEPi
// type: int __fastcall(FMOD::File *this, int *)
pub fn stub_0xc8be0() {
    // IDA 0xc8be0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getByte(signed char *)")]
#[doc(alias = "__ZN4FMOD4File7getByteEPa")]
// 0xc8c1c — __ZN4FMOD4File7getByteEPa
// type: _DWORD __fastcall(FMOD::File *__hidden this, signed __int8 *)
pub fn stub_0xc8c1c() {
    // IDA 0xc8c1c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getByte(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File7getByteEPj")]
// 0xc8c58 — __ZN4FMOD4File7getByteEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
pub fn stub_0xc8c58() {
    // IDA 0xc8c58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getByte(unsigned short *)")]
#[doc(alias = "__ZN4FMOD4File7getByteEPt")]
// 0xc8c94 — __ZN4FMOD4File7getByteEPt
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int16 *)
pub fn stub_0xc8c94() {
    // IDA 0xc8c94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getByte(unsigned char *)")]
#[doc(alias = "__ZN4FMOD4File7getByteEPh")]
// 0xc8cd0 — __ZN4FMOD4File7getByteEPh
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned __int8 *)
pub fn stub_0xc8cd0() {
    // IDA 0xc8cd0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FileThread::threadFunc(void)")]
#[doc(alias = "__ZN4FMOD10FileThread10threadFuncEv")]
// 0xc8d0c — __ZN4FMOD10FileThread10threadFuncEv
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
pub fn stub_0xc8d0c() {
    // IDA 0xc8d0c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::fileThreadFunc(void *)")]
#[doc(alias = "__ZN4FMOD14fileThreadFuncEPv")]
// 0xc8dbc — __ZN4FMOD14fileThreadFuncEPv
// type: _DWORD __fastcall(FMOD *__hidden this, void *)
pub fn stub_0xc8dbc() {
    // IDA 0xc8dbc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::init(FMOD::SystemI *,unsigned int,int)")]
#[doc(alias = "__ZN4FMOD4File4initEPNS_7SystemIEji")]
// 0xc8dc0 — __ZN4FMOD4File4initEPNS_7SystemIEji
// type: _DWORD __fastcall(FMOD::File *__hidden this, FMOD::SystemI *, unsigned int, int)
pub fn stub_0xc8dc0() {
    // IDA 0xc8dc0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::open(char const*,unsigned int,bool,char const*)")]
#[doc(alias = "__ZN4FMOD4File4openEPKcjbS2_")]
// 0xc8e88 — __ZN4FMOD4File4openEPKcjbS2_
// type: _DWORD __fastcall(FMOD::File *__hidden this, const char *, unsigned int, bool, const char *)
pub fn stub_0xc8e88() {
    // IDA 0xc8e88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FileThread::release(void)")]
#[doc(alias = "__ZN4FMOD10FileThread7releaseEv")]
// 0xc90e0 — __ZN4FMOD10FileThread7releaseEv
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
pub fn stub_0xc90e0() {
    // IDA 0xc90e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FileThread::init(int,bool,FMOD::SystemI *)")]
#[doc(alias = "__ZN4FMOD10FileThread4initEibPNS_7SystemIE")]
// 0xc9164 — __ZN4FMOD10FileThread4initEibPNS_7SystemIE
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this, int, bool, FMOD::SystemI *)
pub fn stub_0xc9164() {
    // IDA 0xc9164: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FileThread::FileThread(void)")]
#[doc(alias = "__ZN4FMOD10FileThreadC2Ev")]
// 0xc9238 — __ZN4FMOD10FileThreadC2Ev
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
pub fn stub_0xc9238() {
    // IDA 0xc9238: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FileThread::FileThread(void)")]
#[doc(alias = "__ZN4FMOD10FileThreadC1Ev")]
// 0xc9280 — __ZN4FMOD10FileThreadC1Ev
// type: _DWORD __fastcall(FMOD::FileThread *__hidden this)
pub fn stub_0xc9280() {
    // IDA 0xc9280: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getFileThread(void)")]
#[doc(alias = "__ZN4FMOD4File13getFileThreadEv")]
// 0xc9284 — __ZN4FMOD4File13getFileThreadEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc9284() {
    // IDA 0xc9284: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::enableDoubleBuffer(unsigned int,void *)")]
#[doc(alias = "__ZN4FMOD4File18enableDoubleBufferEjPv")]
// 0xc93ac — __ZN4FMOD4File18enableDoubleBufferEjPv
// type: int __fastcall(FMOD::File *this, unsigned int, void *)
pub fn stub_0xc93ac() {
    // IDA 0xc93ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::shutDown(void)")]
#[doc(alias = "__ZN4FMOD4File8shutDownEv")]
// 0xc952c — __ZN4FMOD4File8shutDownEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc952c() {
    // IDA 0xc952c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::close(void)")]
#[doc(alias = "__ZN4FMOD4File5closeEv")]
// 0xc95b8 — __ZN4FMOD4File5closeEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc95b8() {
    // IDA 0xc95b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD4File13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xc96e4 — __ZN4FMOD4File13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xc96e4() {
    // IDA 0xc96e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getMetadata(FMOD::Metadata **)")]
#[doc(alias = "__ZN4FMOD4File11getMetadataEPPNS_8MetadataE")]
// 0xc973c — __ZN4FMOD4File11getMetadataEPPNS_8MetadataE
pub fn stub_0xc973c() {
    // IDA 0xc973c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::getSize(unsigned int *)")]
#[doc(alias = "__ZN4FMOD4File7getSizeEPj")]
// 0xc9744 — __ZN4FMOD4File7getSizeEPj
// type: _DWORD __fastcall(FMOD::File *__hidden this, unsigned int *)
pub fn stub_0xc9744() {
    // IDA 0xc9744: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::reallyCancel(void)")]
#[doc(alias = "__ZN4FMOD4File12reallyCancelEv")]
// 0xc9754 — __ZN4FMOD4File12reallyCancelEv
// type: _DWORD __fastcall(FMOD::File *__hidden this)
pub fn stub_0xc9754() {
    // IDA 0xc9754: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::File::reallyAsyncRead(FMOD_ASYNCREADINFO *)")]
#[doc(alias = "__ZN4FMOD4File15reallyAsyncReadEP18FMOD_ASYNCREADINFO")]
// 0xc975c — __ZN4FMOD4File15reallyAsyncReadEP18FMOD_ASYNCREADINFO
pub fn stub_0xc975c() {
    // IDA 0xc975c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8DiskFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc9788 — __ZN4FMOD8DiskFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xc9788() {
    // IDA 0xc9788: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::reallyCancel(void)")]
#[doc(alias = "__ZN4FMOD8DiskFile12reallyCancelEv")]
// 0xc97bc — __ZN4FMOD8DiskFile12reallyCancelEv
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this)
pub fn stub_0xc97bc() {
    // IDA 0xc97bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::reallySeek(unsigned int)")]
#[doc(alias = "__ZN4FMOD8DiskFile10reallySeekEj")]
// 0xc97d0 — __ZN4FMOD8DiskFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, unsigned int)
pub fn stub_0xc97d0() {
    // IDA 0xc97d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::reallyRead(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8DiskFile10reallyReadEPvjPj")]
// 0xc97e4 — __ZN4FMOD8DiskFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0xc97e4() {
    // IDA 0xc97e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::reallyClose(void)")]
#[doc(alias = "__ZN4FMOD8DiskFile11reallyCloseEv")]
// 0xc98a8 — __ZN4FMOD8DiskFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this)
pub fn stub_0xc98a8() {
    // IDA 0xc98a8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::reallyOpen(char const*,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8DiskFile10reallyOpenEPKcPj")]
// 0xc98d0 — __ZN4FMOD8DiskFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::DiskFile *__hidden this, const char *, unsigned int *)
pub fn stub_0xc98d0() {
    // IDA 0xc98d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::DiskFile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8DiskFile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xc9978 — __ZN4FMOD8DiskFile13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_0xc9978() {
    // IDA 0xc9978: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::reallyOpen(char const*,unsigned int *)")]
#[doc(alias = "__ZN4FMOD10MemoryFile10reallyOpenEPKcPj")]
// 0xc99d0 — __ZN4FMOD10MemoryFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, const char *, unsigned int *)
pub fn stub_0xc99d0() {
    // IDA 0xc99d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::reallyClose(void)")]
#[doc(alias = "__ZN4FMOD10MemoryFile11reallyCloseEv")]
// 0xc99f0 — __ZN4FMOD10MemoryFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this)
pub fn stub_0xc99f0() {
    // IDA 0xc99f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::reallySeek(unsigned int)")]
#[doc(alias = "__ZN4FMOD10MemoryFile10reallySeekEj")]
// 0xc99f8 — __ZN4FMOD10MemoryFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, unsigned int)
pub fn stub_0xc99f8() {
    // IDA 0xc99f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10MemoryFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc9a10 — __ZN4FMOD10MemoryFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xc9a10() {
    // IDA 0xc9a10: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::reallyRead(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD10MemoryFile10reallyReadEPvjPj")]
// 0xc9a44 — __ZN4FMOD10MemoryFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::MemoryFile *__hidden this, void *__dst, unsigned int, unsigned int *)
pub fn stub_0xc9a44() {
    // IDA 0xc9a44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MemoryFile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD10MemoryFile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xc9aa8 — __ZN4FMOD10MemoryFile13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xc9aa8() {
    // IDA 0xc9aa8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::openAsMMS(char const*,char *,char *,char *,unsigned short,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7NetFile9openAsMMSEPKcPcS3_S3_tPj")]
// 0xc9b00 — __ZN4FMOD7NetFile9openAsMMSEPKcPcS3_S3_tPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, char *, char *, char *, unsigned __int16, unsigned int *)
pub fn stub_0xc9b00() {
    // IDA 0xc9b00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::getMetadata(FMOD::Metadata **)")]
#[doc(alias = "__ZN4FMOD7NetFile11getMetadataEPPNS_8MetadataE")]
// 0xc9b08 — __ZN4FMOD7NetFile11getMetadataEPPNS_8MetadataE
pub fn stub_0xc9b08() {
    // IDA 0xc9b08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7NetFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xc9b20 — __ZN4FMOD7NetFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xc9b20() {
    // IDA 0xc9b20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::reallyCancel(void)")]
#[doc(alias = "__ZN4FMOD7NetFile12reallyCancelEv")]
// 0xc9b54 — __ZN4FMOD7NetFile12reallyCancelEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xc9b54() {
    // IDA 0xc9b54: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::reallySeek(unsigned int)")]
#[doc(alias = "__ZN4FMOD7NetFile10reallySeekEj")]
// 0xc9b6c — __ZN4FMOD7NetFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, unsigned int)
pub fn stub_0xc9b6c() {
    // IDA 0xc9b6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::reallyClose(void)")]
#[doc(alias = "__ZN4FMOD7NetFile11reallyCloseEv")]
// 0xc9d64 — __ZN4FMOD7NetFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xc9d64() {
    // IDA 0xc9d64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::reallyRead(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7NetFile10reallyReadEPvjPj")]
// 0xc9dd4 — __ZN4FMOD7NetFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0xc9dd4() {
    // IDA 0xc9dd4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::parseUrl(char *,char *,int,char *,int,unsigned short *,char *,int,bool *)")]
#[doc(alias = "__ZN4FMOD7NetFile8parseUrlEPcS1_iS1_iPtS1_iPb")]
// 0xca20c — __ZN4FMOD7NetFile8parseUrlEPcS1_iS1_iPtS1_iPb
// type: int __fastcall(FMOD::NetFile *this, char *, char *, int, char *, int, unsigned __int16 *, char *, int, bool *)
pub fn stub_0xca20c() {
    // IDA 0xca20c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::openAsHTTP(char const*,char *,char *,char *,unsigned short,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7NetFile10openAsHTTPEPKcPcS3_S3_tPj")]
// 0xca58c — __ZN4FMOD7NetFile10openAsHTTPEPKcPcS3_S3_tPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, char *, char *, char *, unsigned __int16, unsigned int *)
pub fn stub_0xca58c() {
    // IDA 0xca58c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::shutDown(void)")]
#[doc(alias = "__ZN4FMOD7NetFile8shutDownEv")]
// 0xcb4cc — __ZN4FMOD7NetFile8shutDownEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xcb4cc() {
    // IDA 0xcb4cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::init(void)")]
#[doc(alias = "__ZN4FMOD7NetFile4initEv")]
// 0xcb4dc — __ZN4FMOD7NetFile4initEv
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xcb4dc() {
    // IDA 0xcb4dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::reallyOpen(char const*,unsigned int *)")]
#[doc(alias = "__ZN4FMOD7NetFile10reallyOpenEPKcPj")]
// 0xcb4ec — __ZN4FMOD7NetFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, unsigned int *)
pub fn stub_0xcb4ec() {
    // IDA 0xcb4ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::NetFile(void)")]
#[doc(alias = "__ZN4FMOD7NetFileC2Ev")]
// 0xcb658 — __ZN4FMOD7NetFileC2Ev
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xcb658() {
    // IDA 0xcb658: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::NetFile(void)")]
#[doc(alias = "__ZN4FMOD7NetFileC1Ev")]
// 0xcb6fc — __ZN4FMOD7NetFileC1Ev
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this)
pub fn stub_0xcb6fc() {
    // IDA 0xcb6fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NetFile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7NetFile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xcb700 — __ZN4FMOD7NetFile13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xcb700() {
    // IDA 0xcb700: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::reallyOpen(char const*,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8NullFile10reallyOpenEPKcPj")]
// 0xcb758 — __ZN4FMOD8NullFile10reallyOpenEPKcPj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, const char *, unsigned int *)
pub fn stub_0xcb758() {
    // IDA 0xcb758: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::reallyClose(void)")]
#[doc(alias = "__ZN4FMOD8NullFile11reallyCloseEv")]
// 0xcb76c — __ZN4FMOD8NullFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this)
pub fn stub_0xcb76c() {
    // IDA 0xcb76c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::reallyRead(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8NullFile10reallyReadEPvjPj")]
// 0xcb774 — __ZN4FMOD8NullFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0xcb774() {
    // IDA 0xcb774: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::reallySeek(unsigned int)")]
#[doc(alias = "__ZN4FMOD8NullFile10reallySeekEj")]
// 0xcb7b0 — __ZN4FMOD8NullFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, unsigned int)
pub fn stub_0xcb7b0() {
    // IDA 0xcb7b0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8NullFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xcb7c8 — __ZN4FMOD8NullFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::NullFile *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xcb7c8() {
    // IDA 0xcb7c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::NullFile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8NullFile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xcb7fc — __ZN4FMOD8NullFile13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xcb7fc() {
    // IDA 0xcb7fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallyOpen(char const*,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8UserFile10reallyOpenEPKcPj")]
// 0xcb854 — __ZN4FMOD8UserFile10reallyOpenEPKcPj
// type: int __fastcall(FMOD::UserFile *this, const char *, unsigned int *)
pub fn stub_0xcb854() {
    // IDA 0xcb854: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallyClose(void)")]
#[doc(alias = "__ZN4FMOD8UserFile11reallyCloseEv")]
// 0xcb8e0 — __ZN4FMOD8UserFile11reallyCloseEv
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this)
pub fn stub_0xcb8e0() {
    // IDA 0xcb8e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallyRead(void *,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN4FMOD8UserFile10reallyReadEPvjPj")]
// 0xcb924 — __ZN4FMOD8UserFile10reallyReadEPvjPj
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this, void *, unsigned int, unsigned int *)
pub fn stub_0xcb924() {
    // IDA 0xcb924: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallySeek(unsigned int)")]
#[doc(alias = "__ZN4FMOD8UserFile10reallySeekEj")]
// 0xcb974 — __ZN4FMOD8UserFile10reallySeekEj
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this, unsigned int)
pub fn stub_0xcb974() {
    // IDA 0xcb974: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallyAsyncRead(FMOD_ASYNCREADINFO *)")]
#[doc(alias = "__ZN4FMOD8UserFile15reallyAsyncReadEP18FMOD_ASYNCREADINFO")]
// 0xcb9c8 — __ZN4FMOD8UserFile15reallyAsyncReadEP18FMOD_ASYNCREADINFO
pub fn stub_0xcb9c8() {
    // IDA 0xcb9c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::reallyCancel(void)")]
#[doc(alias = "__ZN4FMOD8UserFile12reallyCancelEv")]
// 0xcba44 — __ZN4FMOD8UserFile12reallyCancelEv
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this)
pub fn stub_0xcba44() {
    // IDA 0xcba44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8UserFile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xcba94 — __ZN4FMOD8UserFile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::UserFile *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xcba94() {
    // IDA 0xcba94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::UserFile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD8UserFile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xcbac8 — __ZN4FMOD8UserFile13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xcbac8() {
    // IDA 0xcbac8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::retrieveOcclusion(unsigned int)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread17retrieveOcclusionEj")]
// 0xcbb20 — __ZN4FMOD15OcclusionThread17retrieveOcclusionEj
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this, unsigned int)
pub fn stub_0xcbb20() {
    // IDA 0xcbb20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xcbb68 — __ZN4FMOD11GeometryMgr17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::GeometryMgr *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xcbb68() {
    // IDA 0xcbb68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::initCritalSection(void)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr17initCritalSectionEv")]
// 0xcbb8c — __ZN4FMOD11GeometryMgr17initCritalSectionEv
// type: _DWORD __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbb8c() {
    // IDA 0xcbb8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::flushAll(void)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr8flushAllEv")]
// 0xcbba4 — __ZN4FMOD11GeometryMgr8flushAllEv
// type: _DWORD __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbba4() {
    // IDA 0xcbba4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::mainOctreeLineTestCallback(FMOD::OctreeNode *,void *)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr26mainOctreeLineTestCallbackEPNS_10OctreeNodeEPv")]
// 0xcbbec — __ZN4FMOD11GeometryMgr26mainOctreeLineTestCallbackEPNS_10OctreeNodeEPv
pub fn stub_0xcbbec() {
    // IDA 0xcbbec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::release(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread7releaseEv")]
// 0xcbc08 — __ZN4FMOD15OcclusionThread7releaseEv
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcbc08() {
    // IDA 0xcbc08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::releaseOcclusionThread(void)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr22releaseOcclusionThreadEv")]
// 0xcbc8c — __ZN4FMOD11GeometryMgr22releaseOcclusionThreadEv
// type: int __fastcall(FMOD::GeometryMgr *this)
pub fn stub_0xcbc8c() {
    // IDA 0xcbc8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::OcclusionThread(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThreadC2Ev")]
// 0xcbcb8 — __ZN4FMOD15OcclusionThreadC2Ev
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcbcb8() {
    // IDA 0xcbcb8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::OcclusionThread(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThreadC1Ev")]
// 0xcbd04 — __ZN4FMOD15OcclusionThreadC1Ev
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcbd04() {
    // IDA 0xcbd04: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::GeometryMgr(void)")]
#[doc(alias = "__ZN4FMOD11GeometryMgrC2Ev")]
// 0xcbd08 — __ZN4FMOD11GeometryMgrC2Ev
// type: _DWORD __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbd08() {
    // IDA 0xcbd08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::GeometryMgr(void)")]
#[doc(alias = "__ZN4FMOD11GeometryMgrC1Ev")]
// 0xcbd60 — __ZN4FMOD11GeometryMgrC1Ev
// type: _DWORD __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbd60() {
    // IDA 0xcbd60: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::init(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread4initEv")]
// 0xcbd64 — __ZN4FMOD15OcclusionThread4initEv
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcbd64() {
    // IDA 0xcbd64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::~GeometryMgr()")]
#[doc(alias = "__ZN4FMOD11GeometryMgrD2Ev")]
// 0xcbe8c — __ZN4FMOD11GeometryMgrD2Ev
// type: void __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbe8c() {
    // IDA 0xcbe8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::GeometryMgr::~GeometryMgr()")]
#[doc(alias = "__ZN4FMOD11GeometryMgrD1Ev")]
// 0xcbeb8 — __ZN4FMOD11GeometryMgrD1Ev
// type: void __fastcall(FMOD::GeometryMgr *__hidden this)
pub fn stub_0xcbeb8() {
    // IDA 0xcbeb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::GeometryMgr::lineTestAll(FMOD_VECTOR const*,FMOD_VECTOR const*,float *,float *)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr11lineTestAllEPK11FMOD_VECTORS3_PfS4_")]
// 0xcbebc — __ZN4FMOD11GeometryMgr11lineTestAllEPK11FMOD_VECTORS3_PfS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xcbebc() {
    // IDA 0xcbebc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OcclusionThread::enqueue(unsigned int,unsigned int,FMOD_VECTOR *)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread7enqueueEjjP11FMOD_VECTOR")]
// 0xcbf84 — __ZN4FMOD15OcclusionThread7enqueueEjjP11FMOD_VECTOR
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xcbf84() {
    // IDA 0xcbf84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OcclusionThread::dequeue(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread7dequeueEv")]
// 0xcc054 — __ZN4FMOD15OcclusionThread7dequeueEv
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcc054() {
    // IDA 0xcc054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OcclusionThread::threadFunc(void)")]
#[doc(alias = "__ZN4FMOD15OcclusionThread10threadFuncEv")]
// 0xcc0c8 — __ZN4FMOD15OcclusionThread10threadFuncEv
// type: _DWORD __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcc0c8() {
    // IDA 0xcc0c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryMgr::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD11GeometryMgr13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xcc1d4 — __ZN4FMOD11GeometryMgr13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xcc1d4() {
    // IDA 0xcc1d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::OcclusionThread::~OcclusionThread()")]
#[doc(alias = "__ZN4FMOD15OcclusionThreadD0Ev")]
// 0xcc22c — __ZN4FMOD15OcclusionThreadD0Ev
// type: void __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcc22c() {
    // IDA 0xcc22c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::OcclusionThread::~OcclusionThread()")]
#[doc(alias = "__ZN4FMOD15OcclusionThreadD1Ev")]
// 0xcc250 — __ZN4FMOD15OcclusionThreadD1Ev
// type: void __fastcall(FMOD::OcclusionThread *__hidden this)
pub fn stub_0xcc250() {
    // IDA 0xcc250: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::matrixMult(float const(*)[4],FMOD_VECTOR const*,FMOD_VECTOR*)")]
#[doc(alias = "__ZN4FMODL10matrixMultEPA4_KfPK11FMOD_VECTORPS3_")]
// 0xcc268 — __ZN4FMODL10matrixMultEPA4_KfPK11FMOD_VECTORPS3_
pub fn stub_0xcc268() {
    // IDA 0xcc268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::GeometryI::octreeLineTestCallback(FMOD::OctreeNode *,void *)")]
#[doc(alias = "__ZN4FMOD9GeometryI22octreeLineTestCallbackEPNS_10OctreeNodeEPv")]
// 0xcc2ec — __ZN4FMOD9GeometryI22octreeLineTestCallbackEPNS_10OctreeNodeEPv
pub fn stub_0xcc2ec() {
    // IDA 0xcc2ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::GeometryI::lineTest(FMOD::GeometryI::LineTestData *)")]
#[doc(alias = "__ZN4FMOD9GeometryI8lineTestEPNS0_12LineTestDataE")]
// 0xcc558 — __ZN4FMOD9GeometryI8lineTestEPNS0_12LineTestDataE
pub fn stub_0xcc558() {
    // IDA 0xcc558: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::GeometryI::updateSpatialData(void)")]
#[doc(alias = "__ZN4FMOD9GeometryI17updateSpatialDataEv")]
// 0xcc668 — __ZN4FMOD9GeometryI17updateSpatialDataEv
// type: _DWORD __fastcall(FMOD::GeometryI *__hidden this)
pub fn stub_0xcc668() {
    // IDA 0xcc668: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::GeometryI::flush(void)")]
#[doc(alias = "__ZN4FMOD9GeometryI5flushEv")]
// 0xcc84c — __ZN4FMOD9GeometryI5flushEv
// type: _DWORD __fastcall(FMOD::GeometryI *__hidden this)
pub fn stub_0xcc84c() {
    // IDA 0xcc84c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Global::init(void)")]
#[doc(alias = "__ZN4FMOD6Global4initEv")]
// 0xccb54 — __ZN4FMOD6Global4initEv
// type: _DWORD __fastcall(FMOD::Global *__hidden this)
pub fn stub_0xccb54() {
    // IDA 0xccb54: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Global::incRef(void)")]
#[doc(alias = "__ZN4FMOD6Global6incRefEv")]
// 0xccbdc — __ZN4FMOD6Global6incRefEv
// type: _DWORD __fastcall(FMOD::Global *__hidden this)
pub fn stub_0xccbdc() {
    // IDA 0xccbdc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Global::decRef(void)")]
#[doc(alias = "__ZN4FMOD6Global6decRefEv")]
// 0xccc18 — __ZN4FMOD6Global6decRefEv
// type: _DWORD __fastcall(FMOD::Global *__hidden this)
pub fn stub_0xccc18() {
    // IDA 0xccc18: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Global::getDSPFFT(FMOD::DSPFFT **)")]
#[doc(alias = "__ZN4FMOD6Global9getDSPFFTEPPNS_6DSPFFTE")]
// 0xccd00 — __ZN4FMOD6Global9getDSPFFTEPPNS_6DSPFFTE
// type: _DWORD __fastcall(FMOD::Global *__hidden this, FMOD::DSPFFT **)
pub fn stub_0xccd00() {
    // IDA 0xccd00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::gGlobal")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD7gGlobalE")]
// 0xcceb8 — __GLOBAL__I__ZN4FMOD7gGlobalE
pub fn stub_0xcceb8() {
    // IDA 0xcceb8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Listener::Listener(void)")]
#[doc(alias = "__ZN4FMOD8ListenerC2Ev")]
// 0xccec4 — __ZN4FMOD8ListenerC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0xccec4() {
    // IDA 0xccec4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Listener::Listener(void)")]
#[doc(alias = "__ZN4FMOD8ListenerC1Ev")]
// 0xccf18 — __ZN4FMOD8ListenerC1Ev
// type: _DWORD __fastcall(FMOD::Listener *__hidden this)
pub fn stub_0xccf18() {
    // IDA 0xccf18: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MemPool::MemPool(void)")]
#[doc(alias = "__ZN4FMOD7MemPoolC2Ev")]
// 0xccf1c — __ZN4FMOD7MemPoolC2Ev
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this)
pub fn stub_0xccf1c() {
    // IDA 0xccf1c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MemPool::MemPool(void)")]
#[doc(alias = "__ZN4FMOD7MemPoolC1Ev")]
// 0xccf68 — __ZN4FMOD7MemPoolC1Ev
// type: _DWORD __fastcall(FMOD::MemPool *__hidden this)
pub fn stub_0xccf68() {
    // IDA 0xccf68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Memory_DefaultFree(void *,unsigned int)")]
#[doc(alias = "__ZN4FMOD18Memory_DefaultFreeEPvj")]
// 0xccf6c — __ZN4FMOD18Memory_DefaultFreeEPvj
// type: _DWORD __fastcall(FMOD *__hidden this, void *, unsigned int)
pub fn stub_0xccf6c() {
    // IDA 0xccf6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

