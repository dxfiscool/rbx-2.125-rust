//! audio generated_17 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2541 distinct) — 28 missing distinct + 72 filler workspace EA-sorted asc after 0x1dcc28 (skip existing, rbx_core::SharedPtr not boost)
//! Batch: 100 stubs | skeleton batch | range 0x719e0..0x1e1ff0 EA-sorted (28 audio gaps + 72 filler), SharedPtr = rbx_core::SharedPtr
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x719e0 — __ZN4FMOD11ChannelReal13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelReal *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t)
#[doc(alias = "FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_719e0() -> ! {
    todo!("0x719e0 FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x76b3c — __ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_76b3c() -> ! {
    todo!("0x76b3c FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x987e4 — __ZN4FMOD9CodecMPEG16decodeXingHeaderEPhS1_Pj
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned __int8 *, unsigned __int8 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")]
pub fn stub_987e4() -> ! {
    todo!("0x987e4 FMOD::CodecMPEG::decodeXingHeader(unsigned char *,unsigned char *,unsigned int *)")
}

// 0xaf528 — __ZN4FMOD5Codec20defaultGetWaveFormatEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int *, int, void *__dst)
#[doc(alias = "FMOD::Codec::defaultGetWaveFormat(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
pub fn stub_af528() -> ! {
    todo!("0xaf528 FMOD::Codec::defaultGetWaveFormat(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")
}

// 0xb4e2c — __ZN4FMOD11DSPHighPass21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPHighPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_b4e2c() -> ! {
    todo!("0xb4e2c FMOD::DSPHighPass::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")
}

// 0xb7abc — __ZN4FMOD11DSPLowPass221getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::DSPLowPass2::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_b7abc() -> ! {
    todo!("0xb7abc FMOD::DSPLowPass2::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")
}

// 0xb8bd4 — __ZN4FMOD16DSPLowPassSimple20getParameterCallbackEP14FMOD_DSP_STATEiPfPc
#[doc(alias = "FMOD::DSPLowPassSimple::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")]
pub fn stub_b8bd4() -> ! {
    todo!("0xb8bd4 FMOD::DSPLowPassSimple::getParameterCallback(FMOD_DSP_STATE *,int,float *,char *)")
}

// 0xb982c — __ZN4FMOD12DSPNormalize21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
#[doc(alias = "FMOD::DSPNormalize::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_b982c() -> ! {
    todo!("0xb982c FMOD::DSPNormalize::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")
}

// 0xc1c2c — __ZN4FMOD12DSPSfxReverb21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPSfxReverb *this)
#[doc(alias = "FMOD::DSPSfxReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_c1c2c() -> ! {
    todo!("0xc1c2c FMOD::DSPSfxReverb::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")
}

// 0xc3750 — __ZN4FMOD12DSPSoundCard4readEPvPj16FMOD_SPEAKERMODEij
#[doc(alias = "FMOD::DSPSoundCard::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
pub fn stub_c3750() -> ! {
    todo!("0xc3750 FMOD::DSPSoundCard::read(void *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")
}

// 0xc4788 — __ZN4FMOD4DSPI4readEPPfPiPj16FMOD_SPEAKERMODEij
#[doc(alias = "FMOD::DSPI::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")]
pub fn stub_c4788() -> ! {
    todo!("0xc4788 FMOD::DSPI::read(float **,int *,unsigned int *,FMOD_SPEAKERMODE,int,unsigned int)")
}

// 0xd3978 — __ZN4FMOD15OutputCoreAudio19prepareAudioSessionE27FMOD_IPHONE_SESSIONCATEGORYbb
#[doc(alias = "FMOD::OutputCoreAudio::prepareAudioSession(FMOD_IPHONE_SESSIONCATEGORY,bool,bool)")]
pub fn stub_d3978() -> ! {
    todo!("0xd3978 FMOD::OutputCoreAudio::prepareAudioSession(FMOD_IPHONE_SESSIONCATEGORY,bool,bool)")
}

// 0xd5c8c — __ZN4FMOD15OutputWavWriter21getDriverNameCallbackEP17FMOD_OUTPUT_STATEiPci
#[doc(alias = "FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
pub fn stub_d5c8c() -> ! {
    todo!("0xd5c8c FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")
}

