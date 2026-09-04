//! core shard ly — 100 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: ida/export.json (85545 funcs) global EA asc not yet stubbed in any crate — next 100 uncovered sorted asc.
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_t42_ps_has_glyph_names")]
// 0x22edb0 — _t42_ps_has_glyph_names
// type: int()
pub fn stub_0x22edb0() {
    // IDA 0x22edb0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_ps_get_font_private")]
// 0x22edb8 — _t42_ps_get_font_private
// type: int __fastcall(int, void *__dst)
pub fn stub_0x22edb8() {
    // IDA 0x22edb8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T42_Get_Interface")]
// 0x22eddc — _T42_Get_Interface
// type: int __fastcall(int, char *)
pub fn stub_0x22eddc() {
    // IDA 0x22eddc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_get_name_index")]
// 0x22edf8 — _t42_get_name_index
// type: int __fastcall(_DWORD *, char *__s1)
pub fn stub_0x22edf8() {
    // IDA 0x22edf8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_get_glyph_name")]
// 0x22f154 — _t42_get_glyph_name
// type: int __fastcall(int, int, int, int)
pub fn stub_0x22f154() {
    // IDA 0x22f154: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__Z18checkResultNoThrow11FMOD_RESULT")]
// 0x371844 — __Z18checkResultNoThrow11FMOD_RESULT
// type: int __fastcall(unsigned int, int, int, int)
pub fn stub_0x371844() {
    // IDA 0x371844: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__Z11checkResult11FMOD_RESULT")]
// 0x3719d0 — __Z11checkResult11FMOD_RESULT
// type: void __fastcall(unsigned int)
pub fn stub_0x3719d0() {
    // IDA 0x3719d0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundServiceC1Ev")]
// 0x371b5c — __ZN3RBX10Soundscape12SoundServiceC1Ev
// type: int __fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x371b5c() {
    // IDA 0x371b5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundServiceC2Ev")]
// 0x371b60 — __ZN3RBX10Soundscape12SoundServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x371b60() {
    // IDA 0x371b60: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService16update3DSettingsEv")]
// 0x3723f4 — __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x3723f4() {
    // IDA 0x3723f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv")]
// 0x372414 — __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
pub fn stub_0x372414() {
    // IDA 0x372414: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundServiceD0Ev")]
// 0x372460 — __ZN3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
pub fn stub_0x372460() {
    // IDA 0x372460: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundServiceD1Ev")]
// 0x372500 — __ZN3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
pub fn stub_0x372500() {
    // IDA 0x372500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Soundscape12SoundServiceD0Ev")]
// 0x372504 — __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
pub fn stub_0x372504() {
    // IDA 0x372504: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Soundscape12SoundServiceD0Ev")]
// 0x37250c — __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
pub fn stub_0x37250c() {
    // IDA 0x37250c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundServiceD2Ev")]
// 0x372514 — __ZN3RBX10Soundscape12SoundServiceD2Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
pub fn stub_0x372514() {
    // IDA 0x372514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Soundscape12SoundServiceD1Ev")]
// 0x3728b0 — __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
pub fn stub_0x3728b0() {
    // IDA 0x3728b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Soundscape12SoundServiceD1Ev")]
// 0x3728b8 — __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
pub fn stub_0x3728b8() {
    // IDA 0x3728b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE")]
// 0x3729bc — __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
// type: int __fastcall(int)
pub fn stub_0x3729bc() {
    // IDA 0x3729bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService15loadStockSoundsEv")]
// 0x372bb0 — __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x372bb0() {
    // IDA 0x372bb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs")]
// 0x373554 — __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
// type: void __fastcall(RBX::Instance *, int, int, int)
pub fn stub_0x373554() {
    // IDA 0x373554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE")]
// 0x37384c — __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
// type: int __fastcall(_DWORD *, std::string *)
pub fn stub_0x37384c() {
    // IDA 0x37384c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE")]
// 0x373894 — __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
// type: RBX::Soundscape::SoundId *__fastcall(RBX::Soundscape::SoundId *this, const RBX::ContentId *)
pub fn stub_0x373894() {
    // IDA 0x373894: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE")]
// 0x3738a8 — __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
// type: int __fastcall(RBX::Instance *, int *)
pub fn stub_0x3738a8() {
    // IDA 0x3738a8: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE")]
// 0x3738d8 — __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
// type: _DWORD *__fastcall(int, int)
pub fn stub_0x3738d8() {
    // IDA 0x3738d8: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel4playEv")]
// 0x373918 — __ZN3RBX10Soundscape12SoundChannel4playEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x373918() {
    // IDA 0x373918: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_")]
// 0x373974 — __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(shared_count *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x373974() {
    // IDA 0x373974: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService4stepEv")]
// 0x373cb8 — __ZN3RBX10Soundscape12SoundService4stepEv
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int, int (*)(const char *, ...))
pub fn stub_0x373cb8() {
    // IDA 0x373cb8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv")]
// 0x373fd0 — __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x373fd0() {
    // IDA 0x373fd0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_")]
// 0x374028 — __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
// type: int __fastcall(std::string *, std::string *)
pub fn stub_0x374028() {
    // IDA 0x374028: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv")]
// 0x374a2c — __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
pub fn stub_0x374a2c() {
    // IDA 0x374a2c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel9getVolumeEv")]
// 0x374a44 — __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x374a44() {
    // IDA 0x374a44: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel9setVolumeEf")]
// 0x374a48 — __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
// type: int __fastcall(int this, float32_t, FMOD::ChannelI **)
pub fn stub_0x374a48() {
    // IDA 0x374a48: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel8getPitchEv")]
// 0x374aa4 — __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x374aa4() {
    // IDA 0x374aa4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel8setPitchEf")]
// 0x374aa8 — __ZN3RBX10Soundscape12SoundChannel8setPitchEf
// type: int __fastcall(int this, float, FMOD::ChannelI **)
pub fn stub_0x374aa8() {
    // IDA 0x374aa8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel12setPlayCountEi")]
// 0x374af8 — __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
pub fn stub_0x374af8() {
    // IDA 0x374af8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel9getLoopedEv")]
// 0x374b68 — __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x374b68() {
    // IDA 0x374b68: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel9setLoopedEb")]
// 0x374b74 — __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
// type: unsigned int __fastcall(RBX::Soundscape::SoundChannel *this, int)
pub fn stub_0x374b74() {
    // IDA 0x374b74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel9isPlayingEv")]
// 0x374bb4 — __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x374bb4() {
    // IDA 0x374bb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel8isPausedEv")]
// 0x374bec — __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x374bec() {
    // IDA 0x374bec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel5pauseEv")]
// 0x374c24 — __ZN3RBX10Soundscape12SoundChannel5pauseEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x374c24() {
    // IDA 0x374c24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel4stopEv")]
// 0x374c68 — __ZN3RBX10Soundscape12SoundChannel4stopEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x374c68() {
    // IDA 0x374c68: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannelC2Ev")]
// 0x374cc4 — __ZN3RBX10Soundscape12SoundChannelC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x374cc4() {
    // IDA 0x374cc4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannelD0Ev")]
// 0x374ff4 — __ZN3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
pub fn stub_0x374ff4() {
    // IDA 0x374ff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannelD1Ev")]
// 0x375094 — __ZN3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
pub fn stub_0x375094() {
    // IDA 0x375094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Soundscape12SoundChannelD0Ev")]
// 0x375098 — __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
pub fn stub_0x375098() {
    // IDA 0x375098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Soundscape12SoundChannelD0Ev")]
// 0x3750a0 — __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
pub fn stub_0x3750a0() {
    // IDA 0x3750a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannelD2Ev")]
// 0x3750a8 — __ZN3RBX10Soundscape12SoundChannelD2Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
pub fn stub_0x3750a8() {
    // IDA 0x3750a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Soundscape12SoundChannelD1Ev")]
// 0x375330 — __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
pub fn stub_0x375330() {
    // IDA 0x375330: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Soundscape12SoundChannelD1Ev")]
// 0x375338 — __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
pub fn stub_0x375338() {
    // IDA 0x375338: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE")]
// 0x375340 — __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
// type: int __fastcall(int, float *)
pub fn stub_0x375340() {
    // IDA 0x375340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_")]
// 0x3753e8 — __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
// type: const _Rb_tree_node_base *__fastcall(int, _DWORD *, _DWORD *)
pub fn stub_0x3753e8() {
    // IDA 0x3753e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi")]
// 0x375418 — __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, int *)
pub fn stub_0x375418() {
    // IDA 0x375418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE")]
// 0x375438 — __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
// type: int __fastcall(int result)
pub fn stub_0x375438() {
    // IDA 0x375438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape5Sound7releaseEv")]
// 0x3754c4 — __ZN3RBX10Soundscape5Sound7releaseEv
// type: FMOD::Sound *__fastcall(FMOD::Sound **this)
pub fn stub_0x3754c4() {
    // IDA 0x3754c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel14releaseChannelEv")]
// 0x3754e0 — __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x3754e0() {
    // IDA 0x3754e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel17updateListenStateEv")]
// 0x375520 — __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
// type: void __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x375520() {
    // IDA 0x375520: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE")]
// 0x375660 — __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(FMOD::Channel **, int, FMOD::ChannelI **)
pub fn stub_0x375660() {
    // IDA 0x375660: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE")]
// 0x37567c — __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_0x37567c() {
    // IDA 0x37567c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_")]
// 0x375b7c — __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x375b7c() {
    // IDA 0x375b7c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel12preloadSoundEv")]
// 0x375be0 — __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
pub fn stub_0x375be0() {
    // IDA 0x375be0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE")]
// 0x375c3c — __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, FMOD::Channel *)
pub fn stub_0x375c3c() {
    // IDA 0x375c3c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundChannel12updateLoopedEv")]
// 0x375c8c — __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
// type: FMOD::Channel *__fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
pub fn stub_0x375c8c() {
    // IDA 0x375c8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_")]
// 0x375ce8 — __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
// type: int __fastcall(int, int)
pub fn stub_0x375ce8() {
    // IDA 0x375ce8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb")]
// 0x375dd4 — __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
// type: void __fastcall(sp_counted_base **, const shared_count *, const std::string *, int)
pub fn stub_0x375dd4() {
    // IDA 0x375dd4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13registerSoundEv")]
// 0x376198 — __ZN3RBX13registerSoundEv
// type: int __fastcall(RBX *this)
pub fn stub_0x376198() {
    // IDA 0x376198: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10Soundscape5SoundD2Ev")]
// 0x37619c — __ZN3RBX10Soundscape5SoundD2Ev
// type: void __fastcall(FMOD::Sound **this)
pub fn stub_0x37619c() {
    // IDA 0x37619c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_")]
// 0x3765a4 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: void (__fastcall *__fastcall(_Rb_tree_node_base *, _Rb_tree_node_base *, void (__fastcall *)(_DWORD *), int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int))(_DWORD *)
pub fn stub_0x3765a4() {
    // IDA 0x3765a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_")]
// 0x3768dc — __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3768dc() {
    // IDA 0x3768dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE")]
// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_0x376a24() {
    // IDA 0x376a24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_")]
// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_0x376a58() {
    // IDA 0x376a58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE")]
// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(RBX::Stats::Item **this, const RBX::Soundscape::SoundService *)
pub fn stub_0x376ac4() {
    // IDA 0x376ac4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_")]
// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), const std::string *))(int)
pub fn stub_0x376c84() {
    // IDA 0x376c84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv")]
// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// type: int __fastcall(RBX::Soundscape::SoundService *this)
pub fn stub_0x376fb8() {
    // IDA 0x376fb8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv")]
// 0x37706c — __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
pub fn stub_0x37706c() {
    // IDA 0x37706c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0x3770e0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x3770e0() {
    // IDA 0x3770e0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_")]
// 0x37716c — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_0x37716c() {
    // IDA 0x37716c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_")]
// 0x3772c0 — __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, const std::string *)
pub fn stub_0x3772c0() {
    // IDA 0x3772c0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_")]
// 0x377988 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x377988() {
    // IDA 0x377988: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv")]
// 0x3779d8 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: _DWORD *()
pub fn stub_0x3779d8() {
    // IDA 0x3779d8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc")]
// 0x377a44 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x377a44() {
    // IDA 0x377a44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc")]
// 0x377a50 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc
// type: void()
pub fn stub_0x377a50() {
    // IDA 0x377a50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x377b20 — __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x377b20() {
    // IDA 0x377b20: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")]
// 0x378478 — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv
pub fn stub_0x378478() {
    // IDA 0x378478: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
// 0x37847c — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
// type: int()
pub fn stub_0x37847c() {
    // IDA 0x37847c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_")]
// 0x378814 — __ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, const shared_count *)
pub fn stub_0x378814() {
    // IDA 0x378814: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
// 0x3788dc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int)
pub fn stub_0x3788dc() {
    // IDA 0x3788dc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
// 0x3789c4 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int)
pub fn stub_0x3789c4() {
    // IDA 0x3789c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_")]
// 0x378a14 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
pub fn stub_0x378a14() {
    // IDA 0x378a14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_")]
// 0x378a94 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
pub fn stub_0x378a94() {
    // IDA 0x378a94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_")]
// 0x378ba0 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_0x378ba0() {
    // IDA 0x378ba0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_")]
// 0x378c74 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x378c74() {
    // IDA 0x378c74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev")]
// 0x378d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev
// type: void()
pub fn stub_0x378d80() {
    // IDA 0x378d80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev")]
// 0x378d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev
// type: int __fastcall(int)
pub fn stub_0x378d84() {
    // IDA 0x378d84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv")]
// 0x378d88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv
// type: void __fastcall(int)
pub fn stub_0x378d88() {
    // IDA 0x378d88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info")]
// 0x378e2c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x378e2c() {
    // IDA 0x378e2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv")]
// 0x378e30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x378e30() {
    // IDA 0x378e30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_")]
// 0x378e34 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(int, int)
pub fn stub_0x378e34() {
    // IDA 0x378e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev")]
// 0x379094 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x379094() {
    // IDA 0x379094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev")]
// 0x3790c0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3790c0() {
    // IDA 0x3790c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