// 0xd6cb8 — __ZN4FMOD13PluginFactory9getOutputEjPPNS_26FMOD_OUTPUT_DESCRIPTION_EXE
#[doc(alias = "FMOD::PluginFactory::getOutput(unsigned int,FMOD::FMOD_OUTPUT_DESCRIPTION_EX **)")]
pub fn stub_d6cb8() -> ! {
    todo!("0xd6cb8 FMOD::PluginFactory::getOutput(unsigned int,FMOD::FMOD_OUTPUT_DESCRIPTION_EX **)")
}

// 0xd73c0 — __ZN4FMOD13PluginFactory11registerDSPEPNS_23FMOD_DSP_DESCRIPTION_EXEPj
#[doc(alias = "FMOD::PluginFactory::registerDSP(FMOD::FMOD_DSP_DESCRIPTION_EX *,unsigned int *)")]
pub fn stub_d73c0() -> ! {
    todo!("0xd73c0 FMOD::PluginFactory::registerDSP(FMOD::FMOD_DSP_DESCRIPTION_EX *,unsigned int *)")
}

// 0xdaad8 — __ZN4FMOD6Sample13setLoopPointsEjjjj
// type: _DWORD __fastcall(FMOD::Sample *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
#[doc(alias = "FMOD::Sample::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_daad8() -> ! {
    todo!("0xdaad8 FMOD::Sample::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0xe0804 — __ZN4FMOD6SoundI13setLoopPointsEjjjj
// type: _DWORD __fastcall(FMOD::SoundI *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
#[doc(alias = "FMOD::SoundI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_e0804() -> ! {
    todo!("0xe0804 FMOD::SoundI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0xe5f98 — __ZN4FMOD7SystemI12createSampleEjP21FMOD_CODEC_WAVEFORMATPPNS_6SampleE
#[doc(alias = "FMOD::SystemI::createSample(unsigned int,FMOD_CODEC_WAVEFORMAT *,FMOD::Sample **)")]
pub fn stub_e5f98() -> ! {
    todo!("0xe5f98 FMOD::SystemI::createSample(unsigned int,FMOD_CODEC_WAVEFORMAT *,FMOD::Sample **)")
}

// 0xe8024 — __ZN4FMOD7SystemI9playSoundE17FMOD_CHANNELINDEXPNS_6SoundIEbPPNS_8ChannelIE
#[doc(alias = "FMOD::SystemI::playSound(FMOD_CHANNELINDEX,FMOD::SoundI *,bool,FMOD::ChannelI **)")]
pub fn stub_e8024() -> ! {
    todo!("0xe8024 FMOD::SystemI::playSound(FMOD_CHANNELINDEX,FMOD::SoundI *,bool,FMOD::ChannelI **)")
}

// 0x10582c — __ZN4FMOD10DSPTremolo21getMemoryUsedCallbackEP14FMOD_DSP_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::DSPTremolo *this)
#[doc(alias = "FMOD::DSPTremolo::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_10582c() -> ! {
    todo!("0x10582c FMOD::DSPTremolo::getMemoryUsedCallback(FMOD_DSP_STATE *,FMOD::MemoryTracker *)")
}

// 0x106afc — __ZN4FMOD15CodecAudioQueue8fileReadExmPvPm
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, __int64, unsigned int, void *, unsigned int *)
#[doc(alias = "FMOD::CodecAudioQueue::fileRead(long long,unsigned long,void *,unsigned long *)")]
pub fn stub_106afc() -> ! {
    todo!("0x106afc FMOD::CodecAudioQueue::fileRead(long long,unsigned long,void *,unsigned long *)")
}

// 0x106c20 — __ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, OpaqueAudioQueue *, AudioQueueBuffer *)
#[doc(alias = "FMOD::CodecAudioQueue::processAudioQueue(OpaqueAudioQueue *,AudioQueueBuffer *)")]
pub fn stub_106c20() -> ! {
    todo!("0x106c20 FMOD::CodecAudioQueue::processAudioQueue(OpaqueAudioQueue *,AudioQueueBuffer *)")
}

// 0x3775f8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")]
pub fn stub_3775f8() -> ! {
    todo!("0x3775f8 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")
}

// 0x3779d8 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
pub fn stub_3779d8() -> ! {
    todo!("0x3779d8 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")
}

// 0x37fb78 — __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
pub fn stub_37fb78() -> ! {
    todo!("0x37fb78 RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")
}

// 0x7fd648 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::dispose(void)")]
pub fn stub_7fd648() {
    // IDA 0x7fd648: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf30584 — j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
pub fn stub_f30584() -> ! {
    todo!("0xf30584 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")
}

// 0xf30994 — j___ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
pub fn stub_f30994() -> ! {
    todo!("0xf30994 RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")
}

// 0x1dcee8 — _FT_List_Finalize
#[doc(alias = "_FT_List_Finalize")]
pub fn stub_1dcee8() -> ! {
    todo!("0x1dcee8 _FT_List_Finalize")
}

// 0x1dcf4c — _FT_Remove_Module
#[doc(alias = "_FT_Remove_Module")]
pub fn stub_1dcf4c() -> ! {
    todo!("0x1dcf4c _FT_Remove_Module")
}

// 0x1dd2dc — _destroy_face
#[doc(alias = "_destroy_face")]
pub fn stub_1dd2dc() -> ! {
    todo!("0x1dd2dc _destroy_face")
}

// 0x1dd3b4 — _FT_Done_Face
// type: int(void)
#[doc(alias = "_FT_Done_Face")]
pub fn stub_1dd3b4() -> ! {
    todo!("0x1dd3b4 _FT_Done_Face")
}

// 0x1dd428 — _FT_Done_Library
#[doc(alias = "_FT_Done_Library")]
pub fn stub_1dd428() -> ! {
    todo!("0x1dd428 _FT_Done_Library")
}

// 0x1dd4f0 — _ft_highpow2
#[doc(alias = "_ft_highpow2")]
pub fn stub_1dd4f0() -> ! {
    todo!("0x1dd4f0 _ft_highpow2")
}

// 0x1dd504 — _ft_mem_dup
#[doc(alias = "_ft_mem_dup")]
pub fn stub_1dd504() -> ! {
    todo!("0x1dd504 _ft_mem_dup")
}

// 0x1dd570 — _ft_mem_strdup
#[doc(alias = "_ft_mem_strdup")]
pub fn stub_1dd570() -> ! {
    todo!("0x1dd570 _ft_mem_strdup")
}

// 0x1dd5ac — _FT_Stream_ReadAt
// type: int __fastcall(int, int, void *__dst)
#[doc(alias = "_FT_Stream_ReadAt")]
pub fn stub_1dd5ac() -> ! {
    todo!("0x1dd5ac _FT_Stream_ReadAt")
}

// 0x1dd62c — _FT_Stream_Read
// type: int __fastcall(int, void *__dst)
#[doc(alias = "_FT_Stream_Read")]
pub fn stub_1dd62c() -> ! {
    todo!("0x1dd62c _FT_Stream_Read")
}

// 0x1dd640 — _FT_Raccess_Get_HeaderInfo
#[doc(alias = "_FT_Raccess_Get_HeaderInfo")]
pub fn stub_1dd640() -> ! {
    todo!("0x1dd640 _FT_Raccess_Get_HeaderInfo")
}

// 0x1dd958 — _ft_mem_alloc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_ft_mem_alloc")]
pub fn stub_1dd958() -> ! {
    todo!("0x1dd958 _ft_mem_alloc")
}

// 0x1dd9b4 — _ft_mem_qrealloc
#[doc(alias = "_ft_mem_qrealloc")]
pub fn stub_1dd9b4() -> ! {
    todo!("0x1dd9b4 _ft_mem_qrealloc")
}

// 0x1dda94 — _ft_mem_realloc
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_ft_mem_realloc")]
pub fn stub_1dda94() -> ! {
    todo!("0x1dda94 _ft_mem_realloc")
}

// 0x1ddb1c — _FT_GlyphLoader_CheckSubGlyphs
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_GlyphLoader_CheckSubGlyphs")]
pub fn stub_1ddb1c() -> ! {
    todo!("0x1ddb1c _FT_GlyphLoader_CheckSubGlyphs")
}

// 0x1ddba0 — _FT_GlyphLoader_CreateExtra
#[doc(alias = "_FT_GlyphLoader_CreateExtra")]
pub fn stub_1ddba0() -> ! {
    todo!("0x1ddba0 _FT_GlyphLoader_CreateExtra")
}

// 0x1ddc14 — _FT_New_Library
#[doc(alias = "_FT_New_Library")]
pub fn stub_1ddc14() -> ! {
    todo!("0x1ddc14 _FT_New_Library")
}

// 0x1ddcb8 — _FT_CMap_New
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_FT_CMap_New")]
pub fn stub_1ddcb8() -> ! {
    todo!("0x1ddcb8 _FT_CMap_New")
}

// 0x1dddc8 — _FT_New_Size
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_New_Size")]
pub fn stub_1dddc8() -> ! {
    todo!("0x1dddc8 _FT_New_Size")
}

// 0x1ddecc — _open_face
#[doc(alias = "_open_face")]
pub fn stub_1ddecc() -> ! {
    todo!("0x1ddecc _open_face")
}

// 0x1de154 — _ft_glyphslot_alloc_bitmap
// type: int __fastcall(_DWORD)
#[doc(alias = "_ft_glyphslot_alloc_bitmap")]
pub fn stub_1de154() -> ! {
    todo!("0x1de154 _ft_glyphslot_alloc_bitmap")
}

// 0x1de1bc — _FT_GlyphLoader_New
#[doc(alias = "_FT_GlyphLoader_New")]
pub fn stub_1de1bc() -> ! {
    todo!("0x1de1bc _FT_GlyphLoader_New")
}

// 0x1de1fc — _FT_New_GlyphSlot
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_New_GlyphSlot")]
pub fn stub_1de1fc() -> ! {
    todo!("0x1de1fc _FT_New_GlyphSlot")
}

// 0x1de34c — _FT_Request_Metrics
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_Request_Metrics")]
pub fn stub_1de34c() -> ! {
    todo!("0x1de34c _FT_Request_Metrics")
}

// 0x1de5c4 — _FT_Request_Size
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_Request_Size")]
pub fn stub_1de5c4() -> ! {
    todo!("0x1de5c4 _FT_Request_Size")
}

// 0x1de674 — _FT_Set_Char_Size
#[doc(alias = "_FT_Set_Char_Size")]
pub fn stub_1de674() -> ! {
    todo!("0x1de674 _FT_Set_Char_Size")
}

// 0x1de6f0 — _FT_Load_Glyph
#[doc(alias = "_FT_Load_Glyph")]
pub fn stub_1de6f0() -> ! {
    todo!("0x1de6f0 _FT_Load_Glyph")
}

// 0x1debcc — _FT_Load_Char
#[doc(alias = "_FT_Load_Char")]
pub fn stub_1debcc() -> ! {
    todo!("0x1debcc _FT_Load_Char")
}

// 0x1dec10 — _FT_Get_Advances
#[doc(alias = "_FT_Get_Advances")]
pub fn stub_1dec10() -> ! {
    todo!("0x1dec10 _FT_Get_Advances")
}

// 0x1def04 — _FT_Get_Advance
#[doc(alias = "_FT_Get_Advance")]
pub fn stub_1def04() -> ! {
    todo!("0x1def04 _FT_Get_Advance")
}

// 0x1defcc — _raccess_make_file_name
// type: int __fastcall(int, char *__s)
#[doc(alias = "_raccess_make_file_name")]
pub fn stub_1defcc() -> ! {
    todo!("0x1defcc _raccess_make_file_name")
}

// 0x1df090 — _raccess_guess_linux_cap
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_linux_cap")]
pub fn stub_1df090() -> ! {
    todo!("0x1df090 _raccess_guess_linux_cap")
}

// 0x1df0d0 — _raccess_guess_vfat
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_vfat")]
pub fn stub_1df0d0() -> ! {
    todo!("0x1df0d0 _raccess_guess_vfat")
}

// 0x1df110 — _FT_Raccess_Get_DataOffsets
#[doc(alias = "_FT_Raccess_Get_DataOffsets")]
pub fn stub_1df110() -> ! {
    todo!("0x1df110 _FT_Raccess_Get_DataOffsets")
}

// 0x1df4c8 — _FT_Get_Module
// type: int __fastcall(int, char *__s2)
#[doc(alias = "_FT_Get_Module")]
pub fn stub_1df4c8() -> ! {
    todo!("0x1df4c8 _FT_Get_Module")
}

// 0x1df620 — _FT_Get_Module_Interface
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_Get_Module_Interface")]
pub fn stub_1df620() -> ! {
    todo!("0x1df620 _FT_Get_Module_Interface")
}

// 0x1df63c — _FT_Add_Module
#[doc(alias = "_FT_Add_Module")]
pub fn stub_1df63c() -> ! {
    todo!("0x1df63c _FT_Add_Module")
}

// 0x1dfa3c — _ft_service_list_lookup
// type: int __fastcall(int, char *__s2)
#[doc(alias = "_ft_service_list_lookup")]
pub fn stub_1dfa3c() -> ! {
    todo!("0x1dfa3c _ft_service_list_lookup")
}

// 0x1dfa8c — _FT_Stream_New
#[doc(alias = "_FT_Stream_New")]
pub fn stub_1dfa8c() -> ! {
    todo!("0x1dfa8c _FT_Stream_New")
}

// 0x1dfb90 — _raccess_guess_linux_double_from_file_name
#[doc(alias = "_raccess_guess_linux_double_from_file_name")]
pub fn stub_1dfb90() -> ! {
    todo!("0x1dfb90 _raccess_guess_linux_double_from_file_name")
}

// 0x1dfc0c — _raccess_guess_linux_netatalk
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_linux_netatalk")]
pub fn stub_1dfc0c() -> ! {
    todo!("0x1dfc0c _raccess_guess_linux_netatalk")
}

// 0x1dfc7c — _raccess_guess_linux_double
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_linux_double")]
pub fn stub_1dfc7c() -> ! {
    todo!("0x1dfc7c _raccess_guess_linux_double")
}

// 0x1dfcec — _raccess_guess_darwin_ufs_export
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_darwin_ufs_export")]
pub fn stub_1dfcec() -> ! {
    todo!("0x1dfcec _raccess_guess_darwin_ufs_export")
}

// 0x1dfd5c — _FT_Open_Face
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_Open_Face")]
pub fn stub_1dfd5c() -> ! {
    todo!("0x1dfd5c _FT_Open_Face")
}

// 0x1e0798 — _open_face_from_buffer
// type: int __fastcall(int, int, int, int, char *__s2, int)
#[doc(alias = "_open_face_from_buffer")]
pub fn stub_1e0798() -> ! {
    todo!("0x1e0798 _open_face_from_buffer")
}

// 0x1e08a0 — _open_face_PS_from_sfnt_stream
#[doc(alias = "_open_face_PS_from_sfnt_stream")]
pub fn stub_1e08a0() -> ! {
    todo!("0x1e08a0 _open_face_PS_from_sfnt_stream")
}

// 0x1e0b4c — _IsMacResource
#[doc(alias = "_IsMacResource")]
pub fn stub_1e0b4c() -> ! {
    todo!("0x1e0b4c _IsMacResource")
}

// 0x1e12d4 — _FT_New_Memory_Face
#[doc(alias = "_FT_New_Memory_Face")]
pub fn stub_1e12d4() -> ! {
    todo!("0x1e12d4 _FT_New_Memory_Face")
}

// 0x1e1318 — _ft_validator_error
#[doc(alias = "_ft_validator_error")]
pub fn stub_1e1318() -> ! {
    todo!("0x1e1318 _ft_validator_error")
}

// 0x1e1330 — _FT_GlyphLoader_CheckPoints
#[doc(alias = "_FT_GlyphLoader_CheckPoints")]
pub fn stub_1e1330() -> ! {
    todo!("0x1e1330 _FT_GlyphLoader_CheckPoints")
}

// 0x1e14d8 — _FT_Stream_ReadFields
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_FT_Stream_ReadFields")]
pub fn stub_1e14d8() -> ! {
    todo!("0x1e14d8 _FT_Stream_ReadFields")
}

// 0x1e1704 — _FT_Stream_TryRead
// type: int __fastcall(int, void *__dst)
#[doc(alias = "_FT_Stream_TryRead")]
pub fn stub_1e1704() -> ! {
    todo!("0x1e1704 _FT_Stream_TryRead")
}

// 0x1e1780 — _raccess_guess_darwin_newvfs
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_darwin_newvfs")]
pub fn stub_1e1780() -> ! {
    todo!("0x1e1780 _raccess_guess_darwin_newvfs")
}

// 0x1e1814 — _raccess_guess_darwin_hfsplus
// type: int __fastcall(int, int, char *__s, int, int)
#[doc(alias = "_raccess_guess_darwin_hfsplus")]
pub fn stub_1e1814() -> ! {
    todo!("0x1e1814 _raccess_guess_darwin_hfsplus")
}

// 0x1e18a8 — _FT_GlyphLoader_CopyPoints
#[doc(alias = "_FT_GlyphLoader_CopyPoints")]
pub fn stub_1e18a8() -> ! {
    todo!("0x1e18a8 _FT_GlyphLoader_CopyPoints")
}

// 0x1e1978 — _FT_Done_FreeType
#[doc(alias = "_FT_Done_FreeType")]
pub fn stub_1e1978() -> ! {
    todo!("0x1e1978 _FT_Done_FreeType")
}

// 0x1e19a0 — _FT_Add_Default_Modules
#[doc(alias = "_FT_Add_Default_Modules")]
pub fn stub_1e19a0() -> ! {
    todo!("0x1e19a0 _FT_Add_Default_Modules")
}

// 0x1e19dc — _FT_Init_FreeType
#[doc(alias = "_FT_Init_FreeType")]
pub fn stub_1e19dc() -> ! {
    todo!("0x1e19dc _FT_Init_FreeType")
}

// 0x1e1a24 — _FT_Done_Memory
#[doc(alias = "_FT_Done_Memory")]
pub fn stub_1e1a24() -> ! {
    todo!("0x1e1a24 _FT_Done_Memory")
}

// 0x1e1a34 — _ft_free
// type: int __fastcall(int, void *)
#[doc(alias = "_ft_free")]
pub fn stub_1e1a34() -> ! {
    todo!("0x1e1a34 _ft_free")
}

// 0x1e1a48 — _FT_New_Memory
#[doc(alias = "_FT_New_Memory")]
pub fn stub_1e1a48() -> ! {
    todo!("0x1e1a48 _FT_New_Memory")
}

// 0x1e1a9c — _ft_alloc
// type: int __fastcall(int, size_t __size)
#[doc(alias = "_ft_alloc")]
pub fn stub_1e1a9c() -> ! {
    todo!("0x1e1a9c _ft_alloc")
}

// 0x1e1ab0 — _ft_realloc
// type: int __fastcall(int, int, size_t __size, void *__ptr)
#[doc(alias = "_ft_realloc")]
pub fn stub_1e1ab0() -> ! {
    todo!("0x1e1ab0 _ft_realloc")
}

// 0x1e1ac8 — _FT_Stream_Open
// type: int __fastcall(int, char *__filename)
#[doc(alias = "_FT_Stream_Open")]
pub fn stub_1e1ac8() -> ! {
    todo!("0x1e1ac8 _FT_Stream_Open")
}

// 0x1e1b5c — _ft_ansi_stream_close
#[doc(alias = "_ft_ansi_stream_close")]
pub fn stub_1e1b5c() -> ! {
    todo!("0x1e1b5c _ft_ansi_stream_close")
}

// 0x1e1b84 — _ft_ansi_stream_io
// type: int __fastcall(int, int, void *__ptr)
#[doc(alias = "_ft_ansi_stream_io")]
pub fn stub_1e1b84() -> ! {
    todo!("0x1e1b84 _ft_ansi_stream_io")
}

// 0x1e1bdc — __bdf_list_shift
#[doc(alias = "__bdf_list_shift")]
pub fn stub_1e1bdc() -> ! {
    todo!("0x1e1bdc __bdf_list_shift")
}

// 0x1e1c48 — __bdf_list_join
#[doc(alias = "__bdf_list_join")]
pub fn stub_1e1c48() -> ! {
    todo!("0x1e1c48 __bdf_list_join")
}

// 0x1e1d00 — __bdf_atoul
#[doc(alias = "__bdf_atoul")]
pub fn stub_1e1d00() -> ! {
    todo!("0x1e1d00 __bdf_atoul")
}

// 0x1e1de4 — __bdf_atol
#[doc(alias = "__bdf_atol")]
pub fn stub_1e1de4() -> ! {
    todo!("0x1e1de4 __bdf_atol")
}

// 0x1e1ee4 — __bdf_atos
// type: int __fastcall(char *, char **, int)
#[doc(alias = "__bdf_atos")]
pub fn stub_1e1ee4() -> ! {
    todo!("0x1e1ee4 __bdf_atos")
}

// 0x1e1ff0 — _by_encoding
#[doc(alias = "_by_encoding")]
pub fn stub_1e1ff0() -> ! {
    todo!("0x1e1ff0 _by_encoding")
}
